# Git 发布规范

本目录的 workflow 负责把 `sys_plugins` 里的插件包发布到 dytes。以下规范同时约束
人工发布与 CI 发布，发布前逐项确认。

## 1. 版本

- `dytes.json` 的 `version` 与 `Cargo.toml` 的 `[package].version` **必须**一致；
- 版本使用 semver，**不得**复用已发布版本号；
- 破坏性变更**必须**提升 minor 以上：删除或重命名 MCP 工具、修改必填参数或类型、
  更换能力库绑定、改变返回结构语义；
- 兼容变更（新增可选字段、补充描述、修文档）可走 patch。

## 2. 提交

- `main` 分支**必须**保持可发布状态；
- 发布提交**必须**同时包含：源码、`dytes.json`、`mcp.json`、工具描述文件、
  `prompts.json`、`taskflow.json` 与相关文档；
- 提交信息沿用 `feat:` / `fix:` / `chore:` / `docs:` 前缀，一个发布一个提交；
- **不得**提交 `target/`、构建产物、密钥或本机绝对路径。

## 3. Tag

- 格式：`<package>-v<version>`，例如 `manager_bashkit-v0.1.0`；
- tag 中的版本**必须**与 `dytes.json` 的 `version` 完全一致，否则 workflow 直接失败；
- 使用 annotated tag：`git tag -a manager_bashkit-v0.1.0 -m "manager_bashkit 0.1.0"`；
- tag **必须**打在发布提交上；已推送的 tag **不得**移动、删除后复用或覆盖。

## 4. 本地发布前检查

在包目录执行：

```bash
cargo test
cargo clippy --all-targets
cargo build --release
dytes check .          # 或 dytes check manager_bashkit
```

四项全部通过后再打 tag。`cargo build --release` 是必须步骤：`platform-exports`
指向 `target/release/` 下的动态库，未构建会导致发布缺文件。

本地 `dytes` **必须**使用 ≥ 0.2.4：更早版本不支持 `mcp.json` 的内联工具对象，
会把对象误判为文件名并让 `check` 失败。

## 5. CI 发布（`release.yml`）

触发方式：

- push tag：`manager_*-v*`，包名与版本从 tag 解析；
- 手动：`workflow_dispatch`，输入包目录名（如 `manager_bashkit`）。

流程：

1. 校验 tag 版本、`dytes.json` 版本与 `Cargo.toml` 版本三者一致；
2. 安装 Rust 工具链，在包目录执行 `cargo test` 与 `cargo build --release`；
3. 调用 `dytes publish`（action 版本 `v0.2.4`）发布，**禁止**使用 `--skip-check`。

**平台**：发布产物是当前 runner 平台的动态库。现有 workflow 使用 `ubuntu-latest`，
只产出 Linux `.so`。Windows 的 `bashkit.dll` 与 macOS 的 `.dylib` **必须**在对应平台的
runner 或本地按相同流程发布；不要用 Linux 产物冒充 Windows 包。

## 6. 导出产物

发布时只上传 `exports` / 当前平台 `platform-exports` 中列出的文件：

| 文件 | 说明 |
| --- | --- |
| `dytes.json` | 清单 |
| `mcp.json` | MCP 清单（包根，工具内联在 `tools` 中） |
| `prompts.json` | 提示词数组 |
| `taskflow.json` | 任务流数组 |
| `target/release/<lib>` | 当前平台动态库 |

`src/`、`docs/`、`skills/`、`tests/`、`Cargo.*` **不**导出。

## 7. 失败与回滚

- 发布失败**不得**通过 `--skip-check` 绕过；修复后提升版本、重新打 tag；
- 已发布版本**不删除**、不回滚 tag；需要撤销时发布新的 patch；
- 不要手工修改云端 `design_plugins/packages.json`；
- 重跑失败 job 前确认版本号未被占用。

## 8. 禁止项

- 移动或复用已推送的 tag；
- 在版本不一致时发布；
- 未构建动态库就发布；
- 用 `--skip-check` 发布；
- 发布包含密钥、绝对路径或内部地址的文件。
