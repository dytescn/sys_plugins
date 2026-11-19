# manager_bashkit

基于 [everruns/bashkit](https://github.com/everruns/bashkit) 的 bash 执行插件。
bashkit 是纯 Rust 的虚拟 bash 解释器：167 个内建命令在进程内重实现，
不 fork / exec，默认不访问宿主文件系统与网络，文件系统是内存中的虚拟 VFS。

本包只暴露一个 MCP 工具 `bash_exec`：执行任意 bash 脚本并返回执行结果。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_bashkit` |
| 版本 | `0.2.0` |
| 类型 | `design_plugins`（`plugins_type: sys_plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: false` |
| 上游库 | `bashkit = "0.21"`（crates.io / GitHub `everruns/bashkit`） |
| 产物 | `target/release/bashkit.dll` / `bashkit.so` / `bashkit.dylib` |
| HTTP 端点 | `POST http://127.0.0.1:44944/bashkit` |
| FFI 符号 | `bash_exec`（导出函数 `post_bash_exec`） |
| 工具数 | 1 |
| 输出缓冲 | 1 MiB（1,048,576 字节，含结尾 NUL） |
| MCP | `mcp.json`（包根，内联工具定义） |
| 提示词 | `prompts.json`（2 条，安装时整体覆盖） |
| 任务流 | `taskflow.json`（1 条，`post_bash_exec`） |
| 技能 | 无（`use_ai: false`，不声明 / 导出 skills.json） |

本包自带能力库 DLL：Cargo 依赖上游 `bashkit`，导出 `post_bash_exec`，
安装时随包复制到 `<应用根>/libary/`。因此工具的 `transport` 只有
`endpoint` + `ffiSymbol`，没有 `library` / `library_uuid`，也不需要
`dependencies-sys`。

## 1. 目录

```text
manager_bashkit/
├── Cargo.toml
├── Cargo.lock
├── dytes.json
├── mcp.json                    # 包根扫描入口，内联 bash_exec
├── prompts.json                # 提示词数组
├── taskflow.json               # 任务流数组
├── docs/
│   └── ffi.md
├── src/
│   ├── lib.rs
│   ├── ffi/
│   │   ├── mod.rs
│   │   └── bash_exec.rs
│   └── utils/
│       ├── mod.rs
│       ├── cjson.rs
│       └── exec.rs
├── skills/
│   └── manager_bashkit.md      # 说明文档，不导出
├── tests/
│   └── ffi_validation.rs
└── README.md
```

`docs/` 与 `skills/` 只放包内说明文档，不写入 `exports` / `platform-exports`，
安装后不随包分发。

`mcp.json` 直接放在包根，工具以**内联对象**声明（不再有独立工具描述文件），
dytes 扫描时优先命中，安装后也在同一目录。

## 2. 工具

| MCP 工具 | FFI-Symbol | 调用说明 |
| --- | --- | --- |
| `bash_exec` | `bash_exec` | 入参 `commands`（string）；返回 `exit_code`、`stdout`、`stderr` 与两个截断标记。 |

请求 / 响应字段、错误码、截断规则见 [`docs/ffi.md`](docs/ffi.md)。

## 3. 调用

```http
POST http://127.0.0.1:44944/bashkit
Content-Type: application/libary
FFI-Symbol: bash_exec

{"commands": "echo hello"}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 沙箱行为

| 能力 | 行为 |
| --- | --- |
| 进程 | 不启动真实进程，全部命令在解释器内运行 |
| 宿主文件系统 | 不可见；`/` 是内存 VFS，`/etc/passwd` 等是虚拟文件 |
| 网络 | 默认拒绝；`curl` / `wget` 返回 `network access not configured` |
| 默认身份 | `whoami=sandbox`、`hostname=bashkit-sandbox`、`cwd=/home/user` |
| 调用间状态 | 不保留；每次调用新建 `Bash` 实例，变量、cwd、文件都不跨调用 |
| 退出码 | 脚本非零退出码不算调用失败：`code=200`，看 `data.exit_code` |
| 资源限制 | 沿用 bashkit 默认值（命令数、循环、函数深度、输入 / 输出大小等） |

## 5. MCP 规范对齐

依据 `dytes/docs/mcp.md`（v1.1，2026-11-18）逐项对照：

| 规范项 | 本包实现 |
| --- | --- |
| §4.1 包布局 | `dytes.json` + 包根 `mcp.json`（内联工具）；DLL 在 `target/release/` |
| §5 `dytes.json` | `mcp: "mcp.json"`、`use_ai: false`、无 `skills` 字段；`platform-exports` 的 windows / linux / macos 都包含 `mcp.json` 与 DLL |
| §6.1 `mcp.json` | `tools` 为非空数组；工具以内联对象声明 |
| §6.3 内联形态 | `tools[0]` 直接携带 `name`、`ffi_symbol`、`title`、`description`、`inputSchema`、`outputSchema`、`annotations`、`transport` |
| §6.4 工具字段 | 内联对象含规范要求的名称与 schema；`transport.symbol` 为导出函数 `post_bash_exec` |
| §7.1 命名 | `bash_exec` 为 snake_case，无空白/控制字符；跨插件重名时由宿主按 `插件名/工具名` 消歧 |
| §7.2 参数 schema | 根为 `object`，声明 `properties` / `required`，`additionalProperties: false`，属性带 `description` |
| §7.3 输出 | 根为 `object`；返回体为 `{code, data, msg}`，`code == 200` 表示成功 |
| annotations | 四个 hint 均为 `false`：沙箱内执行，不读写真实世界、不具幂等性 |
| §9.2 安装期绑定 | 无 `transport.library` / `library_uuid`，跳过能力库绑定校验；DLL 随包复制到 `libary/` |
| `prompts.json` | 顶层 `{ "prompts": [...] }`；每条含稳定 `id`、`name`、非空 `content`，`content` ≤ 150 字节 |
| `taskflow.json` | 顶层数组；4 个必填字段齐全，`ffi_symbol` 为真实导出符号 `post_bash_exec` |

提示词与任务流依据 `sys/designplugin_lib/docs/plugin-spec/content.md`：
`sys_plugins` 类型下两者均为可选，本包提供是为了让安装流程把提示词写入
`prompts` 集合与 `sys_plugins` kv，并把任务流合并进共享 `taskflow` 集合。

已知差异（有意保留）：

- `mcp.json` 顶层的 `server` / `category` / `tags` 是 `sys_plugins` 的历史扩展。
  dytes 规范只识别 `tools` 与 `schemaVersion`，其余字段会被忽略，0.2.4 起输出
  `mcp.json 字段已知` 警告。保留这些字段是为了兼容现有 sys 宿主读取端点与分类；
  宿主确认不再依赖后可删除（端点同时记录在工具 `transport.endpoint`）。
- `transport` 同时写规范字段 `symbol`（`post_bash_exec`）与 cyctron 字段
  `endpoint` / `ffiSymbol`；dytes 只定义 `symbol`，宿主按 `symbol` 或工具名解析符号。
- 输出缓冲为 1 MiB，而 dytes 通用约定是 64 KiB，宿主侧必须同步扩大（见 §7）。

## 6. 构建与测试

```bash
cargo build --release   # Windows: target/release/bashkit.dll
                        # Linux/macOS: 构建后复制 libbashkit.{so,dylib} 为 bashkit.{so,dylib}
cargo test              # 9 个 FFI 用例
cargo clippy --all-targets
```

`Cargo.lock` 需要提交。`dytes.json` 的 `platform-exports` 指向
`target/release/` 下的动态库，发布前必须先构建。

测试覆盖：空指针、非法 JSON、`commands` 缺失 / 类型错误、echo、非零退出码、
stderr 捕获、调用间状态隔离、超 1 MiB 输出的裁剪与标记。

## 7. 安装

```bash
dytes install plugin manager_bashkit
```

安装后目录为 `plugins/manager_bashkit/`，其中包含 `dytes.json`、`mcp.json`
（内联工具）、`prompts.json` 与 `taskflow.json`；当前平台的
`bashkit.dll` / `bashkit.so` / `bashkit.dylib` 复制到
`<应用根>/libary/`。安装流程会整体覆盖该插件的提示词数组，并按
`plugin_uuid` + `ffi_symbol` 合并任务流元素。

> 宿主侧必须按 1 MiB 分配 `json_out`（含结尾 NUL）。dytes 文档里的通用约定是
> 64 KiB，cyctron / 宿主未同步扩大缓冲时会出现越界写。

## 8. 约束

- 输出超过 1 MiB 时，工具把较大的一侧流反复减半并追加 `\n...[truncated]`，
  只给被裁剪的流置位 `*_truncated`；响应本身始终小于 1 MiB。
- 需要保留状态的流程必须写在同一个 `commands` 脚本里。
- 需要真实文件、进程或系统信息时，使用宿主提供的其它系统插件，不要指望 bashkit 访问宿主。
