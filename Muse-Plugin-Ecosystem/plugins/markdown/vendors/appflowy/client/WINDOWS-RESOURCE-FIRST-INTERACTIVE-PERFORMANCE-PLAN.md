# Windows Resource First-Interactive 性能方案与实施计划

> 适用范围：Host 内嵌的 `vendors/helix` 与 `vendors/open-file-viewer` 资源引擎。  
> 核心指标：从用户点击资源到该资源在 Host 当前 Tab 内首次可见、可操作（TTFI），而不是“路由完成”“进程创建”或“收到第一个字节”。  
> 状态日期：2026-09-22。本文把已实施项与平台专属待验证项明确分开。

## 1. 目标与非目标

### 1.1 产品目标

1. 点击资源后 100 ms 内给出稳定的当前 Tab 反馈，不出现独立窗口、空白闪烁或旧资源残影。
2. 优先缩短首屏可交互时间；语法高亮、LSP、缩略图、全文索引等增强能力不得阻塞首屏。
3. 同一套 Trace 能跨 Host 路由、资源解析、Tab、引擎启动、文件传输与引擎首帧，不与 Helix、Viewer 或 Flutter UI 绑定。
4. 性能数据默认不记录绝对路径和文件内容，可导出为 JSON 或 Chrome/Perfetto Trace。
5. 冷启动、会话内首次打开、同引擎再次打开分别测量，不能用平均值掩盖长尾。

### 1.2 本阶段非目标

- 不用 loading 动画掩盖同步阻塞；必须消除或下沉阻塞。
- 不把 Ontology Runtime、语义索引或 LSP 完整就绪计入首屏门槛。
- 不以 `Pty.start` 返回、WebView 导航完成、首个 PTY 字节作为“可交互完成”。

## 2. 统一时间模型

一次打开使用一个 `traceId`，按以下节点记录：

| 节点 | 定义 | 当前事件/Span |
| --- | --- | --- |
| T0 | 用户/DSH 发起打开 | `resource.open` start |
| T1 | Resource 解析完成 | `resource.resolve` end |
| T2 | Tab 请求被 Host 接受 | `host.tab.dispatch`、`host.tab.accepted` |
| T3 | 引擎 Surface 建立 | `engine.surface.created` |
| T4 | 引擎运行时可用 | `engine.helix.boot` / `engine.viewer.boot` |
| T5 | 资源被引擎接受 | `engine.viewer.resource.accepted` / Helix PTY start |
| T6 | 第一帧真实资源内容已提交 | `engine.first-interactive-frame` |
| T7 | Trace 完成 | `resource.open` end |

主指标：

- `TTFI = T6 - T0`。
- `Host overhead = T3 - T0`。
- `Engine TTFI = T6 - T3`。
- `UI blocking`: 单个同步任务占用 UI isolate 的时间；目标最大值 `< 50 ms`，理想 `< 16 ms`。
- 所有指标按引擎、扩展名、文件大小桶、cold/warm/session-reuse 分组，绝不记录完整 locator。

## 3. 性能预算与验收线

| 场景 | P50 | P95 | 首屏定义 |
| --- | ---: | ---: | --- |
| Host Tab 反馈 | 50 ms | 100 ms | Tab 与 loading skeleton 已出现 |
| Helix 冷打开，小型文本/代码 | 650 ms | 1000 ms | 真实文件内容可见，键盘可输入 |
| Helix 会话内切换（目标架构） | 250 ms | 500 ms | 同一运行时加载目标文件完成 |
| Viewer 文本/图片 | 250 ms | 500 ms | 首屏内容可滚动/缩放 |
| Viewer PDF 首页面 | 500 ms | 900 ms | 第一页可见并可滚动 |
| Viewer Office 首页面 | 900 ms | 1800 ms | 第一页或首个 Sheet 可交互 |

文件大小分桶：`<1 MiB`、`1–16 MiB`、`16–64 MiB`、`>64 MiB`。当前 Base64 bridge 只支持前 3 桶；`>64 MiB` 必须由后续 URL/range transport 覆盖。

## 4. 解耦 Trace 基建

### 4.1 设计

`lib/core/performance/muse_performance_trace.dart` 是纯 Dart 基建：

