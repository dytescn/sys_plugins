# bashkit FFI

`bashkit` 是 [everruns/bashkit](https://github.com/everruns/bashkit) 的插件封装，
编译为 `cdylib`（`bashkit.dll` / `bashkit.so` / `bashkit.dylib`），
由宿主通过 `POST /bashkit` + `FFI-Symbol: bash_exec` 调用
（cyctron 查找 `libary/` 下对应动态库，并按 `<method>_<FFI-Symbol>` 拼接导出函数）。

只导出一个符号：`post_bash_exec`。

## 1. 调用约定

```c
int (*)(const char *json_in, char *json_out);
```

| 约定 | 说明 |
| --- | --- |
| 输入 | NUL 结尾的 UTF-8 JSON |
| 输出 | 调用方提供 1,048,576 字节缓冲区（含结尾 NUL） |
| 返回值 | `0` 已写入；`-1` 输出指针为空；`-2` 缓冲区不足 |
| 响应 | `{"code":200,"data":{...},"msg":"success"}`；失败时 `code != 200`，读取 `msg` |

HTTP 形态：

```http
POST http://127.0.0.1:44944/bashkit
Content-Type: application/libary
FFI-Symbol: bash_exec

{"commands": "echo hello"}
```

> 缓冲契约：宿主必须分配 1 MiB。dytes 通用约定是 64 KiB；若宿主仍按 64 KiB
> 分配，本库按 1 MiB 写入会越界。升级宿主前不要发布本包。

## 2. 接口

### 2.1 `bash_exec`

请求：

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `commands` | string | 是 | 要执行的 bash 脚本，可含多条命令、管道、重定向与控制流；允许空串 |

未知字段忽略。空脚本返回 `code=200`、`exit_code=0`、输出为空。

响应 `data`：

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `exit_code` | integer | 脚本退出码；非零不视为调用失败 |
| `stdout` | string | 标准输出 |
| `stderr` | string | 标准错误 |
| `stdout_truncated` | bool | stdout 是否被 bashkit 输出上限或 1 MiB 缓冲裁剪 |
| `stderr_truncated` | bool | stderr 是否被裁剪 |

示例：

```json
{
  "code": 200,
  "data": {
    "exit_code": 0,
    "stdout": "hello\n",
    "stderr": "",
    "stdout_truncated": false,
    "stderr_truncated": false
  },
  "msg": "success"
}
```

## 3. 截断规则

1. bashkit 自身按默认 `max_output_bytes`（1 MiB / 流）截断，并置位对应标记；
2. 序列化响应后若仍 `>= 1,048,576` 字节，取较长的一侧流反复减半，
   追加 `\n...[truncated]`，并把该侧标记置为 `true`，直到整体放得下；
3. 只标记真正被本层裁剪的流——一侧被裁剪不代表另一侧也被裁剪；
4. 两侧都已为空但仍超限时返回 `400201`（正常配置下不会发生）。

因此响应 JSON 的字节数始终小于 1 MiB；单侧 `stdout` / `stderr` 可能接近 1 MiB。

## 4. 错误码

| 码 | 含义 |
| --- | --- |
| `200` | 成功；脚本自身失败时看 `data.exit_code` |
| `400100` | `json_in` 为空指针 |
| `400101` | 请求不是 UTF-8 |
| `400102` | 请求 JSON 非法 |
| `400103` | `commands` 缺失或不是字符串 |
| `400200` | bashkit 执行失败（解释器错误、资源限制触发等） |
| `400201` | 裁剪后响应仍超出输出缓冲区 |

参数类错误（`4001xx`）返回时 `data` 为 `null`。

## 5. 沙箱语义

- 基于 `bashkit::Bash`，167 个内建命令在进程内重实现，不 fork / exec。
- 宿主文件系统不可见：`/` 是内存 VFS，`/etc/passwd` 等是虚拟文件。
- 网络默认拒绝：`curl` / `wget` 返回 `network access not configured`。
- 默认虚拟身份：`whoami=sandbox`、`hostname=bashkit-sandbox`、`cwd=/home/user`。
- 每次调用创建独立 `Bash` 实例：变量、别名、cwd 与虚拟文件不跨调用保留。
- 资源限制沿用 bashkit 默认：命令数、循环次数、函数深度、输入 / 输出大小等。

## 6. MCP 工具定义

工具定义直接内联在包根 `mcp.json` 的 `tools[0]`，字段遵循
`dytes/docs/mcp.md` §6.3 / §7：

| 字段 | 取值 | 说明 |
| --- | --- | --- |
| `name` | `bash_exec` | snake_case，包内唯一 |
| `ffi_symbol` | `bash_exec` | 供安装流程映射为能力符号 |
| `title` | 执行 Bash 脚本 | ≤ 120 字符 |
| `description` | 见 `mcp.json` | 面向模型，≤ 500 字符 |
| `inputSchema` | 根 `object` | 仅 `commands`，`required`，`additionalProperties: false` |
| `outputSchema` | 根 `object` | `{code, data, msg}`；`data` 为对象或 `null` |
| `annotations` | 4 个 hint 均为 `false` | 沙箱内执行，不读写真实世界，不幂等 |
| `transport.symbol` | `post_bash_exec` | dytes 规范字段，导出函数名 |
| `transport.endpoint` / `ffiSymbol` | `/bashkit` / `bash_exec` | cyctron 约定，与 `symbol` 并存 |

结构：

```json
{
  "tools": [
    {
      "name": "bash_exec",
      "ffi_symbol": "bash_exec",
      "inputSchema": { "type": "object", "properties": { "commands": { "type": "string" } } },
      "transport": { "endpoint": "/bashkit", "ffiSymbol": "bash_exec", "symbol": "post_bash_exec" }
    }
  ]
}
```

该文件顶层另有 `server` / `category` / `tags`，属 `sys_plugins` 历史扩展；
dytes 规范只识别 `tools` / `schemaVersion`，其余字段会被忽略（见包 README §5）。

## 7. 实现

| 文件 | 内容 |
| --- | --- |
| `src/ffi/bash_exec.rs` | `post_bash_exec` 入口与参数校验 |
| `src/utils/exec.rs` | `Bash::exec` 封装、响应组装与截断 |
| `src/utils/cjson.rs` | JSON 协议、统一响应与 1 MiB 缓冲 |

上游依赖：`bashkit = "0.21"`（Cargo 中重命名为 `bashkit_core`，避免与库名冲突）。
