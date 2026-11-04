# 09_function_subdocument_own — 依赖分界与收敛

> 分析日期：2026-08-19  
> 问题：`SubDocument.java`（1173 行）ctor 真实构造了一长串类型；按「类」去 java2rust 会把 **PLC / PLCSection / KRange / CPieceTable / PLCHeader / HelperHeaderFooter** 整棵树拉进本 function，编译面与队列不可控。  
> 结论：**09 按 I/O 切，不按类切。** 存储模型的语义属于 **wc-01**；09 只做「装配并拥有」；`KRange` / 页眉页脚编辑不在本 function、也不是 wc-02 parse MVP 的 feature-critical。

对照：

| 文档 | 角色 |
|------|------|
| [STRUCTURE_ANALYSIS.md](../STRUCTURE_ANALYSIS.md) §4 / **§4.5** | feature 级 Must-write vs stub |
| [FUNCTIONS.md](../FUNCTIONS.md) | 01–10 I/O 串联 |
| [functions/09_…/FUNCTION.md](../functions/09_function_subdocument_own/FUNCTION.md) | 本 function 测试矩阵 |
| [SubDocument.md](./SubDocument.md) | 类职责与 ctor 顺序 |
| wc-01 [FUNCTIONS.md](../../wc-01-storage-write/FUNCTIONS.md) | rope / CHPX / PAPX / PLCSection / CPieceTable 已测绿 |

---

## 0. 三层所有权（先记这个）

```text
┌─────────────────────────────────────────────────────────────────┐
│ wc-02 FEATURE（对象图：谁拥有、create 顺序、styleId/numId）        │
│  种子：Document 常量 / KStyle* / Listeners / SubDocument 装配     │
│        / TextDocument._create / StyleId2Sti / NumId / Normal     │
└────────────────────────────┬────────────────────────────────────┘
                             │ 09 只消费、不重写语义
┌────────────────────────────▼────────────────────────────────────┐
│ wc-01 FEATURE（存储模型：字符、属性区间、节 PLC、门面）            │
│  CPieceTable / PLC / PLCSection / ChpxTable / PapxTable / rope   │
└─────────────────────────────────────────────────────────────────┘

KRange / HelperHeaderFooter / RangeTool / getContentThumb
  → 编辑与页眉页脚内容 API，不是 parse 对象图，禁止从 09 发散
```

**「feature-critical」≠「SubDocument.java 里出现过的每一个类型」。**  
feature-critical 只覆盖 **parse MVP 热路径上必须可运行、且本 feature 负责的语义**。ctor 里为了 Java 完整性而 `new` 出来、导入期不读写的对象，一律 **defer / 编译壳**，并标注理由。

---

## 1. 这些模型和概念到底是谁的

### 1.1 概念对照（不要混层）

| 概念 | 它是什么 | 权威语义在哪 | 09 怎么用 |
|------|----------|--------------|-----------|
| **TextRope / KNotifyTextRope / TextPool** | 可变字符流；多故事共享 pool | wc-01 `01_function_textrope_append` | ctor：`new` + `setTextPool(parent.getTextPool())`；`getLength` ≡ `rope.length()` |
| **CHPX / PAPX + Factory** | run/段属性区间表 | wc-01 `03`/`04` | ctor：工厂 `create*(this)`；getter 非空。**不**重测 insert 矩阵 |
| **TextRopeListeners** | 多播 before/after + IO skip | wc-02 function **08** | ctor：`add(chpx); add(papx); setListener`。**不**重测顺序/IO skip |
| **CPieceTable** | rope+CHPX+PAPX 门面 | wc-01 `06_function_cpiecetable` | ctor：`new CPieceTable(this)`；`getPieceTable()` 非空。**不**重测 guessed length / charAt 委托 |
| **PLC** | 沿 CP 的结构节点树（Head.position()=`getLength()`） | wc-01 `05` 的底座 + plc-deep-dive | 09 **不**重写 PLC 算法；只 `new` 具体子类并挂到字段 |
| **PLCSection** | 分节节点（type=0 才有） | wc-01 `05_function_plcsection` | type==0：`mSections = new PLCSection(this)`；`getPLCSection()` **Option**（Java null） |
| **PLCField / ShapeAnchor / Bookmark / Header / FootnoteTxt / …** | 其它故事的结构 PLC | 各故事 importer（wc-04/05）才写入 | Java `createPLC` 会 new；**09 验收不要求这些 getter 非空** |
| **KRange** | 选区/编辑范围（start/end、insertParagraph…） | 编辑层；Java 4924 行 | **09 零引用。禁止 java2rust / 禁止为 09 扩编译面** |
| **HelperHeaderFooter / PLCHeader 深层** | 页眉页脚节点与内容 range | 页眉故事写入 / 版式 | type=2 的 `new PLCHeader(this)` 可空壳；**不要**顺着 `getRangeHeader` 去改 KRange |
| **TextDocument** | 文档根：7 槽、共享 pool/styles/tx | wc-02 function **10** | 09 只要 **最小 parent**：`getTextPool` / `getStyles` / `getTransaction`（`bMock` 或软 parent）。**禁止**为 09 导出完整 `TextDocument.rs` |

