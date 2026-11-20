//! HTTP Range 기반 분할 다운로드 백엔드.
//!
//! 배포 사이트 자동화는 `dist_host.rs`가 담당하고, 이 모듈은 최종 파일 URL을 받은 뒤의
//! 전송·재시도·재개만 담당한다. 서버가 Range를 지원하지 않으면 단일 스트림으로 폴백한다.

use reqwest::header::{ACCEPT_ENCODING, CONTENT_LENGTH, CONTENT_RANGE, COOKIE, RANGE, REFERER};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::task::JoinSet;

/// 취소는 협조적이다: 실행 중인 전송이 이 sentinel을 Err로 올리면 `download`가
/// 실패가 아니라 취소(`Ok(None)`)로 변환한다. sidecar는 남겨 재개할 수 있게 한다.
const CANCEL_SENTINEL: &str = "__DIST_DOWNLOAD_CANCELLED__";

/// 진행 중인 다운로드의 취소 플래그 레지스트리(토큰 → 플래그).
/// 다운로드 task는 `dist_host`가 spawn하므로, 취소 커맨드가 토큰으로 찾아 신호할 수 있게
/// 한 곳에 모은다.
static CANCELLERS: LazyLock<std::sync::Mutex<HashMap<String, Arc<AtomicBool>>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// 토큰에 대한 취소 플래그를 만들어 등록하고 돌려준다.
pub fn register_cancel(token: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    if let Ok(mut map) = CANCELLERS.lock() {
        map.insert(token.to_string(), flag.clone());
    }
    flag
}

/// 해당 토큰의 다운로드에 취소를 신호한다. 등록된 게 있으면 true.
pub fn request_cancel(token: &str) -> bool {
    if let Ok(map) = CANCELLERS.lock() {
        if let Some(flag) = map.get(token) {
            flag.store(true, Ordering::SeqCst);
            return true;
        }
    }
    false
}

/// 완료·실패·취소로 끝난 토큰의 플래그를 제거한다.
pub fn clear_cancel(token: &str) {
    if let Ok(mut map) = CANCELLERS.lock() {
        map.remove(token);
    }
}

fn cancelled(flag: &AtomicBool) -> bool {
    flag.load(Ordering::Relaxed)
}

const CONNECTIONS: usize = 8;
const SEGMENTS_PER_CONNECTION: usize = 2;
const MAX_RETRIES: usize = 4;
const PROGRESS_EMIT_INTERVAL: Duration = Duration::from_millis(250);
const RESUME_CHECKPOINT_BYTES: u64 = 1024 * 1024;
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
     (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

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

#[derive(Debug, Serialize, Deserialize)]
struct ResumeMeta {
    version: u8,
    total: u64,
    segment_count: usize,
}

struct ProgressEmitter {
    received: Arc<AtomicU64>,
    total: Option<u64>,
    mode: DownloadMode,
    last_emit_ms: AtomicU64,
    started: Instant,
    callback: Arc<dyn Fn(DownloadProgress) + Send + Sync>,
}

impl ProgressEmitter {
    fn new(
        received: Arc<AtomicU64>,
        total: Option<u64>,
        mode: DownloadMode,
        callback: Arc<dyn Fn(DownloadProgress) + Send + Sync>,
    ) -> Self {
        Self {
            received,
            total,
            mode,
            last_emit_ms: AtomicU64::new(0),
            started: Instant::now(),
            callback,
        }
    }

    fn emit_if_due(&self) {
        let elapsed = self.started.elapsed().as_millis().min(u64::MAX as u128) as u64;
        let last = self.last_emit_ms.load(Ordering::Relaxed);
        if elapsed.saturating_sub(last) < PROGRESS_EMIT_INTERVAL.as_millis() as u64 {
            return;
        }
        if self
            .last_emit_ms
            .compare_exchange(last, elapsed, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            self.emit();
        }
    }

    fn emit(&self) {
        (self.callback)(DownloadProgress {
            received: self.received.load(Ordering::Relaxed),
            total: self.total,
            mode: self.mode,
        });
    }
}

fn client() -> Result<Client, String> {
    Client::builder()
        .connect_timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::limited(10))
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| format!("HTTP 클라이언트 생성 실패: {e}"))
}

