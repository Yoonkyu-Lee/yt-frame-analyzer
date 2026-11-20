//! HTTP Range downloads with resumable sidecars and a single-stream fallback.

use reqwest::header::{ACCEPT_ENCODING, CONTENT_RANGE, COOKIE, RANGE, REFERER};
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::task::JoinSet;

type TransferResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const CONNECTIONS: usize = 8;
const SEGMENTS_PER_CONNECTION: usize = 2;
const MAX_RETRIES: usize = 4;
const RESUME_CHECKPOINT_BYTES: u64 = 1024 * 1024;
const CANCEL_SENTINEL: &str = "__DIST_DOWNLOAD_CANCELLED__";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

static CANCELLERS: LazyLock<Mutex<HashMap<String, Arc<AtomicBool>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn register_cancel(token: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    if let Ok(mut map) = CANCELLERS.lock() {
        map.insert(token.to_owned(), flag.clone());
    }
    flag
}

pub fn request_cancel(token: &str) -> bool {
    if let Ok(map) = CANCELLERS.lock() {
        if let Some(flag) = map.get(token) {
            flag.store(true, Ordering::SeqCst);
            return true;
        }
    }
    false
}

pub fn clear_cancel(token: &str) {
    if let Ok(mut map) = CANCELLERS.lock() {
        map.remove(token);
    }
}

#[derive(Debug, Clone)]
pub struct DownloadRequest {
    pub url: String,
    pub destination: PathBuf,
    pub cookie_header: Option<String>,
    pub referer: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DownloadMode {
    Segmented,
    Single,
}

#[derive(Debug, Clone, Copy)]
pub struct DownloadProgress {
    pub received: u64,
    pub total: Option<u64>,
    pub mode: DownloadMode,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ResumeMeta {
    version: u8,
    total: u64,
    segment_count: usize,
}

fn request(client: &Client, info: &DownloadRequest) -> reqwest::RequestBuilder {
    let mut builder = client.get(&info.url).header(ACCEPT_ENCODING, "identity");
    for (name, value) in [(COOKIE, &info.cookie_header), (REFERER, &info.referer)] {
        if let Some(value) = value.as_deref().filter(|s| !s.is_empty()) {
            builder = builder.header(name, value);
        }
    }
    builder
}

fn parse_content_range(value: &str) -> Option<u64> {
    value
        .strip_prefix("bytes 0-0/")?
        .parse()
        .ok()
        .filter(|n| *n > 0)
}

fn sidecar_path(destination: &Path, suffix: &str) -> PathBuf {
    let mut name = destination.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".{suffix}"));
    destination.with_file_name(name)
}

fn valid_resume_offset(saved: u64, segment_len: u64) -> u64 {
    if saved <= segment_len {
        saved
    } else {
        0
    }
}

fn segment_bounds(total: u64, count: usize, index: usize) -> (u64, u64) {
    let count = count as u64;
    let index = index as u64;
    let start = index * (total / count) + index.min(total % count);
    let len = total / count + u64::from(index < total % count);
    (start, start + len - 1)
}

async fn prepare_resume(
    part: &Path,
    meta: &Path,
    state: &Path,
    total: u64,
    segment_count: usize,
) -> TransferResult<Vec<u64>> {
    let expected = ResumeMeta {
        version: 1,
        total,
        segment_count,
    };
    let saved = fs::read(meta)
        .await
        .ok()
        .and_then(|bytes| serde_json::from_slice::<ResumeMeta>(&bytes).ok());
    let reusable = saved.as_ref() == Some(&expected)
        && fs::metadata(part).await.is_ok_and(|m| m.len() == total);
    if !reusable {
        // Invalidate old offsets before replacing the body or metadata.
        fs::write(state, vec![0; segment_count * 8]).await?;
        File::create(part).await?.set_len(total).await?;
        fs::write(meta, serde_json::to_vec(&expected)?).await?;
    }
    let raw = fs::read(state).await.unwrap_or_default();
    let offsets = (0..segment_count)
        .map(|index| {
            let saved = raw
                .get(index * 8..index * 8 + 8)
                .map(|bytes| u64::from_le_bytes(bytes.try_into().unwrap()))
                .unwrap_or(0);
            let (start, end) = segment_bounds(total, segment_count, index);
            valid_resume_offset(saved, end - start + 1)
        })
        .collect();
    Ok(offsets)
}