### 1.2 Java ctor 真实顺序 vs 09 必做步骤

Java `SubDocument(parent, nType)` 121–158 行（不可乱的**语义序**仍有效，但不是每步都要在本 function 真实现）：

| 步 | Java | 09 必做？ | 说明 |
|----|------|-----------|------|
| 1 | `mParent` / `mType` | **是** | SD1/SD8 |
| 2 | `setTransaction(parent.getTransaction())` | **是**（接到 parent 的 tx） | CPieceTable ctor 会 `document.getTextDocument().getTransaction()` |
| 3 | ErrorCollector / DocumentCorrector / Dispatcher | **否** | 错误收集与事件派发，导入热路径不验收 |
| 4–5 | `KNotifyTextRope` + `setTextPool(parent)` | **是** | SD7 共享 pool |
| 6–7 | Chpx/Papx Factory | **是** | SD3 |
| 8–9 | `TextRopeListeners` + add chpx/papx + `setListener` | **是**（挂上即可） | 行为由 08 保证 |
| 10 | `new CPieceTable(this)` | **是** | SD3；调用 **已有** wc-01 实现 |
| 11 | `TextData.createTextStream` | **否** | 流过滤器；CoreHelper 写入走 rope/表，不经本 function |
| 12 | `createPLC()` | **部分** | 见 §2 |
| 13 | `addTextListener(mShapeAnchors)` | **否** | 形状锚点 |
| 14 | AutoNum / TableInfos + listeners.add | **否** | numbering/table 缓存，wc-03/04 |
| 15 | type==0 AutoWordStat | **否** | 统计 |
| 16 | `DebugPropertyDescriptor.init()` | **否** | debug |

---

## 2. `createPLC`：feature-critical 切到哪一刀

Java `createPLC` 738–782：

```text
switch (mType):
  0 → PLCSection + PLCField
  1 → PLCField + FootnoteTxt + createPLCOfFootnote
  2 → PLCField + PLCHeader
  3 → CommentTxt + …
  4 → Endnote …
  5/6 → TextboxText
然后：PLCShapeAnchor + bookmark + documentFields + 全部 PLC 注册 text listener
```

| type | Java 会建 | **09 验收（FUNCTION.md SD1–SD8）** | feature-critical（wc-02 对下游） |
|------|-----------|-------------------------------------|----------------------------------|
| **0 Main** | Section + Field | `getPLCSection()` **非 null**；Field **不测** | Main 必须能拿到 **PLCSection**（wc-04 分节 / wc-01 已测绿的表） |
| **1 Footnote** | Field + FootnoteTxt | `getPLCSection()` **null**（SD5） | 槽位存在即可；脚注 PLC **defer** |
| **2 Header** | Field + PLCHeader | `getType()==2` 且 **无 PLCSection**（SD8） | PLCHeader **可空壳**；禁止展开 HelperHeaderFooter/KRange |
| 3–6 | 各故事 PLC | 本 function **不测** | 写入在 wc-04/05；构造可 skip 或壳 |

**09 的 `createPLC` 最小真写：**

1. `mSections: Option<PLCSection>`（不要按值返回，对齐 Java null）。
2. `type == 0`：`mSections = Some(PLCSection::new(this))`。
3. `type != 0`：不赋值 `mSections`（保持 `None`）。
4. 其余 `new PLCField` / `PLCHeader` / ShapeAnchor / bookmark：**允许不调用**；若为了贴近 Java 顺序而调用，必须是 **已有实现或已标注 `COMPILE_SHELL`**，且 **不得** 为了让它们编译而去 java2rust `KRange`。

`PLCField` 出现在 Java case 0 里，STRUCTURE 旧文曾写「type 0 必须 PLCSection+PLCField+AutoWordStat **全链**」。对 **parse 写入**（CoreHelper.append / importer 段），Field 域字符是后续 feature；**09 / wc-02 MVP 把 PLCField、AutoWordStat 从 function 必测中拿掉**，降为 feature 级 defer（与 [SubDocument.md](./SubDocument.md) §3「其它 PLC 可空壳」一致）。

---

## 3. 判定表：critical / 本 function / 禁止

### 3.1 wc-02 **feature-critical**（整 feature 必须真写，但分摊到 01–10）

