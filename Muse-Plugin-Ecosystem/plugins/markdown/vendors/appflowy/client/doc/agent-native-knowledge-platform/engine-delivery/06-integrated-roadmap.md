# 三引擎集成交付路线

## 1. 路线目标

本路线把公共平台和三个引擎拆成可独立合并、可灰度、可回滚的工作包。时间以 Gate 为准，不以日历承诺替代技术完成。

```text
F0 合同与 Host/DSH seam
 ├─ F1 Viewer Wave 1
 ├─ F2 Word unified view
 └─ F3 Helix H0/H1
          │
          ├─ F4 Word toDocx → edit
          ├─ F5 Helix working-copy → edit
          └─ F6 Viewer progressive/Wave 2
                    │
                    └─ F7 multi-platform + ecosystem
```

Ontology Runtime 不在依赖图中；reserved hooks 随 F0/F1–F3 验证。

## 2. Foundation F0：公共合同和贯通

### 2.1 设计包

- Resource、Presentation、EngineSession schema/ADR。
- Host/DSH/Flutter/Web 时序、错误和取消。
- DSH Service Definition/Provider/Consumer seam。
- materialization、data plane、lease/disposer。
- feature flag、route receipt、telemetry。

### 2.2 开发包

| Workstream | 任务 |
|---|---|
| Contracts | `contract-resource`、`contract-presentation`、`contract-engine-session` |
| Host | Resource Provider Registry、materialization manager、commit、events |
| DSH | OpenResource Service Definition、Host Provider、UI/Agent Consumers |
| Flutter | Engine Adapter API、Surface Orchestrator、Host intent receiver |
| Web/Mobile | parent bridge envelope adapter |
| Quality | TCK harness、golden fixtures、fake provider/adapter |

### 2.3 测试包

- TS/Rust/Dart round-trip 和 schema digest。
- duplicate request/cancel/late event/plugin reload。
- materialization TTL/revoke/consumer mismatch。
- DSH session snapshot：card/mention/ProducedFile。
- Desktop intent 下行和 Surface lease。
- 无 Ontology Runtime 的 composition test。

### 2.4 Gate F0

- 一个 fake Resource 可从 DSH 打开 fake Surface，产生唯一 receipt。
- 故障注入后 registry/session/materialization/leak 计数归零。
- 旧 DSH Sidebar fallback 可启用。

### 2.5 当前实施状态

| 子阶段 | 状态 | 证据 |
|---|---|---|
| F0.1 Contracts | Pass | 14 schemas / 32 fixtures / 51 tests |
| F0.2 Host Resource Core | Pass | Provider/materialization/commit/event 25 tests |
| F0.3 DSH Seam | Pass | Service/Host Provider/UI+Agent Consumers 20 tests + DSH Session producer 1 test |
| F0.4 Client Orchestrator | Next | Fake Adapter/Surface 与 Dart projection 待实施 |
| F0.5 Web/Mobile | Planned | data-plane policy 待实施 |
| F0.6 Hardening | Planned | soak/perf/rollback 待实施 |

## 3. F1：open-file-viewer Wave 1

为什么先做：只读模式没有格式回写风险，能最早验证统一打开、fallback、Web Surface 和跨端 URL broker。

交付：

- O0 + O1；
- text/image/PDF；
- Desktop/Web，Mobile Small 子集；
- CSP、自托管 worker/assets、Host toolbar；
- Viewer 作为 Word/Helix unavailable fallback。

Gate：见 O1；未通过 sandbox/no-network 不进入后续复杂插件。

## 4. F2：ioffice Word 统一只读

可与 F1 后半并行：

- I0 + I1；
- 当前 ViewLayout/blob/Word Surface 迁移到 opaque resourceRef；
- macOS arm64 runtime probe；
-无 `toDocx` 时 effective mode=view；
- Viewer Office fallback 可在 O3 前暂时只显示 metadata。

Gate：打开、关闭、Context、撤权、FFI 崩溃和 Markdown 回归通过。

## 5. F3：Helix H0/H1

H0 与 F1/F2 并行；H1 依赖 F0：

- build/runtime/grammar/signing；
- PTY raw stream；
- Flutter VT renderer/IME；
- read-only working copy；
- DSH open/fallback。

Gate：PTY/stream/process tree 无泄漏；中文输入方案已选；只读写保护通过。

## 6. F4/F5：写入轨道

### F4 Word

只有 vendor `toDocx` round-trip corpus 通过才启动：

1. vendor API；
2. bytes commit；
3. user Save/conflict/recovery；
4. DSH Word query；
5. DSH apply 最后启用。

### F5 Helix

H0 watcher 方案通过后：

1. working-copy commit；
2. direct-safe capability；
3. dirty/close/conflict/recovery；
4. DSH Context；
5. Agent 仍不获得 PTY input。

写入轨道分别灰度；任何一个不阻塞另一个只读产品。

## 7. F6：Viewer 扩展

- O2 progressive/context/reliability。
- O3 plugin families 每组独立 certification。
- ioffice Word edit 始终优先于 Viewer Office view。
- Archive/Email/CAD 等不得因包中存在插件而批量打开。

