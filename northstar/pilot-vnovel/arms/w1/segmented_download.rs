//! Intent: 최종 파일 URL을 받아 VN 폴더에 손상 없는 완성 파일로 만든다. 쓰로틀 호스트에서 빠르게(Range 병렬, 연결 최대 8), 취소·재시작 후 이어받기.
//! Data: `<dest>.download` 한 파일 = [본문 total바이트][꼬리: total(u64) + 구간별 완료 바이트 u64 x N(LE)]. 완료되면 꼬리를 잘라내고 rename.
//! Assumes: 같은 URL은 진행·중단 동안 같은 바이트를 준다(ETag 검증 없음). 한 번에 한 다운로드. total이 같으면 같은 파일로 본다.
//! Weak: 단일 스트림은 이어받기 불가(중단하면 .download가 남고 다음엔 처음부터). 재개 상태는 크기·구간 길이만 검증하고 내용 해시는 없다. 크래시 시 체크포인트가 데이터보다 앞설 수 있다(fsync 안 함). 구간 16개 고정이라 극단적으로 느린 연결 하나가 끝을 늦출 수 있다. 모든 오류를 재시도(최대 4회, 약 3초)한다. 오류 문장 뒷부분은 라이브러리의 영어 원문이다.
//! Cost: 구간 요청 16+1회, 구간당 파일 핸들 1개, 체크포인트는 구간당 1MiB마다 8바이트 쓰기. 메모리는 청크 크기 수준.

use reqwest::header::{ACCEPT_ENCODING, CONTENT_RANGE, COOKIE, RANGE, REFERER};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::SeekFrom::Start;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::Relaxed};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::{Duration, Instant};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::task::JoinSet;

const CONNECTIONS: usize = 8; // D3
const SEGMENTS: u64 = 16; // 연결당 2개: 느린 연결이 있어도 빠른 연결이 남은 구간을 가져간다(P1)
const MAX_RETRIES: usize = 4; // D6
const CHECKPOINT: u64 = 1 << 20; // D4
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

type Callback = Box<dyn Fn(DownloadProgress) + Send + Sync>;
type Res<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>; // D10: 경계(`download`)에서 한 번만 한국어 문장으로 감싼다

// D7: 취소 토큰 레지스트리(토큰 → 플래그)
static CANCELLERS: LazyLock<Mutex<HashMap<String, Arc<AtomicBool>>>> =
    LazyLock::new(Default::default);

pub fn register_cancel(token: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    CANCELLERS
        .lock()
        .unwrap()
        .insert(token.into(), flag.clone());
    flag
}

pub fn request_cancel(token: &str) -> bool {
    let map = CANCELLERS.lock().unwrap();
    map.get(token).map(|f| f.store(true, Relaxed)).is_some()
}

pub fn clear_cancel(token: &str) {
    CANCELLERS.lock().unwrap().remove(token);
}

// D8: 진행 이벤트. 콜백을 잠금 안에서 불러 received가 뒤로 가지 않고, 초당 4회까지만 낸다.
struct Progress {
    received: AtomicU64,
    last: Mutex<Instant>,
    info: DownloadProgress,
    callback: Callback,
}

impl Progress {
    fn new(received: u64, total: Option<u64>, mode: DownloadMode, callback: Callback) -> Self {
        let info = DownloadProgress {
            received,
            total,
            mode,
        };
        let p = Progress {
            received: received.into(),
            last: Mutex::new(Instant::now()),
            info,
            callback,
        };
        p.emit();
        p
    }

    fn emit(&self) {
        (self.callback)(DownloadProgress {
            received: self.received.load(Relaxed),
            ..self.info
        });
    }

    fn add(&self, n: u64) {
        self.received.fetch_add(n, Relaxed);
        let mut last = self.last.lock().unwrap();
        if last.elapsed() >= Duration::from_millis(250) {
            *last = Instant::now();
            self.emit();
        }
    }
}

// D9: 모든 요청의 공통 헤더. identity는 바이트 오프셋 = 파일 오프셋을 지키기 위해서다.
fn get(client: &Client, req: &DownloadRequest) -> reqwest::RequestBuilder {
    let mut b = client.get(&req.url).header(ACCEPT_ENCODING, "identity");
    if let Some(c) = req.cookie_header.as_deref().filter(|v| !v.is_empty()) {
        b = b.header(COOKIE, c);
    }
    if let Some(r) = req.referer.as_deref().filter(|v| !v.is_empty()) {
        b = b.header(REFERER, r);
    }
    b
}

