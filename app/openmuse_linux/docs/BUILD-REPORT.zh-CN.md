# Linux 构建与连接记录

验证日期：2026-10-29（Asia/Shanghai）。工程位于 `app/openmuse_linux`，沿用主仓库版本管理。

## 已完成

- 在本机 Docker 中使用 **Debian 12 / aarch64**，实际构建 Flutter **3.44.2** Release 原生程序。
- 使用 Rust **1.90.0** 从仓库 Helix fork 编译 Linux ELF；从固定的语法源码构建 **36 个 .so 解析器**，未复制 macOS dylib。
- 官方 Node **22.19.0 Linux ARM64** 归档与仓库保留的 SHA256 一致。
- 以固定 npm lockfile 构建 DSH **0.2.0-rc.2** 闭包，闭包验证和无模型密钥的 web/runtime 认证测试通过。
- Linux 原生 C++ WebKitGTK 桥接编译通过；GTK/Flutter 窗口、账号恢复、DSH 页面实际启动，日志出现 `OpenMuse DSH WebKit page loaded`。
- Linux 运行环境的静态分析通过，Flutter 测试 **109 项通过 / 4 项跳过**。其中外部插件 release-fixture 测试需显式设置 `OPENMUSE_ECOSYSTEM_TEST_ROOT`，默认不依赖相邻仓库；其余跳过项受现有平台/E2E 条件限制。
- 公共 DSH 插件测试 **7 项通过 / 1 项跳过**。Dart CLI 原生构建完成，`openmuse --help` 可执行。
- `.deb` 与 `.tar.gz` 已生成；目标架构、ELF 动态依赖、包内符号链接及最终 SHA256 检查通过。剔除 npm 包中不供 Debian/glibc ARM64 使用的 musl 和其他架构 PTY 变体。
- 使用用户指定的同一账号登录 Web 与 Linux。Linux 会话保存在 Secret Service 密钥环；密码没有写入源码、配置或安装包。
- Web App 设备名显示 **OpenMuse Linux ARM64**；通过真实账号设备目录、授权连接及 outbound relay 读取 Linux Workspace。
- 在 Linux 本地 Workspace 创建 `README-LINUX.md` 作为连接验证文档，Web 端已列出并打开该文档。

## 产物

位于 `build/dist/arm64/`：

| 文件 | SHA256 |
|---|---|
| `OpenMuse-linux-arm64-1.0.0.deb` | `6d548f339397fccb4181c85b37a275c6c53aaf60198da3ece378fe509501fbab` |
| `OpenMuse-linux-arm64-1.0.0.tar.gz` | `3b0b8a1bada7e21eaf5a7da8ebc7bfee32003eb790172f40847dec5454bef3aa` |

同目录提供 `SHA256SUMS` 和 `build-info.json`。后者记录工具链、架构、glibc、WebKit、锁文件及源文件摘要。包内不包含用户认证会话和工作区文件。

## 本机运行入口

- Linux 原生客户端运行在容器 `openmuse-linux-build` 的 Xvfb 显示器 `:99`，不是在 macOS 上运行 Linux ELF。
- 图形入口：`http://127.0.0.1:6080/vnc.html?autoconnect=true&resize=scale`，宿主机只发布 localhost 端口；转发容器为 `openmuse-linux-view`。
- Web App：`https://app.openmuseai.com/zh`，已保留已绑定的浏览器页面。
- 本地工作区、密钥环和构建工具数据保存在 `openmuse-linux-tools` Docker volume 的 `/opt` 下。不要删除该卷，否则会丢失这次运行实例的数据。安装包自身不依赖这个开发卷。

## 未完成及限制

- 线上 Web DSH 的静态 kernel 版本错配已修复并发布，聊天界面正常加载；未执行付费模型请求。Web 菜单、远程工作区操作及授权测试见 [Web 远程验证](WEB-REMOTE-VERIFICATION.zh-CN.md)。
- 本机构建产物是 **ARM64**。amd64 构建工作流已配置，尚未执行和验收；没有产出 amd64 安装包。
- 当前是工程验证发行包。未完成干净系统安装、Ubuntu 24.04/Wayland/KDE、多屏缩放、输入法实机、卸载/升级、所有退出场景的子进程清理及完整许可证/SBOM 发布审查。
- 本地 noVNC 输入出现过丢字和组合键异常，因此账号通过同一 GoTrue 接口获取正常会话，并用客户端相同的 Secret Service 存储路径写入后，由客户端自身完成恢复、云身份校验、设备注册和连接。图形登录流程仍需在真实 Linux 桌面验收。
- 未打包原生 Office 编辑组件；Markdown/代码编辑及公共文件预览按首版产品范围提供。
