//! Hidden intent tests: public API only (INTENT.md D1-D10). Arms never see this file.

use seg_harness::segmented_download::{
    clear_cancel, download, register_cancel, request_cancel, DownloadMode, DownloadRequest,
};
use seg_harness::server::{self, Behavior, RangeMode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

type Events = Arc<Mutex<Vec<(u64, Option<u64>, DownloadMode, Instant)>>>;
type Outcome = Result<Option<PathBuf>, String>;

fn data(n: usize, seed: u8) -> Arc<Vec<u8>> {
    Arc::new((0..n).map(|i| ((i * 31 + seed as usize) % 253) as u8).collect())
}

fn temp_dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!(
        "seg-hidden-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn req(url: &str, dest: &Path) -> DownloadRequest {
    DownloadRequest { url: url.to_string(), destination: dest.to_path_buf(), cookie_header: None, referer: None }
}

/// Runs one download; `stop_at` cancels once `received` passes that many bytes.
fn run(request: DownloadRequest, cancel: Arc<AtomicBool>, stop_at: Option<u64>) -> (Outcome, Events) {
    let events: Events = Arc::new(Mutex::new(Vec::new()));
    let ev = events.clone();
    let flag = cancel.clone();
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    let out = rt.block_on(async move {
        tokio::time::timeout(
            Duration::from_secs(60),
            download(request, cancel, move |p| {
                ev.lock().unwrap().push((p.received, p.total, p.mode, Instant::now()));
                if stop_at.is_some_and(|s| p.received >= s) {
                    flag.store(true, Ordering::SeqCst);
                }
            }),
        )
        .await
        .unwrap_or_else(|_| Err("TIMEOUT (60s)".into()))
    });
    (out, events)
}

fn no_cancel() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

/// Files in `dir` other than the destination itself.
fn leftovers(dir: &Path, dest: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p != dest)
        .collect()
}

// ---------- D1: the saved file is byte-identical, and nothing else is left behind ----------
#[test]
fn d1_exact_bytes_for_many_sizes() {
    for &n in &[1usize, 2, 15, 16, 17, 4096, 65_537, 3_000_001] {
        let body = data(n, 7);
        let s = server::start(body.clone(), Behavior::default());
        let dir = temp_dir("d1");
        let dest = dir.join("game.bin");
        let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
        assert_eq!(out.unwrap(), Some(dest.clone()), "size {n}");
        assert_eq!(std::fs::read(&dest).unwrap(), *body, "bytes differ, size {n}");
        assert!(leftovers(&dir, &dest).is_empty(), "sidecars left, size {n}: {:?}", leftovers(&dir, &dest));
    }
}

