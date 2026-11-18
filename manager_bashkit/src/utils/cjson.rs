//! JSON 协议层：入参解析、统一响应与输出缓冲。
//!
//! 错误码：
//! - `400100` json_in 为空
//! - `400101` 请求非 UTF-8
//! - `400102` 请求 JSON 非法
//! - `400103` 必填字段缺失或类型错误
//! - `400200` bashkit 执行失败
//! - `400201` 响应超出输出缓冲区

// C ABI 约定：裸指针由宿主保证有效。
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::{c_char, CStr};

use serde_json::{json, Value};

/// 宿主提供的输出缓冲区大小（含结尾 NUL）。
pub const MAX_OUTPUT_LEN: usize = 1_048_576;

/// 解析宿主传入的 NUL 结尾 UTF-8 JSON。
pub fn parse_json_input(json_in: *const c_char) -> Result<Value, (i32, String)> {
    if json_in.is_null() {
        return Err((400100, "json_in 为空".to_string()));
    }
    let bytes = unsafe { CStr::from_ptr(json_in) }.to_bytes();
    let text = std::str::from_utf8(bytes).map_err(|_| (400101, "请求不是 UTF-8".to_string()))?;
    serde_json::from_str(text).map_err(|_| (400102, "请求 JSON 非法".to_string()))
}

/// 写入成功响应 `{"code":200,"data":...,"msg":"success"}`。
pub fn write_success(json_out: *mut c_char, data: Value) -> i32 {
    write_body(
        json_out,
        &json!({ "code": 200, "data": data, "msg": "success" }),
    )
}

/// 写入失败响应 `{"code":...,"data":null,"msg":...}`。
pub fn write_error(json_out: *mut c_char, code: i32, msg: &str) -> i32 {
    write_body(
        json_out,
        &json!({ "code": code, "data": Value::Null, "msg": msg }),
    )
}

fn write_body(json_out: *mut c_char, body: &Value) -> i32 {
    if json_out.is_null() {
        return -1;
    }
    let text = body.to_string();
    let bytes = text.as_bytes();
    if bytes.len() >= MAX_OUTPUT_LEN {
        return -2;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), json_out.cast::<u8>(), bytes.len());
        *json_out.add(bytes.len()) = 0;
    }
    0
}
