//! Intent: 최종 파일 URL을 최대 8개 연결로 나눠 받아 재개 가능하게 완성한다. 잘못된 바이트로 끝난 파일은 만들지 않는다(D1-D10).
//! Data: 남은 구간 목록 `[start,end)`(미배정 `todo` + 연결별 진행 중 `slots`)이 전부다. 재개 상태는 이 목록을 적은 고정 크기 텍스트 sidecar 한 개다(FNV 합 + 끝 표식).
//! Assumes: 같은 URL은 진행/중단 동안 같은 바이트를 준다(크기만 검증). 서버는 `Content-Range`(시작·총량)를 지킨다. 파일 쓰기는 짧아서 async 안에서 블로킹해도 된다.
//! Weak: 같은 크기의 다른 파일은 못 알아챈다(섞인다). fsync를 하지 않아 정전 시 상태가 데이터보다 앞설 수 있다. 단일 스트림은 재개도 자동 재시도도 없다(0부터 다시 받게 되므로). 파일 쓰기가 오래 멈추면 그동안 비동기 작업자를 붙잡는다.
//! Cost: 청크마다 mutex 하나(기록+진행 갱신), 1 MiB마다 상태 파일 제자리 덮어쓰기. 느린 연결은 끝에서 남은 구간을 빈 연결에 넘겨(steal) 보완하되 버려지는 전송 중 바이트가 있다.

use reqwest::header::{ACCEPT_ENCODING, CONTENT_RANGE, COOKIE, RANGE, REFERER};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::Serialize;
use std::collections::HashMap;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

const CONNECTIONS: usize = 8; // D3: 동시 연결 상한
const MAX_TRIES: usize = 4; // D6: 프로브·구간당 시도 횟수 (대기 0.5s, 1s, 1.5s)
// D6: 연결만 열어 두고 이만큼 아무것도 보내지 않으면 실패로 본다 (테스트에서는 짧게)
const STALL_SECS: u64 = if cfg!(test) { 2 } else { 30 };
const PROGRESS_EVERY: Duration = Duration::from_millis(250); // D8
const CHECKPOINT_BYTES: u64 = 1024 * 1024; // D4: 상태 기록 간격 = 재수신 상한
const MIN_STEAL: u64 = 128 * 1024; // 남은 구간이 이 두 배 미만이면 쪼개지 않는다
const STATE_BYTES: usize = 4096; // 상태 파일은 항상 이 크기로 덮어써 낡은 꼬리가 남지 않는다
const CANCELLED: &str = "취소됨";
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

// D7: 토큰 → 취소 플래그. dist_host가 spawn한 task를 취소 커맨드가 토큰으로 찾는다.
static CANCELLERS: LazyLock<Mutex<HashMap<String, Arc<AtomicBool>>>> = LazyLock::new(Default::default);

fn registry() -> MutexGuard<'static, HashMap<String, Arc<AtomicBool>>> {
    CANCELLERS.lock().unwrap_or_else(PoisonError::into_inner)
}

pub fn register_cancel(token: &str) -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    registry().insert(token.to_string(), flag.clone());
    flag
}

pub fn request_cancel(token: &str) -> bool {
    registry().get(token).map(|f| f.store(true, Ordering::SeqCst)).is_some()
}

pub fn clear_cancel(token: &str) {
    registry().remove(token);
}

/// D9: 모든 요청의 공통 헤더. identity가 아니면 바이트 오프셋이 파일 오프셋과 달라진다.
fn get(client: &Client, req: &DownloadRequest) -> RequestBuilder {
    let mut b = client.get(&req.url).header(ACCEPT_ENCODING, "identity");
    if let Some(v) = req.cookie_header.as_deref().filter(|v| !v.is_empty()) {
        b = b.header(COOKIE, v);
    }
    if let Some(v) = req.referer.as_deref().filter(|v| !v.is_empty()) {
        b = b.header(REFERER, v);
    }
    b
}

fn side(dest: &Path, suffix: &str) -> PathBuf {
    let mut s = dest.as_os_str().to_owned();
    s.push(suffix);
    s.into()
}

type Span = (u64, u64); // [start, end)

/// 스케줄링 상태 전부: 미배정 구간, 연결별 진행 중 구간(start가 곧 재개 오프셋), 받은 총량.
struct Shared {
    todo: Vec<Span>,
    slots: Vec<Span>,
    state: std::fs::File,
    done: u64,
    last_ckpt: u64,
    last_emit: Instant,
}

impl Shared {
    /// 연결 `slot`에 구간을 배정한다: 미배정 구간 우선, 없으면 가장 많이 남은 연결의 뒤 절반을
    /// 가져온다(느린 연결을 빠른 연결이 돕는다). 줄 게 없으면 false.
    fn claim(&mut self, slot: usize) -> bool {
        let span = self.todo.pop().or_else(|| {
            let (v, (pos, end)) =
                self.slots.iter().copied().enumerate().max_by_key(|(_, (p, e))| e - p)?;
            let rest = end - pos;
            if rest < 2 * MIN_STEAL {
                return None;
            }
            self.slots[v].1 = pos + rest / 2;
            Some((pos + rest / 2, end))
        });
        self.slots[slot] = span.unwrap_or((0, 0));
        span.is_some()
    }

    /// D4: 남은 구간을 상태 파일에 제자리로 쓴다. 상태는 항상 실제보다 덜 진행된 쪽(남은 구간의 상위
    /// 집합)이라 낡은 기록도 안전하다. 쓰기 실패는 재개 보장이 깨진 것이므로 숨기지 않고 오류로 올린다.
    fn checkpoint(&mut self, total: u64) -> Result<(), String> {
        let text = state_text(total, &self.remaining());
        self.state
            .seek(SeekFrom::Start(0))
            .and_then(|_| self.state.write_all(text.as_bytes()))
            .map_err(|e| format!("재개 상태 저장 실패: {e}"))
    }

    /// 아직 안 받은 모든 구간 (D4 상태이자 완료 판정 근거).
    fn remaining(&self) -> Vec<Span> {
        self.todo.iter().chain(&self.slots).copied().filter(|(a, b)| a < b).collect()
    }
}

struct Ctx {
    client: Client,
    req: DownloadRequest,
    total: u64,
    part: PathBuf,
    cancel: Arc<AtomicBool>,
    on_progress: Box<dyn Fn(DownloadProgress) + Send + Sync>,
    sh: Mutex<Shared>,
}

enum Fail {
    Retry(String),
    Fatal(String),
}

impl Fail {
    fn text(self) -> String {
        match self {
            Fail::Retry(e) | Fail::Fatal(e) => e,
        }
    }
}

