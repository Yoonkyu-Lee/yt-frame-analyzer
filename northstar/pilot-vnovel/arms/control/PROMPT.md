# Control prompt (Codex, no INTENT.md, no tests beyond the module's own)

This directory is a small Rust harness crate. `arm/segmented_download.rs` is a module from a desktop app: an HTTP Range based segmented downloader (resume, retry, cancel, progress). Its public items (`DownloadRequest`, `DownloadMode`, `DownloadProgress`, `register_cancel`, `request_cancel`, `clear_cancel`, `download`) are used by other code in the app and must keep their signatures. Make the module as minimal as possible while keeping it working. Edit `arm/segmented_download.rs` in place.

Run: `codex exec --skip-git-repo-check --ephemeral -s workspace-write --json` in an isolated copy of `arms/control/work`.