fn part_path(dest: &Path) -> PathBuf {
    let mut name = dest.file_name().unwrap_or_default().to_os_string();
    name.push(".download");
    dest.with_file_name(name)
}

async fn put(f: &mut File, at: u64, bytes: &[u8]) -> Res<()> {
    f.seek(Start(at)).await?;
    Ok(f.write_all(bytes).await?)
}

/// 분할 다운로드 작업 하나(D3/D4).
struct Job {
    client: Client,
    req: DownloadRequest,
    part: PathBuf,
    total: u64,
    n: u64,
    offsets: Vec<u64>, // 구간별 이미 받은 바이트(재개 상태)
    next: AtomicUsize,
    progress: Progress,
    cancel: Arc<AtomicBool>,
}

/// 구간 i = [total*i/n, total*(i+1)/n): 빈틈·겹침 없이 파일을 덮는다.
fn bounds(total: u64, n: u64, i: u64) -> (u64, u64) {
    (total * i / n, total * (i + 1) / n)
}

/// D4/D5: 임시 파일을 열고 꼬리의 재개 상태를 읽는다. 크기·total이 다르거나 구간 오프셋이
/// 구간 길이를 넘으면(손상) 그 부분을 버리고 처음부터 받는다.
async fn open_part(part: &Path, total: u64, n: u64) -> Res<(File, Vec<u64>)> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(part)
        .await?;
    let want = total + 8 * (n + 1);
    let mut tail = vec![0u8; 8 * (n as usize + 1)];
    f.seek(Start(total)).await?;
    let word = |t: &[u8], i: usize| u64::from_le_bytes(t[8 * i..8 * i + 8].try_into().unwrap());
    let valid = f.metadata().await?.len() == want && f.read_exact(&mut tail).await.is_ok();
    if !valid || word(&tail, 0) != total {
        tail.fill(0);
        tail[..8].copy_from_slice(&total.to_le_bytes());
        f.set_len(0).await?;
        f.set_len(want).await?;
        put(&mut f, total, &tail).await?;
        f.flush().await?;
    }
    let done = |i| {
        let (s, e) = bounds(total, n, i);
        Some(word(&tail, i as usize + 1))
            .filter(|v| *v <= e - s)
            .unwrap_or(0)
    };
    Ok((f, (0..n).map(done).collect()))
}

/// D5/D6: 구간 i의 남은 부분을 최대 MAX_RETRIES번 받는다. 이미 받은 바이트(done)는 유지한다.
async fn segment(job: &Job, i: usize) -> Res<()> {
    let (s, e) = bounds(job.total, job.n, i as u64);
    let mut f = OpenOptions::new().write(true).open(&job.part).await?;
    let mut done = job.offsets[i];
    let mut result = Ok(());
    for attempt in 0..MAX_RETRIES {
        if done == e - s {
            break;
        }
        result = fetch(job, &mut f, i, s, e - s, &mut done).await;
        if result.is_ok() || job.cancel.load(Relaxed) {
            break;
        }
        if attempt + 1 < MAX_RETRIES {
            tokio::time::sleep(Duration::from_millis(500 * (attempt as u64 + 1))).await;
        }
    }
    let _ = f.flush().await;
    result
}

