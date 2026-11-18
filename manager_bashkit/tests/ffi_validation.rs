//! bash_exec 的 FFI 校验：空指针、非法 JSON、字段缺失与实际执行。

use std::ffi::{c_char, CStr, CString};

use serde_json::Value;

fn call(raw: Option<&str>) -> (i32, Value) {
    let input = raw.map(|text| CString::new(text).expect("no NUL"));
    let input_ptr = input
        .as_ref()
        .map_or(std::ptr::null(), |value| value.as_ptr());
    let mut output: Vec<c_char> = vec![0; 1_048_576];
    let rc = bashkit::post_bash_exec(input_ptr, output.as_mut_ptr());
    let text = unsafe { CStr::from_ptr(output.as_ptr()) }
        .to_str()
        .expect("UTF-8 output");
    let body = if text.is_empty() {
        Value::Null
    } else {
        serde_json::from_str(text).expect("JSON output")
    };
    (rc, body)
}

#[test]
fn null_input_returns_400100() {
    let (rc, body) = call(None);
    assert_eq!(rc, 0);
    assert_eq!(body["code"], 400100);
}

#[test]
fn invalid_json_returns_400102() {
    let (rc, body) = call(Some("not-json"));
    assert_eq!(rc, 0);
    assert_eq!(body["code"], 400102);
}

#[test]
fn missing_commands_returns_400103() {
    let (_, body) = call(Some("{}"));
    assert_eq!(body["code"], 400103);
}

#[test]
fn non_string_commands_returns_400103() {
    let (_, body) = call(Some(r#"{"commands": 42}"#));
    assert_eq!(body["code"], 400103);
}

#[test]
fn echo_returns_stdout_and_zero_exit() {
    let (rc, body) = call(Some(r#"{"commands": "printf 'hello\\n'"}"#));
    assert_eq!(rc, 0);
    assert_eq!(body["code"], 200);
    assert_eq!(body["data"]["exit_code"], 0);
    assert_eq!(body["data"]["stdout"], "hello\n");
    assert_eq!(body["data"]["stderr"], "");
}

#[test]
fn nonzero_exit_is_not_an_error() {
    let (_, body) = call(Some(r#"{"commands": "exit 3"}"#));
    assert_eq!(body["code"], 200);
    assert_eq!(body["data"]["exit_code"], 3);
}

#[test]
fn stderr_is_captured() {
    let (_, body) = call(Some(r#"{"commands": "echo oops >&2"}"#));
    assert_eq!(body["code"], 200);
    assert_eq!(body["data"]["stderr"], "oops\n");
}

#[test]
fn shell_state_does_not_leak_between_calls() {
    let (_, first) = call(Some(r#"{"commands": "export FOO=1; echo set"}"#));
    assert_eq!(first["data"]["stdout"], "set\n");
    let (_, second) = call(Some(r#"{"commands": "echo ${FOO:-unset}"}"#));
    assert_eq!(second["data"]["stdout"], "unset\n");
}

#[test]
fn oversized_output_is_truncated_and_flagged() {
    // 约 1.2 MiB 输出，超过 1 MiB 输出缓冲与 bashkit 默认输出上限。
    let (rc, body) = call(Some(
        r#"{"commands": "for i in $(seq 1 2000); do printf '%0600d\\n' \"$i\"; done"}"#,
    ));
    assert_eq!(rc, 0);
    assert_eq!(body["code"], 200);
    assert_eq!(body["data"]["stdout_truncated"], true);
    assert!(body["data"]["stdout"].as_str().unwrap().len() < 1_048_576);
}