| 类 / 子集 | 由哪个 function 验收 | 不要在 09 重做 |
|-----------|----------------------|----------------|
| Document 类型常量 0..6、COUNT=7 | **01** | 只使用常量 |
| BuildinStyleSheet / StyleId2Sti / UUSti | **02–03** | — |
| KStyle / KStyles / addNormalStyle | **04–06** | `getStyles()` 委托 parent |
| NumIdRedirection | **07** | — |
| TextRopeListeners 顺序与 IO skip | **08** | ctor 只接线 |
| SubDocument **装配** rope/表/piece + type0 PLCSection | **09** | 本表 |
| TextDocument `_create` 7 槽 + 共享 pool + 修 `i+=1` | **10** | 09 不用满 7 槽、不调 `_create` |

### 3.2 **09 function-critical**（本 function 源码 + 测试必须覆盖）

入口（与 FUNCTION.md 一致）：

```text
SubDocument(parent, nType)
getType / getLength
getTextRope / getChpxTable / getPapxTable / getPieceTable
getPLCSection → Option   // type0 Some，type1/2 None
getStyles → 同一 parent.styles
rope.getTextPool 指针：type0 与 type1 相同
```

Parent 最小面（假 `TextDocument(true)` / 软 parent，**不是** 6672 行 `TextDocument.rs`）：

```text
getTransaction()
getTextPool()      // 共享
getStyles()
```

`CPieceTable` / `ChpxTableFactory` / `PapxTableFactory` / `PLCSection::new` / `KNotifyTextRope`：**调用 wc-01 已接线 API**，禁止为 09 再跑一遍这些类的 java2rust。

### 3.3 **明确非 09、也非 wc-02 parse-critical**（发散源，硬禁止）

| 类型 | 为何会出现 | 约束 |
|------|------------|------|
| **KRange**（~4924 行） | `HelperHeaderFooter.RangeHeaderFooter extends KRange`；编辑 `insertParagraph` | **禁止**因 09/页眉去 0t→6。编译面不要为 09 `pub mod` 完整 KRange 树 |
| **HelperHeaderFooter** | 页眉页脚 range / `establishHeaderFooter` | **禁止**作为 09 依赖。type=2 不测 header 内容 |
| **RangeTool / KRangeBase 深层** | 选区工具 | 同 KRange |
| **PLCHeader 深层**（addNodeEvenHeader、与 SectionNode 互指） | `createPLC` case 2 | 可 `COMPILE_SHELL`；禁止顺着它改 PLCSection 未审计的 setter 闭包去拉 KRange |
| **getContentThumb / Bitmap / Shape / MediaLib** | SubDocument 后部编辑 API | 不进 09 src 编译面 |
| **完整 TextDocument.rs** | ctor 参数类型是 TextDocument | 09 用最小 parent；10 再导出 `_create` 子集 |
| **完整 1886 行 SubDocument.rs** | 队列曾标 done | **保留完整交错模板**；09 只真写 ctor+矩阵 getters；其余方法空体但 **Java 注释留在原位** |

---

## 4. 为何会「依赖发散不可控」（根因）

1. **按类改写 vs 按 I/O 切 function。** `dep_topo` 把 `PLCField;PLCSection;PLCShapeAnchor;CPieceTable;…` 写成 SubDocument 的 `depends`。那是 **类拓扑**，不是 09 的验收闭包。java2rust 对「ctor 里 new 过的类型」会判 feature-critical，于是 PLCHeader → HelperHeaderFooter → KRange。
2. **STRUCTURE §4.1 旧表述**「`SubDocument.rs` 构造顺序链 + createPLC」被理解成 **整文件逐行移植**（§1.5 也曾写 Rust 1886 行整体一致）。那会把步骤 3/11/13–16 和 createPLC 全 case 都变成 must-write。
3. **编译面 `pub mod SubDocument`** 会编译全部 `use`。对策是 **暂缓非 critical 方法体**，不是裁掉模板。`PLCSection.rs` 尤其禁止删 `// java` 注释。
4. **KRange 与 09 无 I/O 边。** 09 测试从不构造 Range、不调 `insertParagraph`。把它当 09 依赖是错误分类。

---

## 5. 收敛方案（执行约束）

### 5.1 编译面（`libs/word-core/src/lib.rs`）

09 Loop **只允许**为下列模块增加导出（若尚未导出）：

- 已有 wc-01：collections/textrope、Chpx/Papx 工厂与表、PLC、PLCSection、CPieceTable  
- 已有 wc-02：Document 常量、KStyles/KStyle（软 TextDocument parent）、TextRopeListeners  
- **本 function**：完整 `SubDocument.rs` 交错模板（ctor + 09 getters **真写**；其余方法留注释+签名，可空体）

**禁止**为 09 新增：`TextDocument` 全文件、`KRange` 全文件、`HelperHeaderFooter`、`RangeTool`、`PLCHeader` 全语义、shape/heading/words。

