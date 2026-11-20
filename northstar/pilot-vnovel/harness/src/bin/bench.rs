//! P1 benchmark: median wall time of 3 runs per scenario (release build).
use seg_harness::segmented_download::{download, DownloadRequest};
use seg_harness::server::{self, Behavior};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;

fn once(body: &Arc<Vec<u8>>, b: &Behavior) -> f64 {
    let s = server::start(body.clone(), b.clone());
    let dir = std::env::temp_dir().join(format!("seg-bench-{}-{}", std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("f.bin");
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    let t = Instant::now();
    let out = rt.block_on(download(
        DownloadRequest { url: s.url.clone(), destination: dest.clone(), cookie_header: None, referer: None },
        Arc::new(AtomicBool::new(false)),
        |_| {},
    ));
    let secs = t.elapsed().as_secs_f64();
    assert!(matches!(out, Ok(Some(_))), "bench download failed: {out:?}");
    assert_eq!(std::fs::metadata(&dest).unwrap().len(), body.len() as u64);
    std::fs::remove_dir_all(dir).ok();
    secs
}

fn median(body: &Arc<Vec<u8>>, b: &Behavior) -> f64 {
    let mut v: Vec<f64> = (0..3).map(|_| once(body, b)).collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[1]
}

fn main() {
    const MB: usize = 1_000_000;
    let body80 = Arc::new((0..80 * MB).map(|i| (i % 251) as u8).collect::<Vec<u8>>());
    let body512 = Arc::new((0..512 * MB).map(|i| (i % 251) as u8).collect::<Vec<u8>>());
    let throttled = Behavior { throttle_bps: Some(10 * MB as u64), ..Default::default() };
    let uneven = Behavior { throttle_bps: Some(10 * MB as u64), slow_every: 3, slow_bps: MB as u64, ..Default::default() };
    println!(
        "perf  throttled 80MB@10MB/s/conn {:.2}s | uneven 80MB (every 3rd conn 1MB/s) {:.2}s | unthrottled 512MB {:.2}s",
        median(&body80, &throttled),
        median(&body80, &uneven),
        median(&body512, &Behavior::default()),
    );
}