fn request(
    client: &Client,
    method: reqwest::Method,
    request: &DownloadRequest,
) -> reqwest::RequestBuilder {
    let mut builder = client
        .request(method, &request.url)
        // 압축된 응답은 바이트 오프셋과 실제 파일 오프셋을 다르게 만들 수 있다.
        .header(ACCEPT_ENCODING, "identity");
    if let Some(cookie) = request.cookie_header.as_deref().filter(|v| !v.is_empty()) {
        builder = builder.header(COOKIE, cookie);
    }
    if let Some(referer) = request.referer.as_deref().filter(|v| !v.is_empty()) {
        builder = builder.header(REFERER, referer);
    }
    builder
}

fn parse_content_range(value: &str) -> Option<u64> {
    // `bytes 0-0/4722087959`; 크기를 모르는 `bytes 0-0/*`는 거부한다.
    let (_, total) = value.rsplit_once('/')?;
    total.parse::<u64>().ok().filter(|n| *n > 0)
}

async fn probe_range(
    client: &Client,
    request_info: &DownloadRequest,
) -> Result<(bool, Option<u64>), String> {
    let response = request(client, reqwest::Method::GET, request_info)
        .header(RANGE, "bytes=0-0")
        .send()
        .await
        .map_err(|e| format!("다운로드 서버 연결 실패: {e}"))?;

    if response.status() == StatusCode::PARTIAL_CONTENT {
        let total = response
            .headers()
            .get(CONTENT_RANGE)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_content_range);
        return Ok((total.is_some(), total));
    }

    if response.status().is_success() {
        let total = response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|n| *n > 0);
        return Ok((false, total));
    }

    Err(format!(
        "다운로드 서버가 요청을 거부했습니다: HTTP {}",
        response.status()
    ))
}

fn sidecar_path(destination: &Path, suffix: &str) -> PathBuf {
    let name = destination
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "download".to_string());
    destination.with_file_name(format!("{name}.{suffix}"))
}


/// 저장된 구간 오프셋을 검증한다. 정상 checkpoint는 `completed`(≤ 구간 길이)만 기록하므로
/// 구간 길이를 넘는 값은 손상(예: 하드 크래시 중 상태 파일 torn-write)이다.
/// 그런 값을 완료로 오인하면 미수신 바이트를 건너뛰어 파일이 조용히 손상되므로,
/// 처음부터 다시 받도록 0으로 되돌린다.
fn valid_resume_offset(saved: u64, segment_len: u64) -> u64 {
    if saved <= segment_len {
        saved
    } else {
        0
    }
}

fn segment_bounds(total: u64, count: usize, index: usize) -> (u64, u64) {
    let count = count.max(1) as u64;
    let index = index as u64;
    let base = total / count;
    let remainder = total % count;
    // 나머지 바이트를 앞쪽 구간에 하나씩 배분하면 모든 구간이 빈틈·중복 없이 이어진다.
    let start = index * base + index.min(remainder);
    let size = base + u64::from(index < remainder);
    let end = start.saturating_add(size).saturating_sub(1);
    (start, end)
}

