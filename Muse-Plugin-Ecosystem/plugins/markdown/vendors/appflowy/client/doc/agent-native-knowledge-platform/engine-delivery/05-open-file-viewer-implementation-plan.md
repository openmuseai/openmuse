# open-file-viewer Engine Adapter：实施、开发与测试方案

状态：**目标实施计划**  
上游：`vendors/open-file-viewer` 0.1.45  
产品模式：只读 Viewer/Fallback  
首发：Desktop Web Surface + Web；Mobile Wave 1 子集  
依赖：公共 G0/G1；不依赖 Ontology Runtime。

## 1. 产品定位

open-file-viewer 是 Muse 的安全、多格式、浏览器端只读引擎：

- 当原生/专业编辑器不可用或用户选择预览时提供一致的查看体验；
- 支持文本、图片、PDF，并按安全认证逐步开放 Office、Archive、Email、CAD、3D、GIS；
- 在 Desktop 中运行于隔离 Web Surface，在 Web 中运行于受限 viewer origin，在 Mobile 中只启用资源可承受的插件；
- 不拥有 Resource 权限、不读取 Host path、不执行资源写入；
- download、print、fullscreen、外链和远程 iframe 是独立 Host policy capability。

它不能被视为“所有格式都已经支持”。代码中有插件只表示存在解析路径，只有通过 Muse format certification 的插件才进入生产 Manifest。

## 2. 当前可复用 API

公共核心：

```text
createViewer(options) → FileViewer
isPreviewSupported(source, plugins, options) → boolean

PreviewPlugin
  name
  match(file)
  render(ctx) → PreviewInstance

PreviewInstance
  resize()
  goToPage()
  command()/canCommand()
  preparePrint()
  destroy()
```

已有生命周期优势：

- render generation token；
- `AbortController` 在 reload/destroy 时取消；
- `ResizeObserver`；
- loading/error/unsupported；
- plugin 顺序和 terminal fallback；
- queue、page navigation、zoom/rotate/search/print。

需要 Muse Adapter 补充：

- runtime effective plugin manifest；
- Resource materialization broker；
- sandbox/CSP/origin/message contract；
- Host toolbar policy；
- page/selection Context hook；
- format wave、大小和内存限制；
- crash/worker/remote fetch telemetry。

## 3. 目标架构

```text
Resource Host
  ├─ describe/authorize
  ├─ loopback/remote URL or bytes handle
  └─ range, TTL, revoke
          │
          ▼
Viewer Surface Adapter
  ├─ plugin allowlist + probe
  ├─ sandbox document
  ├─ postMessage bridge
  ├─ toolbar policy
  └─ receipt/context
          │
          ▼
isolated viewer origin
  createViewer(...)
  text/image/pdf/... certified plugins
```

建议代码落点：

```text
Muse-Clients/middlewares/dsh/plugins/dsh-engine-open-file-viewer/
Muse-Clients/frontend/client/frontend/appflowy_flutter/packages/muse_web_viewer_surface/
Muse-Clients/frontend/client/frontend/appflowy_flutter/assets/open-file-viewer/
Muse-Clients/frontend/client/frontend/web/resource-viewer/
```

## 4. 关键决策

### O-D1：固定 viewer origin

Desktop 使用独立 WebView/iframe 文档和固定内部 origin，不把 Viewer 注入 DSH 页面或主 App DOM。Web 使用独立 route/origin，避免插件 CSS、全局事件和依赖污染主应用。

### O-D2：默认无外网

所有 worker、CMap、standard fonts 和插件依赖随包或由受控资产域提供。当前 PDF 默认可指向 jsDelivr 的行为在 Muse 配置中必须覆盖；文档打开不得隐式访问公共 CDN。

### O-D3：Host toolbar

Viewer 内建 previous/zoom/rotate/search 可在沙箱内执行。download、print、fullscreen、open external 由 Adapter 注入 Host 授权 action，不直接信任插件 `<a>` 或 `window.open`。

### O-D4：格式分波认证

