//! Counter-page sizes from `resuma/benchmark/results.json`.
//!
//! Regenerated 2026-09-03 with `node benchmark/run.mjs --resuma-only`
//! (Node `gzipSync` / `brotliCompressSync` on minified `runtime/dist`).
//! First interaction = `loader.js` + `core.js` + Counter handler chunk.

pub const RESUMA_INITIAL: &str = "1021 B";
pub const RESUMA_FIRST: &str = "9.90 KiB";
pub const RESUMA_STATIC: &str = "0 B";

pub const LOADER_RAW: &str = "2.08 KiB";
pub const LOADER_GZIP: &str = "1021 B";
pub const LOADER_BROTLI: &str = "854 B";

pub const CORE_RAW: &str = "26.06 KiB";
pub const CORE_GZIP: &str = "8.78 KiB";
pub const CORE_BROTLI: &str = "7.90 KiB";

pub const NEXT_GZIP: &str = "142.43 KiB";
/// 145851 B Next first-load gzip / 1021 B Resuma loader gzip.
pub const SMALLER_THAN_NEXT: &str = "143×";
