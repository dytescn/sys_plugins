# sys_plugins

Piksel 系统能力库（`sys`）的 MCP 插件包集合。当前仓库只包含 `manager_bashkit`：

| 目录 | 对应能力库 | 端点 | 工具数 |
| --- | --- | --- | --- |
| `manager_bashkit/` | `bashkit`（自带 DLL） | `/bashkit` | 1 |

每个包把底层能力整理为：

- **MCP 工具**：包根 `mcp.json`，工具可引用独立文件或内联声明，
  `transport.endpoint` / `transport.ffiSymbol` 指回真实能力库；
- **提示词**：`prompts.json`，安装时整体覆盖该插件的提示词数组；
- **任务流**：`taskflow.json`，安装时按 `plugin_uuid` + `ffi_symbol` 合并；
- **技能**：`skills.json` + `skills/<name>.md`（`use_ai: true` 时使用）。

## 安装

```bash
dytes install plugin manager_bashkit
```

包安装到 `plugins/<name>/`。`manager_bashkit` 自带能力库（`Cargo.toml` + `src/`，
依赖上游 `bashkit`），`mcp.json` 直接放包根并内联工具定义，
`platform-exports` 包含 `target/release/` 下的动态库，安装时复制到
`<应用根>/libary/`；因此它的 `transport` 没有 `library` / `library_uuid`，
也没有 `dependencies-sys`，且 `use_ai: false`、不打包 skills。

## 约定

- MCP 清单遵循 `dytes/docs/mcp.md`：工具可引用独立文件或内联声明，工具名、schema、
  `transport` 按规范填写；`mcp.json` 顶层只依赖 `tools` / `schemaVersion`，
  历史扩展字段（`server` / `category` / `tags`）仅作兼容保留；
- `dytes.json`：`type: design_plugins`、`plugins_type: sys_plugins`、`targets: ["common"]`，
  `mcp` 指向包根 `mcp.json`；`use_ai` 按是否需要技能设置，为 `true` 时再声明
  `skills: "skills.json"`（`manager_bashkit` 为 `false`，不声明、不导出 skills）；
- `mcp.json` 的 `tools` 必须非空，每个工具文件都通过 `exports` 导出；
- `prompts.json` 顶层为数组或 `{ "prompts": [...] }`，每条 `content` 去掉首尾空白后
  UTF-8 不超过 150 字节；
- `taskflow.json` 顶层为数组或 `{ "taskflows": [...] }`，元素必须包含
  `plugin_uuid`、`plugin_name`、`desc`、`ffi_symbol`，且 `ffi_symbol` 必须是库真实导出符号；
- 更新 FFI 时同步修改工具描述文件、`mcp.json` 的 `tools`、`dytes.json` 的
  `exports` / `platform-exports`；启用 skills 时还要同步 `skills.json` 与
  `skills/<name>.md`。

## 发布

git tag 与发布流程见 [`.github/workflows/README.md`](.github/workflows/README.md)。
