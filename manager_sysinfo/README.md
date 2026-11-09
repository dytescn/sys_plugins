# manager_sysinfo

系统信息与文件目录插件：将 sysinfo 的系统信息、文件、目录与对话框 FFI 整理为 MCP 工具。

| 项 | 内容 |
| --- | --- |
| 包名 | `manager_sysinfo` |
| 版本 | `0.1.0` |
| 类型 | `design_plugins`（`plugins_type: sys_plugins`） |
| 适用 | `targets: ["common"]`，`use_ai: true` |
| 底层库 | `sysinfo` |
| HTTP 端点 | `POST http://127.0.0.1:44944/sysinfo` |
| 工具数 | 13 |
| MCP | `mcp/mcp.json` → 安装为 `mcp.json` |
| 技能 | `skills.json`（`use_ai: true` 必需） |
| 技能文档 | `skills/manager_sysinfo.md` |

本包只包含清单、MCP 工具定义与技能，不额外打包 DLL。
`transport` 字段把每个工具指回已安装的 `sysinfo` 能力库，
因此安装后不会复制或重命名系统库。

## 1. 目录

```text
manager_sysinfo/
├── dytes.json
├── mcp/
│   ├── mcp.json
│   └── <tool>.json
├── skills/
│   └── manager_sysinfo.md
├── skills.json
└── README.md
```

## 2. 工具

| MCP 工具 | FFI-Symbol | 说明 |
| --- | --- | --- |
| `sysinfo_system_basic_info` | `system_basic_info` | 名称、内核版本、OS 版本、主机名、运行时长、启动时间与负载。 |
| `sysinfo_cpu_info` | `cpu_info` | CPU 型号、核心数与使用率等。 |
| `sysinfo_memory_info` | `memory_info` | 物理内存与交换区使用情况。 |
| `sysinfo_disk_info` | `disk_info` | 各磁盘容量与使用情况。 |
| `sysinfo_process_list` | `process_list` | 分页读取进程列表，按 PID、名称、内存、CPU、状态返回。 |
| `sysinfo_network_info` | `network_info` | 网络接口、地址与流量信息。 |
| `sysinfo_component_info` | `component_info` | 温度传感器等硬件组件信息。 |
| `sysinfo_file_exists` | `file_exists` | 检查文件或目录是否存在。 |
| `sysinfo_file_info` | `file_info` | 读取文件 / 目录大小、类型、修改与创建时间；不跟随符号链接。 |
| `sysinfo_create_dir` | `create_dir` | 在当前工作目录下创建目录。 |
| `sysinfo_cur_dir` | `cur_dir` | 返回 Piksel 的当前运行目录。 |
| `sysinfo_choose_file` | `choose_file` | 弹出原生文件选择框；阻塞式，需要人工操作。 |
| `sysinfo_choose_dir` | `choose_dir` | 弹出原生目录选择框；阻塞式，需要人工操作。 |

## 3. 调用

```http
POST http://127.0.0.1:44944/sysinfo
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

cyctron 拼接 `<method>_<FFI-Symbol>`。工具 JSON 的
`transport.endpoint` / `transport.ffiSymbol` 与上表一致。

## 4. 安装

```bash
dytes install plugin manager_sysinfo
```

安装后目录为 `plugins/manager_sysinfo/`，其中包含 `dytes.json`、`mcp.json`、
各工具 JSON、`skills.json` 与 `manager_sysinfo.md`。

## 5. 约束

- 系统信息类工具只读；create_dir 只创建目录，不复制文件。
- create_dir 的输入字段是 dir_name，不是 path。
- process_list 用 limit / offset 分页，避免超过 64 KiB 输出。
- choose_file / choose_dir 是阻塞式原生弹窗，需要人工操作，不能自动化。