/// D6/D7: 재시도 전 대기(0.5s × 시도 번호). 기다리는 중에도 취소를 듣는다. 취소됐으면 true.
async fn nap(attempt: usize, cancel: &AtomicBool) -> bool {
    let until = Instant::now() + Duration::from_millis(500 * attempt as u64);
    while Instant::now() < until && !cancel.load(Ordering::Relaxed) {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    cancel.load(Ordering::Relaxed)
}

/// D6/D7: 응답·청크를 기다리되 1초마다 취소를 보고, STALL_SECS 동안 아무것도 오지 않으면 재시도할
/// 실패로 본다. 연결만 열어 두고 침묵하는 서버 앞에서 영원히 기다리며 취소도 듣지 않던 것을 막는다.
async fn wait<T, E: std::fmt::Display>(
    fut: impl std::future::Future<Output = Result<T, E>>,
    cancel: &AtomicBool,
    what: &str,
) -> Result<T, Fail> {
    let mut fut = std::pin::pin!(fut);
    for _ in 0..STALL_SECS {
        match tokio::time::timeout(Duration::from_secs(1), &mut fut).await {
            Ok(r) => return r.map_err(|e| Fail::Retry(format!("{what}: {e}"))),
            Err(_) if cancel.load(Ordering::Relaxed) => return Err(Fail::Fatal(CANCELLED.into())),
            Err(_) => {}
        }
    }
    Err(Fail::Retry(format!("{what}: {STALL_SECS}초 동안 서버가 아무것도 보내지 않았습니다")))
}

impl Ctx {
    fn lock(&self) -> MutexGuard<'_, Shared> {
        self.sh.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn emit(&self, received: u64) {
        (self.on_progress)(DownloadProgress {
            received,
            total: Some(self.total),
            mode: DownloadMode::Segmented,
        });
    }

    /// 청크를 자기 구간(steal로 줄었을 수 있음)만큼만 쓰고 진행/상태를 갱신한다. 구간을 다 채웠으면 그 끝.
    fn accept(&self, slot: usize, file: &mut std::fs::File, chunk: &[u8]) -> Result<Option<u64>, String> {
        let mut sh = self.lock();
        let (pos, end) = sh.slots[slot];
        let n = (chunk.len() as u64).min(end - pos);
        file.write_all(&chunk[..n as usize]).map_err(|e| format!("파일 쓰기 실패: {e}"))?;
        sh.slots[slot].0 += n;
        sh.done += n;
        if sh.done - sh.last_ckpt >= CHECKPOINT_BYTES {
            sh.last_ckpt = sh.done;
            sh.checkpoint(self.total)?;
        }
        if sh.last_emit.elapsed() >= PROGRESS_EVERY {
            sh.last_emit = Instant::now();
            self.emit(sh.done); // D8: 잠금 안에서 보내야 두 연결의 진행률 순서가 뒤바뀌지 않는다
        }
        Ok((pos + n == end).then_some(end))
    }

    /// 한 번의 Range 요청으로 `slot`의 구간을 이어 받는다. D5: 시작이 다른 Content-Range나 요청보다 긴 본문은 치명적.
    async fn stream(&self, slot: usize, file: &mut std::fs::File, pos: u64, end: u64) -> Result<(), Fail> {
        let request = get(&self.client, &self.req).header(RANGE, format!("bytes={pos}-{}", end - 1)).send();
        let mut resp = wait(request, &self.cancel, "구간 연결 실패").await?;
        if resp.status() != StatusCode::PARTIAL_CONTENT {
            return Err(Fail::Retry(format!("Range 응답이 아닙니다: HTTP {}", resp.status())));
        }
        // 총량까지 맞아야 한다: 프로브와 다른 크기를 말하는 서버의 파일을 잘린 채 완성하지 않는다
        let (head, tail) = (format!("bytes {pos}-"), format!("/{}", self.total));
        let range = resp.headers().get(CONTENT_RANGE).and_then(|v| v.to_str().ok());
        if !range.is_some_and(|v| v.starts_with(&head) && v.ends_with(&tail)) {
            return Err(Fail::Fatal("Content-Range가 요청 범위와 다릅니다".into()));
        }
        // D5: 요청한 길이를 다 채우면 남은 본문을 읽지 않고 끝내므로(넘겨받기 때문), 더 긴 응답은 미리 거른다
        if resp.content_length().is_some_and(|n| n != end - pos) {
            return Err(Fail::Fatal("응답 길이가 요청한 구간과 다릅니다".into()));
        }
        file.seek(SeekFrom::Start(pos)).map_err(|e| Fail::Fatal(format!("seek 실패: {e}")))?;
        let mut at = pos;
        while let Some(chunk) = wait(resp.chunk(), &self.cancel, "수신 실패").await? {
            if self.cancel.load(Ordering::Relaxed) {
                return Err(Fail::Fatal(CANCELLED.into()));
            }
            at += chunk.len() as u64;
            if at > end {
                return Err(Fail::Fatal("응답이 요청한 크기를 초과했습니다".into()));
            }
            if let Some(filled) = self.accept(slot, file, &chunk).map_err(Fail::Fatal)? {
                // 넘겨받기로 줄어든 구간이면 나머지 본문은 다른 연결 몫이라 읽지 않는다. 줄지 않았는데
                // 본문이 더 온다면 범위를 지키지 않는 응답이다(D5) — 길이 헤더가 없을 때를 여기서 잡는다.
                if filled == end {
                    if let Some(extra) = wait(resp.chunk(), &self.cancel, "수신 실패").await? {
                        if !extra.is_empty() {
                            return Err(Fail::Fatal("응답이 요청한 크기를 초과했습니다".into()));
                        }
                    }
                }
                return Ok(());
            }
        }
        Err(Fail::Retry("응답이 요청한 크기보다 짧습니다".into()))
    }

    /// D6: 배정된 구간을 끝까지 받는다. 재시도는 받은 곳부터 이어 간다.
    async fn fetch(&self, slot: usize) -> Result<(), String> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open(&self.part)
            .map_err(|e| format!("임시 파일 열기 실패: {e}"))?;
        let mut last = String::new();
        for attempt in 0..MAX_TRIES {
            if nap(attempt, &self.cancel).await {
                return Err(CANCELLED.into());
            }
            let (pos, end) = self.lock().slots[slot];
            if pos >= end {
                return Ok(());
            }
            match self.stream(slot, &mut file, pos, end).await {
                Ok(()) => return Ok(()),
                Err(Fail::Fatal(e)) => return Err(e),
                Err(Fail::Retry(e)) => last = e,
            }
        }
        Err(format!("다운로드 실패(재시도 {MAX_TRIES}회 초과): {last}"))
    }
}