## 8. F7：多平台和格式生态

- Helix Windows/Linux。
- ioffice Word macOS x64/Windows/Linux/Web。
- Viewer Mobile/复杂 GPU profiles。
- ioffice Excel/Slides/PDF 只有各自 Admission Gate 后进入。
- 第三方 Engine Adapter SDK/TCK 文档化。

## 9. 阶段交付文档模板

每个阶段开始前必须存在并评审：

```text
phase-X/
├─ DESIGN.md
├─ DEVELOPMENT.md
├─ TEST-MATRIX.md
├─ FIXTURES.md
├─ RELEASE.md
└─ RESULTS.md
```

本目录的 engine plan 是这些文件的规划基线。进入代码实施时，可在对应代码包附近创建阶段文档或 Agent Note，但不得只在 issue/聊天中保存决定。

### DESIGN 必须回答

- 当前事实和前置 Gate；
- 用户行为、状态机、责任边界；
- API/schema、错误、取消、并发；
- 安全、数据、生命周期、回滚；
- 明确不做。

### DEVELOPMENT 必须包含

- PR/工作包顺序；
- 新增/修改路径；
- migration/feature flag；
- 依赖、owner、阻塞项；
- 构建和本地验证命令；
- 文档与 generated artifacts。

### TEST-MATRIX 必须包含

- requirement/acceptance ID；
- fixture；
- OS/arch/端；
- 正常、负例、race、fault、安全、性能；
- 自动/手工/CI；
- 预期结果和证据位置；
- blocked/skipped 理由与解除条件。

### RESULTS 必须包含

- 实际执行的命令/版本/设备；
- 通过/失败/blocked；
- 指标与 corpus 摘要；
- 未知风险；
- Gate 决定和签字 owner。

## 10. PR 拆分

建议单 PR 单责任：

| 序号 | PR | 可独立回滚 |
|---|---|---|
| 1 | schema + fixtures | 是 |
| 2 | Host Resource Registry/materialization | 是 |
| 3 | DSH Service Definition/Provider | 是 |
| 4 | DSH UI Consumers + fallback | 是 |
| 5 | Flutter Orchestrator/fake adapter | 是 |
| 6 | Viewer O0/O1 | 是 |
| 7 | Word I0/I1 | 是 |
| 8 | Helix H0 build | 是 |
| 9 | Helix H1 Surface | 是 |
| 10+ | 每项 write/platform/plugin wave | 是 |

跨 DSH repo 的非平凡 PR 要按其规则提供 Agent Note、100% per-file coverage 相关测试、product-visible snapshot 和 docs。Vendor 子仓库改动单独提交，不与 Muse Adapter 混在一个不可审阅 diff。

## 11. Feature flag

| Flag | 默认 |
|---|---|
| `resourcePresentationV2` | internal/shadow |
| `viewerAdapter.wave1` | internal → cohort |
| `iofficeWordAdapter.view` | internal → macOS cohort |
| `iofficeWordAdapter.edit` | off，I2 后启用 |
| `helixAdapter.view` | off，H1 后启用 |
| `helixAdapter.edit` | off，H2 后启用 |
| `viewerAdapter.wave2.*` | 每插件 off |
| `engineOntologyHooks` | on（只发事实事件，无 Runtime consumer） |

Flag 是发布控制，不替代 runtime probe 和权限。

## 12. 团队分工

| Team/DRI | 所有权 |
|---|---|
| Protocol/Host | schema、registry、authority、materialization、commit |
| DSH | capability seam、UI/Agent Consumers、session events/snapshots |
| Client Platform | Orchestrator、Surface lifecycle、transport、cross-end |
| Helix | build/runtime/PTY/working-copy/terminal UX |
| ioffice | vendor kernel、Word Adapter、Office format quality |
| Viewer | sandbox、bundle、plugins、format certification |
| Security/Release | threat model、signing、SBOM、package/rollout |
| QA | TCK、corpus、device/platform matrix、fault/performance |

## 13. 集成演示里程碑

### Demo 1：统一只读

DSH 同一条消息包含 docx、rs、pdf：

- docx → ioffice Word view；
- rs → Helix view；
- pdf → Viewer；
- 三者 route receipt/Surface lifecycle 一致。

### Demo 2：安全降级

- 缺 dylib → docx Viewer/metadata；
- 缺 hx → code Viewer；
- Viewer 插件不认证 → metadata/download；
- 所有错误明确，无白屏。

### Demo 3：安全保存

- Helix working-copy 修改、commit、冲突恢复；
- Word 在 `toDocx` Gate 后修改、导出、commit、重开；
- Viewer 无 Save。

## 14. Program 完成定义

- 三引擎至少各一条生产认证路径。
- 公共 route/session/receipt/error 不因引擎分叉。
- 写入路径无旧 revision 覆盖或不可恢复丢失。
- 目标端 package/签名/runtime/worker 均可离线验证。
- DSH 消息、工具和模型可见事件可重放。
- Ontology Runtime 完全缺席时所有交付仍工作；后续接入不需修改 vendor API。
