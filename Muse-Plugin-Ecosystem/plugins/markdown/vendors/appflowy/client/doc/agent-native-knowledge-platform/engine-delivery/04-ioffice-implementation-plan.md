# ioffice Engine Adapters：实施、开发与测试方案

状态：**目标实施计划**  
当前可用内核：Word/macOS arm64（排版 + heap 编辑，无法导出保存）  
占位：Excel、Slides、PDF  
依赖：公共 G0/G1；Save 依赖 vendor `toDocx` 硬门；不依赖 Ontology Runtime。

## 1. 产品定位

ioffice 是 Muse 的高保真 Office 引擎族。它不是统一资源层，也不拥有 workspace 权限：

- Resource Host 提供 docx/xlsx/pptx/pdf 字节、revision 和 commit；
- ioffice 负责解析、布局、交互、编辑及格式导出；
- Muse Word/Excel/Slides/PDF Adapter 负责引擎 capability、Surface、Context 和生命周期；
- DSH 通过格式领域合同读取/提出修改，不直接调用 FFI；
- open-file-viewer 是只读 fallback，不与 ioffice 共用编辑状态。

本计划整合统一引擎链，不替代 `Muse-Clients/local/docs/office/word/` 已有 Word 内核/Folder/Facet 阶段文档。

## 2. 当前能力真相

### 2.1 Word

当前链：

```text
ViewLayout.Word(9)
  → OfficeBlobStore / WordBlobStore
  → WordPage
  → WordEditorController
  → WordCoreEngine
  → bundled macOS arm64 libword_core_ffi.dylib
```

当前 `WordSession` 可 open/layout/insertText/removeRange/caretAt 等；`WordEditorController.canExportDocx` 固定为 false。`_docx` 是打开时原字节，不能代表编辑后内容。

硬不变量：

```text
没有 session.toDocx()
  ⇒ 不注册 Resource commit
  ⇒ 不显示 Save
  ⇒ 不允许 Agent apply
  ⇒ dirty close 只能明确丢弃
  ⇒ 禁止把原 _docx 当新文档写回
```

### 2.2 Excel、Slides、PDF

目前只有：

- ViewLayout：Excel=10、Slides=11、Pdf=12；
- Office Manifest、blob subdir、扩展名和最小占位模板；
- `engineBound=false`；
- vendor 目录无真实引擎代码。

这些能力只能称为 schema/slot reservation，不能称为可创建、可查看或可编辑产品。目标 UI 必须计算：

```text
effectiveCreatable = manifest.creatable
                   && runtimeProbe.engineAvailable
                   && platformSupported
                   && policyAllowed
```

## 3. Adapter 划分

每种格式独立 Adapter，不创建一个携带 `kind` 大 switch 的 `iofficeAdapter`：

```text
muse.ioffice.word
muse.ioffice.excel       # admission gate 后
muse.ioffice.slides      # admission gate 后
muse.ioffice.pdf         # admission gate 后
```

共享：

- native artifact resolver/digest/health；
- bytes materialization；
- Surface lifecycle 基础；
- theme/locale/accessibility integration；
- Resource commit helper；
- package verification。

不共享：

- 文档模型、编辑 mutation、Anchor selector；
- Excel formula/cell、Slides shape、Word CP、PDF annotation；
- format revision 以外的业务合同。

## 4. Word Session 设计

### 4.1 打开

```text
Resource bytes@R1
  → validate ZIP + OOXML Word content type
  → bytes materialization handle
  → WordSession.open(docx, fonts, view)
  → first LayoutFrame
  → Muse Word Surface ready
```

格式验证不能只看 PK magic；Format Provider 应验证 OOXML content types/main document part，并拒绝 xlsx/pptx 假装 docx。

### 4.2 编辑状态

Surface 显示四种模式：

- `view`：只读。
- `ephemeral-edit`：heap 可编辑但不能导出；明确标记“本次修改无法保存”。
- `edit`：`toDocx` 和 Resource commit 可用。
- `degraded-view`：FFI 不可用，转 Viewer/错误页。

默认不应给普通用户开放容易误解的 ephemeral-edit。首发建议 Word 无 `toDocx` 时路由为 `view`，内部测试才打开 ephemeral-edit。

### 4.3 保存

```text
WordSession.toDocx()
  → validate output OOXML
  → calculate digest
  → Resource.commit(expectedRevision=R1, bytesHandle)
  → committed R2
  → session baseRevision=R2, dirty=false
  → resource.changed(R1,R2)
```