- `MusePerformanceTracer`：生成 Trace，使用单调时钟。
- `MusePerformanceTrace` / `MusePerformanceSpan`：显式跨异步边界传播；不依赖 Zone 隐式上下文。
- `MusePerformanceSink`：输出端口；业务不感知日志、文件或遥测实现。
- `MuseRingBufferPerformanceSink`：有界内存，避免长期运行无限增长。
- `MuseCompositePerformanceSink` / `MuseCallbackPerformanceSink`：可组合本地诊断、测试断言和未来遥测。
- `exportJson()` 与 `exportChromeTraceJson()`：后者可直接导入 Chrome/Perfetto。

约束：

- Trace 只随 `MuseResourceOpenRequest` 显式传入，不由具体引擎创建全局隐藏状态。
- 属性只允许标量；字符串截断到 256 字符。
- 默认记录 `origin`、扩展名、文件大小和阶段，不记录绝对路径、文件名、URL token 或正文。
- Sink 失败不得影响资源打开；生产遥测 Sink 后续必须自行做批处理、背压与采样。

### 4.2 已实施链路

```text
Workspace / DSH click
  -> MuseResourceSurfaceOpener (start resource.open)
  -> Resource resolver
  -> TabsBloc dispatch
  -> ResourceFilePlugin
  -> HelixResourceSurface / OpenFileViewerResourceSurface
  -> engine first interactive frame
  -> resource.open finish
```

已有单测覆盖：单调 duration、事件顺序、Ring Buffer 容量与导出、失败状态。

## 5. Helix 首屏方案

### 5.1 已实施的止损优化

1. **删除本地 Windows PTY 的固定 1 秒休眠。** 旧 `flutter_pty_win.c` 在每次 `CreateProcessW` 前无条件 `Sleep(1000)`；CI patch 已删除，本地 vendor 也同步删除，消除构建来源差异。
2. **关闭生产路径上的 legacy warm pool。** 当前池通过 PTY 向已启动 Helix 逐字符输入 `:open <path>`，路径补全会反复扫盘，实测比直接冷启动慢 2.3–3.8 倍；同时补池的同步 `Pty.start` 会再次冻结 UI。代码保留为显式实验开关，默认不进入打开链路。
3. **安装解析进程级缓存。** `HelixInstall.resolve()` 复用 Future，失败会清缓存以允许重试。
4. **Grammar 编译移出打开链路。** 打开仅探测已安装 grammar；缺失时显示设置入口，不再在首屏前访问网络或启动编译器。
5. **首屏判据修正。** 不在 ConPTY 控制字节到达时撤掉遮罩；Terminal 模型出现真实非空内容后，再等一个 Flutter frame 完成 Trace。
6. **插入模式不再使用固定 280 ms 猜测。** 等待首屏提交后进入插入模式，超时则不注入按键。
7. **完整分段 Trace。** 设置、安装解析、grammar、languages、项目根、PTY 同步启动、首字节、首个交互帧分别计时，并把 `Pty.start` 标为 `blocksUiIsolate`。

### 5.2 已实施：Async PTY Broker（P0）

当前最大架构债是 `Pty.start` 通过同步 FFI 在 Flutter UI isolate 调用 `CreatePseudoConsole/CreateProcessW`。Dart isolate 不能安全搬运原生 PTY 句柄，因此不能简单用 `compute()` 修复。

实现结构：

```text
Flutter UI
  -> async method channel / FFI request
Native PtyBroker worker thread
  -> CreatePseudoConsole + CreateProcessW
  -> sessionId
Native IO loop
  -> output events(sessionId, bytes)
Flutter Terminal model
```

`flutter_pty` 新增 `Pty.startAsync()` 和进程级长驻 worker。Native handle 只在进程内以地址返回，stdout/exit 直接投递到调用侧的 Dart Port；`write`、`resize`、`kill`、`exitCode`、`output` API 保持兼容。`Pty.prewarmAsync()` 在 Windows Host 启动时完成 isolate、动态库和 Dart Native API 初始化。

