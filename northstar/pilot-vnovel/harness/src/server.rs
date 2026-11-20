//! Scriptable HTTP/1.1 file server for the hidden tests and the benchmark.
//! One thread per connection; `Connection: close` unless `keep_alive` is set.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RangeMode {
    /// 206 + `Content-Range: bytes a-b/total` for any Range request.
    Full,
    /// Ignores Range, always 200 with the whole body.
    Ignore,
    /// Range requests get 206 with an unknown total (`bytes a-b/*`); plain GET gets 200.
    UnknownTotal,
}

#[derive(Clone, Debug)]
pub struct Behavior {
    pub range: RangeMode,
    /// Respond with this status (and empty body) to every request.
    pub status: Option<u16>,
    /// The first `drop_count` non-probe body responses send only `drop_after` bytes, then close.
    pub drop_after: usize,
    pub drop_count: usize,
    /// Non-probe range responses claim a start one byte later than requested.
    pub wrong_range: bool,
    /// Non-probe range responses carry one extra byte.
    pub oversize: bool,
    /// Per-connection bandwidth in bytes/s (None = unthrottled).
    pub throttle_bps: Option<u64>,
    /// Every `slow_every`-th connection (1-based) is throttled to `slow_bps` instead.
    pub slow_every: usize,
    pub slow_bps: u64,
    /// Path that redirects (302) to the file path.
    pub redirect: bool,
    /// Keep connections open between requests (HTTP/1.1 keep-alive) instead of closing each one.
    pub keep_alive: bool,
    /// The probe (`bytes=0-0`) claims this total instead of the real one.
    pub probe_total: Option<u64>,
    /// A plain GET (no Range) is answered with `206 bytes 100-199/total` and those 100 bytes.
    pub single_partial: bool,
    /// The first `stall_count` non-probe body responses send `stall_after` bytes, then go silent
    /// with the connection left open (a stalled server).
    pub stall_after: usize,
    pub stall_count: usize,
    /// The first `drop_probe` probe requests are closed without any response.
    pub drop_probe: usize,
}

impl Default for Behavior {
    fn default() -> Self {
        Behavior {
            range: RangeMode::Full,
            status: None,
            drop_after: 0,
            drop_count: 0,
            wrong_range: false,
            oversize: false,
            throttle_bps: None,
            slow_every: 0,
            slow_bps: 0,
            redirect: false,
            keep_alive: false,
            probe_total: None,
            single_partial: false,
            stall_after: 0,
            stall_count: 0,
            drop_probe: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Logged {
    pub path: String,
    pub headers: Vec<(String, String)>, // lower-case names
    pub range: Option<(u64, Option<u64>)>,
}

impl Logged {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }
}

pub struct Server {
    pub url: String,
    pub redirect_url: String,
    pub log: Arc<Mutex<Vec<Logged>>>,
    pub active: Arc<AtomicUsize>,
    pub max_active: Arc<AtomicUsize>,
    pub served_bytes: Arc<AtomicU64>,
    /// Open TCP connections (keep-alive aware) and their maximum.
    pub open: Arc<AtomicUsize>,
    pub max_open: Arc<AtomicUsize>,
}

impl Server {
    pub fn requests(&self) -> Vec<Logged> {
        self.log.lock().unwrap().clone()
    }
}

pub fn start(data: Arc<Vec<u8>>, behavior: Behavior) -> Server {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let log = Arc::new(Mutex::new(Vec::new()));
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let served = Arc::new(AtomicU64::new(0));
    let drops_left = Arc::new(AtomicUsize::new(behavior.drop_count));
    let stalls_left = Arc::new(AtomicUsize::new(behavior.stall_count));
    let probe_drops_left = Arc::new(AtomicUsize::new(behavior.drop_probe));
    let conn_no = Arc::new(AtomicUsize::new(0));
    let open = Arc::new(AtomicUsize::new(0));
    let max_open = Arc::new(AtomicUsize::new(0));
    {
        let (log, active, max_active, served) =
            (log.clone(), active.clone(), max_active.clone(), served.clone());
        let (open, max_open) = (open.clone(), max_open.clone());
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                let n = conn_no.fetch_add(1, Ordering::SeqCst) + 1;
                let ctx = Ctx {
                    data: data.clone(),
                    b: behavior.clone(),
                    log: log.clone(),
                    active: active.clone(),
                    max_active: max_active.clone(),
                    served: served.clone(),
                    drops_left: drops_left.clone(),
                    stalls_left: stalls_left.clone(),
                    probe_drops_left: probe_drops_left.clone(),
                    conn_no: n,
                };
                let (open, max_open) = (open.clone(), max_open.clone());
                std::thread::spawn(move || {
                    let now = open.fetch_add(1, Ordering::SeqCst) + 1;
                    max_open.fetch_max(now, Ordering::SeqCst);
                    let _ = handle(stream, ctx);
                    open.fetch_sub(1, Ordering::SeqCst);
                });
            }
        });
    }
    Server {
        url: format!("http://{addr}/file.bin"),
        redirect_url: format!("http://{addr}/redirect"),
        log,
        active,
        max_active,
        served_bytes: served,
        open,
        max_open,
    }
}

struct Ctx {
    data: Arc<Vec<u8>>,
    b: Behavior,
    log: Arc<Mutex<Vec<Logged>>>,
    active: Arc<AtomicUsize>,
    max_active: Arc<AtomicUsize>,
    served: Arc<AtomicU64>,
    drops_left: Arc<AtomicUsize>,
    stalls_left: Arc<AtomicUsize>,
    probe_drops_left: Arc<AtomicUsize>,
    conn_no: usize,
}

