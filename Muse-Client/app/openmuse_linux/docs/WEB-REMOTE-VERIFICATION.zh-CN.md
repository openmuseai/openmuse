# Web 与 Linux 远程工作区验证（2026-10-29）

## 已发布的修复

- Web 的 DSH kernel 与 Linux 动态插件使用匹配的依赖。构建会检查 `observeComposition`、`modalSelector`，按 SHA-256 生成模块版本参数，避免继续使用旧的缓存模块。
- 浏览器在展示工作区前完成一次性启动地址与 HttpOnly Cookie 的交换。DSH 随后从 `/openmuse/dsh/` 加载，不重复消费一次性启动令牌。
- Web 接通添加 Workspace、目录右键菜单、多文件标签与标签右键菜单。
- 远程目录操作使用 Desktop 的有效同账号 grant，只接受 JSON POST；检验 workspace、账号、目录引用、名称和符号链接边界。浏览器不会取得客户端绝对路径。
- 添加工作区可输入名称，在当前 Linux 账号的 `Workspaces` 下创建目录；也可显式输入已有的远程绝对目录路径。
- 移除文件会移至挂载根目录内的 `.openmuse-trash`，支持从目录右键菜单恢复到工作区根目录；目标同名时拒绝覆盖。移除 Workspace 只解除挂载。

## 验证

- Flutter Web analyze 与 release 构建通过。
- Web 工作台交互测试 3 项通过，Chrome 认证与设备连接测试 6 项通过。
- 配对网关 8 项测试通过，包括无授权、错误 workspace、跨站请求及账号切换拒绝访问。
- Linux 镜像服务 2 项测试通过，包括新建、重命名、回收与恢复、路径越界及替换为符号链接后拒绝访问。
- Linux ARM64 release 构建通过。
- 线上 `https://app.openmuseai.com/zh` 已实际连接 `OpenMuse Linux ARM64`，DSH 正常进入聊天界面。
- 线上实际创建 `Remote Verification` 工作区及 `WEB-REMOTE-CHECK.md`；文件已在 Linux 账号目录落盘。目录和文件标签的右键菜单已实际显示。

## 发布与恢复

线上更新仅替换 `/opt/appflowy-web/dist/openmuse-app`，保留其他网站目录。更新前版本保留在 `/opt/appflowy-web/backups/openmuse-app-20261029-151735`。通过既有 SSH 部署配置上传，未把凭据写入源码。

Android 真机 PKM110 已通过 USB 识别并启动客户端。用户已在真机完成指定同账号密码登录；设备头部实际显示 `OpenMuse Linux ARM64 | Workspace`，手机的 Workspace 文件页已显示由 Web 创建的 `WEB-REMOTE-CHECK.md`，确认同账号设备绑定与目录访问成功。截图见 `../build/evidence/android-linux-workspace.png`。

手机文件页当前显示“编辑器已关闭，点击重试可再次打开”，本轮没有把文件内容预览算作验收通过。手机此前发出的 `111` 对话已到达 Linux DSH，但返回 `MISSING_CREDENTIAL`，提示未配置 `deepseek-official` 的 API Key；这与配对、目录访问是不同环节。未替用户配置或使用模型密钥。
