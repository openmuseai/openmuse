# OpenMuse Linux 技术方案

## 架构选择

保留 Flutter 3.44.2 + GTK 3 桌面宿主、Helix Rust 子进程、DSH Node 子进程。Linux 差异收敛在 GTK runner、DSH 原生面板、运行时布局及发行工具。公共插件与协议继续使用主仓库代码，避免另建一套账号、存储和 Agent 实现。

```text
app/openmuse_linux
  lib/                    Linux Host 产品组合及桌面服务
  linux/runner/           GTK 窗口、Flutter View、DSH WebKit overlay
  tool/                   环境准备、构建、ELF 验证、发行打包
  packaging/              freedesktop desktop entry
  docs/                   产品方案、架构与实际构建记录
       │
       ├── ../../packages & ../../plugins  公共协议、SDK、UI、服务
       ├── Helix fork ── PTY ── Flutter xterm
       └── DSH + Node ── authenticated loopback HTTP ── WebKitGTK
```

Flutter 的原生 Linux runner 和系统依赖依据 [Flutter Linux 构建文档](https://docs.flutter.dev/platform-integration/linux/building)。Linux 不具备现有 macOS AppKitView 的接入路径，使用 GTK overlay 显示 WebKitGTK 子视图，与 Windows 现有 bounds channel 共用 Dart 布局适配。采用 [WebKitGTK 4.1 API](https://webkitgtk.org/reference/webkit2gtk/stable/)；不依赖未锁定的第三方 Flutter WebView 插件。

## DSH 面板

`com.openmuse.dsh/webview` 提供 show、hide、reload、activateWorkspace、openSession。Dart 向 GTK 传逻辑像素，Windows 原路径继续传设备像素。WebView 作为 Flutter 窗口的 GTK overlay，按右面板的位置和大小布局，模态路由出现时隐藏。

只接受 `http://127.0.0.1:<port>`，禁止 userinfo，页面跳转限制在同一端口的同源地址，拒绝新窗口。URL 中 token 由 DSH 处理并转为 HttpOnly cookie；每次 sidecar URL 改变重建 ephemeral WebContext，避免不同端口间 cookie 累积。日志只记录加载状态和通用错误，不能输出启动 URL/token。

页面桥接只注入主框架，按消息 type 白名单转发 resource.open、workspace.activate 和 clipboard.write；消息长度上限 16 KiB，JSON 类型校验后转发。文件路径和权限由既有 Host Broker 继续校验。工作区和请求会话在页面载入后重新同步。

## 本地运行时布局

```text
/opt/openmuse/
  openmuse_linux
  lib/                       Flutter 与平台插件 .so
  data/flutter_assets/       字体、Flutter 资源、公共插件资源
    packages/openmuse_helix_plugin/assets/engines/helix/
      hx                     Linux ELF，从仓库 fork 编译
      runtime/               queries、themes、languages、目标架构 grammars
  openmuse/dsh/
    node/bin/node            官方 22.19.0 Linux 运行时
    node_modules/            npm ci 依据固定 lockfile 构建
  licenses/
  build-info.json
```

用户数据由 `path_provider` / XDG 路径体系定位，安装目录不保存项目或账号数据。认证凭据使用公共 secure storage 的 Linux Secret Service 实现，要求桌面会话的密钥环可用。模型凭据与用户项目不进入安装包。

## 构建与供应链

Debian 12 容器提供 GTK/WebKit 开发包；Flutter 固定 3.44.2，Helix 使用仓库固定的 Rust 1.90.0。Node 下载后与仓库 `SHASUMS256.txt` 比较。DSH 使用 `build_dsh_closure.py`，npm ci 根据 integrity 解析平台对应的依赖，并运行原有 runtime smoke test。

Helix 必须从 fork 重新编译 Linux ELF，不能复制现有 Mach-O 二进制或 dylib grammars。Tree-sitter 只构建 `scripts/helix-grammars.toml` 中选定的语法；语法源码 revision 来自随仓库提供的 languages 配置。`--without-grammars` 会在构建记录标注能力缺口。

打包前验证主程序、Helix、Node 的 ELF 架构；ldd 检查系统库；检查全部符号链接。deb 依赖由 dpkg-shlibdeps 从真实 ELF 计算，不能用一份泛化的依赖表掩盖 glibc 版本差异。便携包包含相同文件，仍需要相应系统依赖。

构建记录包含主仓库提交、脏工作区标记、工具链、glibc、WebKit、锁文件摘要和 Linux 源码摘要。Docker 基础镜像与 apt 安全更新尚未按 digest/快照固定，因此当前是版本和产物可追溯构建，不承诺逐字节可复现。

## 维护与边界

初始 Host 源码固定在主仓库 `cbd243e35446dbded217d7c1676128d2c5c5af41`，从已提交版本导入。移除外部 AppFlowy Markdown 原生编辑包依赖，继续通过 Helix、预览插件与通用 MCP 适配使用 Markdown。Office 未提交分支不进入首版 Linux。

公共 DSH Dart 适配只增加 Linux bounds/session 路径及错误重试。后续公共 Host 服务应提取为 package，让 macOS、Windows、Linux 共享并保留各自组合入口。应用更新检查可复用，但 Linux 首版通过系统包升级，不调用 macOS ZIP 安装器。

## 验证层次

工程测试验证工作区、布局、调度任务、MCP 路由和存储逻辑；DSH runtime test 验证真实启动、认证和默认模型插件；Xvfb smoke 验证原生窗口及 WebKit 页面加载；干净桌面验收覆盖 Wayland/X11、输入法、GPU、多屏、凭据、升级和进程清理。每层分别记录结果，不将单元测试或虚拟显示运行等同于完整产品验收。