/// 요청 한 번. 206이고 Content-Range가 요청한 시작점이어야 하며, 정확히 구간 끝까지 받아야 Ok(D5).
async fn fetch(job: &Job, f: &mut File, i: usize, s: u64, len: u64, done: &mut u64) -> Res<()> {
    if job.cancel.load(Relaxed) {
        return Err("취소됨".into());
    }
    let from = s + *done;
    let range = format!("bytes={from}-{}", s + len - 1);
    let mut res = get(&job.client, &job.req)
        .header(RANGE, range)
        .send()
        .await?;
    let start = res
        .headers()
        .get(CONTENT_RANGE)
        .and_then(|v| v.to_str().ok());
    if res.status() != StatusCode::PARTIAL_CONTENT
        || !start.is_some_and(|v| v.starts_with(&format!("bytes {from}-")))
    {
        return Err(format!("요청한 범위가 아닌 응답입니다(HTTP {})", res.status()).into());
    }
    while let Some(chunk) = res.chunk().await? {
        if job.cancel.load(Relaxed) {
            return Err("취소됨".into());
        }
        let n = chunk.len() as u64;
        if n > len - *done {
            return Err("구간이 예정 크기를 초과했습니다".into());
        }
        put(f, s + *done, &chunk).await?;
        *done += n;
        job.progress.add(n);
        if *done == len || *done / CHECKPOINT > (*done - n) / CHECKPOINT {
            // 같은 핸들이라 데이터가 먼저 쓰이고 상태가 뒤따른다.
            put(f, job.total + 8 + 8 * i as u64, &done.to_le_bytes()).await?;
        }
    }
    f.flush().await?;
    match *done == len {
        true => Ok(()),
        false => Err(format!("구간 {i} 크기 불일치: {done}/{len} bytes").into()),
    }
}

/// D1/D3/D4: 연결 최대 8개가 구간 큐를 비운다. 다 받으면 꼬리를 자르고 rename(D1: 완성된 뒤에만 목적지가 생긴다).
async fn segmented(
    client: Client,
    req: DownloadRequest,
    total: u64,
    cancel: Arc<AtomicBool>,
    cb: Callback,
) -> Res<PathBuf> {
    let n = SEGMENTS.min(total);
    let (part, dest) = (part_path(&req.destination), req.destination.clone());
    let (f, offsets) = open_part(&part, total, n).await?;
    let progress = Progress::new(
        offsets.iter().sum(),
        Some(total),
        DownloadMode::Segmented,
        cb,
    );
    let job = Arc::new(Job {
        client,
        req,
        part,
        total,
        n,
        offsets,
        next: 0.into(),
        progress,
        cancel,
    });
    let mut workers = JoinSet::new();
    for _ in 0..CONNECTIONS.min(n as usize) {
        let job = job.clone();
        workers.spawn(async move {
            loop {
                let i = job.next.fetch_add(1, Relaxed);
                if i as u64 >= job.n {
                    return Ok::<(), String>(());
                }
                segment(&job, i)
                    .await
                    .map_err(|e| format!("구간 {i}: {e}"))?;
            }
        });
    }
    while let Some(r) = workers.join_next().await {
        r??;
    }
    job.progress.emit();
    f.set_len(total).await?;
    drop(f);
    Ok(finish(&job.part, &dest).await?)
}

/// D2: Range 없는 서버나 총 크기를 모르는 서버는 한 줄기로 받는다(이어받기 없음).
async fn single(
    client: Client,
    req: DownloadRequest,
    cancel: Arc<AtomicBool>,
    cb: Callback,
) -> Res<PathBuf> {
    let mut res = get(&client, &req).send().await?.error_for_status()?;
    let total = res.content_length();
    let progress = Progress::new(0, total, DownloadMode::Single, cb);
    let part = part_path(&req.destination);
    let mut f = File::create(&part).await?;
    while let Some(chunk) = res.chunk().await? {
        if cancel.load(Relaxed) {
            return Err("취소됨".into());
        }
        f.write_all(&chunk).await?;
        progress.add(chunk.len() as u64);
    }
    f.flush().await?;
    drop(f);
    if total.is_some_and(|t| t != progress.received.load(Relaxed)) {
        return Err("받은 크기가 서버가 알린 크기와 다릅니다".into());
    }
    progress.emit();
    Ok(finish(&part, &req.destination).await?)
}

async fn finish(part: &Path, dest: &Path) -> std::io::Result<PathBuf> {
    tokio::fs::rename(part, dest).await?;
    Ok(dest.into())
}

/// 최종 URL을 받아 파일로 만든다. `Some(path)` 성공, `None` 취소(재개 상태 유지), `Err` 실패.
pub async fn download<F>(
    req: DownloadRequest,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    let result = run(req, cancel.clone(), Box::new(on_progress)).await;
    match result {
        Ok(path) => Ok(Some(path)),
        Err(_) if cancel.load(Relaxed) => Ok(None), // D7: 취소로 인한 오류는 실패가 아니다
        Err(e) => Err(format!("다운로드 실패: {e}")),
    }
}

