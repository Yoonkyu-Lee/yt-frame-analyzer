Simplified `arm/segmented_download.rs`, reducing production code by about 36% while preserving all public signatures, resume compatibility, retries, cancellation, progress, and single-stream fallback.

Validation passed: formatting check, all 7 existing tests, and 7 temporary regression tests. Temporary files were removed.