fn read_head(stream: &mut TcpStream) -> Option<String> {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1024];
    while !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = stream.read(&mut byte).ok()?;
        if n == 0 {
            return None;
        }
        buf.extend_from_slice(&byte[..n]);
        if buf.len() > 64 * 1024 {
            return None;
        }
    }
    Some(String::from_utf8_lossy(&buf).into_owned())
}

fn parse_range(v: &str) -> Option<(u64, Option<u64>)> {
    let v = v.trim().strip_prefix("bytes=")?;
    let (a, b) = v.split_once('-')?;
    let a = a.trim().parse().ok()?;
    let b = b.trim();
    Some((a, if b.is_empty() { None } else { Some(b.parse().ok()?) }))
}

fn handle(mut stream: TcpStream, c: Ctx) -> std::io::Result<()> {
    loop {
        let Some(head) = read_head(&mut stream) else { return Ok(()) };
        let mut lines = head.split("
");
        let request_line = lines.next().unwrap_or("");
        let path = request_line.split_whitespace().nth(1).unwrap_or("/").to_string();
        let headers: Vec<(String, String)> = lines
            .filter_map(|l| l.split_once(':'))
            .map(|(k, v)| (k.trim().to_ascii_lowercase(), v.trim().to_string()))
            .collect();
        let range = headers.iter().find(|(k, _)| k == "range").and_then(|(_, v)| parse_range(v));
        c.log.lock().unwrap().push(Logged { path: path.clone(), headers, range });

        let now = c.active.fetch_add(1, Ordering::SeqCst) + 1;
        c.max_active.fetch_max(now, Ordering::SeqCst);
        let result = respond(&mut stream, &c, &path, range);
        c.active.fetch_sub(1, Ordering::SeqCst);
        match result {
            Ok(true) if c.b.keep_alive => continue,
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}

fn respond(
    stream: &mut TcpStream,
    c: &Ctx,
    path: &str,
    range: Option<(u64, Option<u64>)>,
) -> std::io::Result<bool> {
    let conn = if c.b.keep_alive { "keep-alive" } else { "close" };
    if c.b.redirect && path == "/redirect" {
        write!(stream, "HTTP/1.1 302 Found\r\nLocation: /file.bin\r\nContent-Length: 0\r\nConnection: {conn}\r\n\r\n")?;
        return Ok(true);
    }
    if let Some(code) = c.b.status {
        write!(stream, "HTTP/1.1 {code} Status\r\nContent-Length: 0\r\nConnection: {conn}\r\n\r\n")?;
        return Ok(true);
    }
    let total = c.data.len() as u64;
    let probe = range == Some((0, Some(0)));
    if probe && take(&c.probe_drops_left) {
        return Ok(false); // close without a response
    }
    let (status, start, end, content_range) = match (c.b.range, range) {
        (RangeMode::Full, Some((a, b))) | (RangeMode::UnknownTotal, Some((a, b))) if a < total => {
            let e = b.unwrap_or(total - 1).min(total - 1);
            let shown = if c.b.wrong_range && !probe { a + 1 } else { a };
            let t = match (c.b.range, c.b.probe_total) {
                (RangeMode::Full, Some(lie)) if probe => lie.to_string(),
                (RangeMode::Full, _) => total.to_string(),
                _ => "*".into(),
            };
            ("206 Partial Content", a, e, Some(format!("bytes {shown}-{e}/{t}")))
        }
        (_, None) if c.b.single_partial && total > 200 => {
            ("206 Partial Content", 100, 199, Some(format!("bytes 100-199/{total}")))
        }
        _ if total == 0 => ("200 OK", 0, 0, None),
        _ => ("200 OK", 0, total - 1, None),
    };
    let mut body: Vec<u8> = if total == 0 { Vec::new() } else { c.data[start as usize..=end as usize].to_vec() };
    if c.b.oversize && content_range.is_some() && !probe {
        body.push(0xAB);
    }
    let mut head = format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\n", body.len());
    if let Some(cr) = content_range {
        head.push_str(&format!("Content-Range: {cr}\r\n"));
    }
    head.push_str(&format!("Accept-Ranges: bytes\r\nConnection: {conn}\r\n\r\n"));
    stream.write_all(head.as_bytes())?;

    let stall = !probe && take(&c.stalls_left);
    let mut limit = body.len();
    if stall {
        limit = c.b.stall_after.min(body.len());
    } else if !probe && c.b.drop_count > 0 {
        let took = c
            .drops_left
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
            .is_ok();
        if took {
            limit = c.b.drop_after.min(body.len());
        }
    }
    let bps = if c.b.slow_every > 0 && c.conn_no % c.b.slow_every == 0 {
        Some(c.b.slow_bps)
    } else {
        c.b.throttle_bps
    };
    const CHUNK: usize = 64 * 1024;
    let mut sent = 0;
    while sent < limit {
        let n = CHUNK.min(limit - sent);
        stream.write_all(&body[sent..sent + n])?;
        c.served.fetch_add(n as u64, Ordering::SeqCst);
        sent += n;
        // no sleep after the last chunk: the client already has every byte, and a lingering
        // handler would be counted as a concurrent connection.
        if let Some(bps) = bps.filter(|_| sent < limit) {
            std::thread::sleep(Duration::from_secs_f64(n as f64 / bps as f64));
        }
    }
    if stall {
        // go silent with the connection open; the client must give up on its own
        let _ = stream.flush();
        std::thread::sleep(Duration::from_secs(300));
        return Ok(false);
    }
    if limit < body.len() {
        let _ = stream.shutdown(std::net::Shutdown::Both);
        return Ok(false);
    }
    Ok(true)
}

/// Takes one from a countdown; true while it was still positive.
fn take(left: &AtomicUsize) -> bool {
    left.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1)).is_ok()
}