- Wave 1：text、image、PDF。
- Wave 2：OOXML/ODF、archive、email、drawing/xmind。
- Wave 3：CAD、3D、GIS、媒体。

每个插件独立声明 size、network、worker、GPU、active content 和 context 能力。

### O-D5：Viewer 永远只读

任何编辑/批注不写入原 Resource。未来批注使用独立 Annotation Resource 和 overlay Adapter，本计划只预留 toolbar contribution。

## 5. Sandbox 设计

最低策略：

```text
default-src 'none'
script-src <bundled-hash-or-internal-origin>
style-src 'self' 'unsafe-inline'  # 若依赖要求；后续 nonce/hash 收紧
img-src 'self' blob: data:
font-src 'self' blob: data:
media-src 'self' blob: <scoped-resource-origin>
connect-src <scoped-resource-origin>
worker-src 'self' blob:
frame-src 'none'
object-src 'none'
base-uri 'none'
form-action 'none'
```

具体插件如 PDF web fallback 需要 iframe 时，必须使用更窄的派生 profile；不能为所有格式全局开放 `frame-src` 或 scripts。

postMessage：

- 固定 origin；
- `muse.viewer-bridge/v1` schema；
- 每 session nonce/generation；
- 最大消息 64 KiB；
- 只传状态、命令、page/selection 摘要，不传整文件；
- destroy 后消息失效。

## 6. Resource 输入策略

| Source | 场景 | 规则 |
|---|---|---|
| ArrayBuffer/Blob | Small、需完整解析的 zip 格式 | Adapter 从 bytes handle 获取；受 maxBytes |
| loopback URL | Desktop 大资源/PDF/media | bearer、origin、TTL、range、无日志 |
| remote scoped URL | Web/Mobile | CORS/range/audience/TTL；不暴露云原凭据 |
| string external URL | 默认禁止 | 只有 trusted external-url Provider + policy 才允许 |

`PreviewSource=string` 不意味着任意 URL 可进入 Viewer。Adapter 只接受 Host 签发的 materialization。

## 7. 阶段 O0：安全、构建与格式 Spike

### 7.1 阶段目标

证明 bundle、sandbox、PDF worker/range、WebView/iframe、内存和销毁清理；建立 format certification 模板。

### 7.2 方案设计

- 从 vendor 构建固定版本 core bundle + CSS + PDF worker/cmaps/fonts。
- 建最小隔离 viewer page，只注册 text/image/PDF。
- Host 生成 loopback/scoped URL，验证 range/TTL/revoke。
- 禁止网络，观察每个插件隐式 fetch/worker/iframe。
- 记录 Small/Medium/Large 的 CPU、内存、首屏和 destroy 后回收。
- 制作恶意 corpus：HTML/SVG/script、损坏 PDF、zip bomb、超大图片、跨域 URL。

### 7.3 开发任务

| ID | 任务 |
|---|---|
| O0-D1 | vendor bundle/asset 固定与 digest |
| O0-D2 | sandbox viewer shell + CSP |
| O0-D3 | postMessage v1 handshake |
| O0-D4 | loopback/remote materialization spike |
| O0-D5 | PDF worker/cmaps/fonts 自托管 |
| O0-D6 | format certification 模板 |
| O0-D7 | 内存/销毁/worker 检测 |
| O0-D8 | threat model 与恶意 corpus |

### 7.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| O0-T1 | 无公网环境打开 PDF | worker/font/cmap 不访问 CDN |
| O0-T2 | 错 origin/nonce message | 忽略并审计 |
| O0-T3 | materialization URL 过期 | 后续读取失败，Viewer 显示可重试 |
| O0-T4 | HTTP range PDF | 请求范围正确，可取消 |
| O0-T5 | destroy/reload | Abort、worker、Object URL、listener 全释放 |
| O0-T6 | SVG/HTML/script fixture | 无脚本执行/顶层导航 |
| O0-T7 | 超大图片/PDF | 预算前拒绝或渐进，不崩溃 Host |
| O0-T8 | PDF iframe fallback | 默认关闭或在专用 profile 中隔离 |
| O0-T9 | zip bomb | Wave 1 不解析；通用 size guard 拒绝 |
| O0-T10 | Desktop/Web 同 bundle | API/locale/theme 行为一致 |

