# Open File Viewer：Tree-sitter 与文内文件链接

日期：2026-11-13。运行环境为国内，使用本次对话指定的同一账号。密码和令牌不保存到本文、仓库或验收证据。

## 分支与远端集成

已 fetch origin，创建并切换到 `codex/file-viewer-treesitter-links`，合入 `origin/main` 的 `84299545`（合并提交 `83f5676f`）。保留当前分支已有的运行时配置和 CLI 重构，保留已经迁移删除的旧测试；采用远端设置保存并发修复和明确定位底部关闭按钮的测试修复。与远端重叠的三个工作区文件先临时保存，合并后自动恢复；未提交改动没有混入远端合并提交。本轮功能实现仍在工作区，未推送。

## Tree-sitter 集成

- 共享 Flutter 查看组件依赖精确版本 `duskmoon_highlight: 2.2.11`，两端都由 `open-file-viewer` 插件使用。它封装官方 Tree-sitter C 运行时与固定语法/查询，并通过 Dart native assets 随应用构建。原生解析器、语法及相应许可证均来自锁定的依赖包；运行时不下载语法，不上传代码。版权清单与来源校验记录随共享包 assets 一并进入两端正常构建，发布产物中已逐字节核对。
- 在 native worker isolate 内解析，将 UTF-16 范围传回 UI；解析器在 `finally` 释放。重叠 capture 按优先级、范围和查询顺序合并，避免多个样式重复输出源码。
- 支持关键字、字符串、注释、函数、类型、数值等颜色，适配浅色与深色主题；保留行号、横向滚动和宿主传入的行定位。
- 单次语法分析限制为 256 Ki UTF-16 code units、最多 20,000 个 capture。未知语言、超限或解析异常回退到可读文本，不把回退伪装成成功高亮。
- 查看 intent 中，插件的只读预览 priority 为 60；用户显式 handler 偏好仍优先，edit intent 仍不被查看器接管。旧本地文件编辑入口也复用代码高亮组件。

