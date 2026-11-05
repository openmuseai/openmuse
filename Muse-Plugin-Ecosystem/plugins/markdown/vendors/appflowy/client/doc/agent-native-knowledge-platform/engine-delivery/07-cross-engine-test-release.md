# 公共测试、发布与质量门

## 1. 测试分层

```text
Schema fixtures
  → Host/Provider unit
  → Adapter TCK
  → Engine native/browser/PTY integration
  → Host–DSH–Surface E2E
  → package/sign/security
  → performance/soak/fault
  → cohort telemetry
```

任何阶段不得用手工演示替代可自动化的合同、race 或数据损坏测试。平台/IME/视觉 fidelity 等必须保留手工/设备证据。

## 2. 公共 Adapter TCK

| ID | 类别 | 场景 | 预期 |
|---|---|---|---|
| TCK-M01 | Manifest | 合法 manifest | 注册成功，digest 固定 |
| TCK-M02 | Manifest | 重复 adapterId/version | 组合失败 |
| TCK-M03 | Manifest | 未知 major | 拒绝 |
| TCK-P01 | Probe | 支持格式/平台 | effective modes 正确 |
| TCK-P02 | Probe | 缺 artifact/worker/GPU | unavailable + reason |
| TCK-P03 | Probe | timeout/cancel | 无副作用，无缓存错误可用 |
| TCK-R01 | Route | 多候选 | 决定可解释且稳定 |
| TCK-R02 | Route | 无候选 | explicit unsupported/fallback |
| TCK-O01 | Open | 正常 | 一个 terminal receipt |
| TCK-O02 | Open | duplicate requestRef | 不重复创建 session |
| TCK-O03 | Open | cancel/late ready | 无 late Surface |
| TCK-O04 | Open | adapter failure | 逆序清理并按策略 fallback |
| TCK-L01 | Lifecycle | focus/background/close | 状态合法、generation 正确 |
| TCK-L02 | Lifecycle | plugin unload | 停新会话、旧会话 drain |
| TCK-MAT01 | Materialize | expired/revoked | 读取停止/明确失败 |
| TCK-MAT02 | Materialize | wrong consumer | denied |
| TCK-C01 | Context | revision/TTL/limit | 过期和截断正确 |
| TCK-S01 | Save | read-only adapter | commit capability 不存在 |
| TCK-S02 | Save | revision conflict | 原资源不覆盖 |
| TCK-S03 | Save | idempotency replay | 不重复写 |
| TCK-X01 | Shutdown | Host/app crash | 进程/worker/temp/lease 恢复或清理 |
| TCK-N01 | No Ontology | Runtime 未安装 | 功能与结果相同 |

## 3. DSH 测试矩阵

| ID | 场景 | 测试类型 | 预期 |
|---|---|---|---|
| DSH-T1 | deliverable card 点击 | client unit/snapshot | OpenResource Consumer 收到受权 ref |
| DSH-T2 | ProducedFiles 点击 | client unit/snapshot | 与 card 同路径 |
| DSH-T3 | inline mention 点击 | client unit/snapshot | 与 card 同路径 |
| DSH-T4 | explicit system-open | integration | 仍经 Host authority，可选择系统 Adapter |
| DSH-T5 | Agent open tool | tool/schema | 只接受 discovered resourceRef |
| DSH-T6 | path/engineId 注入 | negative | schema/authority 拒绝 |
| DSH-T7 | Provider 缺失 | composition | earliest resolvable loud failure |
| DSH-T8 | session replay | snapshot | 可重建 intent/result，不需 raw data |
| DSH-T9 | model-visible Context | snapshot | 对应 durable event，TTL/revision 可见 |
| DSH-T10 | plugin unload | lifecycle | effect disposers 运行 |

DSH 代码实施按其仓库要求运行相关 unit、coverage gate 范围、snapshot、typecheck、lint/doc gate；不要默认运行整个仓库套件替代聚焦证据。

## 4. 跨引擎路由矩阵

| Resource/条件 | ioffice | Helix | Viewer | 预期 |
|---|---|---|---|---|
| docx，mac arm64，Word healthy | edit/view（视 toDocx） | no | Office view | ioffice 优先 |
| docx，Word artifact missing | unavailable | no | Office view/metadata | Viewer/fallback |
| rs，Desktop，edit allowed | no | edit | text view | Helix |
| rs，Mobile | no | unavailable | text view | Viewer |
| PDF | unavailable 直到真实引擎 | no | PDF view | Viewer |
| xlsx，ioffice 占位 | unavailable | no | Wave 2 view 或 metadata | 不显示 ioffice 创建/编辑 |
| unknown binary | no | no | fallback only | metadata/download |
| encrypted/active content | policy-dependent | no | restricted | 安全错误/下载 |
| > Adapter max size | unavailable | limit-specific | range/metadata | 可解释降级 |

## 5. 资源语料库

### 5.1 Text/Code

- UTF-8/UTF-16/BOM；
- LF/CRLF；
- CJK/emoji/combining/RTL；
- NUL/binary、超长行、无末尾换行；
- 1 KiB/1 MiB/10 MiB/100 MiB；
- symlink、rename、权限变化。

