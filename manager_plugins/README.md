# manager_plugins

设计插件管理插件：将 plugin_lib 的插件、能力、技能、安装与扫描 FFI 整理为 MCP 工具与提示词。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_plugins` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: true` |
| 底层库 | `plugin_lib` |
| HTTP 端点 | `POST http://127.0.0.1:44944/plugin_lib` |
| 工具数 | 36 |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| 技能 | `skills.json`（`use_ai: true` 必需） |
| 提示词 | `prompts/manager_plugins.json`、`skills/manager_plugins.md` |

本包只包含清单、MCP 工具定义、技能与提示词，不额外打包 DLL。
`transport` 字段把每个工具指回已安装的 `plugin_lib` 能力库，
因此安装后不会复制或重命名系统库。

## 1. 目录

```text
manager_plugins/
├── dytes.json
├── mcp/
│   ├── mcp.json
│   └── <tool>.json
├── prompts/
│   └── manager_plugins.json
├── skills/
│   └── manager_plugins.md
├── skills.json
└── README.md
```

## 2. 工具

| MCP 工具 | FFI-Symbol | 说明 |
| --- | --- | --- |
| `plugin_get_plugins` | `get_plugins` | 分页查询设计插件，支持分类、关键词、状态与目标软件过滤。 |
| `plugin_get_plugin` | `get_plugin` | 按 uuid 或 name 读取单个插件。 |
| `plugin_get_categories` | `get_categories` | 读取插件分类，可按父分类过滤。 |
| `plugin_get_versions` | `get_versions` | 读取指定插件的版本列表。 |
| `plugin_get_configs` | `get_configs` | 读取插件配置定义与当前值。 |
| `plugin_get_install` | `get_install` | 读取本机安装记录。 |
| `plugin_get_libs` | `get_libs` | 读取插件动态库登记。 |
| `plugin_get_functions` | `get_functions` | 读取插件能力清单，支持库、插件、本地、AI、向量状态过滤。 |
| `plugin_get_skill_categories` | `get_skill_categories` | 读取技能分类。 |
| `plugin_get_skills` | `get_skills` | 读取技能列表，支持分类、id、插件、启用状态与正文过滤。 |
| `plugin_get_skill` | `get_skill` | 按 id 读取技能正文。 |
| `plugin_post_plugin` | `post_plugin` | 新增插件元数据；name 必填。 |
| `plugin_post_category` | `post_category` | 新增插件分类；type_name 必填。 |
| `plugin_post_version` | `post_version` | 新增插件版本；plugin_uuid 与 version 必填。 |
| `plugin_post_config` | `post_config` | 新增插件配置定义；plugin_uuid 与 param_key 必填。 |
| `plugin_post_skill` | `post_skill` | 新增技能；id、name、category 必填。 |
| `plugin_post_skill_category` | `post_skill_category` | 新增技能分类；code 与 name 必填。 |
| `plugin_post_scan` | `post_scan` | 扫描 ./plugins，登记动态库、能力与技能；幂等。 |
| `plugin_post_check_updates` | `post_check_updates` | 调用 dytes update --check 同步检查插件更新。 |
| `plugin_post_install` | `post_install` | 通过 dytes 安装设计插件；plugin_uuid 或 name 至少一个。 |
| `plugin_post_uninstall` | `post_uninstall` | 卸载插件并删除安装目录；plugin_uuid 或 name 至少一个。 |
| `plugin_post_update` | `post_update` | 通过 dytes 重新安装当前最新版本；plugin_uuid 或 name 至少一个。 |
| `plugin_post_delete` | `post_delete` | 删除插件元数据与安装文件；不可恢复。plugin_uuid 或 name 至少一个。 |
| `plugin_patch_plugin` | `patch_plugin` | 局部更新插件 enabled / status / category_id / cover。 |
| `plugin_patch_install` | `patch_install` | 局部更新安装记录。 |
| `plugin_patch_config` | `patch_config` | 保存插件配置值；plugin_uuid、key、value 必填。 |
| `plugin_patch_function` | `patch_function` | 回写能力的 AI 召回与向量化状态。 |
| `plugin_update_plugin` | `update_plugin` | 全量更新插件元数据；uuid 与 name 必填。 |
| `plugin_update_category` | `update_category` | 全量更新插件分类；uuid 与 type_name 必填。 |
| `plugin_update_version` | `update_version` | 全量更新插件版本；id 与 version 必填。 |
| `plugin_update_config` | `update_config` | 全量更新配置定义；id 与 param_key 必填。 |
| `plugin_update_skill` | `update_skill` | 更新技能；id 必填。 |
| `plugin_update_skill_category` | `update_skill_category` | 更新技能分类；code 必填。 |
| `plugin_delete_skill` | `delete_skill` | 物理删除技能；id 必填。 |
| `plugin_delete_skill_category` | `delete_skill_category` | 物理删除技能分类；code 必填。 |
| `plugin_check_updates_sse` | `check_updates_sse` | 供 cyctron /sse?lib=plugin_lib&symbol=check_updates_sse 使用，输出作为事件 data。 |

## 3. 调用

```http
POST http://127.0.0.1:44944/plugin_lib
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 安装

```bash
dytes install plugin manager_plugins
```

安装后目录为 `plugins/manager_plugins/`，其中包含 `dytes.json`、`mcp.json`、
各工具 JSON、`skills.json`、`manager_plugins.json` 与 `manager_plugins.md`。

## 5. 约束

- get_* 是只读；post_* / patch_* / update_* / delete_* 会写 plugins.db。
- post_install / post_update / post_check_updates 会调用外部 dytes，属于有副作用操作，先向用户确认。
- post_delete 会物理删除插件元数据与安装目录，不可恢复。
- post_scan 幂等；安装或手动放入插件后可用它登记能力与技能。