`dumpJson(LayoutFrame)`、`plainText()` 和打开时 `_docx` 都不能作为保存内容。仅修改样式也必须改变导出字节 revision。

### 4.4 Context

本轮发布：

- `word.surface`：resourceRef、revision、mode、pageCount、ffiReady；
- `word.selection`：caretCp、selStart/selEnd、pageIndex、TTL；
- `word.snapshot`：由 Host/Provider 提供的有界 plainText + bytes revision。

当前 Context 中的 `viewId` 和 `resourceRef=viewId` 要迁移为 opaque resourceRef；viewId 只留在 Host locator。

## 5. 阶段 I0：能力基线与诚实 Manifest

### 5.1 阶段目标

将“槽存在、引擎存在、可查看、可编辑、可保存”拆成独立能力，阻止 Excel/Slides/PDF 占位和 Word heap 编辑被误报。

### 5.2 方案设计

新增 runtime probe：

- native artifact 是否存在、digest/ABI 是否匹配；
- FFI init 与最小文档 layout；
- `canExport`、`canExtractText`、`canEdit`；
- platform/architecture；
- font bundle 和内存限制；
- format-specific capabilities。

Office Catalog 保留静态信息；UI、路由和创建入口只消费 `EffectiveOfficeCapability`。

### 5.3 开发任务

| ID | 任务 |
|---|---|
| I0-D1 | 定义 Office Adapter Manifest 与 effective capability |
| I0-D2 | Word FFI/format smoke probe |
| I0-D3 | 创建菜单/路由改为 effectiveCreatable/effectiveModes |
| I0-D4 | Excel/Slides/PDF unavailable reason |
| I0-D5 | 现有 Word tests 映射到公共 Adapter TCK |
| I0-D6 | capability telemetry 与 package digest |

### 5.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| I0-T1 | macOS arm64 dylib 正常 | Word probe view=true、export=false |
| I0-T2 | dylib 缺失/损坏 | Word unavailable，Viewer fallback |
| I0-T3 | x64/Windows/Linux 无产物 | unavailable，不尝试 dlopen |
| I0-T4 | Excel/Slides/PDF 占位 | effectiveCreatable=false、无编辑入口 |
| I0-T5 | FFI init 成功但 layout smoke 失败 | engine unhealthy，不路由 |
| I0-T6 | `canExportDocx=false` | Save、commit、Agent apply 均不存在 |
| I0-T7 | Manifest 声明与 probe 冲突 | probe 优先并上报 packaging mismatch |
| I0-T8 | Markdown regression | 创建/打开/Agent contract 不受影响 |

### 5.5 Gate I0

- 产品中没有“空 Excel/Slides/PDF 页面可创建”的误导。
- Word capability 明确为 view 或内部 ephemeral-edit，不是可保存 edit。
- capability 结果可由路由、UI、DSH 和测试共同使用。

## 6. 阶段 I1：Word 统一资源只读链

### 6.1 阶段目标

通过 resourceRef 从 DSH、工作区和最近资源打开当前 Word Surface，完成 materialization、route、Surface lifecycle 与 fallback。

### 6.2 方案设计

- Word Resource Provider 包装 AppFlowy ViewLayout.Word + blob，revision 为 docx bytes hash。
- 外部 workspace docx 经 Host 注册 resourceRef，可选择直接临时查看或导入为 Word Artifact；二者语义不能混淆。
- Word Adapter 接受 bytes handle，不直接读取 viewId/path。
- Surface Binding 使用 opaque resourceRef 作为 scope/resource；保留 Host-private view locator。
- FFI/format/size 失败回落 open-file-viewer Office plugin 或 metadata。
- 无 `toDocx` 时 mode=view，关闭无 dirty。

### 6.3 开发任务

| ID | 任务 | 目标 |
|---|---|---|
| I1-D1 | Word Resource Provider | describe/get bytes/subscribe |
| I1-D2 | Word Format Provider | OOXML Word 精确识别 |
| I1-D3 | ioffice Word Adapter | probe/open/navigate/close |
| I1-D4 | Surface Binding 迁移 | resourceRef/revision/generation |
| I1-D5 | DSH OpenResource 消费 | card/mention/ProducedFile |
| I1-D6 | route receipt/fallback | FFI、格式、权限错误 |
| I1-D7 | lifecycle/内存/临时资源清理 | 关闭、切换、崩溃 |
| I1-D8 | feature flag | 旧 Word 打开路径回滚 |

### 6.4 测试矩阵

