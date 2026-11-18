//! 执行接口：在 bashkit 沙箱中运行任意 bash 脚本。

use std::ffi::c_char;

use serde_json::Value;

use crate::utils::cjson::{parse_json_input, write_error};
use crate::utils::exec;

#[unsafe(no_mangle)]
pub extern "C" fn post_bash_exec(json_in: *const c_char, json_out: *mut c_char) -> i32 {
    let input: Value = match parse_json_input(json_in) {
        Ok(value) => value,
        Err((code, msg)) => return write_error(json_out, code, &msg),
    };

    let Some(commands) = input.get("commands").and_then(Value::as_str) else {
        return write_error(json_out, 400103, "commands 字段缺失或不是字符串");
    };

    match exec::run(commands) {
        Ok(output) => exec::respond(json_out, output),
        Err((code, msg)) => write_error(json_out, code, &msg),
    }
}