### 7.5 Gate O0

- 零隐式公网请求。
- sandbox 无 Host/Node/主 App 对象。
- destroy 后资源计数归零。
- text/image/PDF 至少各一个 fixture 可渲染。
- 高风险 fixture 不执行主动内容。

## 8. 阶段 O1：Wave 1 生产只读 Adapter

### 8.1 阶段目标

交付 text、image、PDF 的统一 resourceRef 打开、路由、导航、错误和 fallback。

### 8.2 方案设计

- Adapter Manifest 只列已认证格式和 `view`。
- `probe` 检查 format、size、worker/WebGL（Wave 1 无 WebGL）、端内存和 policy。
- open 后用 postMessage `viewer.open`；ready/error/unsupported 返回 generation。
- Host 外层工具栏接管下载/打印/全屏策略。
- PDF initialPage、text line anchor、图片 fit/zoom 映射到 anchorHint。
- Surface close 调 `viewer.destroy()`，随后销毁文档/URL/session。

### 8.3 开发任务

| ID | 任务 |
|---|---|
| O1-D1 | Viewer Adapter Manifest/probe |
| O1-D2 | Flutter Web Surface |
| O1-D3 | Web route/component |
| O1-D4 | text/image/PDF source adapter |
| O1-D5 | command/toolbar policy |
| O1-D6 | page/line navigation |
| O1-D7 | DSH UI Consumer route/fallback |
| O1-D8 | locale/theme/accessibility |
| O1-D9 | telemetry/feature flag |

### 8.4 测试矩阵

| ID | 场景 | 预期 | 层 |
|---|---|---|---|
| O1-T1 | text/image/PDF probe | 认证格式 available | contract |
| O1-T2 | docx/zip 在 Wave 1 | unavailable/metadata，不误报 | contract |
| O1-T3 | DSH 三入口同 PDF | 同一 Viewer Surface/receipt | E2E |
| O1-T4 | PDF initialPage | 定位目标页，越界 clamp/说明 | browser |
| O1-T5 | text line anchor | 定位且高亮/明确 unsupported | browser |
| O1-T6 | zoom/rotate/search | capability-aware，按钮状态正确 | UI |
| O1-T7 | download 被 policy 禁止 | 按钮隐藏/禁用，无直链逃逸 | security |
| O1-T8 | 打印被禁止 | `preparePrint`/window print 不可达 | security |
| O1-T9 | 同资源重复打开 | focus/reload 规则正确 | lifecycle |
| O1-T10 | 快速切换队列 | 旧 render aborted，不覆盖新文件 | race |
| O1-T11 | Viewer crash | failed receipt，metadata fallback | fault |
| O1-T12 | 键盘/屏幕阅读器 | focus、role/status、toolbar 可用 | a11y |
| O1-T13 | Mobile Small PDF/image | 首屏与手势达标 | device |
| O1-T14 | Ontology 未安装 | 完整打开行为不变 | composition |

### 8.5 Gate O1

Wave 1 可作为其他引擎 unavailable 时的默认只读 fallback；不启用任何未认证插件。

## 9. 阶段 O2：渐进读取、Context 与可靠性

### 9.1 阶段目标

解决大文件、弱网、取消、恢复和有界上下文，使 Viewer 可在 Desktop/Web/Mobile 稳定运行。

### 9.2 方案设计

- PDF 使用 range/disableAutoFetch 策略，按端和大小选择；避免 `useFetchData` 双内存，除非兼容 profile。
- 图片先读取 metadata/thumbnail，大图限制解码像素。
- 文本使用 range/line index 或截断预览；不得对 100 MiB 文本一次性 Prism 高亮。
- 新增通用 vendor API（保持 Muse 无知）：
  - `onViewStateChange({page, zoom, rotation, searchQuery?})`
  - 可选 `getSelection()`/`onSelectionChange`
  - `getCapabilities()`