| ID | 场景 | 预期 | 层 |
|---|---|---|---|
| I1-T1 | AppFlowy Word view describe | format、size、bytes revision 正确 | Rust |
| I1-T2 | docx/xlsx 都是 PK | 只接受 docx，xlsx format mismatch | format |
| I1-T3 | 三个 DSH 入口同 ref | 同一 Word Surface/route result | snapshot/E2E |
| I1-T4 | 同 Word 重复打开 | focus 已有 session | lifecycle |
| I1-T5 | resourceRef 指向非 Word layout | WRONG_FORMAT/LAYOUT，不读 Document collab | isolation |
| I1-T6 | 损坏/加密/超大 docx | 明确错误或 Viewer fallback | negative |
| I1-T7 | FFI init/layout 失败 | 无白屏，cleanup 完成 | fault |
| I1-T8 | Surface close/reopen | blob 不变、session 释放 | integration |
| I1-T9 | Context selection TTL | 有 revision/TTL，失焦后过期 | Dart |
| I1-T10 | 权限撤销 | 清页面/关闭 materialization | security |
| I1-T11 | 无 Ontology Runtime | 打开、Context、关闭完全可用 | composition |
| I1-T12 | 50 页 fixture | 首个页面与内存达到预算 | performance |

### 6.5 Gate I1

- Word 统一只读打开达到 Desktop SLO。
- 所有路径不使用任意 DSH path/viewId 作为权限。
- Save/Agent apply 保持不可达。
- FFI fallback 与 Markdown 回归通过。

## 7. 阶段 I2：Word 内核导出与安全保存

### 7.1 阶段目标

由 ioffice vendor 提供真实 `WordSession.toDocx()`，完成用户 Save、revision、冲突和崩溃恢复。该阶段是 edit 的唯一硬门。

### 7.2 Vendor 方案

最小通用 API：

```text
WordSession.toDocx() -> Uint8List
WordSession.plainText(maxBytes?) -> TextSnapshot
WordSession.isDirty() -> bool          # 可选，Muse 仍可跟踪命令
```

要求：

- 导出当前 session，不返回原输入；
- OOXML 包关系、content types 和未编辑部件尽可能保留；
- 错误可恢复，失败不销毁 session；
- 同一 session 未变化的重复导出结果可稳定比较；
- 仅样式变化也反映到 bytes；
- native 和未来 wasm 行为 fixture 一致。

### 7.3 Muse 开发任务

| ID | 任务 |
|---|---|
| I2-D1 | vendor toDocx/plainText 实现与 FRB surface |
| I2-D2 | round-trip corpus 与 fidelity report |
| I2-D3 | Word dirty/save/close UX |
| I2-D4 | Resource bytes commit/idempotency |
| I2-D5 | revision conflict/compare/reload |
| I2-D6 | crash recovery 与 last exported draft |
| I2-D7 | resource.changed event |
| I2-D8 | Manifest effective mode 从 view 升为 edit |

### 7.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| I2-T1 | 最小 docx 插入字符→导出→重开 | 新字符存在 |
| I2-T2 | 删除跨 run 文本→导出 | 结构有效、内容正确 |
| I2-T3 | 仅段落/字符样式变化 | bytes revision 变化、重开样式可见 |
| I2-T4 | 无修改导出 | no-change 或稳定内容，不产生重复 revision |
| I2-T5 | 表格/图片/页眉脚/编号 fixture | 未编辑结构不丢；能力外差异进入已知清单 |
| I2-T6 | 大文档多次保存 | 无持续内存增长，延迟达标 |
| I2-T7 | Resource 当前 revision 已变 | conflict，不覆盖新字节 |
| I2-T8 | 重复 commit idempotency | 单次写入 |
| I2-T9 | toDocx 抛错 | session/dirty 保留，关闭被拦 |
| I2-T10 | commit 成功后 UI 崩溃 | 重开读取新 revision，不重复写 |
| I2-T11 | 写文件中断/磁盘满 | Provider 原子写，不留下损坏真源 |
| I2-T12 | 导出包安全校验 | 不是合法 Word OOXML 则拒绝 commit |
| I2-T13 | 中文/CJK/字体缺失 | 文本不丢，替代字体行为可解释 |
| I2-T14 | 旧 `_docx` 防回归 | 编辑后任何保存路径不得读取旧字段作为输出 |

### 7.5 Gate I2

- Round-trip 必须覆盖真实 corpus，不仅最小模板。
- 数据损坏、旧字节覆盖、revision overwrite 为零。
- 未支持的 Word 特性有 fidelity 等级和用户提示。
- Gate 通过后才注册 `edit/commit`；否则 I1 继续作为稳定只读产品。

