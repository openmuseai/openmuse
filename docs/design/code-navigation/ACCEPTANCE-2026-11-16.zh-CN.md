# 国内 Desktop 与 OPPO 正常版代码导航验收

验收跨越 2026-11-15 至 2026-11-16（Asia/Shanghai）。本记录覆盖本次实现和真实 UI 路径；大型仓库性能及完整产品稿仍有独立门禁。

## 安装和账号

- Desktop：`1.0.1 / 111502`，`/Applications/OpenMuse.app`，国内正常 release；安装后 `codesign --verify --deep --strict` 通过。DSH bridge `0.1.24`，独立代码导航服务 `0.1.0`。
- OPPO PLP110 / Android 16：`com.openmuseai.mobile`，`1.0.1 / 111501`，国内正常 release，同发布签名，无 DEBUGGABLE 标志。通过 `adb install -r` 覆盖更新，保留账号数据；没有安装测试入口。
- 两端可见账号均核对为用户指定的同一国内账号。本次复用并恢复已有登录态，未退出或清除数据；证据中不保存账号密码。
- Desktop 使用原生 CUA 操作；OPPO 使用用户明确授权的 ADB input / UIAutomator。真机没有替换为模拟器。

## 实际通过的 UI 路径

| 场景 | Desktop | OPPO 真机 |
|---|---|---|
| 对话文件 → README → 中文路径源码 | 通过，打开 `导航示例.dart` | 通过，打开 `导航示例.dart` |
| 点击源码 `greeting` → 函数定义 | `服务定义.dart` 第 1 行、第 8 列；打开后显示 `DEFINITION_OWNER_OK` | 相同目标、行列及标记；第 1 行高亮 |
| 点击源码 `service.greet` → 方法定义 | `服务定义.dart` 第 4 行、第 10 列；打开后显示 `METHOD_OWNER_OK` | 相同目标、行列及标记；第 4 行高亮 |
| 函数及方法跳转后返回 | 返回原源码页面，保留原位置 | 返回 `导航示例.dart`，原第 4 行和页面位置保留 |
| 点击代码中的 `import '服务定义.dart'` | 本次未单独记录 | 正确打开被引用文件，第 1 行、第 1 列；返回原源码 |
| README → 下一文档 → 文内返回 README | 本次未重复此项 | 显示 `DOCUMENT_LINK_OWNER_OK`；文内返回成功 |
| 文内链接定位第 120 行 | 本次未重复此项 | 第 120 行出现在视口顶部，`LINE_120_OWNER_OK` 可见且高亮 |
| Tree-sitter 高亮 | Dart 源码显示语法颜色 | 中文路径的两个源码文件及第 120 行样例均有实际语法颜色 |
| 可选服务开关 | 默认关闭，显式启用后查询，关闭后显示默认关闭 | 相同流程通过；关闭后已确认语言服务进程退出 |

函数与方法验收点击的是实际渲染的代码字符，经宿主 Dart language-server 解析；结果没有用固定候选或文本搜索替代。Markdown 文件链接、代码 import 与语义符号是分别验证的路径。

OPPO 原始截图及 SHA-256：[结构化验收证据](evidence/2026-11-15-16-desktop-oppo/acceptance.json)。Desktop 证据为本任务的原生 UI 截图/可访问性观察，没有另存 Desktop PNG。完成后再次取证时 Mac 已锁屏，因此不补造 Desktop 文件。

关键真机证据：

- [函数定义](evidence/2026-11-15-16-desktop-oppo/function_definition.png)、[函数返回](evidence/2026-11-15-16-desktop-oppo/function_return.png)。
- [方法候选行列](evidence/2026-11-15-16-desktop-oppo/method_result.png)、[方法定义](evidence/2026-11-15-16-desktop-oppo/method_definition.png)、[方法返回](evidence/2026-11-15-16-desktop-oppo/method_return.png)。
- [import 文件目标](evidence/2026-11-15-16-desktop-oppo/import_result.png)、[第 120 行与高亮](evidence/2026-11-15-16-desktop-oppo/line120_final.png)、[关闭状态](evidence/2026-11-15-16-desktop-oppo/disabled.png)。

## 实现与修复

