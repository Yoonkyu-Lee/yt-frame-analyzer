//! Transparency probe: observes facts a reader should be able to predict from the code alone.
//! `cargo test --test probe -- --nocapture --test-threads=1` prints `PROBE <id> = <value>` lines.
//! The same questions are given to a fresh reader who sees only the module (probe/QUESTIONS.md).

use seg_harness::segmented_download::{download, DownloadMode, DownloadRequest};
use seg_harness::server::{self, Behavior, RangeMode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

type Ev = Arc<Mutex<Vec<(u64, Option<u64>, DownloadMode, Instant)>>>;

fn data(n: usize, seed: u8) -> Arc<Vec<u8>> {
    Arc::new((0..n).map(|i| ((i * 31 + seed as usize) % 253) as u8).collect())
}
fn temp_dir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("seg-probe-{tag}-{}-{}", std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&d).unwrap();
    d
}
fn run(url: &str, dest: &Path, stop_at: Option<u64>) -> (Result<Option<PathBuf>, String>, Ev, f64) {
    let ev: Ev = Arc::new(Mutex::new(Vec::new()));
    let e2 = ev.clone();
    let cancel = Arc::new(AtomicBool::new(false));
    let flag = cancel.clone();
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    let t = Instant::now();
    let req = DownloadRequest { url: url.into(), destination: dest.into(), cookie_header: None, referer: None };
    let out = rt.block_on(async move {
        tokio::time::timeout(Duration::from_secs(90), download(req, cancel, move |p| {
            e2.lock().unwrap().push((p.received, p.total, p.mode, Instant::now()));
            if stop_at.is_some_and(|s| p.received >= s) { flag.store(true, Ordering::SeqCst); }
        })).await.unwrap_or_else(|_| Err("TIMEOUT".into()))
    });
    (out, ev, t.elapsed().as_secs_f64())
}
fn leftovers(dir: &Path, dest: &Path) -> String {
    let name = dest.file_name().unwrap().to_string_lossy().into_owned();
    let mut v: Vec<String> = std::fs::read_dir(dir).unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|n| *n != name)
        .map(|n| n.replacen(&name, "<dest>", 1))
        .collect();
    v.sort();
    if v.is_empty() { "none".into() } else { v.join(", ") }
}
fn non_probe(s: &server::Server) -> Vec<(u64, Option<u64>)> {
    s.requests().iter().filter_map(|r| r.range).filter(|r| *r != (0, Some(0))).collect()
}
fn p(id: &str, v: impl std::fmt::Display) { println!("PROBE {id} = {v}"); }