## 8. 阶段 I3：Word 与 DSH/Agent 领域合同

### 8.1 阶段目标

DSH 可读取有界 Word snapshot，并在 I2 通过后走 `propose → approval → apply → receipt`。本阶段不实现 Ontology。

### 8.2 方案设计

- 保留独立 `muse.word` 合同，不复用 Markdown `replace_paragraphs`。
- query 返回 `plainText`、selection/CP、bytes revision 和截断信息。
- proposal 使用 Word 可验证 mutation（如 insert/remove by CP）；不接受整文件任意 bytes。
- apply 在 Host 重检 layout、actor、workspace、revision、approval；临时 session 执行 mutation、toDocx、commit。
- 已打开 Surface 收到 resource.changed 后 reload/reconcile；本地 dirty 时进入 conflict。

### 8.3 开发任务

| ID | 任务 |
|---|---|
| I3-D1 | `muse.word` query/propose/apply schema 与 fixtures |
| I3-D2 | DSH Service Definition/Host Provider/tool Consumer |
| I3-D3 | Host authority 与 approval digest |
| I3-D4 | Word mutation validator |
| I3-D5 | apply→toDocx→commit→event |
| I3-D6 | Surface reconcile/conflict |
| I3-D7 | model-visible session event/snapshot |

### 8.4 测试矩阵

| ID | 场景 | 预期 |
|---|---|---|
| I3-T1 | query 正确 Word ref | 有界 text + bytes revision |
| I3-T2 | query Markdown/Excel ref | WRONG_FORMAT，不走错误 Provider |
| I3-T3 | proposal 缺 expectedRevision | schema 拒绝 |
| I3-T4 | apply 无 approval/过期 approval | denied，不写 |
| I3-T5 | apply insert/remove | 导出并持久化，重开可见 |
| I3-T6 | proposal revision 过期 | conflict |
| I3-T7 | apply 重放 | receipt 相同、不双写 |
| I3-T8 | 已开 Surface clean | 自动 reconcile 到新 revision |
| I3-T9 | 已开 Surface dirty | 不覆盖，进入 conflict |
| I3-T10 | 模型请求任意 bytes/path/viewId | schema 拒绝 |
| I3-T11 | session replay | 请求、审批、结果可重建 |
| I3-T12 | Ontology 未安装 | 所有 Word 合同行为不变 |

### 8.5 Gate I3

I2 未通过则 I3 只交付 query，apply 明确 blocked。任何测试不得用 mock `toDocx` 宣告生产 apply 完成。

## 9. 阶段 I4：多平台与 Web

### 9.1 阶段目标

逐平台交付真实 artifact 和相同行为合同，而不是仅让代码编译。

### 9.2 平台方案

| 平台 | 引擎形态 | 交付条件 |
|---|---|---|
| macOS arm64 | 当前 dylib | 签名、公证、round-trip、性能 |
| macOS x64 | dylib | 构建/ABI/签名与 corpus |
| Windows x64 | dll | FRB loader、打包、IME/font/file semantics |
| Linux x64/arm64 | so | loader、字体、容器/桌面支持 |
| Web | wasm + React/Web Adapter | 不走 Flutter FRB；worker/CSP/字体/内存/导出 |
| Mobile | 暂不承诺 native edit | Viewer/remote handoff；独立立项 |

当前 wasm MVP 能编译不等于完整 Word；同样受 `toDocx`、表格/图片和 stub 能力限制。

### 9.3 开发任务

- 各平台 native build、artifact resolver、digest、签名。
- 相同 docx corpus 在各端生成 layout/text/export 结果。
- Web 建独立 JS/WASM Adapter，复用公共 Manifest/Resource 合同。
- 字体包、fallback、CJK、RTL 认证。
- installer/update/rollback 和 package verifier。

### 9.4 测试矩阵

| 维度 | 测试 |
|---|---|
| OS/arch | 每个平台 open/layout/edit/export/reopen |
| ABI | 错架构、缺 symbol、损坏 artifact、旧 runtime |
| Word corpus | 空文档、长文、表格、图片、编号、页眉脚、批注/修订、加密/损坏 |
| 字体/语言 | CJK、Latin、RTL、emoji、缺字体 |
| Web | worker/CSP、wasm trap、内存上限、刷新恢复、Blob URL revoke |
| Package | 签名、公证、安装、升级、回滚、离线 |
| 性能 | 冷/暖启动、首屏、翻页、输入、保存、8 小时泄漏 |

### 9.5 Gate I4

每个平台独立启用。未通过的平台返回 unavailable 并使用 Viewer/系统打开，不降低 macOS 已发布能力。