async fn prepare_resume(
    destination: &Path,
    total: u64,
    segment_count: usize,
) -> Result<(PathBuf, PathBuf, PathBuf, Vec<u64>), String> {
    let part_path = sidecar_path(destination, "download");
    let meta_path = sidecar_path(destination, "download.json");
    let state_path = sidecar_path(destination, "download.state");
    let expected = ResumeMeta {
        version: 1,
        total,
        segment_count,
    };

    let existing_meta = tokio::fs::read_to_string(&meta_path)
        .await
        .ok()
        .and_then(|s| serde_json::from_str::<ResumeMeta>(&s).ok());
    let reusable = existing_meta
        .as_ref()
        .is_some_and(|m| m.version == 1 && m.total == total && m.segment_count == segment_count)
        && tokio::fs::metadata(&part_path)
            .await
            .is_ok_and(|m| m.len() == total);

    if !reusable {
        for path in [&part_path, &meta_path, &state_path] {
            if let Err(e) = tokio::fs::remove_file(path).await {
                if e.kind() != std::io::ErrorKind::NotFound {
                    return Err(format!("이전 임시 파일 정리 실패({}): {e}", path.display()));
                }
            }
        }
        let file = tokio::fs::File::create(&part_path)
            .await
            .map_err(|e| format!("임시 파일 생성 실패({}): {e}", part_path.display()))?;
        file.set_len(total)
            .await
            .map_err(|e| format!("임시 파일 크기 설정 실패: {e}"))?;
        tokio::fs::write(
            &meta_path,
            serde_json::to_vec(&expected).map_err(|e| format!("재개 정보 생성 실패: {e}"))?,
        )
        .await
        .map_err(|e| format!("재개 정보 저장 실패: {e}"))?;
        tokio::fs::write(&state_path, vec![0u8; segment_count * 8])
            .await
            .map_err(|e| format!("구간 상태 생성 실패: {e}"))?;
    }

    let raw_state = tokio::fs::read(&state_path).await.unwrap_or_default();
    let mut offsets = Vec::with_capacity(segment_count);
    for index in 0..segment_count {
        let pos = index * 8;
        let value = raw_state
            .get(pos..pos + 8)
            .and_then(|bytes| bytes.try_into().ok())
            .map(u64::from_le_bytes)
            .unwrap_or(0);
        let (start, end) = segment_bounds(total, segment_count, index);
        let segment_len = end.saturating_sub(start).saturating_add(1);
        offsets.push(valid_resume_offset(value, segment_len));
    }
    Ok((part_path, meta_path, state_path, offsets))
}

async fn download_segment(
    client: &Client,
    request_info: &DownloadRequest,
    part_path: &Path,
    state_file: Arc<Mutex<tokio::fs::File>>,
    progress: Arc<ProgressEmitter>,
    cancel: Arc<AtomicBool>,
    segment_count: usize,
    index: usize,
    initial_offset: u64,
) -> Result<(), String> {
    let total = progress
        .total
        .expect("분할 다운로드는 전체 크기가 필요하다");
    let (segment_start, segment_end) = segment_bounds(total, segment_count, index);
    let segment_len = segment_end.saturating_sub(segment_start).saturating_add(1);
    if initial_offset >= segment_len {
        return Ok(());
    }

    let mut completed = initial_offset;
    let mut checkpointed = initial_offset;
    for attempt in 0..MAX_RETRIES {
        if cancelled(&cancel) {
            return Err(CANCEL_SENTINEL.to_string());
        }
        let start = segment_start + completed;
        let mut response = match request(client, reqwest::Method::GET, request_info)
            .header(RANGE, format!("bytes={start}-{segment_end}"))
            .send()
            .await
        {
            Ok(response) if response.status() == StatusCode::PARTIAL_CONTENT => response,
            Ok(response) => {
                return Err(format!(
                    "Range 응답이 아닙니다(구간 {index}, HTTP {})",
                    response.status()
                ))
            }
            Err(e) if attempt + 1 < MAX_RETRIES => {
                tokio::time::sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
                eprintln!("[segmented-download] 구간 {index} 재시도: {e}");
                continue;
            }
            Err(e) => return Err(format!("구간 {index} 다운로드 실패: {e}")),
        };
        let expected_range_prefix = format!("bytes {start}-");
        let valid_range = response
            .headers()
            .get(CONTENT_RANGE)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.starts_with(&expected_range_prefix));
        if !valid_range {
            return Err(format!(
                "구간 {index}의 Content-Range가 요청 범위와 다릅니다"
            ));
        }

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .open(part_path)
            .await
            .map_err(|e| format!("임시 파일 열기 실패: {e}"))?;
        file.seek(std::io::SeekFrom::Start(start))
            .await
            .map_err(|e| format!("구간 {index} seek 실패: {e}"))?;

        let transfer = async {
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|e| format!("구간 {index} 수신 실패: {e}"))?
            {
                if cancelled(&cancel) {
                    return Err(CANCEL_SENTINEL.to_string());
                }
                let remaining = segment_len.saturating_sub(completed);
                if chunk.len() as u64 > remaining {
                    return Err(format!("구간 {index}이 예정 크기를 초과했습니다"));
                }
                file.write_all(&chunk)
                    .await
                    .map_err(|e| format!("구간 {index} 쓰기 실패: {e}"))?;
                completed += chunk.len() as u64;
                progress
                    .received
                    .fetch_add(chunk.len() as u64, Ordering::Relaxed);
                progress.emit_if_due();

                if completed.saturating_sub(checkpointed) >= RESUME_CHECKPOINT_BYTES
                    || completed == segment_len
                {
                    let mut state = state_file.lock().await;
                    state
                        .seek(std::io::SeekFrom::Start((index * 8) as u64))
                        .await
                        .map_err(|e| format!("구간 상태 seek 실패: {e}"))?;
                    state
                        .write_all(&completed.to_le_bytes())
                        .await
                        .map_err(|e| format!("구간 상태 저장 실패: {e}"))?;
                    checkpointed = completed;
                }
            }
            file.flush()
                .await
                .map_err(|e| format!("구간 {index} flush 실패: {e}"))?;
            Ok::<(), String>(())
        }
        .await;

        match transfer {
            Ok(()) if completed == segment_len => return Ok(()),
            Ok(()) if attempt + 1 < MAX_RETRIES => {
                tokio::time::sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
            }
            Ok(()) => {
                return Err(format!(
                    "구간 {index} 크기 불일치: {completed}/{segment_len} bytes"
                ))
            }
            // 취소는 재시도하지 않고 즉시 올린다.
            Err(e) if e == CANCEL_SENTINEL => return Err(e),
            Err(e) if attempt + 1 < MAX_RETRIES => {
                tokio::time::sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
                eprintln!("[segmented-download] 구간 {index} 재시도: {e}");
            }
            Err(e) => return Err(e),
        }
    }
    Err(format!("구간 {index} 재시도 횟수 초과"))
}

