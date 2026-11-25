# 首期插件市场实施与验证记录

日期：2026-11-17。状态：实现、正常版构建及自动化验证完成；Desktop / Android 真机市场闭环 UI **尚未验收通过**。

产品与协议：[产品规格](PRODUCT.zh-CN.md)、[架构](ARCHITECTURE.zh-CN.md)、[计划](PLAN.zh-CN.md)。结构化证据：[readiness.json](evidence/2026-11-17/readiness.json)。

## 已实现

- 三个独立 curated market 条目及归档：代码导航服务 `0.1.1`、Dart profile `0.1.0`、TS/JS profile `0.1.0`。真实复制安装，完整文件摘要校验，未安装时不加载语言内核。
- 独立 Node 管理器 `0.1.2`：账号隔离 registry、原子配置、revision 冲突拒绝、依赖顺序、配置检测、启用、关闭释放及卸载。关闭失败保留状态和可重试实例；检测过程使用真实语言服务，结束后清理。
- Dart 使用宿主 SDK；TS/JS 随插件安装固定 LS 与 TypeScript。包括 TSX/JSX languageId；初始化和 section-scoped LS configuration 独立配置；不接受 shell 字符串或点击代码时附带启动命令。
- 两端共用设置页，Desktop 市场和 Mobile 的 Desktop 市场区均可进入。管理响应后才显示生效；错误覆盖依赖缺失、冲突、SDK 缺失及停止失败。
- 现有 paired grant 管理接口扩展 configure/detect/uninstall，处理前后重验账号/grant；64KiB 正文、1MiB 响应。通用代理拒绝私有管理路由，由 Host 注入真实账号与 bridge token。
- DSH bridge `0.1.27` 只承担 owner/管理 adapter，查询仍经统一资源能力。插件管理器与内核没有 Flutter/DSH 依赖。
- 配对服务在明确允许时，仅对 loopback 端口占用回退到实际空闲端口并发布 origin；没有结束正在占用默认端口的 SmartDocument QA 应用。

首期条目进入随正常应用签名覆盖的市场快照，**没有向公共 npm/MCP Registry 或线上市场服务执行发布**。后续公共发布需走相应发行链路。

## 自动化验证

| 测试 | 最终通过数 | 说明 |
|---|---:|---|
| 独立插件管理器 | 5 | 安装/依赖/冲突/账号隔离/摘要、停止失败保留与重试、真实 Dart 与 TS 检测 |
| 服务内核 | 21 | 限流、版本、取消、超时、真实 subprocess/worker 释放等既有门禁 |
| 服务真实 TS 跨文件 | 1 | 定义、引用、hover、中文与 UTF-16、热复用 |
| Bridge 普通回归 | 23 | 文件/协议/宿主能力 |
| Bridge 真实语言 owner adapter | 2 | Dart、TS 实际跨中文文件函数与方法定义，版本拒绝、关闭 |
| 共用配置页 | 2 | revision 保存、等待宿主回执、关闭失败不显示成功 |
| Paired 市场与端口 | 2 | grant、固定操作、私有路径绕过拒绝、loopback 端口冲突 |
| 合计 | 56 | 最终运行无失败、无跳过；这些不是双端 UI 通过数量 |

管理器和两个 owner adapter 的真实测试还使用了拟发行 Node `22.22.2`，避免只在开发者 Node 上通过。Host main/管理 client、配置页、Desktop/Mobile 市场、paired gateway 的定向分析通过。

## 正常版与发行验证

Desktop：国内正常 release `1.0.1 / 111705`，已安装到 `/Applications/OpenMuse.app`。代码签名 deep/strict 验证通过；三个运行包的 lib 源码、锁定 tarball 和安装内容逐文件相同。三个插件交付目录共 193 个文件摘要均通过。

Android：国内正常 release `com.openmuseai.mobile / 1.0.1 / 111704` 已构建，APK 路径为 `app/openmuse_mobile/build/app/outputs/apk/release/app-release.apk`，SHA-256 在 JSON 中。使用现有正常发布签名；未清除账号数据，未新增测试入口 APK。

Android 构建先遇到 Gradle variant 缓存缺失与 Java TLS 下载失败。补齐依赖并通过 Flutter 正常构建阶段刷新 generated registrant 后，以离线 `:app:assembleRelease` 构建成功；没有通过把 integration_test 加入生产依赖解决。根级 `assembleRelease` 会额外构建 dev-only 子项目，不能拿它的失败等同于 app release 编译失败。最终正常包是 `111704`。

原 Desktop Node `22.19.0` 低于固定 TS LS 的 engines 要求。更新为官方 `22.22.2` macOS arm64/x64 归档，分别验证官方 SHASUMS256 后合成 universal；相关归档和固定摘要进入发行闭包。来源：[Node 官方发行说明](https://nodejs.org/en/blog/release/v22.22.2)、[官方摘要](https://nodejs.org/dist/v22.22.2/SHASUMS256.txt)。

## 真机尚未完成的步骤

已连接的设备为 OPPO PKM110 / Android 16，与前轮 PLP110 不同。正常包安装进入 OPPO 的“拖动下方滑块完成拼图”安全验证；尚未完成验证码，安装命令仍等待系统结果。当前安装器缓存的是先前构建 `111702`，通过后还需覆盖最终 `111704` 并核对版本/flags，再开始业务 UI 验收。

Desktop 曾显示“继续即表示同意服务条款和隐私政策”的登录页，尚未取得本轮接受协议的明确确认；后续 Mac 锁屏，原生工具返回无法操作。`.account` 在观察时为空，未从其中取得新凭据，也未将此前提供的密码写入文件。

已分别请求：Mac 解锁、协议接受确认、由用户完成或明确授权本次 OPPO 滑块。它们是计算机操作工具/系统的当前约束，不是技能文件额外增加的审批。没有替用户接受未确认协议、处理未授权验证码或绕过锁屏。

恢复后仍须完成：

1. 安装最终 Android 正常版，两端核对同一国内账号。
2. 两端市场看到三项，真实安装、修改配置、检测与启用；另一端刷新显示相同状态。
3. 工作区显式开启后，真实 Dart/TS 首次查询、中文文件/函数/方法跳转、返回及热查询。
4. 关闭后验证进程释放，服务依赖卸载拒绝，按顺序卸载，再安装。
5. 保存本轮真实 UI 截图与结果。前轮文件导航截图不能代替本轮市场闭环证明。

大型仓库冷/热 P95、RSS/CPU、Windows 清理仍是独立门禁。本轮没有用单次语言检测冒充这些指标达标。

## 提交与工作区

`1c86d233` 计划；`fa6a993a` 独立插件管理与交付物；`e68f0734` 双端共用设置和市场入口；`16648594` 授权管理/owner adapter/端口冲突；`b1c80871` 发行闭包与兼容 Node；`0606d89f` 状态说明修复。

混合的 main/gateway/打包/ignore 文件只暂存本任务补丁；既有协作编辑、发布、语言文件和文档重组保持原状。正常构建来自当前工作区，不能把这些提交声称为完全干净的独立发行基线。没有 push。