#[test]
fn d1_empty_file() {
    let s = server::start(Arc::new(Vec::new()), Behavior::default());
    let dir = temp_dir("d1e");
    let dest = dir.join("empty.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap().len(), 0);
}

// ---------- D2: probe with bytes=0-0, fall back to one stream when Range is unusable ----------
#[test]
fn d2_first_request_is_range_probe() {
    let s = server::start(data(100_000, 1), Behavior::default());
    let dir = temp_dir("d2p");
    let (out, _) = run(req(&s.url, &dir.join("f.bin")), no_cancel(), None);
    out.unwrap();
    assert_eq!(s.requests()[0].range, Some((0, Some(0))));
}

#[test]
fn d2_server_ignoring_range_falls_back_to_single() {
    let body = data(200_001, 2);
    let s = server::start(body.clone(), Behavior { range: RangeMode::Ignore, ..Default::default() });
    let dir = temp_dir("d2i");
    let dest = dir.join("f.bin");
    let (out, ev) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    assert!(ev.lock().unwrap().iter().all(|e| e.2 == DownloadMode::Single));
}

#[test]
fn d2_unknown_total_falls_back_to_single() {
    let body = data(150_000, 3);
    let s = server::start(body.clone(), Behavior { range: RangeMode::UnknownTotal, ..Default::default() });
    let dir = temp_dir("d2u");
    let dest = dir.join("f.bin");
    let (out, ev) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    assert!(ev.lock().unwrap().iter().all(|e| e.2 == DownloadMode::Single));
}

#[test]
fn d2_http_error_is_err_without_file() {
    let s = server::start(data(10, 0), Behavior { status: Some(404), ..Default::default() });
    let dir = temp_dir("d2e");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert!(out.is_err());
    assert!(!dest.exists());
}

// ---------- D3: parallel connections, at most 8 ----------
#[test]
fn d3_uses_parallel_connections_up_to_eight() {
    let body = data(4_000_000, 4);
    let s = server::start(body.clone(), Behavior { throttle_bps: Some(4_000_000), ..Default::default() });
    let dir = temp_dir("d3");
    let dest = dir.join("f.bin");
    let (out, ev) = run(req(&s.url, &dest), no_cancel(), None);
    out.unwrap();
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    let max = s.max_active.load(Ordering::SeqCst);
    assert!((2..=8).contains(&max), "max concurrent connections {max}");
    assert!(ev.lock().unwrap().iter().any(|e| e.2 == DownloadMode::Segmented));
}

#[test]
fn d3_keep_alive_server_never_sees_more_than_eight_open_connections() {
    // Real hosts keep connections alive; an unread response (e.g. the probe) then keeps its
    // connection open. Count open TCP connections, not requests.
    let body = data(8_000_000, 4);
    let s = server::start(body.clone(), Behavior { throttle_bps: Some(8_000_000), keep_alive: true, ..Default::default() });
    let dir = temp_dir("d3k");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    out.unwrap();
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    let max = s.max_open.load(Ordering::SeqCst);
    assert!(max <= 8, "max open connections {max}");
}

// ---------- D4: resume after cancel without refetching everything ----------
#[test]
fn d4_resume_after_cancel_refetches_only_missing() {
    // INTENT D4 allows up to ~1 MiB per connection (8) to be refetched; the file is large enough
    // that this allowance cannot hide a resume that keeps nothing.
    let n = 30_000_000u64;
    let stop = n / 2;
    let body = data(n as usize, 5);
    let dir = temp_dir("d4");
    let dest = dir.join("f.bin");
    let slow = server::start(body.clone(), Behavior { throttle_bps: Some(2_000_000), ..Default::default() });
    let (out, _) = run(req(&slow.url, &dest), no_cancel(), Some(stop));
    assert_eq!(out.unwrap(), None, "cancel must be Ok(None)");
    assert!(!dest.exists());
    assert!(!leftovers(&dir, &dest).is_empty(), "resume state must be kept");

    let fast = server::start(body.clone(), Behavior::default());
    let (out, _) = run(req(&fast.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    let refetched = fast.served_bytes.load(Ordering::SeqCst);
    let allowed = (n - stop) + 9 * 1024 * 1024;
    assert!(refetched <= allowed, "refetched {refetched}, allowed {allowed} (missing + 8 MiB + 1 MiB slack)");
    assert!(leftovers(&dir, &dest).is_empty());
}

#[test]
fn d4_state_from_a_different_file_is_not_reused() {
    let dir = temp_dir("d4m");
    let dest = dir.join("f.bin");
    let a = data(5_000_000, 6);
    let slow = server::start(a, Behavior { throttle_bps: Some(2_000_000), ..Default::default() });
    let (out, _) = run(req(&slow.url, &dest), no_cancel(), Some(2_000_000));
    assert_eq!(out.unwrap(), None);
    let b = data(3_333_333, 9); // different size
    let s = server::start(b.clone(), Behavior::default());
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *b);
}

// ---------- D5: never a wrong final file ----------
#[test]
fn d5_corrupted_resume_state_still_gives_exact_bytes() {
    let n = 5_000_000;
    let body = data(n, 8);
    let dir = temp_dir("d5c");
    let dest = dir.join("f.bin");
    let slow = server::start(body.clone(), Behavior { throttle_bps: Some(2_000_000), ..Default::default() });
    let (out, _) = run(req(&slow.url, &dest), no_cancel(), Some(n as u64 / 2));
    assert_eq!(out.unwrap(), None);
    // Overwrite every sidecar with 0xFF (same length), whatever its format is.
    for p in leftovers(&dir, &dest) {
        let len = std::fs::metadata(&p).unwrap().len() as usize;
        std::fs::write(&p, vec![0xFFu8; len.max(1)]).unwrap();
    }
    let s = server::start(body.clone(), Behavior::default());
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
}

#[test]
fn d5_wrong_content_range_is_rejected() {
    let s = server::start(data(500_000, 1), Behavior { wrong_range: true, ..Default::default() });
    let dir = temp_dir("d5w");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert!(out.is_err(), "got {out:?}");
    assert!(!dest.exists());
}

#[test]
fn d5_oversized_segment_is_rejected() {
    let s = server::start(data(500_000, 1), Behavior { oversize: true, ..Default::default() });
    let dir = temp_dir("d5o");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert!(out.is_err(), "got {out:?}");
    assert!(!dest.exists());
}

// added after the skeptic review of W3 (ORACLE_CHANGES #5)
#[test]
fn d5_probe_total_contradicted_by_segments_is_not_a_short_file() {
    let body = data(2_000_000, 13);
    let s = server::start(body.clone(), Behavior { probe_total: Some(1_000_000), ..Default::default() });
    let dir = temp_dir("d5t");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    if let Ok(Some(_)) = out {
        assert_eq!(std::fs::read(&dest).unwrap(), *body, "published a file that is not the server's file");
    }
}

#[test]
fn d5_single_stream_partial_206_is_not_published() {
    let body = data(500_000, 14);
    let s = server::start(body.clone(), Behavior { range: RangeMode::UnknownTotal, single_partial: true, ..Default::default() });
    let dir = temp_dir("d5s");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    if let Ok(Some(_)) = out {
        assert_eq!(std::fs::read(&dest).unwrap(), *body, "published 100 bytes of a 500 KB file");
    }
}

// ---------- D6: transient failures are retried, persistent ones fail in bounded time ----------
#[test]
fn d6_dropped_connections_are_retried() {
    let body = data(2_000_000, 2);
    let s = server::start(body.clone(), Behavior { drop_after: 10_000, drop_count: 3, ..Default::default() });
    let dir = temp_dir("d6");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
}

#[test]
fn d6_persistent_failure_is_err_in_bounded_time() {
    let s = server::start(data(2_000_000, 2), Behavior { drop_after: 1000, drop_count: usize::MAX, ..Default::default() });
    let dir = temp_dir("d6p");
    let dest = dir.join("f.bin");
    let t = Instant::now();
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert!(out.is_err() && !out.as_ref().unwrap_err().contains("TIMEOUT"), "got {out:?}");
    assert!(t.elapsed() < Duration::from_secs(30));
    assert!(!dest.exists());
}

// INTENT revision 1 (owner-approved): stall detection, probe retry, prompt cancel during a stall
#[test]
fn d6_probe_transient_failure_is_retried() {
    let body = data(300_000, 15);
    let s = server::start(body.clone(), Behavior { drop_probe: 1, ..Default::default() });
    let dir = temp_dir("d6p1");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
}

#[test]
fn d6_stalled_transfer_is_given_up_and_retried() {
    // one piece stalls after 1000 bytes; INTENT: 30 s without data = failure, then retry
    let body = data(2_000_000, 16);
    let s = server::start(body.clone(), Behavior { stall_after: 1000, stall_count: 1, ..Default::default() });
    let dir = temp_dir("d6st");
    let dest = dir.join("f.bin");
    let t = Instant::now();
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()), "a stalled piece must be retried, not waited on forever");
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    assert!(t.elapsed() < Duration::from_secs(50), "{:?}", t.elapsed());
}