原始依赖说明及语法来源见 [包说明](https://pub.dev/packages/duskmoon_highlight)。当前依赖包含全部 95 种语法及 Web 资源，带来明显的包体积成本：本轮 arm64 release APK 约 491 MB（构建工具的单位），不是一个轻量的单语法集成。后续若缩减语言集，应通过有来源记录的依赖构建配置或维护分支完成，不能在发布包中临时删除 native asset 或修改依赖缓存。

## 文内文件链接

原实现 `OpenMuseMarkdownBody.onTapLink` 是空回调。本轮改为把原始 href 传给当前 `ResourceEditorSession.openResourceLink`：

1. Host 使用当前资源的统一引用作为 base；Mobile 不读取或拼接 Mac 路径，也不从展示用的 location 猜测 session。
2. `ResourceRelativeLocatorProvider` 是资源源的可选能力。Hub 根据 base 的确切 source 路由，校验账号、有效期、generation、资源源注册状态及返回结果所属 source；不会跨宿主搜索。
3. 通用 RPC `locateRelative` 复用既有授权通道，在异步操作前后验证 peer。非文件系统资源源可自行解释 href；测试包含不使用文件路径的 cloud catalog owner。
4. DSH 插件从自己的私有索引恢复 session 与 canonical 文件定位器，刷新短期 bridge handle，再由 DSH 宿主解析文档相对链接。显式文档上下文优先于 workspace 中的同名文件，支持 `../`、URI 编码的中文/空格与行号；仍受 workspace 和符号链接边界限制。
5. 已挂载目录由 `FileTreeResourceProvider` 在宿主侧做对应解析，禁止越过挂载根目录。旧本地文件入口继续调用 Host 已授权的 `workspace.openResource`，不直接按文档内容读取任意文件。
6. 定位结果继续使用已有歧义选择和统一 `openResourcePage`。文内导航优先复用当前 handler；目标格式不兼容时才选择其他 handler，不写入全局默认偏好。

本轮完成文件跳转及 `#L`/行号定位。Markdown 页面内的 `#heading` 滚动定位、外部网页打开、Tree-sitter 的语言注入/LSP 和大文件分页不属于已验收能力。普通 heading fragment 的文件目标可以打开，但不承诺定位到该标题。

## 验证

- 共享查看器：5 项通过，包含真实 native Tree-sitter、中文/emoji UTF-16 范围、源码保真、回退与 Markdown 原始 href 传递。
- 资源 RPC/挂载目录：11 项通过，包含非文件系统宿主、跨账号拒绝、异步撤销、父目录链接、中文 URI 与越界/符号链接拒绝。
- DSH source：1 项真实 HTTP 测试通过，包含持久身份、过期 handle 与文档 base 恢复。
- Bridge：23 项通过，包含文档目录优先、URI 解码及原有路径/权限回归。以上 40 项相关检查与真实服务 UI 场景分别计数。
- 对新增/修改的共享查看器、资源 RPC、资源 UI、DSH source、插件和 Mobile UI 验收入口完成对应静态检查。

### Desktop 实际 UI

最终正常构建 111319、bridge 0.1.22，指定账号登录国内服务。使用隔离的真实 DSH 会话和宿主 workspace 下的合成文件：

- 从对话的 Viewer README 链接打开只读 README。
- 点击“查看源码”，打开 `示例.dart`；可见注释、关键字、函数、字符串、数字颜色，显示 `Tree-sitter · dart`，第 2 行突出显示，源码包含 `TREE_SITTER_OWNER_OK`。
- 返回 README，点击“下一文档”，可见 `DOCUMENT_LINK_OWNER_OK`；点击“返回 README”回到原文档。

上述结果通过 CUA 直接观察。测试过程曾发现 Desktop 预览被编辑插件抢占，已在插件优先级及文内 handler 延续策略上修复。

### Mobile 实际服务 UI

Android 34 arm64 模拟器运行生产 `app.main()` 的集成测试入口，使用真实国内账号、Desktop 和资源 RPC，没有 mock 文件来源。登录只通过测试入口的临时 loopback 内存通道传入凭据；首次协议操作使用此前用户明确给予的测试模拟器授权。

实际 UI 点击覆盖：对话 → README → 中文文件名 Dart 源码 → 返回 README → 下一文档 → 返回 README。断言同时检查原生高亮完成、关键字和字符串的非空颜色、行号及两份文档的真实可见正文。最后一轮启用了点击未命中即失败，并等待页面可点击状态，没有通过关闭警告或跳过断言获得通过。

前两轮未计为通过：第一轮 Desktop 重新登录/启动期间连接超时；第二轮高亮成功，但返回按钮点击抢在页面动画结束前。最终结果和退出码以 `evidence/2026-11-13/file-viewer-mobile-e2e-result.json` 为准。

测试模拟器完成后恢复正常国内 release 111321，正常 APK 中不含集成测试入口或临时登录服务标记。iOS/Web 本轮未做端到端验收。本地 Mac ad-hoc 签名构建观察到 Keychain 写入 `-34018`，本轮重新登录后完成测试，未把重启免登录计为通过。

### OPPO 真机追加验收：通过

同日随后在 OPPO PLP110 / Android 16 的正常国内 release 111321 上完成独立端到端 UI 验收。先在“我的”页面核对为本次指定账号，再进入任务页，选择真实 Desktop 的 Viewer tree-sitter acceptance 会话。全程未装测试 APK，未清除账号或应用数据，也没有使用测试入口、root 或接口直接打开页面来代替 UI 点击。

镜像键盘导航有效，但镜像点击投递不可靠。用户明确允许使用 ADB input / UIAutomator 后，改用真实 UI 节点的 bounds 操作；每步重新读取 UI 层级，并保留设备原始截图。没有把镜像投递问题算作应用功能缺陷。

- 对话 → Viewer README → “查看源码”：实际打开中文文件名 `示例.dart`，可见 `Tree-sitter · dart`、`TREE_SITTER_OWNER_OK`，关键字、字符串、注释、函数和数字颜色正确，第 2 行突出显示。
- README → “定位到第120行”：使用宿主 workspace 中另一个 160 行合成文件 `定位示例.dart`。未发送滚动操作，第 120 行已成为首个可见代码行，突出显示且包含 `LINE_120_OWNER_OK`；避免只用本来就在首屏的第 2 行证明自动定位。
- README → “下一文档”：实际显示 `DOCUMENT_LINK_OWNER_OK`；点击文内“返回 README”回到原文档，连续跳转通过。

可见节点断言及截图均通过。本轮未发现上述功能的应用缺陷，无需修改应用代码；保留原正常版 111321。记录见 [真机结果](evidence/2026-11-13/oppo-file-viewer-e2e-result.json)，原始截图见 [高亮](evidence/2026-11-13/oppo-tree-sitter.png)、[第120行](evidence/2026-11-13/oppo-line120.png)、[目标文档](evidence/2026-11-13/oppo-document-link.png)、[文内返回](evidence/2026-11-13/oppo-readme-return.png)。

## 交付与诊断

- Desktop：`/Applications/OpenMuse.app`，已校验完整代码签名及 arm64/x86_64 Tree-sitter framework。增补版权 assets 后先重签 App.framework，再签根包，严格深度校验通过。
- Android：`app/openmuse_mobile/build/app/outputs/flutter-apk/app-release.apk`，国内正常版 1.0.1（111321），同一发布签名。
- 桥接闭包：`third_party/dsh/tarballs/openmuse-dsh-bridge-0.1.22.tgz` 及锁文件，嵌入模块与源码一致。
- 证据：`evidence/2026-11-13/file-viewer-mobile-e2e-result.json`、`file-viewer-mobile-trace.jsonl`、`file-viewer-desktop-trace.jsonl`。Trace 仅保留阶段、时间、状态和字节数等固定字段。
- 真机追加 trace：`oppo-file-viewer-trace.jsonl` 和 `oppo-file-viewer-owner-trace.jsonl`。后者按本轮时间窗保留宿主事件，不能据此宣称每个 span 已有跨设备父子关联或计算完整点击到展示耗时。
- Mac 磁盘余量不足曾导致 Android native libs 合并失败。仅清理本任务生成的过时 APK 副本、编译中间产物与临时测试 AVD 后重建；未清理他人的缓存、账号数据或原有工作区修改。