### 5.2 Word

- minimal、真实 PRD、50/500 页；
- paragraphs/runs/styles、table、image、numbering、header/footer；
- track changes/comments、embedded object、external link、macro-enabled；
- encrypted、corrupt、zip bomb；
- CJK/RTL/fonts。

### 5.3 PDF/Image

- 文本/扫描/旋转/表单/签名/加密/附件/JavaScript；
- 1/100/1000 页；
- 超大 page、字体/透明/颜色；
- SVG script/external reference；
- decompression bomb/超大像素。

### 5.4 Complex Viewer

- archive traversal/symlink/nesting/ratio；
- email HTML/tracking/attachment；
- CAD/3D/GIS version/texture/tile/GPU；
- media codec/range/subtitle/autoplay。

所有 fixture 标注来源、许可证、敏感性、期望能力和 digest；生产文档不得进入公开测试仓库。

## 6. 平台和设备矩阵

| 层 | 必测 |
|---|---|
| Desktop | macOS arm64 首发；macOS x64、Windows x64、Linux x64/arm64 按各引擎 Gate |
| Web | Chrome/Edge/Safari/Firefox 支持矩阵；企业旧 Chromium 单独 profile |
| Mobile | iOS/Android 各低/中/高端设备；Wi-Fi/4G/弱网 |
| GPU | WebGL 可用/不可用/context loss/软件渲染 |
| Input | 键盘布局、中文/日文 IME、touch、mouse、screen reader |
| Network | offline、100ms/120ms RTT、1% loss、断线/恢复、range 不支持 |

## 7. 非功能测试

### 7.1 性能

记录：

- route/probe/materialize/start/first-content/ready 分段；
- bytes/range/worker/PTY throughput；
- CPU、RSS/heap/GPU memory；
- close/destroy 后回收；
- save/export/commit；
- p50/p75/p95/p99，按端/格式/大小/冷暖分层。

### 7.2 Soak

- 8 小时连续开关 500 个 Surface；
- 30 分钟 PDF 翻页缩放；
- 30 分钟 Helix 输入/resize/save；
- Word 100 次 open/edit/export/close；
- plugin reload 50 次；
- Host/DSH reconnect 50 次。

通过条件：无单调资源增长、无 orphan process/worker/materialization、无重复 receipt/commit。

### 7.3 Fault injection

- kill engine/worker/DSH/Host/Flutter；
- materialization 过期、断流、慢读；
-磁盘满/只读/rename/symlink swap；
- plugin unload/reload；
- permission revoke/policy revision；
- commit 成功后响应丢失；
- late event/out-of-order/duplicate。

### 7.4 安全

- path traversal、symlink escape、argv injection；
- token replay、wrong origin/audience/consumer；
- CSP escape、postMessage spoof、XSS/SVG/HTML；
- macro/OLE/external link/tracking/tile egress；
- zip/image bomb；
- resourceRef/approval/revision replay；
- secret/path/content in logs and model context。

## 8. 打包测试

| 引擎 | 必备断言 |
|---|---|
| Helix | `hx` executable、runtime queries/themes、certified grammars、config、签名、`hx --health` |
| ioffice | correct OS/ABI dylib/dll/so/wasm、symbols、fonts、FFI smoke、签名 |
| Viewer | JS/CSS、workers、cmaps/fonts、plugin chunks、CSP hashes、offline load |

每个产物记录 source commit、build toolchain、digest、license/SBOM。Package verifier 缺件必须 fail fast，不能运行时静默降级后仍称发布成功。

## 9. 发布阶梯

```text
off
 → developer
 → internal
 → shadow route/probe
 → opt-in
 → small cohort
 → platform cohort
 → default
```

Shadow 不创建真实 Surface/进程，只比较预期 route。Edit 从不因 view 达标自动启用。

## 10. 发布看板

按 adapter/version/platform/format/size 展示：

- requested/opened/focused/fallback/unsupported/failed；
- probe unavailable reason；
- first content/ready；
- crash/forced terminate/leak；
- dirty/commit/conflict/failed/recovery；
- Viewer sandbox/CSP/worker errors；
- Helix PTY/stream/process errors；
- ioffice FFI/export/fidelity errors。

日志不记录本地 path、正文、URL bearer、密码或 PTY raw stream。

## 11. Stop-ship

任一项阻止该 Adapter/Mode/Platform 发布：

- 原资源损坏、旧 revision 被覆盖或重复 commit；
- 跨 workspace/actor/consumer 读取；
-主动内容执行、未授权网络/下载/打印；
- 进程/worker/materialization 泄漏可持续累积；
- UI 宣称 edit/save 但 engine 不支持；
- 取消后仍打开或 late event 覆盖新 Surface；
- 包缺引擎产物但 probe 仍 available；
- DSH model-visible 结果不能从 session log 重建。

## 12. Gate 证据

每个 Gate 的结果至少包含：

- commit/digest、OS/设备、配置；
- 自动测试命令及结果；
- corpus/fixture digest；
- 性能表；
- 手工验收记录；
- blocked/skipped；
- 已知缺陷和 fallback；
- release owner、安全 owner、QA owner 的决定。