#[test]
fn probes() {
    // Q1/Q2: request counts on a Range server, no failures
    for (id, n) in [("Q1_requests_15_bytes", 15usize), ("Q2_requests_20MB", 20_000_000)] {
        let s = server::start(data(n, 1), Behavior::default());
        let dir = temp_dir("q12");
        let (out, _, _) = run(&s.url, &dir.join("f.bin"), None);
        out.unwrap();
        p(id, s.requests().len());
    }
    // Q3: max simultaneous connections
    {
        let s = server::start(data(20_000_000, 2), Behavior { throttle_bps: Some(20_000_000), ..Default::default() });
        let dir = temp_dir("q3");
        run(&s.url, &dir.join("f.bin"), None).0.unwrap();
        p("Q3_max_connections", s.max_active.load(Ordering::SeqCst));
    }
    // Q4: unknown total in Content-Range
    {
        let s = server::start(data(100_000, 3), Behavior { range: RangeMode::UnknownTotal, ..Default::default() });
        let dir = temp_dir("q4");
        let (out, ev, _) = run(&s.url, &dir.join("f.bin"), None);
        out.unwrap();
        let single = ev.lock().unwrap().iter().all(|e| e.2 == DownloadMode::Single);
        p("Q4_mode_unknown_total", if single { "single" } else { "segmented" });
    }
    // Q5: files left after cancelling a segmented download halfway
    {
        let s = server::start(data(6_000_000, 4), Behavior { throttle_bps: Some(2_000_000), ..Default::default() });
        let dir = temp_dir("q5");
        let dest = dir.join("f.bin");
        let (out, _, _) = run(&s.url, &dest, Some(3_000_000));
        p("Q5_left_after_cancel_segmented", format!("{} (result {:?})", leftovers(&dir, &dest), out.map(|o| o.is_some())));
    }
    // Q6: files left after cancelling a single-stream download halfway
    {
        let s = server::start(data(6_000_000, 5), Behavior { range: RangeMode::Ignore, throttle_bps: Some(4_000_000), ..Default::default() });
        let dir = temp_dir("q6");
        let dest = dir.join("f.bin");
        let (out, _, _) = run(&s.url, &dest, Some(3_000_000));
        p("Q6_left_after_cancel_single", format!("{} (result {:?})", leftovers(&dir, &dest), out.map(|o| o.is_some())));
    }
    // Q7 + Q13: persistent connection drops
    {
        let s = server::start(data(2_000_000, 6), Behavior { drop_after: 1000, drop_count: usize::MAX, ..Default::default() });
        let dir = temp_dir("q7");
        let (out, _, secs) = run(&s.url, &dir.join("f.bin"), None);
        p("Q7_seconds_until_err_when_every_transfer_drops", format!("{secs:.1} (is_err {})", out.is_err()));
        let mut per_end = std::collections::HashMap::new();
        for (_, end) in non_probe(&s) { *per_end.entry(end).or_insert(0) += 1; }
        p("Q13_max_attempts_for_one_piece", per_end.values().max().copied().unwrap_or(0));
    }
    // Q8: progress interval (median gap between events, excluding the first and last event)
    {
        let s = server::start(data(12_000_000, 7), Behavior { throttle_bps: Some(1_000_000), ..Default::default() });
        let dir = temp_dir("q8");
        let (_, ev, _) = run(&s.url, &dir.join("f.bin"), None);
        let ev = ev.lock().unwrap();
        let inner: Vec<Instant> = ev.iter().map(|e| e.3).skip(1).take(ev.len().saturating_sub(2)).collect();
        let mut gaps: Vec<u128> = inner.windows(2).map(|w| (w[1] - w[0]).as_millis()).collect();
        gaps.sort();
        p("Q8_typical_ms_between_progress_events", gaps.get(gaps.len() / 2).copied().unwrap_or(0));
    }
    // Q9: resume state from a same-size file with different content
    {
        let dir = temp_dir("q9");
        let dest = dir.join("f.bin");
        let a = server::start(data(30_000_000, 10), Behavior { throttle_bps: Some(2_000_000), ..Default::default() });
        run(&a.url, &dest, Some(15_000_000)).0.ok();
        let other = data(30_000_000, 99);
        let b = server::start(other.clone(), Behavior::default());
        let (out, _, _) = run(&b.url, &dest, None);
        let verdict = match out {
            Ok(Some(_)) if std::fs::read(&dest).unwrap() == *other => "correct",
            Ok(Some(_)) => "corrupt (mixed bytes)",
            Ok(None) => "cancelled",
            Err(_) => "err",
        };
        p("Q9_same_size_different_content_after_cancel", verdict);
    }
    // Q10: destination already exists
    {
        let dir = temp_dir("q10");
        let dest = dir.join("f.bin");
        std::fs::write(&dest, b"old contents").unwrap();
        let body = data(300_000, 11);
        let s = server::start(body.clone(), Behavior::default());
        let (out, _, _) = run(&s.url, &dest, None);
        let now = std::fs::read(&dest).unwrap();
        let verdict = match out {
            Ok(Some(_)) if now == *body => "replaced",
            Ok(Some(_)) => "ok but wrong bytes",
            Ok(None) => "cancelled",
            Err(_) if now == b"old contents" => "err, old file kept",
            Err(_) => "err",
        };
        p("Q10_existing_destination", verdict);
    }
    // Q11: is a slow connection's remaining work taken over?
    {
        let s = server::start(data(40_000_000, 12), Behavior { throttle_bps: Some(10_000_000), slow_every: 3, slow_bps: 1_000_000, ..Default::default() });
        let dir = temp_dir("q11");
        let (_, _, secs) = run(&s.url, &dir.join("f.bin"), None);
        let r = non_probe(&s);
        let overlap = r.iter().enumerate().any(|(i, a)| r.iter().skip(i + 1).any(|b| {
            let ae = a.1.unwrap_or(u64::MAX); let be = b.1.unwrap_or(u64::MAX);
            a.0 <= be && b.0 <= ae && a.1 != b.1
        }));
        p("Q11_slow_connection_work_taken_over", format!("{} ({secs:.1}s for 40MB, {} range requests)", if overlap { "yes" } else { "no" }, r.len()));
    }
    // Q12: empty file
    {
        let s = server::start(Arc::new(Vec::new()), Behavior::default());
        let dir = temp_dir("q12e");
        let dest = dir.join("f.bin");
        let (out, _, _) = run(&s.url, &dest, None);
        p("Q12_zero_byte_file", match out { Ok(Some(_)) if dest.exists() => "empty file created", Ok(_) => "no file", Err(_) => "err" });
    }
}