若某 `use` 挡住 cargo：优先给 **该 import 对应方法** 留 `DEFERRED_NON_FUNCTION` 空体（注释不动），或对非 09 路径的 `use` 用已有类型占位；**禁止**删 `// java` 注释、禁止把模板裁成 IOHelper 式瘦文件。

### 5.2 SubDocument.rs：完整模板 + 暂缓实现（禁止 `--force` 重生）

**保留全部** Java 注释锚点与方法签名（ctor、createPLC 全 case、getRange、lock、thumb…）。09 **只把下列签名写成可运行语义**：

- `SubDocument(TextDocument, int)` 中 FUNCTION.md §1.2 标「是」的步骤  
- `createPLC` 的 **type 0 建 PLCSection** + **非 0 不建 mSections**  
- `getType` / `getLength` / 四个存储 getter / `getPLCSection` / `getStyles`

`getPLCSection` Rust 必须是 `Option<…>`（或等价可空），禁止「未建节仍返回默认 PLCSection 值」。

其余方法：空体 / `DEFERRED_NON_FUNCTION` 即可，**注释不得删、不得错位**。`PLCSection.rs` 同样：05 已测的查询 API 保持真写；页眉级联等暂缓，但 `check-template-layout.mjs --java PLCSection.java` 必须 PASS。

### 5.3 Parent

- 允许：`TextDocument(boolean bMock)` 跳过 `_create` 的最小对象，或 `KStyle::TextDocument` 补上 `textPool` + `styles` 指针 + `transaction`。  
- 禁止：为了 09 去跑 `TextDocument.java` 4264 行的 0t→6。那是 **function 10**。

### 5.4 java2rust / parse-feature-rewrite

| 请求 | 09 进行中是否允许 |
|------|-------------------|
| java2rust `SubDocument.java` 全叶 | **否**（>700 且会吞下 PLC*）。完整模板 **原位修**；非 09 方法空体+留注释 |
| java2rust `CPieceTable` / `PLCSection` / `PLC` | **否**（wc-01 已验收；最多修 09 调用点签名） |
| java2rust `KRange` / `HelperHeaderFooter` | **否**（非 09、非 wc-02 parse-critical） |
| java2rust `PLCHeader` | **否**（09 SD8 不要求 Header PLC 非空） |
| 为 `createPLC` case 0 补 `PLCSection::new` | **是**（调用已有类型） |

队列 `SubDocument rewrite_status=done` **≠** 09 `status=pass`。09 以 `functions.csv` + 非 ignore 测试 + FUNCTION.md §3.6/§3.7/**§3.8** 为准。

### 5.5 测试边界（防回归发散）

必测：FUNCTION.md SD1–SD8。  
**禁止**在 09 测试里：`CoreHelper.append*`、打开 docx、7 槽 `_create`、`KRange`、页眉 `hasHeaderEven`、CHPX insert 矩阵。

覆盖率 `--src` **只** `SubDocument.rs`（完整模板）。不要把 PLCSection.rs / CPieceTable.rs 算进 09 覆盖率（那是 wc-01）。**布局校验**必须另跑 `check-template-layout.mjs`（含 PLCSection.java）。

### 5.6 推荐落地顺序（09 Loop）

```text
1. 冻结 lib.rs：确认未因 09 导出 KRange/HelperHeaderFooter/全量 TextDocument
2. 完整模板原位修：09-critical 真写，其余空体+留 `// java`
3. `getPLCSection` → Option；createPLC 只保证 type0 Some / 其它 None
4. 最小 parent + 活测试 SD1–SD8
5. `check-template-layout.mjs` 对 SubDocument.rs + PLCSection.rs 必须 PASS
6. cargo test --test 09_function_subdocument_own
7. function-coverage.sh 只扫 SubDocument.rs
8. 回归 01–08；csv → pass
9. 下一 function 才是 10（修 i+=1、7 槽、真 TextDocument 子集）
```

### 5.7 一句话给后续 agent

> 09 的产品问题是：**「new 一个带 type 的故事流之后，存储句柄在不在、Main 有没有节表、pool/styles 是否跟 parent 共享」。**  
> 不是：**「把 SubDocument 及其 ctor 闭包里每个 Java 类型都改写成 Rust」。**

---

## 6. 与 STRUCTURE 旧文的差异（以本文为准）

| 旧表述 | 现约束 |
|--------|--------|
| SubDocument.rs 整体逐行移植已 done | 队列 done ≠ 09 pass；09 保留完整模板，critical 真写 + 其余暂缓；`getPLCSection`=`Option` |
| type0 必须 PLCSection+PLCField+AutoWordStat 全链 | 09 只强制 **PLCSection**；Field/WordStat defer |
| createPLC 各 case Rust 已实现故 type2 要真 PLCHeader | 09 只断言 type2 **没有** PLCSection；Header PLC 壳即可 |
| 为 Header 故事去改 HelperHeaderFooter/KRange | **禁止**，与 09 无 I/O 边 |