实测门禁连续创建真实 PTY，UI event-loop 最大间隙必须 `<100 ms`；旧同步实现的 360–1000 ms 卡顿无法通过。最新 macOS 30 次基线中，异步 PTY 总计 `186 ms`、最大 UI event-loop gap `9.5 ms`；完整 Helix Surface 冷样本 `769 ms`，30 次 P95 `147 ms`，均通过预算。

### 5.3 Helix Runtime 决策

当前直接进程 + async PTY 已达到首屏预算，因此不再为了复用而恢复负收益的 TUI warm pool。只有 Helix 后续提供结构化控制通道时才重新评估会话复用，禁止通过 TUI `:open` 模拟 API。可选演进：

1. Helix sidecar/RPC patch：`open(path,line)`、`saved`、`ready`、`diagnostic` 事件。
2. 长驻 engine-host 进程管理每 workspace 的 Helix session。
3. 未完成 RPC 前继续直接启动，保证可预测 TTFI。

## 6. open-file-viewer 首屏方案

### 6.1 已实施

1. **入口分包。** 首屏不再静态打入 PDF、Office、音视频、压缩包等全部插件；入口仅包含 viewer core，根据 mime/扩展名动态 import 对应 domain plugin。
2. **ES Module + code splitting。** 构建输出为 `viewer.js` 与 `lazy-*.js`；chunk 与入口保持在 Flutter 已声明的直接资源目录，`index.html` 使用 module script。
3. **同域资源复用。** 已存在同类型 viewer 时调用 `viewer.reload(file)`，避免销毁/重建插件。
4. **桥接握手。** `viewer.shell-ready`、`viewer.resource-accepted`、`viewer.ready`、`viewer.error` 都携带阶段语义；`requestId` 用于关联 Host Trace。
5. **首屏完成语义。** 仅 `viewer.ready` 后、Flutter 下一帧提交时结束 Trace。
6. **分段 Trace。** WebView 初始化、shell load、读文件、Base64、bridge send 分开计时。

构建级验证：旧单 bundle 约 `18.53 MB`；新首屏 `viewer.js` 约 `53 KB`，CSS 约 `132 KB`，领域代码延迟加载。总功能代码没有消失，而是不再进入每次首屏关键路径。

### 6.2 已实施：本地 Resource URL Transport（P0）

Windows WebView2 已不再读取整个文件并 Base64 编码。每个 controller 把目标文件父目录映射到 `https://resource.openmuse`，Host 只向 JS 传递文件名、mime、requestId 和 URL。它移除了 4/3 膨胀、Dart/JS 全量复制和 64 MiB bridge 上限，并允许 PDF.js 对 URL 使用流与 Range。

通用协议仍保持 Resource Capability 形态：

```text
ResourceCapability {
  requestId,
  mime,
  size,
  read(offset, length),
  stream(),
  revoke()
}
```

- 本地文件：WebView controller 私有的虚拟 host URL，支持 WebView2 资源读取与 HTTP Range。
- SSH/Cloud：Host/DSH 代理的 range stream，不暴露云端 token。
- 小文本可保留 inline bytes 快路径，但阈值由 Trace 数据决定。
- PDF、视频、音频必须按 range 拉取；Office 转换结果按页/Sheet 渐进返回。

### 6.3 已实施：WebView Runtime Broker（P1）

进程级 `OpenFileViewerRuntimeBroker` 保持一个已完成 WebView2 initialize、viewer host mapping 和 `viewer.shell-ready` 的空闲 runtime：

- Host 启动阶段异步预建 shell；不预载任何领域插件。
- Surface 获取预热 controller 后只映射并发送新的 Resource URL。
- 隔离每次 `requestId`，迟到的 `ready/error` 不得覆盖当前资源。
- 空闲 shell 有 TTL 和内存上限，内存压力时可回收。

## 7. 分阶段开发计划

