# manager_materil

素材库管理插件：将 material_lib 的素材、类型、版本、标签与语义搜索 FFI 整理为 MCP 工具。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_materil` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: sys_plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: true` |
| 底层库 | `material_lib` |
| HTTP 端点 | `POST http://127.0.0.1:44944/material_lib` |
| 工具数 | 24 |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| 技能 | `skills.json`（`use_ai: true` 必需） |
| 技能文档 | `skills/manager_materil.md` |

本包只包含清单、MCP 工具定义与技能，不额外打包 DLL。
`transport` 字段把每个工具指回已安装的 `material_lib` 能力库，
因此安装后不会复制或重命名系统库。

## 1. 目录

```text
manager_materil/
├── dytes.json
├── mcp/
│   ├── mcp.json
│   └── <tool>.json
├── skills/
│   └── manager_materil.md
├── skills.json
└── README.md
```

## 2. 工具

| MCP 工具 | FFI-Symbol | 说明 |
| --- | --- | --- |
| `material_setup` | `setup` | 创建素材库 5 张表与 17 个索引；幂等。 |
| `material_file_list` | `file_list` | 分页查询素材文件，支持类型、项目、标签、关键词过滤。 |
| `material_file_get` | `file_get` | 按 id 或 uuid 读取素材文件。 |
| `material_file_create` | `file_create` | 新建素材文件；name 必填。 |
| `material_file_update` | `file_update` | 按 id 或 uuid 更新素材文件白名单字段。 |
| `material_file_delete` | `file_delete` | 删除素材文件；默认软删除，hard=true 时物理删除。 |
| `material_file_search` | `file_search` | 按语义搜索素材内容，返回带 score 的素材记录；依赖 chatllm 向量服务。 |
| `material_type_list` | `type_list` | 分页查询素材类型字典。 |
| `material_type_get` | `type_get` | 按 id 或 uuid 读取素材类型。 |
| `material_type_create` | `type_create` | 新建素材类型；type_name 必填。 |
| `material_type_update` | `type_update` | 按 id 或 uuid 更新 type_name / icon。 |
| `material_type_delete` | `type_delete` | 删除素材类型；默认软删除。 |
| `material_version_list` | `version_list` | 分页查询素材版本，可按设计文件、存储编号过滤。 |
| `material_version_get` | `version_get` | 按 id 或 uuid 读取素材版本。 |
| `material_version_create` | `version_create` | 新建素材版本，关联设计文件与文件存储编号。 |
| `material_version_update` | `version_update` | 按 id 或 uuid 更新素材版本白名单字段。 |
| `material_version_delete` | `version_delete` | 删除素材版本；默认软删除。 |
| `material_tag_list` | `tag_list` | 分页查询标签字典，支持类型与关键词过滤。 |
| `material_tag_get` | `tag_get` | 按 id 或 uuid 读取标签。 |
| `material_tag_create` | `tag_create` | 新建标签；name 必填。 |
| `material_tag_update` | `tag_update` | 按 id 或 uuid 更新标签白名单字段。 |
| `material_tag_delete` | `tag_delete` | 删除标签；默认软删除。 |
| `material_file_tag_list` | `file_tag_list` | 读取指定素材文件的标签列表。 |
| `material_file_tag_set` | `file_tag_set` | 覆盖或追加素材标签；replace=true 时先清空再写入。 |

## 3. 调用

```http
POST http://127.0.0.1:44944/material_lib
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 安装

```bash
dytes install plugin manager_materil
```

安装后目录为 `plugins/manager_materil/`，其中包含 `dytes.json`、`mcp.json`、
各工具 JSON、`skills.json` 与 `manager_materil.md`。

## 5. 约束

- file_* / type_* / version_* / tag_* 分别管理素材文件、类型、版本与标签。
- 删除默认软删除；hard=true 才是物理删除。
- file_search 依赖 chatllm 向量服务，返回结果带 score 与 tags。
- file_tag_set 的 replace=true 会先清空现有标签。
