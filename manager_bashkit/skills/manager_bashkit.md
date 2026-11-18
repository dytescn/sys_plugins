# manager_bashkit

## 定位

基于 everruns/bashkit 的虚拟 bash 沙箱。所有命令在进程内重实现，
不 fork / exec，不访问宿主文件系统与网络。只提供一个工具 `bash_exec`。

端点：`POST http://127.0.0.1:44944/bashkit`，FFI 符号：`bash_exec`。

## 工具

`bash_exec`

- 入参：`commands`（string，必填），一个完整的 bash 脚本。
- 返回 `data`：`exit_code`、`stdout`、`stderr`、`stdout_truncated`、`stderr_truncated`。
- 脚本退出码非零**不是**调用失败：`code` 仍为 200，看 `data.exit_code`。

```json
{"commands": "echo hello"}
```

## 能做什么

- 167 个内建命令：`echo`、`cat`、`grep`、`sed`、`awk`、`sort`、`uniq`、`find`、
  `tar`、`base64`、`jq`（视特性）、`seq`、`bc`、`date` 等。
- 完整 shell 语法：变量与参数展开、算术、管道、重定向、`if` / `for` / `while` /
  `case`、函数、数组、命令替换、heredoc。
- 虚拟文件系统：可在 `/tmp`、`/home/user` 等虚拟路径建目录、写文件、再读取。

## 不能做什么

- 不能读写宿主真实文件：`/` 是内存 VFS，`cat /etc/passwd` 读到的是虚拟文件，
  `C:/` 也不可见。
- 不能访问网络：`curl` / `wget` 返回 `network access not configured`。
- 不能启动宿主进程，也不能操作真实进程或服务。
- 不能跨调用保留状态：每次调用都是新的 `Bash` 实例，`cd`、`export`、写文件都会丢失。

需要真实系统能力时改用宿主提供的其它系统插件，不要指望 bashkit 访问宿主。

## 使用规则

- 有状态流程写在一个 `commands` 里，用 `&&`、管道与控制流串起来。
- 默认身份是 `whoami=sandbox`、`hostname=bashkit-sandbox`、`cwd=/home/user`。
- 输出缓冲 1 MiB：超出时较长的一侧会被裁剪并置位对应 `*_truncated`，
  大结果应先重定向到虚拟文件，再用 `head` / `tail` / `wc` / `grep` 分段读取。
- 命令数量、循环次数、函数深度、输入大小沿用 bashkit 默认限制，超限返回 `400200`。
