# manager_baseinfo

系统基础信息（路由 / 设置项）管理插件：将 info_lib 的 FFI 整理为 MCP 工具与提示词。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_baseinfo` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: true` |
| 底层库 | `info_lib` |
| HTTP 端点 | `POST http://127.0.0.1:44944/info_lib` |
| 工具数 | 8 |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| 技能 | `skills.json`（`use_ai: true` 必需） |
| 提示词 | `prompts/manager_baseinfo.json`、`skills/manager_baseinfo.md` |

本包只包含清单、MCP 工具定义、技能与提示词，不额外打包 DLL。
`transport` 字段把每个工具指回已安装的 `info_lib` 能力库，
因此安装后不会复制或重命名系统库。

## 1. 目录

```text
manager_baseinfo/
├── dytes.json
├── mcp/
│   ├── mcp.json
│   └── <tool>.json
├── prompts/
│   └── manager_baseinfo.json
├── skills/
│   └── manager_baseinfo.md
├── skills.json
└── README.md
```

## 2. 工具

| MCP 工具 | FFI-Symbol | 说明 |
| --- | --- | --- |
| `info_router_get` | `router_get` | 按 id 读取一条菜单 / 页面路由。 |
| `info_router_post` | `router_post` | 新增菜单 / 页面路由；title 与 path 必填，id 省略时自增。 |
| `info_router_update` | `router_update` | 按 id 修改路由；只更新传入的字段，至少提供一个字段。 |
| `info_router_list` | `router_list` | 分页查询路由，支持父路由、隐藏状态、路径与关键词过滤。 |
| `info_info_get` | `info_get` | 按 id 或 key 读取一个设置项，并合并 designdb 中的 note 分类对象。 |
| `info_info_post` | `info_post` | 新增设置项；key 必填。可带 note，note 会 upsert 到 designdb/setting.setting_note。 |
| `info_info_update` | `info_update` | 按 id 或 key 修改设置项；key 不可修改，note 单独 upsert。 |
| `info_info_list` | `info_list` | 分页查询设置项，支持分类与关键词过滤；每项合并 note。 |

## 3. 调用

```http
POST http://127.0.0.1:44944/info_lib
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 安装

```bash
dytes install plugin manager_baseinfo
```

安装后目录为 `plugins/manager_baseinfo/`，其中包含 `dytes.json`、`mcp.json`、
各工具 JSON、`skills.json`、`manager_baseinfo.json` 与 `manager_baseinfo.md`。

## 5. 约束

- router_* 管理系统菜单 / 页面路由；info_* 管理设置项，两者不要混用。
- 更新接口只更新传入字段；router_update 必须带 id，info_update 需要 id 或 key。
- info_post / info_update 的 note 会写入 designdb 的 setting_note 分类对象。
- 当前不提供删除接口。