- Adapter 将 view state 转为短 TTL Context；全文读取仍走 Resource snapshot。
- 断线重连重新签发 URL并 `reload`，验证 revision 未变。

### 9.3 开发任务

| ID | 任务 |
|---|---|
| O2-D1 | range/stream broker 与取消 |
| O2-D2 | size/decode/memory budgets |
| O2-D3 | vendor-neutral view-state callback |
| O2-D4 | Viewer Context contribution |
| O2-D5 | reconnect/reload/revision check |
| O2-D6 | worker crash/restart policy |
| O2-D7 | Mobile plugin/size profiles |
| O2-D8 | long-session leak benchmark |

### 9.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| O2-T1 | 250 MiB PDF 首屏 | 不等待全量下载，内存受限 |
| O2-T2 | range server 不支持 range | 按大小 fallback 或明确拒绝 |
| O2-T3 | 快速翻页/取消 | 不渲染陈旧页，不泄漏 canvas/worker |
| O2-T4 | 100 MiB text | 截断/虚拟化，无全量高亮 |
| O2-T5 | 超大像素图片 | 解码前拒绝或缩略图 |
| O2-T6 | URL 过期后重连 | 重新鉴权；revision 变化时不静默 reload |
| O2-T7 | page Context | revision、TTL、generation 正确 |
| O2-T8 | selection Context 超限 | 截断并标记 |
| O2-T9 | 30 分钟翻页/缩放 | 内存回到稳定区间 |
| O2-T10 | Mobile 后台/前台 | token 重检、无陈旧内容闪现 |
| O2-T11 | worker 崩溃 | 一次恢复或明确失败，不循环 |
| O2-T12 | 网络中断 | loading 可取消，恢复不会重复 session |

### 9.5 Gate O2

达到分端 PRD SLO；Context 可用但不依赖 Ontology；大文件不会把 Host Bridge、WebView 或移动端内存打满。

## 10. 阶段 O3：Wave 2 文档、压缩包与邮件

### 10.1 范围

候选插件：office、epub/xps/ofd、archive、email、drawing、xmind。逐个认证，不整体打开。

### 10.2 共同方案

- zip/压缩格式在解析前执行压缩大小、解压总量、文件数、目录深度和 ratio 限制。
- HTML/Markdown/SVG/邮件正文通过 DOMPurify + CSP；禁止 script、form、自动网络、远端 tracking pixel。
- Office 宏、OLE、外链、模板和嵌入对象不执行。
- archive 内部文件只在虚拟只读子资源中预览；下载/解压到磁盘需要 Host action。
- encrypted 文档只显示受控错误/密码流程；密码不进日志和 Context。
- ioffice 可用时仍优先 ioffice edit；Viewer office 只提供 view/fallback。

### 10.3 开发任务

| ID | 任务 |
|---|---|
| O3-D1 | 每插件 capability/security manifest |
| O3-D2 | zip budget service |
| O3-D3 | active-content sanitizer/egress block |
| O3-D4 | archive virtual child resource |
| O3-D5 | encrypted resource UX |
| O3-D6 | Office fidelity/route preference |
| O3-D7 | corpus/license/dependency report |

### 10.4 测试矩阵

| 插件族 | 功能 | 安全 | 性能 |
|---|---|---|---|
| Office | docx/xlsx/pptx/legacy fallback、页/sheet/slide | 宏/OLE/外链/公式注入 | 25 MiB、复杂 zip |
| Archive | list、nested preview、filename encoding | zip bomb、path traversal、symlink、嵌套深度 | 10k entries |
| Email | eml/msg/mbox、附件 | HTML/script/form/tracking、恶意附件 | 大附件/长线程 |
| EPUB/XPS/OFD | navigation、font/image | script/external link/encryption | 多章节/大页 |
| Drawing/XMind | canvas/tree/navigation | embedded HTML/link、zip bomb | 大节点图 |

