//! Real-network check: `cargo run --release --bin fetch -- <url> <dest>`
//! Prints the result, the mode(s) used, the request time and the file size.
use seg_harness::segmented_download::{download, DownloadMode, DownloadRequest};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (url, dest) = (args[1].clone(), std::path::PathBuf::from(&args[2]));
    let modes: Arc<Mutex<Vec<DownloadMode>>> = Arc::default();
    let m = modes.clone();
    let rt = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build().unwrap();
    let t = Instant::now();
    let out = rt.block_on(download(
        DownloadRequest { url, destination: dest.clone(), cookie_header: None, referer: None },
        Arc::new(AtomicBool::new(false)),
        move |p| m.lock().unwrap().push(p.mode),
    ));
    let mut modes = modes.lock().unwrap().clone();
    modes.dedup();
    let size = std::fs::metadata(&dest).map(|m| m.len()).unwrap_or(0);
    println!("result {out:?} | modes {modes:?} | {:.2}s | {size} bytes", t.elapsed().as_secs_f64());
}
