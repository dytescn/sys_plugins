# manager_ostfil

本地文件存储插件：将 libostfil 的存储、读取与删除 FFI 整理为 MCP 工具。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_ostfil` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: sys_plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: true` |
| 底层库 | `ostfil` |
| HTTP 端点 | `POST http://127.0.0.1:44944/libostfil` |
| 工具数 | 3 |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| 技能 | `skills.json`（`use_ai: true` 必需） |
| 技能文档 | `skills/manager_ostfil.md` |

本包只包含清单、MCP 工具定义与技能，不额外打包 DLL。
`transport` 字段把每个工具指回已安装的 `ostfil` 能力库，
因此安装后不会复制或重命名系统库。

## 1. 目录

```text
manager_ostfil/
├── dytes.json
├── mcp/
│   ├── mcp.json
│   └── <tool>.json
├── skills/
│   └── manager_ostfil.md
├── skills.json
└── README.md
```

## 2. 工具

| MCP 工具 | FFI-Symbol | 说明 |
| --- | --- | --- |
| `ostfil_storage` | `storage` | 把已有本地文件复制到 storage/<uuid>/，返回 uuid、大小、扩展名、结束时间与 md5。 |
| `ostfil_get_fil` | `get_fil` | 按 uuid 读取 storage/<uuid>/ 下的对象信息；必要时解压 original.z。 |
| `ostfil_delete_file` | `delete_file` | 按 uuid 删除 storage/<uuid>/ 整个目录。 |

## 3. 调用

```http
POST http://127.0.0.1:44944/libostfil
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 安装

```bash
dytes install plugin manager_ostfil
```

安装后目录为 `plugins/manager_ostfil/`，其中包含 `dytes.json`、`mcp.json`、
各工具 JSON、`skills.json` 与 `manager_ostfil.md`。

## 5. 约束

- storage 只复制已存在的本地文件，返回 uuid / size / extra / end_time / md5。
- get_fil 与 delete_file 都以 storage 返回的 uuid 定位 storage/<uuid>/。
- 端点名是 /libostfil，不是 /ostfil。
- get_fil 只返回对象信息，不要把大文件正文当作单次 64 KiB 响应读取。
