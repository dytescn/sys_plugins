# manager_workspace

项目与工作区管理插件：将 workspace_lib 的项目、特性、版本、日志、设计文件与版本 FFI 整理为 MCP 工具。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_workspace` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: sys_plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: true` |
| 底层库 | `workspace_lib` |
| HTTP 端点 | `POST http://127.0.0.1:44944/workspace_lib` |
| 工具数 | 44 |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| 技能 | `skills.json`（`use_ai: true` 必需） |
| 技能文档 | `skills/manager_workspace.md` |

本包只包含清单、MCP 工具定义与技能，不额外打包 DLL。
`transport` 字段把每个工具指回已安装的 `workspace_lib` 能力库，
因此安装后不会复制或重命名系统库。

## 1. 目录

```text
manager_workspace/
├── dytes.json
├── mcp/
│   ├── mcp.json
│   └── <tool>.json
├── skills/
│   └── manager_workspace.md
├── skills.json
└── README.md
```

## 2. 工具

| MCP 工具 | FFI-Symbol | 说明 |
| --- | --- | --- |
| `workspace_setup` | `setup` | 创建 8 张表与索引；幂等。 |
| `workspace_project_list` | `project_list` | 分页查询项目，支持关键词、过滤与排序。 |
| `workspace_project_get` | `project_get` | 按 id 或 uuid 读取项目。 |
| `workspace_project_create` | `project_create` | 新建项目；name 必填。 |
| `workspace_project_update` | `project_update` | 按 id 或 uuid 更新项目白名单字段。 |
| `workspace_project_delete` | `project_delete` | 删除项目；默认软删除，hard=true 时物理删除。 |
| `workspace_feature_list` | `feature_list` | 分页查询项目特性，支持关键词、过滤与排序。 |
| `workspace_feature_get` | `feature_get` | 按 id 或 uuid 读取项目特性。 |
| `workspace_feature_create` | `feature_create` | 新建项目特性；project_uid 必填。 |
| `workspace_feature_update` | `feature_update` | 按 id 或 uuid 更新项目特性白名单字段。 |
| `workspace_feature_delete` | `feature_delete` | 删除项目特性；物理删除。 |
| `workspace_version_list` | `version_list` | 分页查询项目版本，支持关键词、过滤与排序。 |
| `workspace_version_get` | `version_get` | 按 id 或 uuid 读取项目版本。 |
| `workspace_version_create` | `version_create` | 新建项目版本；project_uid 必填。 |
| `workspace_version_update` | `version_update` | 按 id 或 uuid 更新项目版本白名单字段。 |
| `workspace_version_delete` | `version_delete` | 删除项目版本；物理删除。 |
| `workspace_version_log_list` | `version_log_list` | 分页查询版本日志，支持关键词、过滤与排序。 |
| `workspace_version_log_get` | `version_log_get` | 按 id 或 uuid 读取版本日志。 |
| `workspace_version_log_create` | `version_log_create` | 新建版本日志；project_uid 必填。 |
| `workspace_version_log_update` | `version_log_update` | 按 id 或 uuid 更新版本日志白名单字段。 |
| `workspace_version_log_delete` | `version_log_delete` | 删除版本日志；物理删除。 |
| `workspace_log_list` | `log_list` | 分页查询项目日志，支持关键词、过滤与排序。 |
| `workspace_log_get` | `log_get` | 按 id 或 uuid 读取项目日志。 |
| `workspace_log_create` | `log_create` | 新建项目日志；project_uid 必填。 |
| `workspace_log_update` | `log_update` | 按 id 或 uuid 更新项目日志白名单字段。 |
| `workspace_log_delete` | `log_delete` | 删除项目日志；物理删除。 |
| `workspace_design_file_list` | `design_file_list` | 分页查询设计文件，支持关键词、过滤与排序。 |
| `workspace_design_file_get` | `design_file_get` | 按 id 或 uuid 读取设计文件。 |
| `workspace_design_file_create` | `design_file_create` | 新建设计文件；project_uid、name 必填。 |
| `workspace_design_file_update` | `design_file_update` | 按 id 或 uuid 更新设计文件白名单字段。 |
| `workspace_design_file_delete` | `design_file_delete` | 删除设计文件；物理删除。 |
| `workspace_design_type_list` | `design_type_list` | 分页查询设计类型，支持关键词、过滤与排序。 |
| `workspace_design_type_get` | `design_type_get` | 按 id 或 uuid 读取设计类型。 |
| `workspace_design_type_create` | `design_type_create` | 新建设计类型；type_name 必填。 |
| `workspace_design_type_update` | `design_type_update` | 按 id 或 uuid 更新设计类型白名单字段。 |
| `workspace_design_type_delete` | `design_type_delete` | 删除设计类型；物理删除。 |
| `workspace_design_version_list` | `design_version_list` | 分页查询设计版本，支持关键词、过滤与排序。 |
| `workspace_design_version_get` | `design_version_get` | 按 id 或 uuid 读取设计版本。 |
| `workspace_design_version_create` | `design_version_create` | 新建设计版本；design_uid 必填。 |
| `workspace_design_version_update` | `design_version_update` | 按 id 或 uuid 更新设计版本白名单字段。 |
| `workspace_design_version_delete` | `design_version_delete` | 删除设计版本；物理删除。 |
| `workspace_project_archive` | `project_archive` | 归档或取消归档项目。 |
| `workspace_project_restore` | `project_restore` | 从回收站恢复项目。 |
| `workspace_save_design` | `save_design` | 保存当前设计到工作区：准备目录、写库并经 libostfil 归档版本文件。 |

## 3. 调用

```http
POST http://127.0.0.1:44944/workspace_lib
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 安装

```bash
dytes install plugin manager_workspace
```

安装后目录为 `plugins/manager_workspace/`，其中包含 `dytes.json`、`mcp.json`、
各工具 JSON、`skills.json` 与 `manager_workspace.md`。

## 5. 约束

- 8 个资源统一使用 list / get / create / update / delete 五组接口。
- get / update / delete 的 id 与 uuid 至少提供一个。
- project 是软删除资源；其余资源删除为物理删除。
- list 支持 page / page_size、limit、order 与 filter.keyword。
- save_design 依赖设计软件导出与 libostfil 归档。