async fn segmented<F>(
    client: Client,
    request_info: DownloadRequest,
    total: u64,
    cancel: Arc<AtomicBool>,
    callback: Arc<F>,
) -> Result<PathBuf, String>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    let segment_count = (CONNECTIONS * SEGMENTS_PER_CONNECTION)
        .min(usize::try_from(total.max(1)).unwrap_or(usize::MAX))
        .max(1);
    let (part_path, meta_path, state_path, offsets) =
        prepare_resume(&request_info.destination, total, segment_count).await?;
    let resumed = offsets.iter().copied().sum::<u64>().min(total);
    let received = Arc::new(AtomicU64::new(resumed));
    let callback: Arc<dyn Fn(DownloadProgress) + Send + Sync> = callback;
    let progress = Arc::new(ProgressEmitter::new(
        received,
        Some(total),
        DownloadMode::Segmented,
        callback,
    ));
    progress.emit();

    let state_file = Arc::new(Mutex::new(
        tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&state_path)
            .await
            .map_err(|e| format!("구간 상태 열기 실패: {e}"))?,
    ));
    let next = Arc::new(AtomicUsize::new(0));
    let offsets = Arc::new(offsets);
    let mut workers = JoinSet::new();

    for _ in 0..CONNECTIONS.min(segment_count) {
        let client = client.clone();
        let request_info = request_info.clone();
        let part_path = part_path.clone();
        let state_file = state_file.clone();
        let progress = progress.clone();
        let next = next.clone();
        let offsets = offsets.clone();
        let cancel = cancel.clone();
        workers.spawn(async move {
            loop {
                if cancelled(&cancel) {
                    return Err::<(), String>(CANCEL_SENTINEL.to_string());
                }
                let index = next.fetch_add(1, Ordering::Relaxed);
                if index >= segment_count {
                    return Ok::<(), String>(());
                }
                download_segment(
                    &client,
                    &request_info,
                    &part_path,
                    state_file.clone(),
                    progress.clone(),
                    cancel.clone(),
                    segment_count,
                    index,
                    offsets[index],
                )
                .await?;
            }
        });
    }

    while let Some(result) = workers.join_next().await {
        let joined = result.map_err(|e| format!("다운로드 작업 실행 실패: {e}"))?;
        if let Err(e) = joined {
            // 취소 sentinel은 감싸지 않고 그대로 올려 `download`가 Ok(None)으로 바꾸게 한다.
            if e == CANCEL_SENTINEL {
                return Err(CANCEL_SENTINEL.to_string());
            }
            return Err(format!("분할 다운로드 실패: {e}"));
        }
    }

    progress.received.store(total, Ordering::Relaxed);
    progress.emit();
    tokio::fs::rename(&part_path, &request_info.destination)
        .await
        .map_err(|e| {
            format!(
                "완료 파일 이동 실패({}): {e}",
                request_info.destination.display()
            )
        })?;
    let _ = tokio::fs::remove_file(meta_path).await;
    let _ = tokio::fs::remove_file(state_path).await;
    Ok(request_info.destination)
}

