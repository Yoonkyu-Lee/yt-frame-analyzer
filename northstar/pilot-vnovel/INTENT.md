# INTENT: `segmented_download.rs` (VNovel Library)

Owner: the user (the app's author). Draft recovered from the code, the module's four commits
(d39e979, d3a2664, d4d4666, 369aed7), `docs/ARCHITECTURE.md` §2/§4 and the only caller
(`dist_host.rs` `download_dist_files`). Changes need the owner's approval.

## Purpose
The file host's web page (driven by `dist_host.rs` in a hidden WebView2) yields a final, signed file
URL plus cookies. This module turns that URL into a complete file in the VN folder: fast on hosts
that throttle each connection, resumable after cancel or an app restart, and never leaving a
silently corrupted file. Real files are large (the module was validated on a 4.22 GiB file by
SHA-256).

## Public contract (the caller must keep compiling unchanged)
```rust
pub struct DownloadRequest { pub url: String, pub destination: PathBuf,
                             pub cookie_header: Option<String>, pub referer: Option<String> }
pub enum DownloadMode { Segmented, Single }          // Serialize, lowercase ("segmented"/"single")
pub struct DownloadProgress { pub received: u64, pub total: Option<u64>, pub mode: DownloadMode }
pub fn register_cancel(token: &str) -> Arc<AtomicBool>
pub fn request_cancel(token: &str) -> bool           // true if the token was registered
pub fn clear_cancel(token: &str)
pub async fn download<F: Fn(DownloadProgress) + Send + Sync + 'static>(
    request: DownloadRequest, cancel: Arc<AtomicBool>, on_progress: F,
) -> Result<Option<PathBuf>, String>                 // Some(path) done, None cancelled, Err failed
```
`DownloadMode`, `DownloadProgress` and `DownloadRequest` may derive more traits, never fewer
(`Debug, Clone, Copy, PartialEq, Eq, Serialize` as today).

## Intents
| ID | Intent |
|---|---|
| D1 | The destination file is byte-identical to what the server holds (any size, including 0 bytes). On success nothing else is left in the folder. The destination appears only when complete |
| D2 | First request: `Range: bytes=0-0`. A 206 with a known total → parallel download. Anything else usable (200, unknown total `*`) → one plain stream. An HTTP error → `Err`, no file |
| D3 | Parallel download uses several connections, **never more than 8** (hosts are shared; the user's other traffic matters) |
| D4 | Cancel or app restart keeps resume state next to the destination. The next `download` of the same URL into the same destination fetches only what is missing (up to ~1 MiB per connection may be refetched). State left by a different file (different size) is discarded |
| D5 | Never a wrong final file: a 206 whose `Content-Range` does not start where it was asked, a body longer than asked, a short body, or damaged resume state must lead to a retry, a re-download or `Err`, never to a completed file with wrong bytes |
| D6 | Transient network failures are retried (about 4 tries per piece, short growing backoff), **the probe included**; a persistent failure ends in `Err` within seconds. **A server that keeps the connection open but sends nothing for 30 s counts as a failure** (so a fully stalled server ends in `Err` within about 2 minutes). The single stream is not retried automatically: it would restart from byte 0 |
| D7 | Cancellation is prompt **(within about 1 s, even while the server sends nothing)**: `download` returns `Ok(None)`, keeps resume state, and the destination does not appear. Tokens: `register_cancel` makes a fresh flag, `request_cancel` sets it, `clear_cancel` forgets it |
| D8 | Progress: `received` never goes backwards, `total` is the full size when known, `mode` tells which path is used; at most about 4 events per second; the last event reports `received == total` |
| D9 | Every request sends `Accept-Encoding: identity` (byte offsets must be file offsets), the cookie and referer when non-empty, and a desktop Chrome user agent; up to 10 redirects; 20 s connect timeout |
| D10 | Errors are human-readable Korean strings (the caller shows them as they are) |
| P1 | No regression vs. the start on: 80 MB with a per-connection limit, 80 MB where some connections are slow, 512 MB unthrottled (`run.sh` bench) |

## Revision 1 (owner-approved, after the W4 skeptic rounds)
- D6/D7 above: stall detection (30 s without data), probe retries, cancel within ~1 s during a stall.
- Deliberately not done: automatic single-stream retry (restarts from 0), fsync of data/state (power-loss
  durability; cost on external HDDs; disclosed in the module header `Weak:`).
- Transition: the resume-state format changed (one text `.download.state` instead of three sidecars). No
  migration code: before switching versions, finish or remove paused downloads in the old app. A leftover
  old download restarts from 0 and leaves an orphan `.download.json`; it never produces a wrong file.

## Scale and assumptions
- Files from a few bytes to tens of GB. One download at a time (the caller's queue is sequential).
- The same URL serves the same bytes while a download is in progress or paused.

## Non-goals
- Resuming the single-stream path (servers without Range cannot resume).
- Checksums, ETag/Last-Modified validation, bandwidth limiting, HTTP/2 tuning, multiple
  simultaneous downloads, a configurable number of connections.
- Keeping the current sidecar file names or format (only this module reads them).