struct Transfer {
    client: Client,
    info: DownloadRequest,
    part: PathBuf,
    cancel: Arc<AtomicBool>,
    progress: Mutex<(DownloadProgress, Instant)>,
    callback: Box<dyn Fn(DownloadProgress) + Send + Sync>,
    state: Option<tokio::sync::Mutex<File>>,
    offsets: Vec<u64>,
}

impl Transfer {
    fn emit(&self, added: u64, force: bool) {
        // Serialize callbacks as well as updates so progress cannot go backwards.
        let mut progress = self.progress.lock().unwrap();
        progress.0.received += added;
        if force || progress.1.elapsed() >= Duration::from_millis(250) {
            progress.1 = Instant::now();
            (self.callback)(progress.0);
        }
    }

    fn check_cancel(&self) -> TransferResult<()> {
        if self.cancel.load(Ordering::Relaxed) {
            Err(CANCEL_SENTINEL.into())
        } else {
            Ok(())
        }
    }

    async fn checkpoint(&self, file: &mut File, index: usize, done: u64) -> TransferResult<()> {
        // Finish pending data writes before recording their resume offset.
        file.flush().await?;
        let mut state = self.state.as_ref().unwrap().lock().await;
        state
            .seek(std::io::SeekFrom::Start((index * 8) as u64))
            .await?;
        state.write_all(&done.to_le_bytes()).await?;
        state.flush().await?;
        Ok(())
    }

    async fn segment(&self, total: u64, index: usize) -> TransferResult<()> {
        self.check_cancel()?;
        let (first, end) = segment_bounds(total, self.offsets.len(), index);
        let len = end - first + 1;
        let mut done = self.offsets[index];
        if done == len {
            return Ok(());
        }
        let mut saved = done;
        let mut file = OpenOptions::new().write(true).open(&self.part).await?;
        for attempt in 0..MAX_RETRIES {
            self.check_cancel()?;
            let start = first + done;
            let result: TransferResult<()> = async {
                let mut response = request(&self.client, &self.info)
                    .header(RANGE, format!("bytes={start}-{end}"))
                    .send()
                    .await?;
                let expected = format!("bytes {start}-{end}/{total}");
                if response.status() != StatusCode::PARTIAL_CONTENT
                    || response
                        .headers()
                        .get(CONTENT_RANGE)
                        .and_then(|h| h.to_str().ok())
                        != Some(expected.as_str())
                    || response.content_length().is_some_and(|n| n != len - done)
                {
                    return Err(format!("Invalid Range response for segment {index}").into());
                }
                file.seek(std::io::SeekFrom::Start(start)).await?;
                while let Some(chunk) = response.chunk().await? {
                    self.check_cancel()?;
                    if chunk.len() as u64 > len - done {
                        return Err(format!("Segment {index} exceeds its expected size").into());
                    }
                    file.write_all(&chunk).await?;
                    done += chunk.len() as u64;
                    self.emit(chunk.len() as u64, false);
                    if done - saved >= RESUME_CHECKPOINT_BYTES && done < len {
                        self.checkpoint(&mut file, index, done).await?;
                        saved = done;
                    }
                }
                if done != len {
                    return Err(format!("Incomplete segment {index}: {done}/{len} bytes").into());
                }
                self.checkpoint(&mut file, index, done).await
            }
            .await;
            if result.is_ok() || done == len || attempt + 1 == MAX_RETRIES {
                return result;
            }
            self.check_cancel()?;
            tokio::time::sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
        }
        unreachable!()
    }

    async fn single(&self, mut response: Response) -> TransferResult<()> {
        self.check_cancel()?;
        for suffix in ["download.json", "download.state"] {
            match fs::remove_file(sidecar_path(&self.info.destination, suffix)).await {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e.into()),
                _ => {}
            }
        }
        let mut file = File::create(&self.part).await?;
        while let Some(chunk) = response.chunk().await? {
            self.check_cancel()?;
            file.write_all(&chunk).await?;
            self.emit(chunk.len() as u64, false);
        }
        self.check_cancel()?;
        file.flush().await?;
        Ok(())
    }
}