| 阶段 | 交付物 | 状态 | 退出标准 |
| --- | --- | --- | --- |
| F0 | 纯 Dart Trace、传播、导出、基础测试 | 已实施 | 可形成完整 `resource.open` 时间线 |
| F1 | Helix 冷路径止损：去固定休眠、缓存、grammar 移出、真实首帧、禁用负收益预热 | 已实施 | 冷路径不再额外等待 1 s/编译/二次 spawn |
| F2 | Viewer 动态分包、阶段握手、同域 reload | 已实施 | 首屏 bundle `<100 KB JS`，只加载目标领域 chunk |
| F3 | Async PTY 长驻 worker + Host 预热 | 已实施 | UI event-loop gap 门禁 `<100 ms`；Helix P95 `<1 s` |
| F4 | Windows 本地 Resource URL/Range Transport | 已实施 | 无 Base64 大文件桥与 64 MiB 限制 |
| F5 | WebView2 Runtime Broker；Helix 采用 async direct process | 已实施 | Viewer shell 在点击前 ready；无 TUI 命令注入 |
| F6 | 跨平台性能 Gate + Windows 一键 runner + CI 强制执行 | 已实施，Windows 真机数值待产出 | bundle、UI gap、Helix TTFI、Viewer TTFI 超预算即失败 |

SSH/Cloud 的 capability stream 属于 Provider 扩展阶段，不重新引入 Base64；它不阻塞本地引擎首屏计划验收。

## 8. 测试矩阵

### 8.1 自动化测试

| 层 | 用例 | 断言 |
| --- | --- | --- |
| Trace 单测 | 正常、异常、重复 finish、容量溢出 | 顺序、duration、status、无重复 end |
| 路由 Widget | Workspace/DSH/手动换引擎 | 同一 traceId 传播到 Surface；Tab 语义不变 |
| Helix 单测 | install cache、grammar 缺失、visible-content 判据 | 不触发网络/编译；控制序列不算首屏 |
| Viewer 构建 | text/image/pdf/office 分包 | 初始 JS 预算；对应 chunk 才被请求 |
| Viewer bridge | request race、reload、error | 迟到事件被忽略；当前请求准确完成 |
| Native Broker | create/write/resize/kill/crash | 无 UI 同步阻塞；session 互不串流 |

### 8.2 Windows E2E 样本

每种场景至少 30 次，前 5 次单独标记 cold，不混入 warm 统计：

- Helix：`.rs`、`.ts`、`.java`、`.c`、`.md`；1 KB、1 MB、10 MB；路径含空格/中文；窄 pane 与 4K pane。
- Viewer：`.txt/.html/.png/.svg/.pdf/.docx/.pptx/.xlsx/.mp3/.mp4/.zip`；小/中/大文件。
- 来源：Workspace 本地、DSH 点击；F4 后加入 SSH/Cloud。
- 操作：首次打开、关闭重开、同域切换、跨域切换、快速连续点击 10 个文件、打开中关闭 Tab。

采集：TTFI P50/P95/P99、最大 UI blocking、CPU 峰值、RSS 增量、读取字节量、首屏 bundle/chunk、错误与 fallback 比例。测试报告必须附 Chrome Trace，而不是只附总耗时。

## 9. 本次实施验证清单

- [x] Trace 基建与单元测试。
- [x] Workspace/DSH → Host Tab → Engine 的 Trace 传播。
- [x] Helix 打开阶段 Trace 与真实内容首帧判据。
- [x] Helix 安装缓存、grammar 非阻塞、legacy warm pool 默认关闭。
- [x] 本地 Windows PTY 固定 1 秒休眠删除；CI patch 已有等价行为。
- [x] Viewer ESM 动态分包与 bridge milestone。
- [x] Viewer 首屏 bundle 构建级预算验证。
- [x] Async PTY worker、启动预热与 UI event-loop gap 门禁。
- [x] Helix Surface 30 次真实 TTFI 门禁；本机冷样本 `769 ms`、P95 `147 ms`。
- [x] Windows Viewer URL transport、WebView2 Runtime Broker 与 Windows 专属 P95 测试。
- [x] macOS Debug 应用构建与动态 chunk 打包验证。
- [x] Windows 构建流水线默认执行 30 次首屏性能基线（独立于 `-SkipTests`，仅可显式 `-SkipPerformanceTests` 跳过）。
- [ ] Windows 真机执行 `tool/performance/run_first_interactive_tests.ps1`，记录该机器的 30 次稳定基线。

代码层计划已经完成；Windows 专属 Viewer/ConPTY 数值必须由 Windows runner 产生，macOS 构建和测试不能替代该平台实测。性能 runner 使用硬门禁而非仅生成报告，任何回归会直接失败。