async fn run(req: DownloadRequest, cancel: Arc<AtomicBool>, cb: Callback) -> Res<PathBuf> {
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::limited(10))
        .referer(false) // D9: 리다이렉트 뒤에도 호출자가 준 referer를 그대로 쓴다
        .user_agent(USER_AGENT)
        .build()?;
    // D2: 프로브. 206 + 총 크기를 알 때만 분할. 오류 상태는 실패(빈 파일의 416은 단일 경로가 받는다).
    let probe = get(&client, &req).header(RANGE, "bytes=0-0").send().await?;
    let status = probe.status();
    if !status.is_success() && status != StatusCode::RANGE_NOT_SATISFIABLE {
        return Err(format!("서버가 요청을 거부했습니다(HTTP {status})").into());
    }
    let total = probe
        .headers()
        .get(CONTENT_RANGE)
        .and_then(|v| v.to_str().ok());
    let total = total.and_then(|v| v.rsplit_once('/')?.1.parse::<u64>().ok());
    match total.filter(|t| *t > 0 && status == StatusCode::PARTIAL_CONTENT) {
        Some(t) => segmented(client, req, t, cancel, cb).await,
        None => single(client, req, cancel, cb).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    /// 테스트 서버의 오작동 스위치. 기본값 = 정상 Range 서버.
    #[derive(Default)]
    struct Opt {
        no_range: bool,                            // Range를 무시하고 항상 200
        star: bool,                                // 프로브에 `bytes 0-0/*`
        status: u16,                               // 모든 요청에 이 상태
        seg_status: u16,                           // 프로브가 아닌 요청에 이 상태
        bad_start: bool,                           // Content-Range 시작점이 요청과 다름
        long: bool,                                // 요청보다 긴 본문
        fail_first: usize,                         // 처음 k개의 구간 요청은 본문을 반만 보내고 끊음
        latency_ms: u64,                           // 요청마다 지연(연결 겹침 관찰용)
        redirect: bool,                            // /start → /file.bin 302
        cancel_at: Option<(u64, Arc<AtomicBool>)>, // 이만큼 보내면 클라이언트 취소 플래그를 켠다
    }

    struct Srv {
        url: String,
        reqs: Mutex<Vec<String>>, // 받은 요청 헤더 전문
        peak: AtomicUsize,        // 동시 연결 최대치
        sent: AtomicU64,          // 보낸 본문 바이트
        active: AtomicUsize,
        failed: AtomicUsize,
        data: Vec<u8>,
        opt: Opt,
    }

    fn serve(data: Vec<u8>, opt: Opt) -> Arc<Srv> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!(
            "http://{}/{}",
            listener.local_addr().unwrap(),
            if opt.redirect { "start" } else { "file.bin" }
        );
        let srv = Arc::new(Srv {
            url,
            reqs: Default::default(),
            peak: 0.into(),
            sent: 0.into(),
            active: 0.into(),
            failed: 0.into(),
            data,
            opt,
        });
        let s = srv.clone();
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let s = s.clone();
                std::thread::spawn(move || handle(&s, stream));
            }
        });
        srv
    }

    fn handle(s: &Srv, mut conn: TcpStream) {
        let mut buf = vec![0u8; 8192];
        let n = conn.read(&mut buf).unwrap_or(0);
        let head = String::from_utf8_lossy(&buf[..n]).to_string();
        s.reqs.lock().unwrap().push(head.clone());
        s.peak
            .fetch_max(s.active.fetch_add(1, Relaxed) + 1, Relaxed);
        std::thread::sleep(Duration::from_millis(s.opt.latency_ms));
        let (o, len) = (&s.opt, s.data.len());
        let range = head
            .lines()
            .find_map(|l| {
                l.to_ascii_lowercase()
                    .strip_prefix("range: bytes=")
                    .map(String::from)
            })
            .and_then(|v| {
                let (a, b) = v.split_once('-')?;
                Some((
                    a.trim().parse::<usize>().ok()?,
                    b.trim().parse::<usize>().ok()?,
                ))
            });
        let probe = range == Some((0, 0));
        let mut cut = false;
        let (status, extra, body): (String, String, Vec<u8>) =
            if o.redirect && head.contains("GET /start") {
                (
                    "302 Found".into(),
                    format!("Location: /file.bin\r\n"),
                    vec![],
                )
            } else if o.status != 0 || (o.seg_status != 0 && range.is_some() && !probe) {
                (
                    format!(
                        "{} X",
                        if o.status != 0 {
                            o.status
                        } else {
                            o.seg_status
                        }
                    ),
                    String::new(),
                    vec![],
                )
            } else if range.is_some() && len == 0 {
                (
                    "416 Range Not Satisfiable".into(),
                    "Content-Range: bytes */0\r\n".into(),
                    vec![],
                )
            } else if let (Some((a, b)), false) = (range, o.no_range) {
                if o.star && probe {
                    (
                        "206 Partial".into(),
                        "Content-Range: bytes 0-0/*\r\n".into(),
                        s.data[..1].to_vec(),
                    )
                } else {
                    let b = b.min(len - 1);
                    let mut body = s.data[a..=b].to_vec();
                    if o.long && !probe {
                        body.extend([7u8; 100]);
                    }
                    cut = !probe && s.failed.fetch_add(1, Relaxed) < o.fail_first;
                    let shown = if o.bad_start && !probe { a + 1 } else { a };
                    (
                        "206 Partial".into(),
                        format!("Content-Range: bytes {shown}-{b}/{len}\r\n"),
                        body,
                    )
                }
            } else {
                ("200 OK".into(), String::new(), s.data.clone())
            };
        let _ = write!(
            conn,
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\n{extra}Connection: close\r\n\r\n",
            body.len()
        );
        let send = if cut {
            &body[..body.len() / 2]
        } else {
            &body[..]
        };
        for piece in send.chunks(16 << 10) {
            if conn.write_all(piece).is_err() {
                break;
            }
            let total = s.sent.fetch_add(piece.len() as u64, Relaxed) + piece.len() as u64;
            if let Some((at, flag)) = &o.cancel_at {
                if total >= *at {
                    flag.store(true, Relaxed);
                }
            }
        }
        s.active.fetch_sub(1, Relaxed);
    }

    fn run(url: &str, dest: &Path, cancel: Arc<AtomicBool>) -> Result<Option<PathBuf>, String> {
        run_with(url, dest, cancel, |_| {})
    }

    fn run_with(
        url: &str,
        dest: &Path,
        cancel: Arc<AtomicBool>,
        cb: impl Fn(DownloadProgress) + Send + Sync + 'static,
    ) -> Result<Option<PathBuf>, String> {
        let req = DownloadRequest {
            url: url.into(),
            destination: dest.into(),
            cookie_header: None,
            referer: None,
        };
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(download(req, cancel, cb))
    }

    fn no_cancel() -> Arc<AtomicBool> {
        Arc::new(AtomicBool::new(false))
    }

    fn pattern(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i * 7 % 251) as u8).collect()
    }

    /// 빈 임시 폴더. 테스트 끝에 지운다.
    fn tmp(tag: &str) -> PathBuf {
        let t = std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("segdl-{tag}-{}-{t}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn names(dir: &Path) -> Vec<String> {
        std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect()
    }

    fn korean(e: &str) -> bool {
        e.chars().any(|c| !c.is_ascii())
    }

    #[test]
    fn bounds_tile_the_file_without_gap_or_overlap() {
        for (total, n) in [(101, 16), (16, 16), (3, 3), (1, 1), (1 << 33, 16)] {
            let mut next = 0;
            for i in 0..n {
                let (s, e) = bounds(total, n, i);
                assert_eq!(s, next);
                assert!(e > s);
                next = e;
            }
            assert_eq!(next, total);
        }
    }

    // D1 D2 D3: 정확한 바이트, 폴더에 목적지만 남음, 첫 요청은 bytes=0-0, 연결은 겹치되 8개 이하
    #[test]
    fn parallel_download_is_exact_clean_and_capped_at_8_connections() {
        let data = pattern(3_000_001);
        let srv = serve(
            data.clone(),
            Opt {
                latency_ms: 50,
                ..Default::default()
            },
        );
        let dir = tmp("par");
        let dest = dir.join("a.bin");
        assert_eq!(
            run(&srv.url, &dest, no_cancel()).unwrap(),
            Some(dest.clone())
        );
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        assert_eq!(names(&dir), ["a.bin"]);
        assert!(srv.reqs.lock().unwrap()[0]
            .to_lowercase()
            .contains("range: bytes=0-0"));
        let peak = srv.peak.load(Relaxed);
        assert!((2..=8).contains(&peak), "동시 연결 {peak}");
        std::fs::remove_dir_all(dir).ok();
    }

    // D1: 크기 0, 1, 작은 크기(구간 수보다 작음), 구간 수의 배수가 아닌 크기
    #[test]
    fn tiny_and_empty_files_are_exact() {
        for n in [0, 1, 5, 17] {
            let data = pattern(n);
            let srv = serve(data.clone(), Opt::default());
            let dir = tmp("tiny");
            let dest = dir.join("t.bin");
            assert_eq!(
                run(&srv.url, &dest, no_cancel()).unwrap(),
                Some(dest.clone()),
                "n={n}"
            );
            assert_eq!(std::fs::read(&dest).unwrap(), data, "n={n}");
            assert_eq!(names(&dir), ["t.bin"], "n={n}");
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D2 D8: Range 미지원(200)과 총 크기 모르는 206(`*`)은 단일 스트림. 마지막 이벤트 received == total
    #[test]
    fn no_range_or_unknown_total_uses_single_stream() {
        for opt in [
            Opt {
                no_range: true,
                ..Default::default()
            },
            Opt {
                star: true,
                ..Default::default()
            },
        ] {
            let data = pattern(100_003);
            let srv = serve(data.clone(), opt);
            let dir = tmp("single");
            let dest = dir.join("s.bin");
            let events = Arc::new(Mutex::new(vec![]));
            let e2 = events.clone();
            run_with(&srv.url, &dest, no_cancel(), move |p| {
                e2.lock().unwrap().push(p)
            })
            .unwrap();
            assert_eq!(std::fs::read(&dest).unwrap(), data);
            assert_eq!(names(&dir), ["s.bin"]);
            let ev = events.lock().unwrap();
            assert!(ev.iter().all(|p| p.mode == DownloadMode::Single));
            let last = ev.last().unwrap();
            assert_eq!((last.received, last.total), (100_003, Some(100_003)));
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D2 D10: HTTP 오류 → Err(한국어, 상태 코드 포함), 파일 없음
    #[test]
    fn http_error_is_err_without_files() {
        let srv = serve(
            pattern(10),
            Opt {
                status: 404,
                ..Default::default()
            },
        );
        let dir = tmp("404");
        let e = run(&srv.url, &dir.join("x.bin"), no_cancel()).unwrap_err();
        assert!(e.contains("404") && korean(&e), "{e}");
        assert!(names(&dir).is_empty());
        std::fs::remove_dir_all(dir).ok();
    }

    // D8: received 단조 증가, total = 전체 크기, mode = Segmented, 이벤트는 초당 약 4회 이하, 마지막은 received == total
    #[test]
    fn progress_is_monotonic_rate_limited_and_ends_complete() {
        let data = pattern(2_000_000);
        let srv = serve(
            data,
            Opt {
                latency_ms: 700,
                ..Default::default()
            },
        );
        let dir = tmp("prog");
        let events = Arc::new(Mutex::new(vec![]));
        let e2 = events.clone();
        let t = Instant::now();
        run_with(&srv.url, &dir.join("p.bin"), no_cancel(), move |p| {
            e2.lock().unwrap().push(p)
        })
        .unwrap();
        let secs = t.elapsed().as_secs_f64();
        let ev = events.lock().unwrap();
        assert!(ev.windows(2).all(|w| w[0].received <= w[1].received));
        assert!(ev
            .iter()
            .all(|p| p.total == Some(2_000_000) && p.mode == DownloadMode::Segmented));
        assert_eq!(ev.last().unwrap().received, 2_000_000);
        assert!(
            ev.len() as f64 <= secs * 4.0 + 3.0,
            "{}개/{secs:.2}s",
            ev.len()
        );
        std::fs::remove_dir_all(dir).ok();
    }

    // D9: identity 인코딩, 쿠키·referer(비어 있으면 생략), Chrome UA, 리다이렉트 추적. 구간 요청도 모두 같은 헤더
    #[test]
    fn requests_carry_required_headers_and_follow_redirects() {
        let data = pattern(50_000);
        let srv = serve(
            data.clone(),
            Opt {
                redirect: true,
                ..Default::default()
            },
        );
        let dir = tmp("hdr");
        let dest = dir.join("h.bin");
        let req = DownloadRequest {
            url: srv.url.clone(),
            destination: dest.clone(),
            cookie_header: Some("sid=abc".into()),
            referer: Some("http://ref.example/".into()),
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(download(req, no_cancel(), |_| {})).unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        let reqs = srv.reqs.lock().unwrap();
        assert!(reqs.len() > 4);
        for r in reqs.iter().map(|r| r.to_lowercase()) {
            assert!(
                r.contains("accept-encoding: identity")
                    && r.contains("cookie: sid=abc")
                    && r.contains("referer: http://ref.example/")
                    && r.contains("chrome/"),
                "{r}"
            );
        }
        drop(reqs);
        // 빈 문자열은 헤더로 보내지 않는다
        let srv = serve(data, Opt::default());
        let req = DownloadRequest {
            url: srv.url.clone(),
            destination: dir.join("e.bin"),
            cookie_header: Some(String::new()),
            referer: Some(String::new()),
        };
        rt.block_on(download(req, no_cancel(), |_| {})).unwrap();
        assert!(srv.reqs.lock().unwrap().iter().all(
            |r| !r.to_lowercase().contains("cookie:") && !r.to_lowercase().contains("referer:")
        ));
        std::fs::remove_dir_all(dir).ok();
    }

    // D5 D10: 시작점이 다른 Content-Range, 요청보다 긴 본문, 계속 짧은 본문 → 완성 파일 없이 Err(수 초 안에)
    #[test]
    fn wrong_range_long_body_and_short_body_never_complete() {
        for opt in [
            Opt {
                bad_start: true,
                ..Default::default()
            },
            Opt {
                long: true,
                ..Default::default()
            },
            Opt {
                fail_first: usize::MAX,
                ..Default::default()
            },
        ] {
            let srv = serve(pattern(200_000), opt);
            let dir = tmp("bad");
            let dest = dir.join("b.bin");
            let t = Instant::now();
            let e = run(&srv.url, &dest, no_cancel()).unwrap_err();
            assert!(korean(&e), "{e}");
            assert!(!dest.exists());
            assert!(t.elapsed() < Duration::from_secs(10), "{:?}", t.elapsed());
            // 실패가 남긴 재개 상태로 이어받아도 이웃 구간이 오염되지 않아 결과가 정확하다
            let good = serve(pattern(200_000), Opt::default());
            run(&good.url, &dest, no_cancel()).unwrap();
            assert_eq!(std::fs::read(&dest).unwrap(), pattern(200_000));
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D6: 일시적 실패(처음 5개 구간 요청이 중간에 끊김)는 재시도로 복구, 파일은 정확
    #[test]
    fn transient_failures_are_retried() {
        let data = pattern(500_000);
        let srv = serve(
            data.clone(),
            Opt {
                fail_first: 5,
                ..Default::default()
            },
        );
        let dir = tmp("retry");
        let dest = dir.join("r.bin");
        assert_eq!(
            run(&srv.url, &dest, no_cancel()).unwrap(),
            Some(dest.clone())
        );
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        std::fs::remove_dir_all(dir).ok();
    }

    // D6: 구간 요청이 계속 5xx면 수 초 안에 Err
    #[test]
    fn persistent_failure_ends_in_seconds() {
        let srv = serve(
            pattern(200_000),
            Opt {
                seg_status: 503,
                ..Default::default()
            },
        );
        let dir = tmp("fail");
        let t = Instant::now();
        let e = run(&srv.url, &dir.join("f.bin"), no_cancel()).unwrap_err();
        assert!(korean(&e) && e.contains("503"), "{e}");
        assert!(t.elapsed() < Duration::from_secs(10));
        assert!(!dir.join("f.bin").exists());
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 시작 전에 취소 → 구간 요청 없이 Ok(None), 목적지 없음, 재개 상태 유지
    #[test]
    fn preset_cancel_returns_none_and_keeps_resume_state() {
        let srv = serve(pattern(100_000), Opt::default());
        let dir = tmp("pre");
        let dest = dir.join("c.bin");
        assert_eq!(
            run(&srv.url, &dest, Arc::new(AtomicBool::new(true))).unwrap(),
            None
        );
        assert!(!dest.exists() && part_path(&dest).exists());
        assert_eq!(srv.reqs.lock().unwrap().len(), 1, "프로브만");
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 토큰 레지스트리
    #[test]
    fn cancel_tokens_register_signal_and_clear() {
        let f = register_cancel("t-a");
        assert!(!f.load(Relaxed));
        assert!(!request_cancel("t-unknown"));
        assert!(request_cancel("t-a") && f.load(Relaxed));
        assert!(!register_cancel("t-a").load(Relaxed), "새 플래그는 꺼진 채");
        clear_cancel("t-a");
        assert!(!request_cancel("t-a"));
    }

    // D4 D7: 중간에 취소(Ok(None)) → 다시 받으면 빠진 것만 요청하고 결과는 정확. 성공 뒤 폴더엔 목적지만 남음
    #[test]
    fn cancel_midway_then_resume_fetches_only_the_rest() {
        let data = pattern(32 << 20);
        let dir = tmp("resume");
        let dest = dir.join("r.bin");
        let cancel = no_cancel();
        let first = serve(
            data.clone(),
            Opt {
                cancel_at: Some((16 << 20, cancel.clone())),
                ..Default::default()
            },
        );
        assert_eq!(run(&first.url, &dest, cancel).unwrap(), None);
        assert!(!dest.exists());
        let second = serve(data.clone(), Opt::default());
        assert_eq!(
            run(&second.url, &dest, no_cancel()).unwrap(),
            Some(dest.clone())
        );
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        assert_eq!(names(&dir), ["r.bin"]);
        // 취소 때까지 16MiB 이상 받았고, 연결당 최대 1MiB만 다시 받을 수 있다 → 남은 요청량은 75% 이하
        let sent = second.sent.load(Relaxed);
        assert!(sent <= (data.len() as u64) * 3 / 4 + 1000, "재요청 {sent}");
        std::fs::remove_dir_all(dir).ok();
    }

    // D4 D5: 다른 크기 파일의 잔여 상태, 손상된(구간 길이 초과) 상태, 길이가 어긋난 임시 파일은 버리고 정확한 파일을 만든다
    #[test]
    fn foreign_or_damaged_resume_state_is_discarded() {
        let data = pattern(1_000_000);
        for case in 0..3 {
            let dir = tmp("dmg");
            let dest = dir.join("d.bin");
            let part = part_path(&dest);
            let cancel = no_cancel();
            // 먼저 (다른 크기 또는 같은 크기의) 다운로드를 중간에 멈춰 상태를 만든다
            let seed = if case == 0 {
                pattern(777_777)
            } else {
                data.clone()
            };
            let s = serve(
                seed,
                Opt {
                    cancel_at: Some((400_000, cancel.clone())),
                    ..Default::default()
                },
            );
            assert_eq!(run(&s.url, &dest, cancel).unwrap(), None);
            if case == 1 {
                let mut raw = std::fs::read(&part).unwrap();
                let tail = raw.len() - 8 * 16; // 구간 오프셋 전부를 구간 길이 초과 값으로 망가뜨린다
                raw[tail..].iter_mut().for_each(|b| *b = 0xFF);
                std::fs::write(&part, raw).unwrap();
            }
            if case == 2 {
                std::fs::OpenOptions::new()
                    .write(true)
                    .open(&part)
                    .unwrap()
                    .set_len(1234)
                    .unwrap();
            }
            let srv = serve(data.clone(), Opt::default());
            assert_eq!(
                run(&srv.url, &dest, no_cancel()).unwrap(),
                Some(dest.clone()),
                "case {case}"
            );
            assert_eq!(std::fs::read(&dest).unwrap(), data, "case {case}");
            assert_eq!(names(&dir), ["d.bin"]);
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D7: 단일 스트림도 받는 도중 취소하면 Ok(None), 목적지 없음
    #[test]
    fn cancel_during_single_stream_returns_none() {
        let cancel = no_cancel();
        let opt = Opt {
            no_range: true,
            cancel_at: Some((16 << 10, cancel.clone())),
            ..Default::default()
        };
        let srv = serve(pattern(8 << 20), opt);
        let dir = tmp("scancel");
        let dest = dir.join("s.bin");
        assert_eq!(run(&srv.url, &dest, cancel).unwrap(), None);
        assert!(!dest.exists());
        std::fs::remove_dir_all(dir).ok();
    }
}