### 10.5 Gate O3

每个插件有独立 certification record。任何 P0 安全失败只禁用该插件，不阻塞 Wave 1。

## 11. 阶段 O4：Wave 3 CAD、3D、GIS 与媒体

### 11.1 范围

CAD/DWG、model3d、GIS、audio/video。它们对 GPU、WASM、网络、外部纹理和大资源要求更高。

### 11.2 方案设计

- WebGL capability probe、GPU denylist、context-loss recovery。
- 模型/地图外部纹理、tiles、字体和媒体分片全部经 Resource broker；默认禁任意 origin。
- CAD/3D entity picking 作为可选 generic selection callback，返回 provider-namespaced selector；本轮仅发布 Context，不创建 Ontology Anchor。
- GIS 不默认加载在线底图；需要 workspace-approved tile Provider。
- media 禁止自动播放；后台停止解码，DRM/codec unsupported 明确降级。

### 11.3 开发任务

| ID | 任务 |
|---|---|
| O4-D1 | WebGL/GPU capability probe 与 denylist |
| O4-D2 | CAD/3D/GIS/Media 各自插件 Manifest 和大小预算 |
| O4-D3 | 纹理、tile、media range 的 Resource broker |
| O4-D4 | generic entity/view-state selection callback |
| O4-D5 | WebGL context-loss、后台暂停和资源回收 |
| O4-D6 | Mobile 热量/电量/内存降级策略 |
| O4-D7 | 各插件 corpus、license、dependency 和安全报告 |

### 11.4 测试矩阵

| 领域 | 必测 |
|---|---|
| CAD | DXF/DWG 版本、图层、单位、大图元、恶意二进制、entity pick |
| 3D | glTF/GLB/OBJ/STL、纹理、坐标/单位、百万三角形、WebGL loss |
| GIS | GeoJSON/KML/SHP、CRS、超大 feature、tile egress |
| Media | codec matrix、seek、range、字幕、autoplay、后台 |
| GPU | 集显/独显/软件渲染、驱动 denylist、context reset |
| Mobile | 热量、内存、电量、前后台、手势冲突 |

### 11.5 Gate O4

按插件 + 端 + GPU profile 独立启用。CAD/3D Context hook 可用不代表 Ontology Runtime 已实现。

## 12. 性能与可靠性目标

| 指标 | 目标 |
|---|---:|
| Adapter probe | local p95 ≤ 50 ms（不解析完整文件） |
| Small Viewer 首屏 | Desktop p95 ≤ 1.2 s；Web/Mobile p95 ≤ 2.5 s |
| Medium Viewer 首屏 | Desktop p95 ≤ 2 s；Web p95 ≤ 4 s |
| Large PDF 首屏 | p95 ≤ 4 s，必须渐进 |
| destroy/取消停止新增 I/O | p95 ≤ 500 ms |
| crash-free sessions | Desktop/Web ≥ 99.8%；Mobile ≥ 99.7% |
| 隐式公网请求 | 0 |
| 未授权 download/print/egress | 0 |
| reload/destroy 泄漏 | 0（TCK 计数） |

## 13. PR/工作包建议

1. O0 bundle/assets/CSP。
2. O0 Resource URL broker。
3. O1 Flutter Viewer Surface。
4. O1 Web Viewer route。
5. O1 Wave 1 plugins + DSH UI integration。
6. O2 range/memory/context。
7. O3 每个插件族独立 PR。
8. O4 CAD、3D、GIS、Media 分别独立 PR。

若需要修改 vendor，应只增加通用 view-state/capability hook 和修复通用安全/生命周期问题，不引入 `resourceRef`、Host Bridge、DSH 或 Muse UI。

## 14. 回滚

- Adapter feature flag 关闭后回到 metadata/download/system-open。
- 单插件 certification 可独立撤销，其他插件不受影响。
- viewer bundle 按 digest 回滚，旧 Surface drain 后切换。
- CSP/worker/asset 更新失败不得放宽 sandbox 作为临时修复。