fn fnv(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3))
}

/// D4/D5: 재개 상태 텍스트 = 총량, 남은 구간들, 그 본문의 FNV-1a 합, 끝 표식. 덮어쓰다 잘려
/// 숫자 일부만 바뀐 파일은 합이 맞지 않아 버려진다(실제보다 앞선 진행을 주장하지 못한다).
fn state_text(total: u64, spans: &[Span]) -> String {
    let mut body = format!("{total}\n");
    for (a, b) in spans {
        body += &format!("{a} {b}\n");
    }
    let sum = fnv(&body);
    format!("{:<w$}", format!("{body}sum {sum}\nend\n"), w = STATE_BYTES)
}

/// D4: 저장된 남은 구간 목록. 크기·합·형식·범위가 하나라도 이상하면 None(→ 처음부터).
fn load_state(state: &Path, part: &Path, total: u64) -> Option<Vec<Span>> {
    let text = std::fs::read_to_string(state).ok()?;
    if std::fs::metadata(part).ok()?.len() != total {
        return None;
    }
    let (body, rest) = text.split_once("sum ")?;
    if rest.split_once("\nend\n")?.0.parse::<u64>().ok()? != fnv(body) {
        return None;
    }
    let mut lines = body.lines();
    if lines.next()?.parse::<u64>().ok()? != total {
        return None;
    }
    let mut spans: Vec<Span> = lines
        .map(|l| {
            let (a, b) = l.split_once(' ')?;
            let (a, b) = (a.parse().ok()?, b.parse().ok()?);
            (a < b && b <= total).then_some((a, b))
        })
        .collect::<Option<_>>()?;
    // D5: 겹치는 구간을 받아들이면 같은 바이트를 두 번 세고 다른 곳을 비운 채 완료된다
    spans.sort_unstable();
    spans.windows(2).all(|w| w[0].1 <= w[1].0).then_some(spans)
}

/// D4/D5: 본문을 새로 만들기 전에 옛 재개 상태부터 지운다. 순서가 반대면, 그 사이 크래시 뒤
/// 옛 완료 기록이 새 바이트(다른 파일일 수도 있다)를 건너뛰게 한다.
fn forget_state(state: &Path) -> Result<(), String> {
    match std::fs::remove_file(state) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(format!("이전 재개 상태 정리 실패: {e}")),
        _ => Ok(()),
    }
}

/// D2: `bytes=0-0` 프로브의 결론. 프로브 응답은 여기서 버려져 연결을 붙잡지 않는다(D3).
enum Plan {
    Segmented(u64),
    Single(Option<u64>), // 200이 알려 준 크기: 본 요청이 크기를 안 알려 줄 때 짧은 파일을 잡는 기준
}

/// D6: 프로브도 일시적 실패는 재시도한다. 첫 연결이 한 번 끊겼다고 작업 전체를 실패시키지 않는다.
async fn probe(client: &Client, req: &DownloadRequest, cancel: &AtomicBool) -> Result<Plan, String> {
    let mut last = String::new();
    for attempt in 0..MAX_TRIES {
        if attempt > 0 && nap(attempt, cancel).await {
            return Err(CANCELLED.into());
        }
        let sent = get(client, req).header(RANGE, "bytes=0-0").send();
        let resp = match wait(sent, cancel, "다운로드 서버 연결 실패").await {
            Ok(resp) => resp,
            Err(Fail::Retry(e)) => {
                last = e;
                continue;
            }
            Err(Fail::Fatal(e)) => return Err(e),
        };
        return match resp.status() {
            StatusCode::PARTIAL_CONTENT => Ok(resp
                .headers()
                .get(CONTENT_RANGE)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.rsplit_once('/')?.1.parse().ok())
                .filter(|n| *n > 0)
                .map_or(Plan::Single(None), Plan::Segmented)),
            s if s.is_success() => Ok(Plan::Single(resp.content_length())),
            s => Err(format!("다운로드 서버가 요청을 거부했습니다: HTTP {s}")),
        };
    }
    Err(format!("{last} (재시도 {MAX_TRIES}회 초과)"))
}

async fn segmented(
    client: Client,
    req: DownloadRequest,
    total: u64,
    cancel: Arc<AtomicBool>,
    on_progress: Box<dyn Fn(DownloadProgress) + Send + Sync>,
) -> Result<(), String> {
    let part = side(&req.destination, ".download");
    let state_path = side(&req.destination, ".download.state");
    let todo = match load_state(&state_path, &part, total) {
        Some(todo) => todo,
        None => {
            forget_state(&state_path)?;
            let file = std::fs::File::create(&part).map_err(|e| format!("임시 파일 생성 실패: {e}"))?;
            file.set_len(total).map_err(|e| format!("임시 파일 크기 설정 실패: {e}"))?;
            let n = CONNECTIONS as u64;
            (0..n).map(|i| (total * i / n, total * (i + 1) / n)).filter(|(a, b)| a < b).collect()
        }
    };
    let left: u64 = todo.iter().map(|(a, b)| b - a).sum();
    let done = total.saturating_sub(left);
    let state = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&state_path)
        .map_err(|e| format!("상태 파일 열기 실패: {e}"))?;
    let slots = vec![(0, 0); CONNECTIONS];
    let sh = Shared { todo, slots, state, done, last_ckpt: done, last_emit: Instant::now() };
    let ctx = Arc::new(Ctx { client, req, total, part, cancel, on_progress, sh: Mutex::new(sh) });
    ctx.emit(done);
    ctx.lock().checkpoint(total)?;

    let mut workers = JoinSet::new();
    for slot in 0..CONNECTIONS {
        let ctx = ctx.clone();
        workers.spawn(async move {
            while ctx.lock().claim(slot) {
                ctx.fetch(slot).await?;
            }
            Ok::<(), String>(())
        });
    }
    let mut result = Ok(());
    while let Some(joined) = workers.join_next().await {
        result = joined.map_err(|e| format!("다운로드 작업 실행 실패: {e}")).and_then(|r| r);
        if result.is_err() {
            break;
        }
    }
    workers.shutdown().await;
    let mut sh = ctx.lock();
    // D7: 남은 구간이 없어도(이미 다 받은 재개) 취소가 켜져 있으면 완성 파일을 내놓지 않는다
    if result.is_err() || !sh.remaining().is_empty() || ctx.cancel.load(Ordering::Relaxed) {
        let saved = sh.checkpoint(total); // D4: 중단/실패 시점의 정확한 위치를 남긴다
        return result.and(saved).and(Err("일부 구간을 받지 못했습니다".into()));
    }
    drop(sh);
    ctx.emit(total);
    std::fs::rename(&ctx.part, &ctx.req.destination).map_err(|e| format!("완료 파일 이동 실패: {e}"))
}