async fn single<F>(
    client: Client,
    request_info: DownloadRequest,
    expected_total: Option<u64>,
    cancel: Arc<AtomicBool>,
    callback: Arc<F>,
) -> Result<PathBuf, String>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    let part_path = sidecar_path(&request_info.destination, "download");
    let mut response = request(&client, reqwest::Method::GET, &request_info)
        .send()
        .await
        .map_err(|e| format!("단일 다운로드 연결 실패: {e}"))?;
    if !response.status().is_success() {
        return Err(format!("다운로드 서버 오류: HTTP {}", response.status()));
    }
    let total = response.content_length().or(expected_total);
    let received = Arc::new(AtomicU64::new(0));
    let callback: Arc<dyn Fn(DownloadProgress) + Send + Sync> = callback;
    let progress = ProgressEmitter::new(received, total, DownloadMode::Single, callback);
    progress.emit();

    let mut file = tokio::fs::File::create(&part_path)
        .await
        .map_err(|e| format!("임시 파일 생성 실패({}): {e}", part_path.display()))?;
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|e| format!("다운로드 수신 실패: {e}"))?
    {
        if cancelled(&cancel) {
            return Err(CANCEL_SENTINEL.to_string());
        }
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("파일 쓰기 실패: {e}"))?;
        progress
            .received
            .fetch_add(chunk.len() as u64, Ordering::Relaxed);
        progress.emit_if_due();
    }
    file.flush()
        .await
        .map_err(|e| format!("파일 flush 실패: {e}"))?;
    progress.emit();
    tokio::fs::rename(&part_path, &request_info.destination)
        .await
        .map_err(|e| {
            format!(
                "완료 파일 이동 실패({}): {e}",
                request_info.destination.display()
            )
        })?;
    Ok(request_info.destination)
}

/// 최종 URL을 받아 파일을 전송한다. `Ok(Some(path))` 성공, `Ok(None)` 취소, `Err` 실패.
/// 취소 시 sidecar를 남겨 다음 실행에서 재개할 수 있게 한다.
pub async fn download<F>(
    request_info: DownloadRequest,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    let client = client()?;
    let (range_supported, total) = probe_range(&client, &request_info).await?;
    let callback = Arc::new(on_progress);
    let result = match (range_supported, total) {
        (true, Some(total)) => segmented(client, request_info, total, cancel, callback).await,
        _ => single(client, request_info, total, cancel, callback).await,
    };
    match result {
        Ok(path) => Ok(Some(path)),
        Err(e) if e == CANCEL_SENTINEL => Ok(None),
        Err(e) => Err(e),
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
        assert_eq!(requests.load(Ordering::Relaxed), 1, "구간 요청 없이 프로브만");
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
        let segment_count = (CONNECTIONS * SEGMENTS_PER_CONNECTION).min(data.len()).max(1);

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
