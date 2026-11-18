//! everruns/bashkit 的执行封装。
//!
//! 每次调用创建独立的 `Bash` 实例：虚拟文件系统与 shell 变量在调用之间不共享。

// C ABI 约定：调用方提供有效的输出指针。
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use bashkit_core::Bash;
use serde_json::json;

use crate::utils::cjson::{write_error, write_success, MAX_OUTPUT_LEN};

/// 一次执行的结果。
pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

/// 在沙箱中执行脚本。脚本自身的非零退出码不算错误。
pub fn run(script: &str) -> Result<ExecOutput, (i32, String)> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|err| (400200, format!("创建 tokio runtime 失败: {err}")))?;

    let mut bash = Bash::new();
    let result = runtime
        .block_on(bash.exec(script))
        .map_err(|err| (400200, format!("执行失败: {err}")))?;

    Ok(ExecOutput {
        stdout: result.stdout.text_lossy().into_owned(),
        stderr: result.stderr.text_lossy().into_owned(),
        exit_code: result.exit_code,
        stdout_truncated: result.stdout_truncated,
        stderr_truncated: result.stderr_truncated,
    })
}

/// 组装响应；超过输出缓冲区时按流裁剪并置位截断标记。
pub fn respond(json_out: *mut std::ffi::c_char, output: ExecOutput) -> i32 {
    let mut stdout = output.stdout;
    let mut stderr = output.stderr;
    let mut stdout_truncated = output.stdout_truncated;
    let mut stderr_truncated = output.stderr_truncated;

    loop {
        let body = json!({
            "exit_code": output.exit_code,
            "stdout": stdout,
            "stderr": stderr,
            "stdout_truncated": stdout_truncated,
            "stderr_truncated": stderr_truncated,
        });
        if serde_json::to_string(&body).map_or(0, |text| text.len()) < MAX_OUTPUT_LEN {
            return write_success(json_out, body);
        }
        if stdout.len() >= stderr.len() && !stdout.is_empty() {
            stdout = shrink(&stdout);
            stdout_truncated = true;
        } else if !stderr.is_empty() {
            stderr = shrink(&stderr);
            stderr_truncated = true;
        } else {
            return write_error(json_out, 400201, "响应超出输出缓冲区");
        }
    }
}

/// 截到一半长度，并对齐 UTF-8 字符边界。
fn shrink(text: &str) -> String {
    let mut end = text.len() / 2;
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let mut out = text[..end].to_string();
    out.push_str("\n...[truncated]");
    out
}
