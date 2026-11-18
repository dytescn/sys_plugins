//! manager_bashkit：把 everruns/bashkit 的沙箱 bash 执行整理为 MCP 能力库。
//!
//! 导出 C ABI `post_bash_exec`，由 cyctron 通过 `POST /bashkit` +
//! `FFI-Symbol: bash_exec` 调用。

pub mod ffi;
pub mod utils;

pub use ffi::*;