#[test]
fn d7_cancel_is_prompt_while_the_server_is_silent() {
    for range in [RangeMode::Full, RangeMode::Ignore] {
        let s = server::start(data(2_000_000, 17), Behavior { range, stall_after: 1000, stall_count: usize::MAX, ..Default::default() });
        let dir = temp_dir("d7st");
        let dest = dir.join("f.bin");
        let cancel = Arc::new(AtomicBool::new(false));
        let flag = cancel.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(800));
            flag.store(true, Ordering::SeqCst);
        });
        let t = Instant::now();
        let (out, _) = run(req(&s.url, &dest), cancel, None);
        assert_eq!(out.unwrap(), None, "{range:?}");
        assert!(t.elapsed() < Duration::from_secs(4), "{range:?}: cancel took {:?}", t.elapsed());
        assert!(!dest.exists());
    }
}

// ---------- D7: cancellation ----------
#[test]
fn d7_preset_cancel_is_ok_none() {
    let s = server::start(data(1_000_000, 3), Behavior::default());
    let dir = temp_dir("d7");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), Arc::new(AtomicBool::new(true)), None);
    assert_eq!(out.unwrap(), None);
    assert!(!dest.exists());
}

#[test]
fn d7_preset_cancel_on_empty_single_stream_publishes_nothing() {
    let s = server::start(Arc::new(Vec::new()), Behavior { range: RangeMode::Ignore, ..Default::default() });
    let dir = temp_dir("d7e");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.url, &dest), Arc::new(AtomicBool::new(true)), None);
    assert_eq!(out.unwrap(), None);
    assert!(!dest.exists());
}

#[test]
fn d7_registry_semantics() {
    let t = format!("tok-{}", std::process::id());
    let flag = register_cancel(&t);
    assert!(!flag.load(Ordering::SeqCst));
    assert!(request_cancel(&t));
    assert!(flag.load(Ordering::SeqCst));
    assert!(!request_cancel("no-such-token"));
    clear_cancel(&t);
    assert!(!request_cancel(&t));
}

