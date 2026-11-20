//! Intent: 서명된 파일 URL을 최대 8연결로 받아 완전한 파일로 만든다(중단/재시작 후 이어받기, 틀린 바이트 금지).
//! Data: 파일을 1MiB 조각으로 나눠 8개 일꾼이 공유 카운터로 하나씩 가져간다. `<dest>.part`(미리 늘린 본문) + `<dest>.map`([총크기 u64][조각당 1바이트 0/1]).
//! Assumes: 같은 URL은 같은 바이트를 준다. 조각 하나는 메모리에 다 받은 뒤 검증하고 쓴다. 서버가 Range에 206+총크기를 준다.
//! Weak: 이어받기는 총크기만 대조한다(내용 검증 없음, 같은 크기의 다른 파일이면 섞임). 느린 연결을 중간에 끊지 않는다. Single 경로는 이어받기 불가.
//! Cost: 조각마다 요청 1회(연결 재사용 없는 서버에선 연결 비용). 일꾼당 최대 1MiB 메모리. 중단 시 조각당 최대 1MiB 재다운로드.

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, CONTENT_RANGE, COOKIE, RANGE, REFERER};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

const CONNECTIONS: usize = 8; // D3: 공유 호스트 배려
const CHUNK: u64 = 1 << 20; // D4/P1: 재다운로드 상한이자 느린 연결의 피해 상한
const TRIES: u32 = 4; // D6
const BACKOFF: Duration = Duration::from_millis(200); // D6: 시도 n 뒤 n*200ms
const REPORT_EVERY: Duration = Duration::from_millis(250); // D8: 초당 4회
const CONNECT_TIMEOUT: Duration = Duration::from_secs(20); // D9
const STALL_TIMEOUT: Duration = Duration::from_secs(30); // 멈춘 연결 방지
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DownloadProgress {
    pub received: u64,
    pub total: Option<u64>,
    pub mode: DownloadMode,
}

// D7: 토큰 -> 취소 깃발
fn tokens() -> &'static Mutex<HashMap<String, Arc<AtomicBool>>> {
    static T: OnceLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = OnceLock::new();
    T.get_or_init(Default::default)
}

pub fn register_cancel(token: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    tokens().lock().unwrap().insert(token.to_string(), flag.clone());
    flag
}

pub fn request_cancel(token: &str) -> bool {
    match tokens().lock().unwrap().get(token) {
        Some(f) => {
            f.store(true, Ordering::SeqCst);
            true
        }
        None => false,
    }
}

pub fn clear_cancel(token: &str) {
    tokens().lock().unwrap().remove(token);
}

// D8: received는 뒤로 가지 않고, 이벤트는 REPORT_EVERY마다 최대 1회
struct Progress<F> {
    received: AtomicU64,
    total: Option<u64>,
    mode: DownloadMode,
    last: Mutex<(u64, Instant)>,
    cb: F,
}

impl<F: Fn(DownloadProgress)> Progress<F> {
    fn new(start: u64, total: Option<u64>, mode: DownloadMode, cb: F) -> Self {
        Progress { received: AtomicU64::new(start), total, mode, last: Mutex::new((start, Instant::now())), cb }
    }
    fn emit(&self, force: bool) {
        let mut last = self.last.lock().unwrap();
        if !force && last.1.elapsed() < REPORT_EVERY {
            return;
        }
        let mut v = self.received.load(Ordering::SeqCst).max(last.0);
        if let Some(t) = self.total {
            v = v.min(t);
        }
        *last = (v, Instant::now());
        (self.cb)(DownloadProgress { received: v, total: self.total, mode: self.mode });
    }
    fn add(&self, n: u64) {
        self.received.fetch_add(n, Ordering::SeqCst);
        self.emit(false);
    }
    fn undo(&self, n: u64) {
        self.received.fetch_sub(n, Ordering::SeqCst); // 재시도 시 다시 센다
    }
    fn finish(&self) {
        let total = self.total.unwrap_or(self.received.load(Ordering::SeqCst).max(self.last.lock().unwrap().0));
        (self.cb)(DownloadProgress { received: total, total: Some(total), mode: self.mode });
    }
}

fn side(dest: &Path, ext: &str) -> PathBuf {
    let mut s = dest.as_os_str().to_owned();
    s.push(ext);
    PathBuf::from(s)
}

fn io(e: std::io::Error) -> String {
    format!("파일 처리 실패: {e}")
}

