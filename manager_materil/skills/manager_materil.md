# manager_materil

## 定位

素材库管理插件：将 material_lib 的素材、类型、版本、标签与语义搜索 FFI 整理为 MCP 工具与提示词。

底层能力库：`material_lib`；端点：`POST http://127.0.0.1:44944/material_lib`。

## 调用约定

```http
POST http://127.0.0.1:44944/material_lib
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

- 响应统一为 `{"code": 200, "data": {}, "msg": "success"}`，`code != 200` 时读取 `msg`。
- 输出缓冲区 64 KiB；超过时 FFI 返回 `-2`，请分页或缩小请求。
- 每个 MCP 工具的 `transport.endpoint` 与 `transport.ffiSymbol` 指向真实 FFI 调用。

## 工具

- 初始化素材库：创建素材库 5 张表与 17 个索引；幂等。
- 查询素材文件列表：分页查询素材文件，支持类型、项目、标签、关键词过滤。
- 获取素材文件详情：按 id 或 uuid 读取素材文件。
- 新建素材文件：新建素材文件；name 必填。
- 更新素材文件：按 id 或 uuid 更新素材文件白名单字段。
- 删除素材文件：删除素材文件；默认软删除，hard=true 时物理删除。
- 语义搜索素材：按语义搜索素材内容，返回带 score 的素材记录；依赖 chatllm 向量服务。
- 查询素材类型列表：分页查询素材类型字典。
- 获取素材类型详情：按 id 或 uuid 读取素材类型。
- 新建素材类型：新建素材类型；type_name 必填。
- 更新素材类型：按 id 或 uuid 更新 type_name / icon。
- 删除素材类型：删除素材类型；默认软删除。
- 查询素材版本列表：分页查询素材版本，可按设计文件、存储编号过滤。
- 获取素材版本详情：按 id 或 uuid 读取素材版本。
- 新建素材版本：新建素材版本，关联设计文件与文件存储编号。
- 更新素材版本：按 id 或 uuid 更新素材版本白名单字段。
- 删除素材版本：删除素材版本；默认软删除。
- 查询标签列表：分页查询标签字典，支持类型与关键词过滤。
- 获取标签详情：按 id 或 uuid 读取标签。
- 新建标签：新建标签；name 必填。
- 更新标签：按 id 或 uuid 更新标签白名单字段。
- 删除标签：删除标签；默认软删除。
- 读取素材标签：读取指定素材文件的标签列表。
- 设置素材标签：覆盖或追加素材标签；replace=true 时先清空再写入。

## 使用规则

- file_* / type_* / version_* / tag_* 分别管理素材文件、类型、版本与标签。
- 删除默认软删除；hard=true 才是物理删除。
- file_search 依赖 chatllm 向量服务，返回结果带 score 与 tags。
- file_tag_set 的 replace=true 会先清空现有标签。
