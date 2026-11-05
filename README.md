# sys_plugins

Piksel 系统能力库（`sys`）的 MCP 插件包集合。

每个子目录对应一个系统能力库，把该库的 FFI 整理为：

- **MCP 工具**：`mcp/mcp.json` + 每个 FFI 一个工具 JSON，`transport.endpoint` /
  `transport.ffiSymbol` 指回真实能力库；
- **技能**：`skills.json`，`use_ai: true` 时由 `plugin_lib` 导入 `plugins.db.skills`；
- **技能文档**：`skills/<name>.md`，说明工具用途与调用规则。

| 目录 | 对应能力库 | 端点 | 工具数 |
| --- | --- | --- | --- |
| `manager_baseinfo/` | `info_lib` | `/info_lib` | 8 |
| `manager_materil/` | `material_lib` | `/material_lib` | 24 |
| `manager_ostfil/` | `ostfil` | `/libostfil` | 3 |
| `manager_plugins/` | `plugin_lib` | `/plugin_lib` | 36 |
| `manager_sysinfo/` | `sysinfo` | `/sysinfo` | 13 |
| `manager_workspace/` | `workspace_lib` | `/workspace_lib` | 44 |

## 安装

```bash
dytes install plugin manager_baseinfo
dytes install plugin manager_materil
dytes install plugin manager_ostfil
dytes install plugin manager_plugins
dytes install plugin manager_sysinfo
dytes install plugin manager_workspace
```

包安装到 `plugins/<name>/`，不会复制或重命名已安装的系统库 DLL；
工具调用仍走 `POST /<endpoint>` + `FFI-Symbol`。

## 约定

- `dytes.json`：`type: design_plugins`、`plugins_type: plugins`、`targets: ["common"]`、
  `use_ai: true`，`mcp` / `skills` 指向包内文件；
- `mcp.json` 的 `tools` 必须非空，每个工具文件都通过 `exports` 导出；
- 更新 FFI 时同步修改 `mcp/<tool>.json`、`mcp.json` 的 `tools`、`dytes.json` 的
  `exports` / `platform-exports`、`skills/<name>.md` 与 `skills.json`。