fn net(e: reqwest::Error) -> String {
    format!("네트워크 오류: {e}")
}

// D9
fn client(req: &DownloadRequest) -> Result<Client, String> {
    let mut h = HeaderMap::new();
    h.insert(ACCEPT_ENCODING, HeaderValue::from_static("identity"));
    for (name, v) in [(COOKIE, &req.cookie_header), (REFERER, &req.referer)] {
        if let Some(v) = v.as_deref().filter(|v| !v.is_empty()) {
            h.insert(name, HeaderValue::from_str(v).map_err(|_| "요청 헤더 값이 올바르지 않습니다".to_string())?);
        }
    }
    Client::builder()
        .user_agent(USER_AGENT)
        .default_headers(h)
        .redirect(reqwest::redirect::Policy::limited(10))
        .referer(false) // 리다이렉트 때 자동 Referer가 우리 값을 덮어쓰지 않게
        .connect_timeout(CONNECT_TIMEOUT)
        .read_timeout(STALL_TIMEOUT)
        .build()
        .map_err(|e| format!("HTTP 클라이언트를 만들 수 없습니다: {e}"))
}

// D6: 최대 TRIES번, 점점 늘어나는 대기
async fn retry<T, Fut: Future<Output = Result<T, String>>>(mut f: impl FnMut() -> Fut) -> Result<T, String> {
    let mut n = 1;
    loop {
        match f().await {
            Err(_) if n < TRIES => {
                tokio::time::sleep(BACKOFF * n).await;
                n += 1;
            }
            r => return r,
        }
    }
}

// "bytes a-b/total" -> (a, b, total); total이 `*`이면 None
fn content_range(r: &Response) -> Option<(u64, u64, Option<u64>)> {
    let v = r.headers().get(CONTENT_RANGE)?.to_str().ok()?.strip_prefix("bytes ")?;
    let (range, total) = v.split_once('/')?;
    let (a, b) = range.split_once('-')?;
    Some((a.trim().parse().ok()?, b.trim().parse().ok()?, total.trim().parse().ok()))
}

pub async fn download<F: Fn(DownloadProgress) + Send + Sync + 'static>(
    request: DownloadRequest,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String> {
    if cancel.load(Ordering::SeqCst) {
        return Ok(None);
    }
    let client = client(&request)?;
    // D2: Range 0-0 탐색 요청
    let probe = retry(|| async { client.get(&request.url).header(RANGE, "bytes=0-0").send().await.map_err(net) }).await?;
    let status = probe.status().as_u16();
    match (status, content_range(&probe)) {
        (206, Some((0, _, Some(total)))) => segmented(client, request, total, cancel, on_progress).await,
        (200, _) => single(&client, &request, Some(probe), cancel, on_progress).await,
        (206, _) | (416, _) => single(&client, &request, None, cancel, on_progress).await,
        _ => Err(format!("서버가 오류를 응답했습니다 (HTTP {status})")),
    }
}