## 10. 阶段 I5：Excel、Slides、PDF 条件式接入

### 10.1 Admission Gate

每个新 vendor 引擎进入 Muse 开发前必须提供：

- 可构建/可分发源码或固定 artifact；
- open/close 与首屏渲染；
- format-specific model；
- 导出能力（编辑器）或明确只读；
- 最小 corpus、已知 fidelity、许可证和安全清单；
- 目标平台、ABI、内存/文件大小限制；
- 无 `todo/stub` 覆盖关键交付路径。

未通过时只保留 Layout reservation，产品 UI 不显示可创建。

### 10.2 Excel

阶段：

1. E0：xlsx 解析/只读网格、sheet 导航、公式显示。
2. E1：selection/range Context、冻结/合并/基本样式。
3. E2：编辑值/公式并 `toXlsx`、Resource commit。
4. E3：DSH `muse.table` query/propose/apply。
5. E4：多平台和大表性能。

测试矩阵：

| 领域 | 必测 |
|---|---|
| 结构 | 多 sheet、隐藏 sheet、合并、冻结、名称范围 |
| 数据 | string/number/date/error/blank、locale |
| 公式 | shared/array/circular/external link、重算策略 |
| 格式 | 条件格式、行列宽、图片/图表保留 |
| 编辑 | 单元格/range、undo、导出重开、冲突 |
| 规模 | 10k×100、稀疏表、首屏虚拟化、内存 |
| 安全 | 宏、外链、公式注入、zip bomb |

### 10.3 Slides

阶段：

1. S0：pptx 只读、缩略图、slide 导航。
2. S1：shape selection、备注/文本 Context。
3. S2：文本/shape 编辑与 `toPptx`。
4. S3：DSH proposal/apply。
5. S4：媒体/动画/多平台 fidelity。

测试矩阵：

| 领域 | 必测 |
|---|---|
| 结构 | master/layout/theme、section、hidden slide |
| 内容 | text、table、chart、image、audio/video |
| 布局 | 字体替代、旋转、group、z-order、crop |
| 编辑 | shape identity、文本、移动/缩放、导出重开 |
| 导航 | initial slide、缩略图、演示/编辑 mode |
| 安全 | 外链、嵌入对象、宏、超大媒体 |

### 10.4 PDF

PDF 首发定义为 view/annotate，不称为 Office 编辑：

1. P0：高保真只读、页导航、搜索。
2. P1：selection/page/bounds Context。
3. P2：Annotation Resource overlay；原 PDF 不改。
4. P3：若引擎真正支持增量 PDF 写入，再单独评审 flatten/export。

测试矩阵：

| 领域 | 必测 |
|---|---|
| 文档 | 多页、旋转、裁剪框、扫描、OCR、表单 |
| 安全 | 加密、签名、JavaScript、附件、恶意字体 |
| 渲染 | 字体、透明、颜色、超大页面 |
| 批注 | Anchor、权限、overlay revision、导出策略 |

## 11. 总体性能指标

| 指标 | Word 目标 |
|---|---:|
| 暖启动 Small 首页可交互 | p95 ≤ 1.5 s |
| Medium 首页可交互 | p95 ≤ 2.5 s |
| 翻到已布局页面 | p95 ≤ 100 ms |
| selection Context | p95 ≤ 100 ms |
| ≤10 MiB toDocx | p95 ≤ 2 s（内核交付后基线校准） |
| local Resource commit | p95 ≤ 1 s |
| crash-free sessions | ≥ 99.8% |
| 旧 revision 覆盖/损坏保存 | 0 |
| close 后 native/session 泄漏 | 0 |

## 12. PR/工作包建议

1. I0 effective capability/probe。
2. I1 Resource/Format Provider。
3. I1 Word Adapter/Surface resourceRef 迁移。
4. I1 DSH UI consumer 与 fallback。
5. I2 vendor `toDocx/plainText`。
6. I2 commit/conflict/recovery。
7. I3 Word contract Provider/Consumer。
8. I4 每个平台独立 PR/发布。
9. I5 每个格式独立立项，禁止一个 PR 同时引入 Excel/Slides/PDF。

## 13. 回滚

- Word Adapter 新路由可回到现有 WordPage 打开链；ResourceRef 保持有效。
- Save 异常时将 effective mode 降为 view，保留只读和 Viewer fallback。
- native artifact 按 digest 回滚；已有 session 使用创建时版本直至关闭。
- Excel/Slides/PDF 无引擎时始终保持 unavailable，不通过空模板“降级”。

