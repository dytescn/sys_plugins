# manager_sysinfo

## 定位

系统信息与文件目录插件：将 sysinfo 的系统信息、文件、目录与对话框 FFI 整理为 MCP 工具与提示词。

底层能力库：`sysinfo`；端点：`POST http://127.0.0.1:44944/sysinfo`。

## 调用约定

```http
POST http://127.0.0.1:44944/sysinfo
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

- 响应统一为 `{"code": 200, "data": {}, "msg": "success"}`，`code != 200` 时读取 `msg`。
- 输出缓冲区 64 KiB；超过时 FFI 返回 `-2`，请分页或缩小请求。
- 每个 MCP 工具的 `transport.endpoint` 与 `transport.ffiSymbol` 指向真实 FFI 调用。

## 工具

- 系统基本信息：名称、内核版本、OS 版本、主机名、运行时长、启动时间与负载。
- CPU 信息：CPU 型号、核心数与使用率等。
- 内存信息：物理内存与交换区使用情况。
- 磁盘信息：各磁盘容量与使用情况。
- 进程列表：分页读取进程列表，按 PID、名称、内存、CPU、状态返回。
- 网络信息：网络接口、地址与流量信息。
- 组件信息：温度传感器等硬件组件信息。
- 文件存在性：检查文件或目录是否存在。
- 文件信息：读取文件 / 目录大小、类型、修改与创建时间；不跟随符号链接。
- 创建目录：在当前工作目录下创建目录。
- 当前运行目录：返回 Piksel 的当前运行目录。
- 选择文件对话框：弹出原生文件选择框；阻塞式，需要人工操作。
- 选择目录对话框：弹出原生目录选择框；阻塞式，需要人工操作。

## 使用规则

- 系统信息类工具只读；create_dir 只创建目录，不复制文件。
- create_dir 的输入字段是 dir_name，不是 path。
- process_list 用 limit / offset 分页，避免超过 64 KiB 输出。
- choose_file / choose_dir 是阻塞式原生弹窗，需要人工操作，不能自动化。