#[test]
fn d7_cancel_through_registry_mid_transfer() {
    let t = format!("mid-{}", std::process::id());
    let s = server::start(data(4_000_000, 4), Behavior { throttle_bps: Some(1_000_000), ..Default::default() });
    let dir = temp_dir("d7m");
    let dest = dir.join("f.bin");
    let flag = register_cancel(&t);
    let t2 = t.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        request_cancel(&t2);
    });
    let started = Instant::now();
    let (out, _) = run(req(&s.url, &dest), flag, None);
    clear_cancel(&t);
    assert_eq!(out.unwrap(), None);
    assert!(started.elapsed() < Duration::from_secs(5), "cancel must stop promptly");
    assert!(!dest.exists());
}

// ---------- D8: progress ----------
#[test]
fn d8_progress_is_monotone_rate_limited_and_complete() {
    let n = 3_000_000u64;
    let s = server::start(data(n as usize, 5), Behavior { throttle_bps: Some(3_000_000), ..Default::default() });
    let dir = temp_dir("d8");
    let started = Instant::now();
    let (out, ev) = run(req(&s.url, &dir.join("f.bin")), no_cancel(), None);
    out.unwrap();
    let ev = ev.lock().unwrap();
    assert!(!ev.is_empty());
    assert!(ev.windows(2).all(|w| w[0].0 <= w[1].0), "received must not go backwards");
    let last = ev.last().unwrap();
    assert_eq!((last.0, last.1), (n, Some(n)), "final event must report completion");
    let ms = started.elapsed().as_millis() as usize;
    assert!(ev.len() <= ms / 250 + 4, "{} events in {ms} ms: not rate limited", ev.len());
}

// ---------- D9: request headers and redirects ----------
#[test]
fn d9_cookie_referer_identity_encoding() {
    let s = server::start(data(300_000, 6), Behavior::default());
    let dir = temp_dir("d9");
    let mut r = req(&s.url, &dir.join("f.bin"));
    r.cookie_header = Some("sid=abc".into());
    r.referer = Some("https://host.example/page".into());
    let (out, _) = run(r, no_cancel(), None);
    out.unwrap();
    for q in s.requests() {
        assert_eq!(q.header("cookie"), Some("sid=abc"));
        assert_eq!(q.header("referer"), Some("https://host.example/page"));
        assert_eq!(q.header("accept-encoding"), Some("identity"));
    }
}

#[test]
fn d9_empty_cookie_and_referer_are_not_sent() {
    let s = server::start(data(1000, 6), Behavior::default());
    let dir = temp_dir("d9e");
    let mut r = req(&s.url, &dir.join("f.bin"));
    r.cookie_header = Some(String::new());
    r.referer = Some(String::new());
    let (out, _) = run(r, no_cancel(), None);
    out.unwrap();
    for q in s.requests() {
        assert!(q.header("cookie").is_none() && q.header("referer").is_none());
    }
}

#[test]
fn d9_redirect_is_followed() {
    let body = data(250_000, 7);
    let s = server::start(body.clone(), Behavior { redirect: true, ..Default::default() });
    let dir = temp_dir("d9r");
    let dest = dir.join("f.bin");
    let (out, _) = run(req(&s.redirect_url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
}

// ---------- D10: downloading into a real library item leaves its other files alone ----------
fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        std::fs::copy(e.path(), to.join(e.file_name())).unwrap();
    }
}

#[test]
fn d10_download_into_library_item_keeps_its_files() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../library/v26537");
    let lib = temp_dir("d10");
    let folder = lib.join("v26537");
    copy_dir(&fixture, &folder);
    let before: Vec<(std::ffi::OsString, Vec<u8>)> = std::fs::read_dir(&folder)
        .unwrap()
        .map(|e| { let e = e.unwrap(); (e.file_name(), std::fs::read(e.path()).unwrap()) })
        .collect();
    let body = data(2_500_000, 3);
    let s = server::start(body.clone(), Behavior::default());
    let dest = folder.join("Game Setup (v1.02).zip");
    let (out, _) = run(req(&s.url, &dest), no_cancel(), None);
    assert_eq!(out.unwrap(), Some(dest.clone()));
    assert_eq!(std::fs::read(&dest).unwrap(), *body);
    for (name, bytes) in &before {
        assert_eq!(&std::fs::read(folder.join(name)).unwrap(), bytes, "{name:?} changed");
    }
    assert_eq!(std::fs::read_dir(&folder).unwrap().count(), before.len() + 1, "extra files left");
}
