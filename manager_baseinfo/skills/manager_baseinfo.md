# manager_baseinfo

## 定位

系统基础信息（路由 / 设置项）管理插件：将 info_lib 的 FFI 整理为 MCP 工具与提示词。

底层能力库：`info_lib`；端点：`POST http://127.0.0.1:44944/info_lib`。

## 调用约定

```http
POST http://127.0.0.1:44944/info_lib
Content-Type: application/libary
FFI-Symbol: <symbol>

{}
```

- 响应统一为 `{"code": 200, "data": {}, "msg": "success"}`，`code != 200` 时读取 `msg`。
- 输出缓冲区 64 KiB；超过时 FFI 返回 `-2`，请分页或缩小请求。
- 每个 MCP 工具的 `transport.endpoint` 与 `transport.ffiSymbol` 指向真实 FFI 调用。

## 工具

- 获取单条路由：按 id 读取一条菜单 / 页面路由。
- 新增路由：新增菜单 / 页面路由；title 与 path 必填，id 省略时自增。
- 修改路由：按 id 修改路由；只更新传入的字段，至少提供一个字段。
- 查询路由列表：分页查询路由，支持父路由、隐藏状态、路径与关键词过滤。
- 获取设置项：按 id 或 key 读取一个设置项，并合并 designdb 中的 note 分类对象。
- 新增设置项：新增设置项；key 必填。可带 note，note 会 upsert 到 designdb/setting.setting_note。
- 修改设置项：按 id 或 key 修改设置项；key 不可修改，note 单独 upsert。
- 查询设置项列表：分页查询设置项，支持分类与关键词过滤；每项合并 note。

## 使用规则

- router_* 管理系统菜单 / 页面路由；info_* 管理设置项，两者不要混用。
- 更新接口只更新传入字段；router_update 必须带 id，info_update 需要 id 或 key。
- info_post / info_update 的 note 会写入 designdb 的 setting_note 分类对象。
- 当前不提供删除接口。
