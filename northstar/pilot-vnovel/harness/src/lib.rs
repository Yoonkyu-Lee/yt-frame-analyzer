//! Harness crate: the module under test plus a scriptable HTTP test server.

#[path = "../arm/segmented_download.rs"]
pub mod segmented_download;

pub mod server;