/// D2: Range 없이 한 번에 받는다(재개 불가). 실패/취소 시 임시 파일은 버린다.
async fn single(
    client: Client,
    req: DownloadRequest,
    expected: Option<u64>,
    cancel: Arc<AtomicBool>,
    on_progress: Box<dyn Fn(DownloadProgress) + Send + Sync>,
) -> Result<(), String> {
    let mut resp = wait(get(&client, &req).send(), &cancel, "다운로드 서버 연결 실패").await.map_err(Fail::text)?;
    // D5: Range 없이 물었으니 전체를 받아야 한다. 206은 파일의 일부라 완성본이 될 수 없다
    if resp.status() == StatusCode::PARTIAL_CONTENT || !resp.status().is_success() {
        return Err(format!("다운로드 서버가 전체 파일을 주지 않았습니다: HTTP {}", resp.status()));
    }
    let total = match (resp.content_length(), expected) {
        (Some(now), Some(probed)) if now != probed => {
            return Err(format!("서버가 알려 준 크기가 서로 다릅니다: {probed} / {now} bytes"));
        }
        (now, probed) => now.or(probed),
    };
    let emit = |received| on_progress(DownloadProgress { received, total, mode: DownloadMode::Single });
    let part = side(&req.destination, ".download");
    forget_state(&side(&req.destination, ".download.state"))?;
    let copied = async {
        let mut file = std::fs::File::create(&part).map_err(|e| format!("임시 파일 생성 실패: {e}"))?;
        let (mut got, mut last) = (0u64, Instant::now());
        emit(0);
        while let Some(chunk) = wait(resp.chunk(), &cancel, "수신 실패").await.map_err(Fail::text)? {
            if cancel.load(Ordering::Relaxed) {
                return Err(CANCELLED.to_string());
            }
            file.write_all(&chunk).map_err(|e| format!("파일 쓰기 실패: {e}"))?;
            got += chunk.len() as u64;
            if last.elapsed() >= PROGRESS_EVERY {
                last = Instant::now();
                emit(got);
            }
        }
        if total.is_some_and(|t| t != got) {
            return Err("응답 크기가 예정 크기와 다릅니다".to_string());
        }
        if cancel.load(Ordering::Relaxed) {
            return Err(CANCELLED.to_string()); // D7: 마지막 청크 뒤(또는 빈 본문)에 켜진 취소
        }
        // D8: 크기를 몰랐어도 끝에서는 안다 — 마지막 이벤트는 received == total
        on_progress(DownloadProgress { received: got, total: Some(got), mode: DownloadMode::Single });
        std::fs::rename(&part, &req.destination).map_err(|e| format!("완료 파일 이동 실패: {e}"))
    }
    .await;
    if copied.is_err() {
        let _ = std::fs::remove_file(&part);
    }
    copied
}