/// `Ok(Some(path))` means success, `Ok(None)` cancellation, and `Err` failure.
/// Segmented downloads retain sidecars on cancellation or failure for later resume.
pub async fn download<F>(
    request_info: DownloadRequest,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    let result: TransferResult<PathBuf> = async {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(20))
            .redirect(reqwest::redirect::Policy::limited(10))
            .referer(false)
            .user_agent(USER_AGENT)
            .build()?;
        let probe = request(&client, &request_info)
            .header(RANGE, "bytes=0-0")
            .send()
            .await?
            .error_for_status()?;
        let range_total = if probe.status() == StatusCode::PARTIAL_CONTENT {
            probe
                .headers()
                .get(CONTENT_RANGE)
                .and_then(|v| v.to_str().ok())
                .and_then(parse_content_range)
        } else {
            None
        };
        let mut total = if probe.status() == StatusCode::PARTIAL_CONTENT {
            range_total
        } else {
            probe.content_length()
        };
        drop(probe);
        let [part, meta, state_path] = ["download", "download.json", "download.state"]
            .map(|suffix| sidecar_path(&request_info.destination, suffix));
        let mut response = None;
        let mut offsets = Vec::new();
        let state = if let Some(total) = range_total {
            let count = total.min((CONNECTIONS * SEGMENTS_PER_CONNECTION) as u64) as usize;
            offsets = prepare_resume(&part, &meta, &state_path, total, count).await?;
            Some(tokio::sync::Mutex::new(
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(&state_path)
                    .await?,
            ))
        } else {
            let body = request(&client, &request_info)
                .send()
                .await?
                .error_for_status()?;
            total = body.content_length().or(total);
            response = Some(body);
            None
        };
        let transfer = Arc::new(Transfer {
            client,
            info: request_info,
            part,
            cancel,
            state,
            progress: Mutex::new((
                DownloadProgress {
                    received: offsets.iter().sum(),
                    total,
                    mode: if range_total.is_some() {
                        DownloadMode::Segmented
                    } else {
                        DownloadMode::Single
                    },
                },
                Instant::now(),
            )),
            callback: Box::new(on_progress),
            offsets,
        });
        transfer.emit(0, true);
        if let Some(total) = range_total {
            let mut next = 0;
            let mut workers = JoinSet::new();
            loop {
                while workers.len() < CONNECTIONS && next < transfer.offsets.len() {
                    let transfer = transfer.clone();
                    let index = next;
                    workers.spawn(async move { transfer.segment(total, index).await });
                    next += 1;
                }
                let Some(result) = workers.join_next().await else {
                    break;
                };
                if let Err(e) = result.map_err(Into::into).and_then(|r| r) {
                    workers.abort_all();
                    while workers.join_next().await.is_some() {}
                    return Err(e);
                }
            }
        } else {
            transfer.single(response.unwrap()).await?;
        }
        transfer.emit(0, true);
        transfer.check_cancel()?;
        let destination = transfer.info.destination.clone();
        let part = transfer.part.clone();
        drop(transfer);
        fs::rename(part, &destination).await?;
        let _ = fs::remove_file(meta).await;
        let _ = fs::remove_file(state_path).await;
        Ok(destination)
    }
    .await;
    match result {
        Ok(path) => Ok(Some(path)),
        Err(e) if e.to_string() == CANCEL_SENTINEL => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::atomic::AtomicUsize;

    fn spawn_test_server(
        data: Vec<u8>,
        supports_range: bool,
        expected_requests: usize,
    ) -> (String, Arc<AtomicUsize>, std::thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let requests = Arc::new(AtomicUsize::new(0));
        let request_count = requests.clone();
        let handle = std::thread::spawn(move || {
            for stream in listener.incoming().take(expected_requests) {
                let mut stream = stream.unwrap();
                let mut buffer = vec![0u8; 8192];
                let size = stream.read(&mut buffer).unwrap();
                let headers = String::from_utf8_lossy(&buffer[..size]);
                request_count.fetch_add(1, Ordering::Relaxed);

                let range = headers
                    .lines()
                    .find(|line| line.to_ascii_lowercase().starts_with("range: bytes="))
                    .and_then(|line| line.split_once('=').map(|(_, value)| value.trim()))
                    .and_then(|value| value.split_once('-'))
                    .and_then(|(start, end)| {
                        Some((start.parse::<usize>().ok()?, end.parse::<usize>().ok()?))
                    });
                let (status, body, extra_header) = if supports_range {
                    let (start, end) = range.unwrap();
                    (
                        "206 Partial Content",
                        &data[start..=end],
                        format!("Content-Range: bytes {start}-{end}/{}\r\n", data.len()),
                    )
                } else {
                    ("200 OK", data.as_slice(), String::new())
                };
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\n{extra_header}\
                     Connection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(body).unwrap();
            }
        });
        (format!("http://{address}/file.bin"), requests, handle)
    }

    fn no_cancel() -> Arc<AtomicBool> {
        Arc::new(AtomicBool::new(false))
    }

    fn test_runtime() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap()
    }

    fn unique_test_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "vnlib-segmented-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn parses_content_range_total() {
        assert_eq!(
            parse_content_range("bytes 0-0/4722087959"),
            Some(4_722_087_959)
        );
        assert_eq!(parse_content_range("bytes 0-0/*"), None);
        assert_eq!(parse_content_range("invalid"), None);
    }

    #[test]
    fn resume_offset_rejects_corrupt_over_length_value() {
        assert_eq!(valid_resume_offset(0, 10), 0);
        assert_eq!(valid_resume_offset(5, 10), 5);
        assert_eq!(valid_resume_offset(10, 10), 10); // 정확히 완료는 유효
        assert_eq!(valid_resume_offset(11, 10), 0); // 길이 초과 = 손상 → 재다운로드
        assert_eq!(valid_resume_offset(u64::MAX, 10), 0);
    }

    #[test]
    fn segment_bounds_cover_file_without_overlap() {
        let total = 101;
        let count = 16;
        let mut next = 0;
        for index in 0..count {
            let (start, end) = segment_bounds(total, count, index);
            assert_eq!(start, next);
            assert!(end >= start);
            next = end + 1;
        }
        assert_eq!(next, total);
    }

    #[test]
    fn range_server_is_downloaded_in_parallel_without_corruption() {
        let data: Vec<u8> = (0..65_537).map(|i| (i % 251) as u8).collect();
        let expected_requests = 1 + CONNECTIONS * SEGMENTS_PER_CONNECTION;
        let (url, requests, server) = spawn_test_server(data.clone(), true, expected_requests);
        let dir = unique_test_dir("range");
        let destination = dir.join("range.bin");

        let result = test_runtime().block_on(download(
            DownloadRequest {
                url,
                destination: destination.clone(),
                cookie_header: None,
                referer: None,
            },
            no_cancel(),
            |_| {},
        ));

        assert_eq!(result.unwrap(), Some(destination.clone()));
        assert_eq!(std::fs::read(&destination).unwrap(), data);
        assert_eq!(requests.load(Ordering::Relaxed), expected_requests);
        server.join().unwrap();
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn preset_cancel_stops_and_keeps_sidecars_for_resume() {
        // 취소 플래그가 켜진 채 시작하면 즉시 Ok(None)이고, 재개용 sidecar가 남는다.
        let data: Vec<u8> = (0..65_537).map(|i| (i % 251) as u8).collect();
        // 서버는 프로브(1)만 받는다 — 취소로 구간 요청은 나가지 않는다.
        let (url, requests, server) = spawn_test_server(data.clone(), true, 1);
        let dir = unique_test_dir("cancel");
        let destination = dir.join("cancel.bin");

        let result = test_runtime().block_on(download(
            DownloadRequest {
                url,
                destination: destination.clone(),
                cookie_header: None,
                referer: None,
            },
            Arc::new(AtomicBool::new(true)),
            |_| {},
        ));

        assert_eq!(result.unwrap(), None, "취소는 Ok(None)");
        assert!(!destination.exists(), "취소 시 최종 파일은 없어야 한다");
        assert!(
            sidecar_path(&destination, "download").exists(),
            "재개용 sidecar 유지"
        );
        assert_eq!(
            requests.load(Ordering::Relaxed),
            1,
            "구간 요청 없이 프로브만"
        );
        server.join().unwrap();
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn server_without_range_support_falls_back_to_single_stream() {
        let data: Vec<u8> = (0..4097).map(|i| (i % 239) as u8).collect();
        let (url, requests, server) = spawn_test_server(data.clone(), false, 2);
        let dir = unique_test_dir("single");
        let destination = dir.join("single.bin");
        let modes = Arc::new(std::sync::Mutex::new(Vec::new()));
        let progress_modes = modes.clone();

        let result = test_runtime().block_on(download(
            DownloadRequest {
                url,
                destination: destination.clone(),
                cookie_header: None,
                referer: None,
            },
            no_cancel(),
            move |progress| progress_modes.lock().unwrap().push(progress.mode),
        ));

        assert!(result.is_ok());
        assert_eq!(std::fs::read(&destination).unwrap(), data);
        assert_eq!(requests.load(Ordering::Relaxed), 2);
        assert!(modes.lock().unwrap().contains(&DownloadMode::Single));
        server.join().unwrap();
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn resume_skips_completed_segments_and_finishes_correctly() {
        let data: Vec<u8> = (0..65_537).map(|i| (i % 251) as u8).collect();
        let total = data.len() as u64;
        let segment_count = (CONNECTIONS * SEGMENTS_PER_CONNECTION)
            .min(data.len())
            .max(1);

        let dir = unique_test_dir("resume");
        let destination = dir.join("resume.bin");
        let part = sidecar_path(&destination, "download");
        let meta = sidecar_path(&destination, "download.json");
        let state = sidecar_path(&destination, "download.state");

        // 전체 크기의 임시 본체를 0으로 만들고, 완료로 표시할 부분만 올바른 바이트로 채운다.
        // 구간 7은 완전 완료, 구간 3은 절반 완료, 나머지는 미착수로 심는다.
        let mut body = vec![0u8; data.len()];
        let mut state_bytes = vec![0u8; segment_count * 8];
        for index in 0..segment_count {
            let (start, end) = segment_bounds(total, segment_count, index);
            let len = end - start + 1;
            let done = match index {
                7 => len,
                3 => len / 2,
                _ => 0,
            };
            let s = start as usize;
            let d = done as usize;
            body[s..s + d].copy_from_slice(&data[s..s + d]);
            state_bytes[index * 8..index * 8 + 8].copy_from_slice(&done.to_le_bytes());
        }
        std::fs::write(&part, &body).unwrap();
        std::fs::write(
            &meta,
            serde_json::to_vec(&ResumeMeta {
                version: 1,
                total,
                segment_count,
            })
            .unwrap(),
        )
        .unwrap();
        std::fs::write(&state, &state_bytes).unwrap();

        // 완전 완료된 구간 7은 요청이 아예 없어야 한다 → 프로브 1 + (구간 수 - 1).
        // 재개가 완료 구간을 건너뛰지 못하면 요청이 하나 더 발생한다.
        let expected_requests = 1 + segment_count - 1;
        let (url, requests, server) = spawn_test_server(data.clone(), true, expected_requests);

        let result = test_runtime().block_on(download(
            DownloadRequest {
                url,
                destination: destination.clone(),
                cookie_header: None,
                referer: None,
            },
            no_cancel(),
            |_| {},
        ));

        assert_eq!(result.unwrap(), Some(destination.clone()));
        assert_eq!(std::fs::read(&destination).unwrap(), data);
        assert_eq!(requests.load(Ordering::Relaxed), expected_requests);
        assert!(!meta.exists() && !state.exists(), "완료 후 sidecar 정리");
        server.join().unwrap();
        std::fs::remove_dir_all(dir).ok();
    }
}
