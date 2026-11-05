# manager_ostfil

## 定位

本地文件存储插件：将 libostfil 的存储、读取与删除 FFI 整理为 MCP 工具。

底层能力库：`ostfil`；端点：`POST http://127.0.0.1:44944/libostfil`。

## 调用约定

```http
POST http://127.0.0.1:44944/libostfil
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

- 响应统一为 `{"code": 200, "data": {}, "msg": "success"}`，`code != 200` 时读取 `msg`。
- 输出缓冲区 64 KiB；超过时 FFI 返回 `-2`，请分页或缩小请求。
- 每个 MCP 工具的 `transport.endpoint` 与 `transport.ffiSymbol` 指向真实 FFI 调用。

## 工具

- 存储本地文件：把已有本地文件复制到 storage/<uuid>/，返回 uuid、大小、扩展名、结束时间与 md5。
- 读取已存储对象：按 uuid 读取 storage/<uuid>/ 下的对象信息；必要时解压 original.z。
- 删除已存储对象：按 uuid 删除 storage/<uuid>/ 整个目录。

## 使用规则

- storage 只复制已存在的本地文件，返回 uuid / size / extra / end_time / md5。
- get_fil 与 delete_file 都以 storage 返回的 uuid 定位 storage/<uuid>/。
- 端点名是 /libostfil，不是 /ostfil。
- get_fil 只返回对象信息，不要把大文件正文当作单次 64 KiB 响应读取。