1. Resource Center 增加可选 `ResourceCodeNavigationProvider` 和窄操作 RPC。通过现有账号、source、workspace、generation 与 peer 校验，await 前后再次检查，结果必须属于相同资源源；不接受客户端语言命令或宿主绝对路径。
2. DSH adapter 从会话原始文件句柄恢复 authoritative workspace、源文件内容与摘要，交给独立服务。每个 LSP 目标再次通过宿主文件解析和授权，返回带版本的统一资源引用。手机不拼接 Mac 路径。
3. 服务默认关闭。工作区启用策略保存在资源宿主，启用仅改变策略；第一次语义查询才启动语言进程。宿主本地 trusted profile 配置 Dart SDK，手机不能提交 command / args / env。关闭后等待释放，Tree-sitter 高亮独立保留。
4. Viewer 插件增加符号点击、定义/引用/实现面板、候选路径与行列、工作区启用和关闭入口。SDK 只提供受限回调，Viewer 不管理语言进程。
5. 修复选择文本包装层导致符号点击未命中的问题：找到实际 `RenderParagraph`，由渲染坐标取得 UTF-16 位置。拖动和长按不触发普通符号点击，保留滚动与选择。
6. 定义目标通过统一资源打开流程压入导航栈，避免 Desktop 将目标替换为原页面；返回保留原源码视口。当前可视定位为整行高亮，尚未实现列范围高亮。

独立服务没有 DSH/Flutter 依赖。此次产品装配提供的是 DSH 本地文件 owner adapter；其他资源宿主仍通过同一可选接口接入，不把通用资源协议绑定为 Mac 文件系统协议。

## 验证与构建记录

本次最终重跑：共享 Viewer 7 项通过；资源导航 RPC 与相对文件链接 3 项通过；DSH bridge 普通测试 23 项通过；真实 Dart owner adapter 1 项通过。Viewer、资源打开接线、DSH source 的定向静态分析均无问题。

```sh
cd packages/openmuse_file_viewer_flutter && flutter test
cd packages/muse_resource_center_io && flutter test test/code_navigation_test.dart test/relative_file_link_test.dart
cd third_party/dsh/plugins/openmuse-dsh-bridge && npm test
OPENMUSE_TEST_DART=/path/to/dart node --test test/language/code_navigation.test.js
```

上方各 `cd` 从仓库根目录分别执行。Dart 真实测试与无 SDK 的普通测试分开，不能把替身测试计为语言能力通过。真实测试还验证默认零实例、启用惰性、跨文件函数/方法、旧摘要拒绝及停止后零实例。

打包 tarball 的运行时源码与仓库逐文件相同，根 lockfile 的 SHA-512 integrity 校验通过。Android 在线构建遇到 Java Maven TLS 连接失败，随后使用已缓存的依赖离线构建正常 release 成功。安装的是 `build/app/outputs/apk/release/app-release.apk`；未误用旧的 `outputs/flutter-apk` 副本。构建期间只清理本任务可重建的 profile 中间目录，未清账号、业务数据或最终安装包。

另在独立临时目录执行 bridge 开发依赖的 `npm ci --offline --ignore-scripts --no-audit --no-fund`，两个运行依赖安装成功，未触碰当前实际安装的依赖目录。

## 诊断与未完成范围

现有 owner HTTP trace 已覆盖 `resource.owner.navigation` 的 ready / response / decoded / total，并传递 `traceparent`；独立服务保留 queue / initialize / query / total 阶段。本次正常 release 未启用 console trace 输出，未保存点击到预览的时延样本，也尚未贯通跨设备到语言进程的完整父子链。不能由此次 UI 通过推断冷/热 P95、RSS 或 CPU 达标。

仍需分别实施/验收：通用挂载目录及非本地执行世界 adapter、插件中心完整设置与语言安装 UI、符号目录、代码摘要预览、列范围高亮、跨设备 trace 关联、大型仓库 30 次以上冷/热性能与 RSS/CPU、Windows 进程树清理。引用和实现已有接口/面板，本次真机专项验收仅对文件、定义、方法与返回作通过声明。

手机保留国内正常版并停在 README，未清除账号数据。Desktop 正常版保持运行。验收结束将该工作区代码导航关闭，已确认无本次遗留 Dart 语言服务进程；后续可在 Viewer 的“代码导航”入口重新按需启用。

## 细粒度提交

- `5402e461`：统一资源导航契约、Hub 与授权 RPC。
- `21bcbdde`：共享 Viewer 点击/面板、SDK 受限回调、Viewer 插件和交互测试。
- `bc01d29a`：独立服务装配、DSH owner adapter、真实 Dart 测试与锁定 tarball。
- `fc959ed0`：资源宿主链接解析、目标导航栈及打开接线。
- 开发依赖闭包另行提交：bridge source lockfile 可离线独立安装。

当前工作区的协作编辑、发布和文档重组等既有修改仍保留。SDK 与资源打开混合文件仅暂存相关补丁，没有整文件提交其他工作，也没有 push。安装和测试基于当前工作区，不将上述提交称为完整干净的产品发布基线。