/// `Ok(Some(path))` 완료, `Ok(None)` 취소(재개 상태 유지), `Err` 실패(D10: 한국어 메시지).
pub async fn download<F>(
    request: DownloadRequest,
    cancel: Arc<AtomicBool>,
    on_progress: F,
) -> Result<Option<PathBuf>, String>
where
    F: Fn(DownloadProgress) + Send + Sync + 'static,
{
    let client = Client::builder()
        .connect_timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::limited(10))
        .referer(false) // D9: 리디렉트 뒤에도 호출자가 준 Referer를 그대로 보낸다
        .user_agent(USER_AGENT)
        .build()
        .map_err(|e| format!("HTTP 클라이언트 생성 실패: {e}"))?;
    let dest = request.destination.clone();
    let on_progress = Box::new(on_progress);
    let flag = cancel.clone();
    let result = match probe(&client, &request, &cancel).await {
        Ok(Plan::Segmented(total)) => segmented(client, request, total, cancel, on_progress).await,
        Ok(Plan::Single(expected)) => single(client, request, expected, cancel, on_progress).await,
        Err(e) => Err(e), // 프로브 중 취소도 아래에서 Ok(None)이 된다
    };
    match result {
        Ok(()) => {
            let _ = std::fs::remove_file(side(&dest, ".download.state")); // D1: 성공 뒤 잔여물 없음
            Ok(Some(dest))
        }
        Err(_) if flag.load(Ordering::Relaxed) => Ok(None),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::{TcpListener, TcpStream};
    use std::sync::atomic::{AtomicU64, AtomicUsize};

    /// 테스트 서버 동작 스위치 (0/false = 정상).
    #[derive(Clone, Default)]
    struct Opt {
        ignore_range: bool,
        unknown_total: bool,
        status: u16,
        wrong_start: bool,
        extra_byte: bool,
        short_first: usize, // 처음 N개의 본문 응답은 절반만 보내고 끊는다
        delay_ms: u64,      // 본문 전 대기 (동시 연결 관찰용)
        chunk_ms: u64,      // 64 KiB마다 대기 (느린 전송)
        drop_probe: usize,  // 처음 N개의 프로브는 응답 없이 끊는다
        stall_first: usize, // 처음 N개의 본문 응답은 절반만 보내고 연결을 연 채 침묵한다
        no_length: bool,    // Content-Length 없이 보낸다 (연결 종료로 끝을 알린다)
        short_plain: bool,  // Range 없는 요청에는 앞 절반만, 그 길이를 Content-Length로 보낸다
    }

    #[derive(Clone)]
    struct Srv {
        url: String,
        log: Arc<Mutex<Vec<String>>>,
        served: Arc<AtomicU64>,
        peak: Arc<AtomicUsize>,
        active: Arc<AtomicUsize>,
        shorts: Arc<AtomicUsize>,
        probe_drops: Arc<AtomicUsize>,
        stalls: Arc<AtomicUsize>,
        data: Arc<Vec<u8>>,
        o: Opt,
    }

    fn serve(data: Vec<u8>, o: Opt) -> Srv {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let s = Srv {
            url: format!("http://{}", l.local_addr().unwrap()),
            log: Default::default(),
            served: Default::default(),
            peak: Default::default(),
            active: Default::default(),
            shorts: Arc::new(AtomicUsize::new(o.short_first)),
            probe_drops: Arc::new(AtomicUsize::new(o.drop_probe)),
            stalls: Arc::new(AtomicUsize::new(o.stall_first)),
            data: Arc::new(data),
            o,
        };
        let srv = s.clone();
        std::thread::spawn(move || {
            for conn in l.incoming().flatten() {
                let srv = srv.clone();
                std::thread::spawn(move || {
                    let _ = handle(&srv, conn);
                });
            }
        });
        s
    }

    fn handle(s: &Srv, mut c: TcpStream) -> std::io::Result<()> {
        let (mut buf, mut tmp) = (Vec::new(), [0u8; 1024]);
        while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
            let n = c.read(&mut tmp)?;
            if n == 0 {
                return Ok(());
            }
            buf.extend_from_slice(&tmp[..n]);
        }
        let head = String::from_utf8_lossy(&buf).into_owned();
        s.log.lock().unwrap().push(head.clone());
        let range = head
            .lines()
            .find_map(|l| l.to_ascii_lowercase().strip_prefix("range: bytes=").map(str::to_string))
            .and_then(|v| {
                let (a, b) = v.split_once('-')?;
                Some((a.trim().parse::<usize>().ok()?, b.trim().parse::<usize>().ok()?))
            });
        if head.starts_with("GET /redirect") {
            return write!(c, "HTTP/1.1 302 Found\r\nLocation: /file.bin\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        }
        if s.o.status != 0 {
            return write!(c, "HTTP/1.1 {} X\r\nContent-Length: 0\r\nConnection: close\r\n\r\n", s.o.status);
        }
        let len = s.data.len();
        let probe = range == Some((0, 0));
        let take = |n: &AtomicUsize| n.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok();
        if probe && take(&s.probe_drops) {
            return Ok(()); // 응답 없이 연결을 닫는다
        }
        let stall = !probe && take(&s.stalls);
        let (status, a, e, cr) = match range {
            Some((a, b)) if !s.o.ignore_range && a < len => {
                let e = b.min(len - 1);
                let shown = if s.o.wrong_start && !probe { a + 1 } else { a };
                let t = if s.o.unknown_total { "*".to_string() } else { len.to_string() };
                ("206 Partial Content", a, e, format!("Content-Range: bytes {shown}-{e}/{t}\r\n"))
            }
            _ => ("200 OK", 0, len.saturating_sub(1), String::new()),
        };
        let mut body = if len == 0 { vec![] } else { s.data[a..=e].to_vec() };
        if s.o.short_plain && range.is_none() {
            body.truncate(body.len() / 2);
        }
        if s.o.extra_byte && !cr.is_empty() && !probe {
            body.push(0xAB);
        }
        let length = if s.o.no_length { String::new() } else { format!("Content-Length: {}\r\n", body.len()) };
        write!(c, "HTTP/1.1 {status}\r\n{length}{cr}Connection: close\r\n\r\n")?;
        let cut = !probe && s.shorts.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok();
        let limit = if cut || stall { body.len() / 2 } else { body.len() };
        if !probe {
            s.peak.fetch_max(s.active.fetch_add(1, Ordering::SeqCst) + 1, Ordering::SeqCst);
            std::thread::sleep(Duration::from_millis(s.o.delay_ms));
        }
        // 초과 바이트는 따로 늦게 보낸다: 요청한 길이를 다 채운 뒤에 도착하는 꼬리를 흉내 낸다
        let split = if s.o.extra_byte && !cr.is_empty() && !probe { limit.saturating_sub(1) } else { limit };
        let sent = (|| {
            for ch in body[..split].chunks(64 * 1024) {
                c.write_all(ch)?;
                s.served.fetch_add(ch.len() as u64, Ordering::SeqCst);
                std::thread::sleep(Duration::from_millis(s.o.chunk_ms));
            }
            if split < limit {
                c.flush()?;
                std::thread::sleep(Duration::from_millis(30));
                c.write_all(&body[split..limit])?;
            }
            std::io::Result::Ok(())
        })();
        if stall {
            std::thread::sleep(Duration::from_secs(60)); // 연결을 연 채 침묵
        }
        if !probe {
            s.active.fetch_sub(1, Ordering::SeqCst);
        }
        sent
    }

    fn pattern(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i * 7 % 251) as u8).collect()
    }

    fn tmp_dir(tag: &str) -> PathBuf {
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("segdl-{tag}-{}-{t}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn files(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> =
            std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
        v.sort();
        v
    }

    fn run(
        url: &str,
        dest: &Path,
        cancel: Arc<AtomicBool>,
        cb: impl Fn(DownloadProgress) + Send + Sync + 'static,
    ) -> Result<Option<PathBuf>, String> {
        let request = DownloadRequest {
            url: url.to_string(),
            destination: dest.to_path_buf(),
            cookie_header: None,
            referer: None,
        };
        let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
        rt.block_on(download(request, cancel, cb))
    }

    fn plain(url: &str, dest: &Path) -> Result<Option<PathBuf>, String> {
        run(&format!("{url}/file.bin"), dest, Arc::new(AtomicBool::new(false)), |_| {})
    }

    // D1: 어떤 크기든(0 바이트 포함) 바이트가 같고 폴더에 목적 파일만 남는다.
    #[test]
    fn exact_bytes_and_no_leftovers_for_any_size() {
        for n in [0usize, 1, 7, 100, 1_000_003] {
            let (data, dir) = (pattern(n), tmp_dir("size"));
            let dest = dir.join("a.bin");
            let s = serve(data.clone(), Opt::default());
            assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()), "size {n}");
            assert_eq!(std::fs::read(&dest).unwrap(), data, "size {n}");
            assert_eq!(files(&dir), vec!["a.bin"], "size {n}");
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D3: 여러 연결을 쓰되 8개를 넘지 않는다.
    #[test]
    fn uses_several_connections_but_at_most_eight() {
        let (dir, s) = (tmp_dir("conn"), serve(pattern(4_000_000), Opt { delay_ms: 80, ..Default::default() }));
        plain(&s.url, &dir.join("a.bin")).unwrap();
        let peak = s.peak.load(Ordering::SeqCst);
        assert!((2..=8).contains(&peak), "peak {peak}");
        std::fs::remove_dir_all(dir).ok();
    }

    // D2 + D9: 첫 요청은 bytes=0-0, 모든 요청에 identity/쿠키/리퍼러/크롬 UA, 리디렉트를 따라간다.
    #[test]
    fn probe_first_and_every_request_carries_headers() {
        let (dir, s) = (tmp_dir("hdr"), serve(pattern(300_000), Opt::default()));
        let dest = dir.join("a.bin");
        let request = DownloadRequest {
            url: format!("{}/redirect", s.url),
            destination: dest.clone(),
            cookie_header: Some("sid=abc".into()),
            referer: Some("https://host.example/page".into()),
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let r = rt.block_on(download(request, Arc::new(AtomicBool::new(false)), |_| {}));
        assert_eq!(r.unwrap(), Some(dest.clone()));
        assert_eq!(std::fs::read(&dest).unwrap(), pattern(300_000));
        let log = s.log.lock().unwrap();
        assert!(log[0].to_ascii_lowercase().contains("range: bytes=0-0"), "{}", log[0]);
        for head in log.iter() {
            let h = head.to_ascii_lowercase();
            assert!(h.contains("accept-encoding: identity"), "{head}");
            assert!(h.contains("cookie: sid=abc"), "{head}");
            assert!(h.contains("referer: https://host.example/page"), "{head}");
            assert!(h.contains("user-agent: mozilla/5.0 (windows nt 10.0") && h.contains("chrome/"), "{head}");
        }
        std::fs::remove_dir_all(dir).ok();
    }

    // D2: Range 무시(200)와 총량 모름(`*`)은 단일 스트림으로 받는다.
    #[test]
    fn range_ignored_or_unknown_total_uses_single_stream() {
        for o in [Opt { ignore_range: true, ..Default::default() }, Opt { unknown_total: true, ..Default::default() }] {
            let (data, dir) = (pattern(100_000), tmp_dir("single"));
            let (s, dest) = (serve(data.clone(), o), dir.join("a.bin"));
            let modes = Arc::new(Mutex::new(Vec::new()));
            let m = modes.clone();
            let r = run(&format!("{}/file.bin", s.url), &dest, Arc::new(AtomicBool::new(false)), move |p| {
                m.lock().unwrap().push((p.mode, p.received, p.total))
            });
            assert_eq!(r.unwrap(), Some(dest.clone()));
            assert_eq!(std::fs::read(&dest).unwrap(), data);
            let modes = modes.lock().unwrap();
            assert!(modes.iter().all(|m| m.0 == DownloadMode::Single));
            assert_eq!(modes.last().unwrap().1, 100_000);
            assert_eq!(files(&dir), vec!["a.bin"]);
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D2 + D10: HTTP 오류는 한국어 Err이고 파일이 남지 않는다.
    #[test]
    fn http_error_is_korean_err_and_leaves_nothing() {
        let (dir, s) = (tmp_dir("http"), serve(pattern(10), Opt { status: 404, ..Default::default() }));
        let err = plain(&s.url, &dir.join("a.bin")).unwrap_err();
        assert!(err.contains("HTTP 404") && err.chars().any(|c| ('가'..='힣').contains(&c)), "{err}");
        assert!(files(&dir).is_empty());
        std::fs::remove_dir_all(dir).ok();
    }

    // D6: 일시적 끊김은 재시도로 복구되고, 계속 실패하면 몇 초 안에 Err로 끝난다.
    #[test]
    fn transient_drops_retry_and_persistent_failure_errs_quickly() {
        let (data, dir) = (pattern(1_000_000), tmp_dir("retry"));
        let s = serve(data.clone(), Opt { short_first: 3, ..Default::default() });
        let dest = dir.join("a.bin");
        assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()));
        assert_eq!(std::fs::read(&dest).unwrap(), data);

        let (dir2, s2) = (tmp_dir("retry2"), serve(data, Opt { short_first: 10_000, ..Default::default() }));
        let t = Instant::now();
        let err = plain(&s2.url, &dir2.join("a.bin")).unwrap_err();
        assert!(t.elapsed() < Duration::from_secs(15), "{:?}", t.elapsed());
        assert!(!err.is_empty() && !dir2.join("a.bin").exists());
        std::fs::remove_dir_all(dir).ok();
        std::fs::remove_dir_all(dir2).ok();
    }

    // D5: 시작이 어긋난 Content-Range / 더 긴 본문은 Err이거나 올바른 파일이어야 하며 틀린 파일은 없다.
    #[test]
    fn wrong_range_or_oversize_never_yields_wrong_file() {
        for o in [Opt { wrong_start: true, ..Default::default() }, Opt { extra_byte: true, ..Default::default() }] {
            let (data, dir) = (pattern(500_000), tmp_dir("bad"));
            let (s, dest) = (serve(data.clone(), o.clone()), dir.join("a.bin"));
            let r = plain(&s.url, &dest);
            // 요청보다 긴 본문은 범위를 지키지 않는 서버다: 앞부분만 받아도 엉뚱한 바이트일 수 있다
            assert!(r.is_err(), "어긋난 Content-Range / 요청보다 긴 본문은 Err");
            if dest.exists() {
                assert_eq!(std::fs::read(&dest).unwrap(), data);
            } else {
                assert!(r.is_err());
            }
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D6: 프로브가 한 번 끊겨도 재시도해서 받는다.
    #[test]
    fn probe_failure_is_retried() {
        let (data, dir) = (pattern(300_000), tmp_dir("probe"));
        let s = serve(data.clone(), Opt { drop_probe: 1, ..Default::default() });
        let dest = dir.join("a.bin");
        assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()));
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        std::fs::remove_dir_all(dir).ok();
    }

    // D6: 연결을 연 채 침묵하는 구간은 STALL_SECS 뒤 포기하고 받은 곳부터 다시 받는다.
    #[test]
    fn stalled_piece_is_given_up_and_retried() {
        let (data, dir) = (pattern(1_000_000), tmp_dir("stall"));
        let s = serve(data.clone(), Opt { stall_first: 1, ..Default::default() });
        let dest = dir.join("a.bin");
        let t = Instant::now();
        assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()));
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        assert!(t.elapsed() < Duration::from_secs(STALL_SECS + 5), "{:?}", t.elapsed());
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 서버가 침묵해도(분할·단일 모두) 취소는 약 1초 안에 듣는다.
    #[test]
    fn cancel_is_prompt_while_the_server_is_silent() {
        for ignore_range in [false, true] {
            let o = Opt { ignore_range, stall_first: 100, ..Default::default() };
            let (dir, s) = (tmp_dir("silent"), serve(pattern(1_000_000), o));
            let dest = dir.join("a.bin");
            let cancel = Arc::new(AtomicBool::new(false));
            let flag = cancel.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(500));
                flag.store(true, Ordering::SeqCst);
            });
            let t = Instant::now();
            assert_eq!(run(&format!("{}/file.bin", s.url), &dest, cancel, |_| {}).unwrap(), None);
            assert!(t.elapsed() < Duration::from_millis(1900), "ignore_range={ignore_range}: {:?}", t.elapsed());
            assert!(!dest.exists());
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D5: 길이 헤더 없이 요청보다 긴 응답(초과 바이트가 늦게 따로 온다)도 Err다.
    #[test]
    fn oversized_response_without_length_is_err() {
        let (dir, s) = (tmp_dir("nolen"), serve(pattern(500_000), Opt { extra_byte: true, no_length: true, ..Default::default() }));
        let dest = dir.join("a.bin");
        assert!(plain(&s.url, &dest).is_err());
        assert!(!dest.exists());
        std::fs::remove_dir_all(dir).ok();
    }

    // D5: 프로브(200)가 알려 준 크기와 본 요청의 Content-Length가 다르면 잘린 파일을 내놓지 않는다.
    #[test]
    fn single_stream_length_contradicting_probe_is_err() {
        let o = Opt { ignore_range: true, short_plain: true, ..Default::default() };
        let (dir, s) = (tmp_dir("shortplain"), serve(pattern(400_000), o));
        let dest = dir.join("a.bin");
        assert!(plain(&s.url, &dest).is_err());
        assert!(!dest.exists());
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 재시도 사이 대기 중에도 취소는 약 1초 안에 듣는다.
    #[test]
    fn cancel_is_prompt_during_retry_backoff() {
        let (dir, s) = (tmp_dir("backoff"), serve(pattern(1_000_000), Opt { short_first: 10_000, ..Default::default() }));
        let dest = dir.join("a.bin");
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = cancel.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(600));
            flag.store(true, Ordering::SeqCst);
        });
        let t = Instant::now();
        assert_eq!(run(&format!("{}/file.bin", s.url), &dest, cancel, |_| {}).unwrap(), None);
        assert!(t.elapsed() < Duration::from_millis(1200), "{:?}", t.elapsed());
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 미리 켠 취소는 프로브만 보내고 Ok(None), 목적 파일 없음, 재개 상태 유지.
    #[test]
    fn preset_cancel_returns_none_and_keeps_state() {
        let (dir, s) = (tmp_dir("pre"), serve(pattern(200_000), Opt::default()));
        let dest = dir.join("a.bin");
        let r = run(&format!("{}/file.bin", s.url), &dest, Arc::new(AtomicBool::new(true)), |_| {});
        assert_eq!(r.unwrap(), None);
        assert!(!dest.exists() && side(&dest, ".download").exists() && side(&dest, ".download.state").exists());
        assert_eq!(s.log.lock().unwrap().len(), 1, "프로브만");
        std::fs::remove_dir_all(dir).ok();
    }

    // D4 + D7: 전송 중 취소 → 상태 유지 → 같은 URL 재실행은 빠진 만큼만 받는다.
    #[test]
    fn cancel_midway_then_resume_fetches_only_missing() {
        let (data, dir) = (pattern(8_000_000), tmp_dir("resume"));
        let dest = dir.join("a.bin");
        let s = serve(data.clone(), Opt { chunk_ms: 20, ..Default::default() });
        let cancel = Arc::new(AtomicBool::new(false));
        let (flag, served) = (cancel.clone(), s.served.clone());
        std::thread::spawn(move || {
            while served.load(Ordering::SeqCst) < 2_000_000 {
                std::thread::sleep(Duration::from_millis(1));
            }
            flag.store(true, Ordering::SeqCst);
        });
        assert_eq!(run(&format!("{}/file.bin", s.url), &dest, cancel, |_| {}).unwrap(), None);
        assert!(!dest.exists());
        let state = std::fs::read_to_string(side(&dest, ".download.state")).unwrap();
        let left: u64 = state.split_once("sum ").unwrap().0.lines().skip(1).map(|l| {
            let (a, b) = l.split_once(' ').unwrap();
            b.parse::<u64>().unwrap() - a.parse::<u64>().unwrap()
        }).sum();

        let s2 = serve(data.clone(), Opt::default());
        assert_eq!(plain(&s2.url, &dest).unwrap(), Some(dest.clone()));
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        let fetched = s2.served.load(Ordering::SeqCst) - 1;
        assert!(fetched >= left && fetched <= left + 8 * 1024 * 1024, "남은 {left} 외 낭비 과다: {fetched}");
        assert!(left < 8_000_000, "취소 전 진행분은 다시 받지 않는다");
        assert_eq!(files(&dir), vec!["a.bin"]);
        std::fs::remove_dir_all(dir).ok();
    }

    // D4: 상태 파일이 가리키는 남은 구간만 요청한다.
    #[test]
    fn saved_state_limits_requests_to_remaining_spans() {
        let (data, dir) = (pattern(1_000_000), tmp_dir("state"));
        let dest = dir.join("a.bin");
        let mut part = data.clone();
        part[300_000..400_000].fill(0);
        part[900_000..].fill(0);
        std::fs::write(side(&dest, ".download"), &part).unwrap();
        std::fs::write(side(&dest, ".download.state"), state_text(1_000_000, &[(300_000, 400_000), (900_000, 1_000_000)]))
            .unwrap();
        let s = serve(data.clone(), Opt::default());
        assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()));
        assert_eq!(std::fs::read(&dest).unwrap(), data);
        assert_eq!(s.served.load(Ordering::SeqCst), 200_000 + 1, "남은 구간 + 프로브 1바이트");
        assert_eq!(files(&dir), vec!["a.bin"]);
        std::fs::remove_dir_all(dir).ok();
    }

    // D4 + D5: 다른 크기/손상/범위 밖 상태는 버리고 처음부터 받는다.
    #[test]
    fn foreign_or_damaged_state_is_discarded() {
        let data = pattern(400_000);
        for (state, part_len) in [
            ("garbage", 400_000),
            ("999\n0 999\n", 400_000),             // 다른 파일(크기 다름)
            ("400000\n0 99999999\n", 400_000),     // 범위가 파일 밖
            ("400000\n5 3\n", 400_000),            // 뒤집힌 범위
            ("400000\nabc def\n", 400_000),        // 숫자가 아님
            ("400000\nend\n", 12_345),            // 임시 파일 크기 불일치
            ("400000\n0 400000\n", 400_000),      // 끝 표식 없음(쓰다 만 파일)
        ] {
            let dir = tmp_dir("dmg");
            let dest = dir.join("a.bin");
            std::fs::write(side(&dest, ".download"), vec![0xFFu8; part_len]).unwrap();
            std::fs::write(side(&dest, ".download.state"), state).unwrap();
            let s = serve(data.clone(), Opt::default());
            assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()), "{state:?}");
            assert_eq!(std::fs::read(&dest).unwrap(), data, "{state:?}");
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D4 + D5: 덮어쓰다 잘려 숫자 일부만 바뀐 상태(합 불일치)와 겹치는 구간은 버리고 처음부터 받는다.
    #[test]
    fn torn_or_overlapping_state_is_discarded() {
        let data = pattern(1_000_000);
        let torn = state_text(1_000_000, &[(200_000, 1_000_000)]).replacen("200000", "900000", 1);
        let overlap = state_text(1_000_000, &[(0, 500_000), (0, 500_000)]);
        for state in [torn, overlap] {
            let dir = tmp_dir("torn");
            let dest = dir.join("a.bin");
            std::fs::write(side(&dest, ".download"), vec![0u8; 1_000_000]).unwrap();
            std::fs::write(side(&dest, ".download.state"), &state).unwrap();
            let s = serve(data.clone(), Opt::default());
            assert_eq!(plain(&s.url, &dest).unwrap(), Some(dest.clone()));
            assert_eq!(std::fs::read(&dest).unwrap(), data, "{state:.40}");
            std::fs::remove_dir_all(dir).ok();
        }
    }

    // D7: 남은 구간이 없는 재개 상태라도 취소가 미리 켜져 있으면 완성 파일을 내놓지 않는다.
    #[test]
    fn completed_state_with_preset_cancel_is_not_published() {
        let (data, dir) = (pattern(300_000), tmp_dir("donecancel"));
        let dest = dir.join("a.bin");
        std::fs::write(side(&dest, ".download"), &data).unwrap();
        std::fs::write(side(&dest, ".download.state"), state_text(300_000, &[])).unwrap();
        let s = serve(data, Opt::default());
        let r = run(&format!("{}/file.bin", s.url), &dest, Arc::new(AtomicBool::new(true)), |_| {});
        assert_eq!(r.unwrap(), None);
        assert!(!dest.exists());
        std::fs::remove_dir_all(dir).ok();
    }

    // D8: received는 뒤로 가지 않고, total은 전체 크기, mode는 경로, 초당 4회 안팎, 마지막은 received == total.
    #[test]
    fn progress_is_monotone_rate_limited_and_ends_at_total() {
        let (dir, s) = (tmp_dir("prog"), serve(pattern(4_000_000), Opt { chunk_ms: 25, ..Default::default() }));
        let events = Arc::new(Mutex::new(Vec::new()));
        let e = events.clone();
        let t = Instant::now();
        run(&format!("{}/file.bin", s.url), &dir.join("a.bin"), Arc::new(AtomicBool::new(false)), move |p| {
            e.lock().unwrap().push(p)
        })
        .unwrap();
        let secs = t.elapsed().as_secs_f64();
        let ev = events.lock().unwrap();
        assert!(ev.windows(2).all(|w| w[0].received <= w[1].received));
        assert!(ev.iter().all(|p| p.total == Some(4_000_000) && p.mode == DownloadMode::Segmented));
        assert_eq!(ev.last().unwrap().received, 4_000_000);
        assert!(ev.len() as f64 <= secs * 4.0 + 4.0, "{} events in {secs}s", ev.len());
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 단일 스트림 취소도 Ok(None)이며 폴더에 아무것도 남기지 않는다.
    #[test]
    fn cancel_during_single_stream_returns_none_and_cleans_up() {
        let (dir, s) = (tmp_dir("scancel"), serve(pattern(2_000_000), Opt { ignore_range: true, chunk_ms: 20, ..Default::default() }));
        let cancel = Arc::new(AtomicBool::new(false));
        let (flag, served) = (cancel.clone(), s.served.clone());
        std::thread::spawn(move || {
            while served.load(Ordering::SeqCst) < 300_000 {
                std::thread::sleep(Duration::from_millis(1));
            }
            flag.store(true, Ordering::SeqCst);
        });
        assert_eq!(run(&format!("{}/file.bin", s.url), &dir.join("a.bin"), cancel, |_| {}).unwrap(), None);
        assert!(files(&dir).is_empty());
        std::fs::remove_dir_all(dir).ok();
    }

    // D9: 비어 있는 쿠키/리퍼러는 보내지 않는다.
    #[test]
    fn empty_cookie_and_referer_are_not_sent() {
        let (dir, s) = (tmp_dir("empty"), serve(pattern(1000), Opt::default()));
        let request = DownloadRequest {
            url: format!("{}/file.bin", s.url),
            destination: dir.join("a.bin"),
            cookie_header: Some(String::new()),
            referer: Some(String::new()),
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(download(request, Arc::new(AtomicBool::new(false)), |_| {})).unwrap();
        let log = s.log.lock().unwrap().join("").to_ascii_lowercase();
        assert!(!log.contains("cookie:") && !log.contains("referer:"));
        std::fs::remove_dir_all(dir).ok();
    }

    // D7: 토큰 레지스트리.
    #[test]
    fn cancel_tokens_register_request_clear() {
        let flag = register_cancel("t-w3");
        assert!(!flag.load(Ordering::SeqCst));
        assert!(request_cancel("t-w3") && flag.load(Ordering::SeqCst));
        clear_cancel("t-w3");
        assert!(!request_cancel("t-w3") && !request_cancel("never-registered"));
        assert!(!register_cancel("t-w3").load(Ordering::SeqCst), "새 등록은 새 플래그");
        clear_cancel("t-w3");
    }

    // 스케줄링: 미배정 구간 먼저, 없으면 가장 많이 남은 연결의 뒤 절반, 너무 작으면 포기. 구간은 겹치지 않는다.
    #[test]
    fn claim_takes_todo_then_steals_half_then_gives_up() {
        let big = 10 * MIN_STEAL;
        let state = std::fs::File::create(std::env::temp_dir().join(format!("segdl-claim-{}", std::process::id()))).unwrap();
        let mut sh = Shared { todo: vec![(0, big)], slots: vec![(0, 0); 3], state, done: 0, last_ckpt: 0, last_emit: Instant::now() };
        assert!(sh.claim(0) && sh.slots[0] == (0, big));
        sh.slots[0].0 = 2 * MIN_STEAL; // 연결 0이 조금 받은 뒤
        assert!(sh.claim(1));
        let mid = 2 * MIN_STEAL + 4 * MIN_STEAL;
        assert_eq!((sh.slots[0], sh.slots[1]), ((2 * MIN_STEAL, mid), (mid, big)));
        sh.slots[0].0 = mid - MIN_STEAL; // 남은 구간이 2*MIN_STEAL 미만
        sh.slots[1].0 = big - MIN_STEAL;
        assert!(!sh.claim(2) && sh.slots[2] == (0, 0));
        assert_eq!(sh.remaining().len(), 2);
    }
}