// D1/D2: 한 줄기로 .part에 받고 끝나면 이름을 바꾼다
async fn single<F: Fn(DownloadProgress)>(
    client: &Client,
    req: &DownloadRequest,
    first: Option<Response>,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String> {
    let (part, dest) = (side(&req.destination, ".part"), &req.destination);
    let mut first = first;
    let result = retry(|| {
        let resp = first.take();
        async {
            let mut resp = match resp {
                Some(r) => r,
                None => client.get(&req.url).send().await.map_err(net)?,
            };
            if !resp.status().is_success() {
                return Err(format!("서버가 오류를 응답했습니다 (HTTP {})", resp.status().as_u16()));
            }
            let total = resp.content_length();
            let p = Progress::new(0, total, DownloadMode::Single, &on_progress);
            let mut f = File::create(&part).await.map_err(io)?;
            let mut got = 0u64;
            while let Some(chunk) = resp.chunk().await.map_err(net)? {
                if cancel.load(Ordering::SeqCst) {
                    return Ok(false);
                }
                f.write_all(&chunk).await.map_err(io)?;
                got += chunk.len() as u64;
                p.add(chunk.len() as u64);
            }
            f.flush().await.map_err(io)?;
            if total.is_some_and(|t| t != got) {
                return Err("받은 크기가 서버가 알린 크기와 다릅니다".to_string()); // D5
            }
            p.finish();
            Ok(true)
        }
    })
    .await;
    match result {
        Ok(true) => {
            tokio::fs::rename(&part, dest).await.map_err(io)?;
            Ok(Some(dest.clone()))
        }
        r => {
            let _ = tokio::fs::remove_file(&part).await; // 이어받기 불가이므로 남기지 않는다
            r.map(|_| None)
        }
    }
}

struct Job<F> {
    client: Client,
    url: String,
    part: PathBuf,
    map: PathBuf,
    total: u64,
    pending: Vec<u64>, // 아직 못 받은 조각 번호
    next: AtomicUsize,
    stop: AtomicBool,
    cancel: Arc<AtomicBool>,
    progress: Progress<F>,
}

impl<F: Fn(DownloadProgress)> Job<F> {
    fn span(&self, i: u64) -> (u64, u64) {
        (i * CHUNK, ((i + 1) * CHUNK).min(self.total) - 1)
    }

    // 조각 하나를 통째로 받아 검증한다. 취소되면 None
    async fn fetch(&self, i: u64) -> Result<Option<Vec<u8>>, String> {
        let (a, b) = self.span(i);
        let want = b - a + 1;
        let mut resp = self.client.get(&self.url).header(RANGE, format!("bytes={a}-{b}")).send().await.map_err(net)?;
        if resp.status().as_u16() != 206 || content_range(&resp) != Some((a, b, Some(self.total))) {
            return Err("서버가 요청한 구간과 다른 구간을 보냈습니다".to_string()); // D5
        }
        let mut buf = Vec::with_capacity(want as usize);
        let mut failed = None;
        while failed.is_none() {
            match resp.chunk().await {
                Ok(Some(chunk)) => {
                    if self.cancel.load(Ordering::SeqCst) {
                        self.progress.undo(buf.len() as u64);
                        return Ok(None);
                    }
                    self.progress.add(chunk.len() as u64);
                    buf.extend_from_slice(&chunk);
                    if buf.len() as u64 > want {
                        break;
                    }
                }
                Ok(None) => break,
                Err(e) => failed = Some(net(e)),
            }
        }
        if failed.is_some() || buf.len() as u64 != want {
            self.progress.undo(buf.len() as u64);
            return Err(failed.unwrap_or_else(|| "받은 조각의 길이가 맞지 않습니다".to_string())); // D5
        }
        Ok(Some(buf))
    }

    // 본문을 쓰고 flush한 뒤에야 map에 완료 표시를 한다
    async fn store(&self, i: u64, data: &[u8]) -> Result<(), String> {
        for (path, at, bytes) in [(&self.part, self.span(i).0, data), (&self.map, 8 + i, &[1u8][..])] {
            let mut f = OpenOptions::new().write(true).open(path).await.map_err(io)?;
            f.seek(SeekFrom::Start(at)).await.map_err(io)?;
            f.write_all(bytes).await.map_err(io)?;
            f.flush().await.map_err(io)?;
        }
        Ok(())
    }

    async fn worker(&self) -> Result<(), String> {
        loop {
            let n = self.next.fetch_add(1, Ordering::SeqCst);
            if n >= self.pending.len() || self.stop.load(Ordering::SeqCst) || self.cancel.load(Ordering::SeqCst) {
                return Ok(());
            }
            let i = self.pending[n];
            let stored = match retry(|| self.fetch(i)).await {
                Ok(Some(data)) => self.store(i, &data).await,
                Ok(None) => return Ok(()),
                Err(e) => Err(e),
            };
            if stored.is_err() {
                self.stop.store(true, Ordering::SeqCst);
                return stored;
            }
        }
    }
}

// D4/D5: 맵이 이 파일 것이 아니면(크기 불일치, 길이 이상, 0/1 아닌 값) 버리고 처음부터
async fn load_state(part: &Path, map: &Path, total: u64, n: u64) -> Result<Vec<u8>, String> {
    if let (Ok(m), Ok(meta)) = (tokio::fs::read(map).await, tokio::fs::metadata(part).await) {
        if m.len() as u64 == 8 + n && m[..8] == total.to_le_bytes() && m[8..].iter().all(|&b| b <= 1) && meta.len() == total {
            return Ok(m[8..].to_vec());
        }
    }
    File::create(part).await.map_err(io)?.set_len(total).await.map_err(io)?;
    let mut m = total.to_le_bytes().to_vec();
    m.resize(8 + n as usize, 0);
    tokio::fs::write(map, m).await.map_err(io)?;
    Ok(vec![0; n as usize])
}

async fn segmented<F: Fn(DownloadProgress) + Send + Sync + 'static>(
    client: Client,
    req: DownloadRequest,
    total: u64,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String> {
    let (part, map) = (side(&req.destination, ".part"), side(&req.destination, ".map"));
    let n = total.div_ceil(CHUNK);
    let done = load_state(&part, &map, total, n).await?;
    let pending: Vec<u64> = (0..n).filter(|&i| done[i as usize] == 0).collect();
    let have = total - pending.iter().map(|&i| ((i + 1) * CHUNK).min(total) - i * CHUNK).sum::<u64>();
    let job = Arc::new(Job {
        client,
        url: req.url,
        part: part.clone(),
        map: map.clone(),
        total,
        next: AtomicUsize::new(0),
        pending,
        stop: AtomicBool::new(false),
        cancel: cancel.clone(),
        progress: Progress::new(have, Some(total), DownloadMode::Segmented, on_progress),
    });
    job.progress.emit(true);
    let workers: Vec<_> = (0..CONNECTIONS.min(job.pending.len()))
        .map(|_| {
            let job = job.clone();
            tokio::spawn(async move { job.worker().await })
        })
        .collect();
    let mut failure = None;
    for w in workers {
        match w.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => failure = failure.or(Some(e)),
            Err(e) => failure = failure.or(Some(format!("내부 작업 오류: {e}"))),
        }
    }
    if let Some(e) = failure {
        return Err(e);
    }
    if cancel.load(Ordering::SeqCst) {
        return Ok(None);
    }
    job.progress.finish();
    tokio::fs::rename(&part, &req.destination).await.map_err(io)?; // D1: 완성 후에만 나타난다
    let _ = tokio::fs::remove_file(&map).await;
    Ok(Some(req.destination))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;

    #[derive(Clone, Default)]
    struct Opt {
        ignore: bool,
        unknown: bool,
        status: u16,
        drop_first: usize,
        wrong: bool,
        over: bool,
    }

    struct Srv {
        url: String,
        heads: Arc<Mutex<Vec<String>>>,
        max: Arc<AtomicUsize>,
        served: Arc<AtomicU64>,
        delay_ms: Arc<AtomicU64>, // 64KiB를 보낼 때마다 쉬는 시간
    }

    fn serve(data: Vec<u8>, opt: Opt) -> Srv {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/file", l.local_addr().unwrap());
        let s = Srv { url, heads: Default::default(), max: Default::default(), served: Default::default(), delay_ms: Default::default() };
        let (heads, max, served, delay) = (s.heads.clone(), s.max.clone(), s.served.clone(), s.delay_ms.clone());
        let (data, opt, active) = (Arc::new(data), Arc::new(Mutex::new(opt)), Arc::new(AtomicUsize::new(0)));
        std::thread::spawn(move || {
            for st in l.incoming().flatten() {
                let (data, opt, heads, max, served, delay, active) =
                    (data.clone(), opt.clone(), heads.clone(), max.clone(), served.clone(), delay.clone(), active.clone());
                std::thread::spawn(move || {
                    let mut st = st;
                    let mut buf = Vec::new();
                    let mut tmp = [0u8; 1024];
                    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        match st.read(&mut tmp) {
                            Ok(n) if n > 0 => buf.extend_from_slice(&tmp[..n]),
                            _ => return,
                        }
                    }
                    let head = String::from_utf8_lossy(&buf).to_lowercase();
                    heads.lock().unwrap().push(head.clone());
                    let now = active.fetch_add(1, Ordering::SeqCst) + 1;
                    max.fetch_max(now, Ordering::SeqCst);
                    let o = opt.lock().unwrap().clone();
                    let total = data.len();
                    let range = head.lines().find_map(|l| l.strip_prefix("range: bytes=")).and_then(|v| {
                        let (a, b) = v.trim().split_once('-')?;
                        Some((a.parse::<usize>().ok()?, b.parse::<usize>().ok()?))
                    });
                    let probe = range == Some((0, 0));
                    let resp: Vec<u8> = if head.starts_with("get /r ") {
                        b"HTTP/1.1 302 Found\r\nLocation: /file\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_vec()
                    } else if o.status != 0 {
                        format!("HTTP/1.1 {} X\r\nContent-Length: 0\r\nConnection: close\r\n\r\n", o.status).into_bytes()
                    } else {
                        let (status, a, b, cr) = match range {
                            Some((a, b)) if !o.ignore && a < total => {
                                let b = b.min(total - 1);
                                let shown = if o.wrong && !probe { a + 1 } else { a };
                                let t = if o.unknown { "*".to_string() } else { total.to_string() };
                                ("206 Partial Content", a, b + 1, format!("Content-Range: bytes {shown}-{b}/{t}\r\n"))
                            }
                            _ => ("200 OK", 0, total, String::new()),
                        };
                        let mut body = data[a..b].to_vec();
                        if o.over && !cr.is_empty() && !probe {
                            body.push(7);
                        }
                        let mut r = format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\n{cr}Connection: close\r\n\r\n", body.len()).into_bytes();
                        if !probe && o.drop_first > 0 {
                            let mut g = opt.lock().unwrap();
                            if g.drop_first > 0 {
                                g.drop_first -= 1;
                                body.truncate(100);
                            }
                        }
                        r.extend(body);
                        r
                    };
                    let head_len = resp.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
                    let _ = st.write_all(&resp[..head_len]);
                    let n_chunks = resp[head_len..].chunks(64 * 1024).count();
                    for (k, c) in resp[head_len..].chunks(64 * 1024).enumerate() {
                        if st.write_all(c).is_err() {
                            break;
                        }
                        served.fetch_add(c.len() as u64, Ordering::SeqCst);
                        if k + 1 < n_chunks {
                            std::thread::sleep(Duration::from_millis(delay.load(Ordering::SeqCst)));
                        }
                    }
                    active.fetch_sub(1, Ordering::SeqCst);
                });
            }
        });
        s
    }

    fn data(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i * 7 % 251) as u8).collect()
    }

    fn tmp(tag: &str) -> PathBuf {
        static N: AtomicUsize = AtomicUsize::new(0);
        let d = std::env::temp_dir().join(format!("segdl-w2-{}-{tag}-{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst)));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn req(s: &Srv, dir: &Path) -> DownloadRequest {
        DownloadRequest { url: s.url.clone(), destination: dir.join("out.bin"), cookie_header: None, referer: None }
    }

    fn run<T>(f: impl Future<Output = T>) -> T {
        tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap().block_on(f)
    }

    fn go(r: DownloadRequest) -> Result<Option<PathBuf>, String> {
        run(download(r, Arc::new(AtomicBool::new(false)), |_| {}))
    }

    fn names(dir: &Path) -> Vec<String> {
        std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect()
    }

    fn korean(e: &str) -> bool {
        e.chars().any(|c| ('가'..='힣').contains(&c))
    }

    #[test]
    fn d1_identical_for_any_size_and_nothing_left() {
        for n in [0, 1, CHUNK as usize, 3 * CHUNK as usize + 17] {
            let d = data(n);
            let (s, dir) = (serve(d.clone(), Opt::default()), tmp("d1"));
            let out = go(req(&s, &dir)).unwrap().unwrap();
            assert_eq!(std::fs::read(&out).unwrap(), d, "size {n}");
            assert_eq!(names(&dir), ["out.bin"]);
        }
    }

    #[test]
    fn d1_destination_absent_until_complete() {
        let (s, dir) = (serve(data(4 * CHUNK as usize), Opt::default()), tmp("d1b"));
        s.delay_ms.store(30, Ordering::SeqCst);
        let r = req(&s, &dir);
        let dest = r.destination.clone();
        let seen = Arc::new(AtomicBool::new(false));
        let seen2 = seen.clone();
        let d2 = dest.clone();
        run(download(r, Arc::new(AtomicBool::new(false)), move |p| {
            if p.received < p.total.unwrap() && d2.exists() {
                seen2.store(true, Ordering::SeqCst);
            }
        }))
        .unwrap();
        assert!(!seen.load(Ordering::SeqCst) && dest.exists());
    }

    #[test]
    fn d2_probe_first_then_parallel_or_single() {
        let (s, dir) = (serve(data(3 * CHUNK as usize), Opt::default()), tmp("d2a"));
        let modes = Arc::new(Mutex::new(Vec::new()));
        let m = modes.clone();
        run(download(req(&s, &dir), Arc::new(AtomicBool::new(false)), move |p| m.lock().unwrap().push(p.mode))).unwrap();
        assert!(s.heads.lock().unwrap()[0].contains("range: bytes=0-0"));
        assert!(modes.lock().unwrap().iter().all(|&m| m == DownloadMode::Segmented));
        for o in [Opt { ignore: true, ..Default::default() }, Opt { unknown: true, ..Default::default() }] {
            let d = data(2 * CHUNK as usize + 5);
            let (s, dir) = (serve(d.clone(), o), tmp("d2b"));
            let modes = Arc::new(Mutex::new(Vec::new()));
            let m = modes.clone();
            let out = run(download(req(&s, &dir), Arc::new(AtomicBool::new(false)), move |p| m.lock().unwrap().push(p.mode))).unwrap().unwrap();
            assert_eq!(std::fs::read(out).unwrap(), d);
            assert!(modes.lock().unwrap().iter().all(|&m| m == DownloadMode::Single));
            assert_eq!(names(&dir), ["out.bin"]);
        }
    }

    #[test]
    fn d2_http_error_is_err_without_file() {
        let (s, dir) = (serve(data(10), Opt { status: 404, ..Default::default() }), tmp("d2c"));
        assert!(go(req(&s, &dir)).is_err());
        assert!(names(&dir).is_empty());
    }

    #[test]
    fn d3_at_most_eight_connections() {
        let (s, dir) = (serve(data(24 * CHUNK as usize), Opt::default()), tmp("d3"));
        s.delay_ms.store(5, Ordering::SeqCst);
        go(req(&s, &dir)).unwrap().unwrap();
        let m = s.max.load(Ordering::SeqCst);
        assert!((2..=CONNECTIONS).contains(&m), "max {m}");
    }

    #[test]
    fn d4_d7_cancel_keeps_state_and_resume_fetches_only_missing() {
        let n = 16 * CHUNK as usize;
        let d = data(n);
        let (s, dir) = (serve(d.clone(), Opt::default()), tmp("d4"));
        s.delay_ms.store(20, Ordering::SeqCst);
        let cancel = Arc::new(AtomicBool::new(false));
        let c = cancel.clone();
        let r = req(&s, &dir);
        let out = run(async {
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(450)).await;
                c.store(true, Ordering::SeqCst);
            });
            download(r.clone(), cancel, |_| {}).await
        });
        assert_eq!(out, Ok(None));
        assert!(!dir.join("out.bin").exists() && dir.join("out.bin.part").exists() && dir.join("out.bin.map").exists());
        s.delay_ms.store(0, Ordering::SeqCst);
        let before = s.served.load(Ordering::SeqCst);
        let out = go(req(&s, &dir)).unwrap().unwrap();
        assert_eq!(std::fs::read(out).unwrap(), d);
        let second = s.served.load(Ordering::SeqCst) - before;
        assert!(second < n as u64 - CHUNK, "second run fetched {second}");
        assert_eq!(names(&dir), ["out.bin"]);
    }

    #[test]
    fn d4_state_of_other_size_or_damaged_is_discarded() {
        let d = data(3 * CHUNK as usize + 1);
        let len = (d.len() as u64).to_le_bytes().to_vec();
        let maps = [
            vec![9u8; 12],
            [(d.len() as u64 + 5).to_le_bytes().to_vec(), vec![1; 4]].concat(),
            [len, vec![2; 4]].concat(),
            vec![1; 3],
        ];
        for map in maps {
            let (s, dir) = (serve(d.clone(), Opt::default()), tmp("d4b"));
            std::fs::write(dir.join("out.bin.part"), vec![0xEEu8; d.len()]).unwrap();
            std::fs::write(dir.join("out.bin.map"), map).unwrap();
            let out = go(req(&s, &dir)).unwrap().unwrap();
            assert_eq!(std::fs::read(out).unwrap(), d);
        }
    }

    #[test]
    fn d5_wrong_range_or_oversize_never_completes() {
        for o in [Opt { wrong: true, ..Default::default() }, Opt { over: true, ..Default::default() }] {
            let (s, dir) = (serve(data(2 * CHUNK as usize), o), tmp("d5"));
            let t = Instant::now();
            assert!(go(req(&s, &dir)).is_err());
            assert!(t.elapsed() < Duration::from_secs(10));
            assert!(!dir.join("out.bin").exists());
        }
    }

    #[test]
    fn d6_transient_failures_are_retried_persistent_ends_fast() {
        let d = data(4 * CHUNK as usize);
        let (s, dir) = (serve(d.clone(), Opt { drop_first: 3, ..Default::default() }), tmp("d6"));
        let out = go(req(&s, &dir)).unwrap().unwrap();
        assert_eq!(std::fs::read(out).unwrap(), d);
        let (s, dir) = (serve(d, Opt { drop_first: 10_000, ..Default::default() }), tmp("d6b"));
        let t = Instant::now();
        assert!(go(req(&s, &dir)).is_err());
        assert!(t.elapsed() < Duration::from_secs(10));
        assert!(!dir.join("out.bin").exists());
    }

    #[test]
    fn d7_precancelled_and_tokens() {
        let (s, dir) = (serve(data(10), Opt::default()), tmp("d7"));
        let flag = register_cancel("w2-t");
        assert!(request_cancel("w2-t") && flag.load(Ordering::SeqCst));
        assert_eq!(run(download(req(&s, &dir), flag, |_| {})), Ok(None));
        assert!(names(&dir).is_empty());
        clear_cancel("w2-t");
        assert!(!request_cancel("w2-t"));
        assert!(!register_cancel("w2-t").load(Ordering::SeqCst));
    }

    #[test]
    fn d7_cancel_single_stream_leaves_no_destination() {
        let (s, dir) = (serve(data(8 * CHUNK as usize), Opt { ignore: true, ..Default::default() }), tmp("d7b"));
        s.delay_ms.store(20, Ordering::SeqCst);
        let cancel = Arc::new(AtomicBool::new(false));
        let c = cancel.clone();
        let out = run(download(req(&s, &dir), cancel, move |p| {
            if p.received > 0 {
                c.store(true, Ordering::SeqCst);
            }
        }));
        assert_eq!(out, Ok(None));
        assert!(!dir.join("out.bin").exists());
    }

    #[test]
    fn d8_progress_monotonic_final_equals_total_rate_limited() {
        for o in [Opt::default(), Opt { ignore: true, ..Default::default() }] {
            let n = 8 * CHUNK as usize;
            let (s, dir) = (serve(data(n), o), tmp("d8"));
            s.delay_ms.store(10, Ordering::SeqCst);
            let ev = Arc::new(Mutex::new(Vec::new()));
            let e = ev.clone();
            let t = Instant::now();
            run(download(req(&s, &dir), Arc::new(AtomicBool::new(false)), move |p| e.lock().unwrap().push(p))).unwrap();
            let secs = t.elapsed().as_secs_f64();
            let ev = ev.lock().unwrap();
            assert!(ev.windows(2).all(|w| w[1].received >= w[0].received));
            let last = ev.last().unwrap();
            assert_eq!((last.received, last.total), (n as u64, Some(n as u64)));
            assert!(ev.len() as f64 <= secs * 4.0 + 3.0, "{} events in {secs}s", ev.len());
        }
    }

    #[test]
    fn d9_headers_and_redirect() {
        let (s, dir) = (serve(data(2 * CHUNK as usize), Opt::default()), tmp("d9"));
        let mut r = req(&s, &dir);
        r.url = r.url.replace("/file", "/r");
        r.cookie_header = Some("a=b".into());
        r.referer = Some("http://ref.example/".into());
        go(r).unwrap().unwrap();
        let heads = s.heads.lock().unwrap();
        assert!(heads.len() >= 3);
        for h in heads.iter() {
            for want in ["accept-encoding: identity", "cookie: a=b", "referer: http://ref.example/", "user-agent: mozilla/5.0"] {
                assert!(h.contains(want), "{want} missing in {h}");
            }
        }
        let (s, dir) = (serve(data(10), Opt::default()), tmp("d9b"));
        let mut r = req(&s, &dir);
        r.cookie_header = Some(String::new());
        go(r).unwrap();
        assert!(s.heads.lock().unwrap().iter().all(|h| !h.contains("cookie:")));
    }

    #[test]
    fn d10_errors_are_korean() {
        let (s, dir) = (serve(data(10), Opt { status: 500, ..Default::default() }), tmp("d10"));
        assert!(korean(&go(req(&s, &dir)).unwrap_err()));
        let (s, dir) = (serve(data(2 * CHUNK as usize), Opt { wrong: true, ..Default::default() }), tmp("d10b"));
        assert!(korean(&go(req(&s, &dir)).unwrap_err()));
    }
}
