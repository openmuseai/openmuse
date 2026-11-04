# ET-Core Rust 重写详细方案

2026-10-30 最新冻结：[R65–71 原记录/PTG/行 atom/稀疏表与行合并](rewrite-review/milestones/m4-edit-save/34-20261030-r0b71-row-merge-xf.md)逐批真机通过，全量 **291/0/80**，2,357 份源/配置完成时指纹未变；M1/M2 CLI/C ABI/样本/M3 复核通过。R72 按原引用类 createFromNptg 继续共享/数组公式真实引用，再接 EvaluationPresent→SheetDataLabel 和 ET 保存链；完整 raw/事务/保存重开/M4 仍未验收。

2026-10-30 续行：R65–67 原 CellRecord/FormulaRecord 与 PTG debug 已接回并逐批真机对照，R67 串行库测试 41/0、所有目标编译通过；[R68 原行范围/数据](rewrite-review/milestones/m4-edit-save/31-20261030-r0b68-row-ranges-data.md)新采集 523,728 字段零差。R69 正在接回真实行 atom、隐藏行/可操作行列及原事务监听器，继续 SparseIndexMap→SheetDataLabel 与 ET 保存链。最新完整冻结全量仍是 R64 283/0/80，完整 raw/事务/save-reopen/M4 未验收。

2026-10-30 续行：[R57–64 名称/窗口/保护/共享及监听链](rewrite-review/milestones/m4-edit-save/28-20261030-r0b64-sheets-finish-regression.md)逐批真机对照，固定源全量 **283/0/80**，完成时源码未变；M1/M2/C ABI/样本/M3 保存观察复核通过。继续原 FormulaRecord/CellRecord→SheetDataLabel 及 ET worksheet/workbook/XlsxWriter/saveImpl，并按原函数实例依赖补完整 raw/事务。完整 M4 未验收。

2026-10-30 续行：[R0b-51～56 名称保存的引用/token 输出依赖](rewrite-review/milestones/m4-edit-save/20-20261030-r0b56-token-output.md)已逐批真机对照。R56 新增 2,945 组历史、43,738 字段零差，修复 choose 表读写与 NaN 规范化，冻结全量已通过 275/0/80，源码完成时未变。原 ExternSheetNameResolver/NamePtg/NameXPtg/FormulaRenderer 已在 R57 真机对照 1,924 组历史零差，真实 DefineNameLabel 已在 R58 双 writer 对照 3,699 组历史、37,008 字段和 171,532 字节零差；R59 原窗口/视图另有 989,508 字段零差；R60–62 原共享记录、密码和工作簿保护另有 261,530 字段零差；继续工作簿共享、完整 raw、事务/函数实例依赖和 ET workbook/worksheet/XlsxWriter/saveImpl。完整 M4 未验收。

2026-10-30 续行：[R0b-46～50 原名称事务与工作簿完成回调](rewrite-review/milestones/m4-edit-save/14-20261030-r0b46-r0b50-names-callback-transactions.md)已真机对照 103,086 字段零差，132 个方法切片及 17 个负控通过；接回默认监听组、SST/外链/延迟名称及事件合并时序。R50 固定源全量 269/0/80，M1/C1、20 组新 M2 真机及 ABI、样本/M3 复核通过。继续完整 raw、函数实例依赖和 ET workbook/worksheet/XlsxWriter/saveImpl，完整 M4 未验收。


2026-10-30 最新：R45 原 SST 生命周期及依赖已真机零差（13 ZIP、157,873 字段），见 rewrite-review/milestones/m4-edit-save/13-20261030-r0b45-sst-original-lifecycle.md。最近全量指纹为 R43～44 的 260/0/79；R46 按原 ExternalReferences/延迟名称/NameRecord 引用、完成监听继续接线，完整 raw/事务/依赖/saveImpl 尚未验收。以下旧记录保留其当时范围。

2026-10-30 续行：R0b-39 原差异格式模型/现代 writer 真机对照已恢复复核；R0b-40 改写中，沿原 CellXfs/CellStyleXfs/CellStyles/TableStyles/StyleSheetLabel → XlsxwStyles 接通实际 styles 部件，保留原收集和输出顺序。原未接分支逐项记录，不以空实现或替代导出器通过验收。完整 raw、完整事务/依赖和整本保存仍持续实施。

2026-10-29 / R0b-36 已接回 `OpcrRelationship → typed OxfrApp/Core/CustomPart → 原 HL 字符串 handler → XlsxrApp/Core/Custom/PropertiesHandler → XlsxReader`。PKM110 Android 16：852 日期、721 整数输入、52 属性夹具、51 属性 ZIP 读写再读、9,378 JSON 字段及 209,699 原文件字节零差；原日期默认时区、bool 写 1 后读 false、partial failure 和 Date 别名保持。限已测范围，未声称整类门面或 M4 验收。R0b-37 继续原 `XlsxwSharedStrings`、`NumFmtsLabel/PatternTypeMap/FillsLabel`，再接完整 style/workbook/sheet writer 与 raw/事务原依赖。

R0b-37～38 已对照原 `XlsxwSharedStrings/NumFmtsLabel/PatternTypeMap/FillsLabel/BordersLabel/ColorsLabel` 与六个 BorderFormatting 选项 getter。两个 short 映射共 131,072 值，原 alias/去重/异常/渐变/调色板历史与实际 SST ZIP 保持；本段合计 149,672 JSON 字段零差。下一原依赖为 CellXfs/CellStyleXfs/CellStyles/DifferentialFormatting/TableStyles/StyleSheetLabel → XlsxwStyles，并继续完整 raw/事务与保存入口，详见里程碑 09。

R0b-39 测试中：继续原 DifferentialFormatting 模型和现代 DifferentialFormattingLabel 写入/追加方法；原 ColorFilter 与 BIFF CFRuleRecord 重载依赖未接回，暂未接受整类门面。先验证同对象模型字段、512 字体、36 格式及原追加/别名历史，再补原依赖及 StyleSheetLabel。

2026-10-29 最新续行：[R0b-32～35 原属性、真实持久化与 SST 保存管线](rewrite-review/milestones/m4-edit-save/08-20261029-r0b32-r0b35-settings-persistence-sst.md)。原 KmoAppSettings/docProps、FontsLabel、匿名 Iterator、真实文件块和 SstLabel 已手工接回并真机对照；111 个具体方法切片、35,268 个 JSON 字段、94,620 个文件字节零差，25,009 项迭代摘要一致。完整 raw、完整事务/依赖、剩余 ET label/XlsxWriter/saveImpl 和当前 Flutter 编辑保存尚未验收，继续原依赖实施；M4 未完成。

2026-10-29 最新续行：[R0b-31 共享引用盒与双 sheet 事务/依赖](rewrite-review/milestones/m4-edit-save/07-20261029-r0b31-shared-box-multisheet-dependencies.md)。修正引用基类的外部 3D 子类分支，接回本工作簿 3D 字节/Ptg 求值；2,880 组盒、288 组双路径与 10 阶段事务历史、39,790 字段零差，全量 242 项通过。完整 raw、跨工作簿 CRN 与 ET 保存持续实施，M4 未验收。

2026-10-29 最新续行：[R0b-30 raw 公式格式前置](rewrite-review/milestones/m4-edit-save/06-20261029-r0b30-formula-number-formats.md)。原建议格式、共享判断、本地数字继承已接回；三方法/82,794 字段真机对照零差。继续共享引用边界、双 sheet 事务/依赖及完整 raw、ET 保存链，M4 未验收。

2026-10-29 最新续行：[R0b-29 原关系策略与 XLSX part 工厂](rewrite-review/milestones/m4-edit-save/05-20261029-r0b29-relations-xlsxw-factories.md)。nullable 关系、XLS/剪贴板策略、XlsxwPackage 的 15 个方法通过真机对照；103 个方法切片/25 个接口方法，10,471 个字段零差。继续完整 raw、事务/依赖、媒体原依赖与 ET writer/saveImpl，M4 未验收。

本文档基于 `koffice/et` 渲染管线拆分后的模块架构，制定逐模块的 Rust 重写方案。

2026-10-29 最新续行：[R0b-28 原 OPC 包关闭链/content-types](rewrite-review/milestones/m4-edit-save/04-20261029-r0b28-package-content-types.md)。原包/关系/content-types/属性关闭链对照通过，235 项测试通过。继续 nullable 关系门面、原关系策略与 ET XLSX writer/saveImpl，以及完整 raw/事务依赖扩面；M4 阶段尚未验收。

2026-10-29 最新续行：[R0b-27 手工识别与 OPC 写流](rewrite-review/milestones/m4-edit-save/03-20261029-r0b27-recognise-opcw.md)。原识别全分支、语言资源身份、rawOnlyNumFmt、OPC part/关系/属性类已接回并对照；146 个方法切片、690,655 个字段零差，232 项测试通过。继续完整 raw、事务扩面、OPC 包/content-types 与 ET writer/saveImpl；M4 仍未验收，产品 execute/export 未开放。

2026-10-28 当前续行：[M4 有界编辑与保存（持续实施中）](rewrite-review/milestones/m4-edit-save/README.md)。[R0b-26](rewrite-review/milestones/m4-edit-save/02-20261028-r0b26-raw-transactions-xml.md)推进 raw 结果写回、数字识别、真实事务/范围重算、共享公式拆解和 XML writer 前置，保留原 Java 逐类结构。完整 raw 入口、事务扩面、XLSX 保存与产品能力尚未接通。阶段状态以 [现行路线图](rewrite-review/milestones/00-roadmap.md) 和里程碑证据为准，下表早期计数不作为验收。

---

## 0. 总体进度面板

| Java 代码库 | Java 文件数 | 重写范围 | Rust 文件 | Rust 测试 | 进度 |
|:------------|----------:|:-------:|----------:|----------:|:----:|
| **core/src** (model+drawing+grid) | 1263 | 全量 | 0 | 0 | 待开始 |
| **et-core-io/src** (parse/write) | 1062 | 全量 | 0 | 0 | 待开始 |
| **et-core-chart/src** (chart) | 796 | 全量 | 0 | 0 | 待开始 |
| **et-core-formula/src** (formula) | 431 | 全量 | 0 | 0 | 待开始 |
| **合计** | **3552** | | **0** | **0** | |

**最后更新**: 2026-05-08 | **状态**: 计划制定中

---

## 0.1 目录级重写计划约定

> **规则**: 对于每个包含具体代码的**最后一级目录**（叶子目录），在对应 Rust 目录中编写一个 `xxx_rewriter_plan.md`，详细说明：
>
> 1. 该目录中哪些 Java 代码文件需要重写（**全量列举**）
> 2. 每个文件的 Java 行数、类型（class/interface/enum）、对应 Rust 文件名
> 3. 当前重写状态（已重写 / 待重写 / 不重写）
> 4. 验证脚本运行结果
> 5. 备注（不重写需写清理由）
>
> **命名规则**: `<java_package_last_segment>_rewriter_plan.md`
>
> **验证方式**: 每完成一个目录的重写，必须运行验证脚本：
> ```bash
> python tools/verify_rewrite.py <java_dir> <rust_dir>
> ```
> 确保 0 个 ❌ 错误后才能标记为"已验证"。

---

## 1. 总体策略

### 1.1 重写目标

将 `koffice/et` 的核心渲染管线从 Java 移植到 Rust (`libs/ss-core`)，实现:
- 100% 逻辑一致性（解析结果、计算结果、排版结果、渲染指令完全一致）
- 跨平台统一（Web/Desktop/Mobile 三端共享 Rust 核心）
- **外观尽可能一致** — 目录、文件名、类名、方法名、字段顺序与 Java 保持对应
- **可验证** — 每个目录完成后必须通过 `verify_rewrite.py` 脚本验证

### 1.2 重写原则

详见 `RUST_REWRITE_RULE.md`，核心要求：
- 逐目录对照：Rust 文件的目录结构与 Java 对应
- 逐类对照：Rust struct/enum ↔ Java class/interface
- 逐方法对照：Rust fn ↔ Java method，确保函数签名、数量、控制流一致
- 逐字段对照：Rust field ↔ Java field，确保类型和语义一致
- 逐行对照：方法体 Java 语句注释 + Rust 语句一一对应
- 通过 Python 验证脚本 (`tools/verify_rewrite.py`) 自动检查

### 1.3 Rust 目录结构映射规则

Java 包路径去除公共前缀，映射到 Rust module 路径：

```
# core/src 映射
Java: cn.wps.moss.core.book.KmoWorkbook      → ss_core::core::book::kmo_workbook
Java: cn.wps.moss.core.sheet.KmoSheet         → ss_core::core::sheet::kmo_sheet
Java: cn.wps.moss.app.KmoAppApi               → ss_core::app::kmo_app_api
Java: cn.wps.moffice.spreadsheet.control.grid → ss_core::grid::*

# et-core-io/src 映射
Java: cn.wps.moss.xlsxr.XlsxReader            → ss_core::io::xlsxr::xlsx_reader
Java: cn.wps.moss.filefmt.biff8.record.*       → ss_core::io::filefmt::biff8::record::*

# et-core-chart/src 映射
Java: cn.wps.chart.data.ChartData              → ss_core::chart::data::chart_data
Java: cn.wps.chart.render.ChartRenderer        → ss_core::chart::render::chart_renderer

# et-core-formula/src 映射
Java: cn.wps.et.ss.formula.FormulaParser       → ss_core::formula::parser::formula_parser
Java: cn.wps.et.ss.formula.functions.*          → ss_core::formula::functions::*
```

---

## 2. 逐代码库子模块重写面板

### 2.1 `koffice/et/core/src` — 1263 Java files

> 数据模型 + Drawing + 网格布局 + 渲染 + 应用逻辑

#### 2.1.1 moss.core — 核心数据模型 (95 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 1 | `moss.core.book` | 13 | `src/core/book/` | P1 | TODO | `book_rewriter_plan.md` |
| 2 | `moss.core.sheet` | 18 | `src/core/sheet/` | P1 | TODO | `sheet_rewriter_plan.md` |
| 3 | `moss.core.table` | 2 | `src/core/table/` | P1 | TODO | `table_rewriter_plan.md` |
| 4 | `moss.core.theme` | 11 | `src/core/theme/` | P1 | TODO | `theme_rewriter_plan.md` |
| 5 | `moss.core.drawing` | 34 | `src/core/drawing/` | P1 | TODO | `drawing_rewriter_plan.md` |
| 6 | `moss.core.drawing.tools` | 8 | `src/core/drawing/tools/` | P1 | TODO | `tools_rewriter_plan.md` |
| 7 | `moss.core.pivottable` | 18 | `src/core/pivottable/` | P4 | TODO | `pivottable_rewriter_plan.md` |
| 8 | `moss.core.pivottable.data` | 7 | `src/core/pivottable/data/` | P4 | TODO | `data_rewriter_plan.md` |

#### 2.1.2 moss (root + utils) (34 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 9 | `moss` (root) | 3 | `src/moss/` | P1 | TODO | `moss_rewriter_plan.md` |
| 10 | `moss.util` | 28 | `src/util/` | P1 | TODO | `util_rewriter_plan.md` |
| 11 | `moss.util.binarytree` | 3 | `src/util/binarytree/` | P1 | TODO | `binarytree_rewriter_plan.md` |
| 12 | `moss.util.binarytree.node` | 3 | `src/util/binarytree/node/` | P1 | TODO | `node_rewriter_plan.md` |
| 13 | `moss.util.cache` | 2 | `src/util/cache/` | P1 | TODO | `cache_rewriter_plan.md` |
| 14 | `moss.util.checksum` | 4 | `src/util/checksum/` | P1 | TODO | `checksum_rewriter_plan.md` |
| 15 | `moss.util.deqlist` | 4 | `src/util/deqlist/` | P1 | TODO | `deqlist_rewriter_plan.md` |
| 16 | `moss.util.objpool` | 4 | `src/util/objpool/` | P1 | TODO | `objpool_rewriter_plan.md` |
| 17 | `moss.util.primitive` | 3 | `src/util/primitive/` | P1 | TODO | `primitive_rewriter_plan.md` |
| 18 | `moss.util.rangelist` | 7 | `src/util/rangelist/` | P1 | TODO | `rangelist_rewriter_plan.md` |

#### 2.1.3 moss.framework + moss.io + moss.engine (75 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 19 | `moss.framework` | 14 | `src/framework/` | P1 | TODO | `framework_rewriter_plan.md` |
| 20 | `moss.io` | 8 | `src/moss_io/` | P2 | TODO | `io_rewriter_plan.md` |
| 21 | `moss.io.clipx` | 2 | `src/moss_io/clipx/` | P4 | TODO | `clipx_rewriter_plan.md` |
| 22 | `moss.io.except` | 9 | `src/moss_io/except/` | P2 | TODO | `except_rewriter_plan.md` |
| 23 | `moss.io.ole` | 3 | `src/moss_io/ole/` | P2 | TODO | `ole_rewriter_plan.md` |
| 24 | `moss.engine.grid` | 32 | `src/engine/grid/` | P1 | ✅ DONE | `grid_rewriter_plan.md` |
| 25 | `moss.engine.grid.griditerator` | 3 | `src/engine/grid/griditerator/` | P1 | TODO | `griditerator_rewriter_plan.md` |
| 26 | `moss.engine.eval` | 14 | `src/engine/eval/` | P1 | TODO | `eval_rewriter_plan.md` |
| 27 | `moss.engine.eval.sup` | 3 | `src/engine/eval/sup/` | P1 | TODO | `sup_rewriter_plan.md` |
| 28 | `moss.engine.linktb` | 11 | `src/engine/linktb/` | P1 | TODO | `linktb_rewriter_plan.md` |
| 29 | `moss.engine.areaevent` | 3 | `src/engine/areaevent/` | P1 | TODO | `areaevent_rewriter_plan.md` |
| 30 | `moss.engine.sort` | 14 | `src/engine/sort/` | P4 | TODO | `sort_rewriter_plan.md` |
| 31 | `moss.sparkline` | 1 | `src/moss_sparkline/` | P4 | TODO | `sparkline_rewriter_plan.md` |
| 32 | `moss.sparkline.source` | 7 | `src/moss_sparkline/source/` | P4 | TODO | `source_rewriter_plan.md` |
| 33 | `moss.compiler.edithint` | 7 | `src/compiler/edithint/` | P4 | TODO | `edithint_rewriter_plan.md` |
| 34 | `moss.external.text` | 3 | `src/external/text/` | P4 | TODO | `text_rewriter_plan.md` |

#### 2.1.4 moss.chart (in core) (20 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 35 | `moss.chart.app` | 4 | `src/moss_chart/app/` | P4 | TODO | `app_rewriter_plan.md` |
| 36 | `moss.chart.io` | 1 | `src/moss_chart/io/` | P4 | TODO | `io_rewriter_plan.md` |
| 37 | `moss.chart.io.reader` | 4 | `src/moss_chart/io/reader/` | P4 | TODO | `reader_rewriter_plan.md` |
| 38 | `moss.chart.io.writer` | 3 | `src/moss_chart/io/writer/` | P4 | TODO | `writer_rewriter_plan.md` |
| 39 | `moss.chart.source` | 15 | `src/moss_chart/source/` | P4 | TODO | `source_rewriter_plan.md` |
| 40 | `moss.chart.source.base` | 3 | `src/moss_chart/source/base/` | P4 | TODO | `base_rewriter_plan.md` |
| 41 | `moss.chart.usermodel` | 5 | `src/moss_chart/usermodel/` | P4 | TODO | `usermodel_rewriter_plan.md` |

#### 2.1.5 moss.app — 应用逻辑 (248 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 42 | `moss.app` (root) | 40 | `src/app/` | P1 | TODO | `app_rewriter_plan.md` |
| 43 | `moss.app.adjuster` | 4 | `src/app/adjuster/` | P4 | TODO | `adjuster_rewriter_plan.md` |
| 44 | `moss.app.condfmt` | 11 | `src/app/condfmt/` | P2 | TODO | `condfmt_rewriter_plan.md` |
| 45 | `moss.app.condfmt.rule` | 12 | `src/app/condfmt/rule/` | P2 | TODO | `rule_rewriter_plan.md` |
| 46 | `moss.app.condfmt.valuebased` | 4 | `src/app/condfmt/valuebased/` | P2 | TODO | `valuebased_rewriter_plan.md` |
| 47 | `moss.app.custom` | 2 | `src/app/custom/` | P4 | TODO | `custom_rewriter_plan.md` |
| 48 | `moss.app.datasplit` | 2 | `src/app/datasplit/` | P4 | TODO | `datasplit_rewriter_plan.md` |
| 49 | `moss.app.datavalidation` | 3 | `src/app/datavalidation/` | P4 | TODO | `datavalidation_rewriter_plan.md` |
| 50 | `moss.app.deduplication` | 2 | `src/app/deduplication/` | P4 | TODO | `deduplication_rewriter_plan.md` |
| 51 | `moss.app.event` | 8 | `src/app/event/` | P1 | TODO | `event_rewriter_plan.md` |
| 52 | `moss.app.except` | 15 | `src/app/except/` | P1 | TODO | `except_rewriter_plan.md` |
| 53 | `moss.app.extract` | 1 | `src/app/extract/` | P4 | TODO | `extract_rewriter_plan.md` |
| 54 | `moss.app.fill` | 17 | `src/app/fill/` | P4 | TODO | `fill_rewriter_plan.md` |
| 55 | `moss.app.fill.combine` | 1 | `src/app/fill/combine/` | P4 | TODO | `combine_rewriter_plan.md` |
| 56 | `moss.app.fill.exchange` | 2 | `src/app/fill/exchange/` | P4 | TODO | `exchange_rewriter_plan.md` |
| 57 | `moss.app.fill.extract` | 3 | `src/app/fill/extract/` | P4 | TODO | `fill_extract_rewriter_plan.md` |
| 58 | `moss.app.fill.insert` | 1 | `src/app/fill/insert/` | P4 | TODO | `insert_rewriter_plan.md` |
| 59 | `moss.app.filter` | 13 | `src/app/filter/` | P4 | TODO | `filter_rewriter_plan.md` |
| 60 | `moss.app.filter.filtercol` | 9 | `src/app/filter/filtercol/` | P4 | TODO | `filtercol_rewriter_plan.md` |
| 61 | `moss.app.find` | 5 | `src/app/find/` | P4 | TODO | `find_rewriter_plan.md` |
| 62 | `moss.app.highlduplication` | 1 | `src/app/highlduplication/` | P4 | TODO | `highlduplication_rewriter_plan.md` |
| 63 | `moss.app.hyperlink` | 5 | `src/app/hyperlink/` | P4 | TODO | `hyperlink_rewriter_plan.md` |
| 64 | `moss.app.ignorederrors` | 4 | `src/app/ignorederrors/` | P4 | TODO | `ignorederrors_rewriter_plan.md` |
| 65 | `moss.app.nameManager` | 1 | `src/app/name_manager/` | P4 | TODO | `name_manager_rewriter_plan.md` |
| 66 | `moss.app.note` | 1 | `src/app/note/` | P4 | TODO | `note_rewriter_plan.md` |
| 67 | `moss.app.paste` | 12 | `src/app/paste/` | P4 | TODO | `paste_rewriter_plan.md` |
| 68 | `moss.app.paste.html` | 2 | `src/app/paste/html/` | P4 | TODO | `html_rewriter_plan.md` |
| 69 | `moss.app.paste.html.copy` | 14 | `src/app/paste/html/copy/` | P4 | TODO | `copy_rewriter_plan.md` |
| 70 | `moss.app.paste.html.copy.model` | 5 | `src/app/paste/html/copy/model/` | P4 | TODO | `model_rewriter_plan.md` |
| 71 | `moss.app.paste.html.paste` | 14 | `src/app/paste/html/paste/` | P4 | TODO | `paste_inner_rewriter_plan.md` |
| 72 | `moss.app.paste.html.paste.context` | 4 | `src/app/paste/html/paste/context/` | P4 | TODO | `context_rewriter_plan.md` |
| 73 | `moss.app.paste.html.paste.model` | 7 | `src/app/paste/html/paste/model/` | P4 | TODO | `paste_model_rewriter_plan.md` |
| 74 | `moss.app.paste.html.paste.utils` | 5 | `src/app/paste/html/paste/utils/` | P4 | TODO | `paste_utils_rewriter_plan.md` |
| 75 | `moss.app.paste.sxml` | 2 | `src/app/paste/sxml/` | P4 | TODO | `sxml_rewriter_plan.md` |
| 76 | `moss.app.persistence` | 3 | `src/app/persistence/` | P4 | TODO | `persistence_rewriter_plan.md` |
| 77 | `moss.app.pivot` | 1 | `src/app/pivot/` | P4 | TODO | `pivot_rewriter_plan.md` |
| 78 | `moss.app.pivot.cache` | 10 | `src/app/pivot/cache/` | P4 | TODO | `pivot_cache_rewriter_plan.md` |
| 79 | `moss.app.pivot.common` | 5 | `src/app/pivot/common/` | P4 | TODO | `pivot_common_rewriter_plan.md` |
| 80 | `moss.app.pivot.connections` | 8 | `src/app/pivot/connections/` | P4 | TODO | `connections_rewriter_plan.md` |
| 81 | `moss.app.pivot.util` | 1 | `src/app/pivot/util/` | P4 | TODO | `pivot_util_rewriter_plan.md` |
| 82 | `moss.app.pivot.view` | 14 | `src/app/pivot/view/` | P4 | TODO | `view_rewriter_plan.md` |
| 83 | `moss.app.protsheet` | 2 | `src/app/protsheet/` | P4 | TODO | `protsheet_rewriter_plan.md` |
| 84 | `moss.app.rowcol` | 2 | `src/app/rowcol/` | P4 | TODO | `rowcol_rewriter_plan.md` |
| 85 | `moss.app.sheetsplit` | 2 | `src/app/sheetsplit/` | P4 | TODO | `sheetsplit_rewriter_plan.md` |
| 86 | `moss.app.sheetsplit.filteritem` | 3 | `src/app/sheetsplit/filteritem/` | P4 | TODO | `filteritem_rewriter_plan.md` |
| 87 | `moss.app.slim` | 4 | `src/app/slim/` | P4 | TODO | `slim_rewriter_plan.md` |
| 88 | `moss.app.sort` | 1 | `src/app/sort/` | P4 | TODO | `sort_rewriter_plan.md` |
| 89 | `moss.app.tables` | 13 | `src/app/tables/` | P4 | TODO | `tables_rewriter_plan.md` |
| 90 | `moss.app.xf` | 11 | `src/app/xf/` | P1 | TODO | `xf_rewriter_plan.md` |

#### 2.1.6 moffice.drawing — Drawing IO/Data (278 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 91 | `moffice.drawing` (root) | 1 | `src/drawing/` | P1 | TODO | `drawing_root_rewriter_plan.md` |
| 92 | `moffice.drawing.app` | 7 | `src/drawing/app/` | P2 | TODO | `drawing_app_rewriter_plan.md` |
| 93 | `moffice.drawing.app.graphic` | 2 | `src/drawing/app/graphic/` | P2 | TODO | `graphic_rewriter_plan.md` |
| 94 | `moffice.drawing.app.text` | 3 | `src/drawing/app/text/` | P2 | TODO | `drawing_text_rewriter_plan.md` |
| 95 | `moffice.drawing.app.util` | 3 | `src/drawing/app/util/` | P2 | TODO | `drawing_app_util_rewriter_plan.md` |
| 96 | `moffice.drawing.data.basic` | 9 | `src/drawing/data/basic/` | P1 | TODO | `basic_rewriter_plan.md` |
| 97 | `moffice.drawing.data.blip` | 1 | `src/drawing/data/blip/` | P1 | TODO | `blip_rewriter_plan.md` |
| 98 | `moffice.drawing.data.color` | 3 | `src/drawing/data/color/` | P1 | TODO | `color_rewriter_plan.md` |
| 99 | `moffice.drawing.data.effect` | 30 | `src/drawing/data/effect/` | P1 | TODO | `effect_rewriter_plan.md` |
| 100 | `moffice.drawing.data.fill` | 9 | `src/drawing/data/fill/` | P1 | TODO | `fill_rewriter_plan.md` |
| 101 | `moffice.drawing.data.font` | 1 | `src/drawing/data/font/` | P1 | TODO | `font_rewriter_plan.md` |
| 102 | `moffice.drawing.data.geometry` | 7 | `src/drawing/data/geometry/` | P1 | TODO | `geometry_rewriter_plan.md` |
| 103 | `moffice.drawing.data.hyperlink` | 1 | `src/drawing/data/hyperlink/` | P1 | TODO | `hyperlink_rewriter_plan.md` |
| 104 | `moffice.drawing.data.media` | 1 | `src/drawing/data/media/` | P1 | TODO | `media_rewriter_plan.md` |
| 105 | `moffice.drawing.data.outline` | 1 | `src/drawing/data/outline/` | P1 | TODO | `outline_rewriter_plan.md` |
| 106 | `moffice.drawing.data.shape` | 3 | `src/drawing/data/shape/` | P1 | TODO | `shape_rewriter_plan.md` |
| 107 | `moffice.drawing.data.style` | 10 | `src/drawing/data/style/` | P1 | TODO | `style_rewriter_plan.md` |
| 108 | `moffice.drawing.data.table` | 11 | `src/drawing/data/table/` | P1 | TODO | `drawing_table_rewriter_plan.md` |
| 109 | `moffice.drawing.data.text` | 8 | `src/drawing/data/text/` | P1 | TODO | `drawing_data_text_rewriter_plan.md` |
| 110 | `moffice.drawing.data.threeD` | 9 | `src/drawing/data/three_d/` | P1 | TODO | `three_d_rewriter_plan.md` |
| 111 | `moffice.drawing.dmlr.handler.chart` | 1 | `src/drawing/dmlr/handler/chart/` | P2 | TODO | `dmlr_chart_rewriter_plan.md` |
| 112 | `moffice.drawing.dmlr.handler.color` | 9 | `src/drawing/dmlr/handler/color/` | P2 | TODO | `dmlr_color_rewriter_plan.md` |
| 113 | `moffice.drawing.dmlr.handler.diagram` | 1 | `src/drawing/dmlr/handler/diagram/` | P2 | TODO | `dmlr_diagram_rewriter_plan.md` |
| 114 | `moffice.drawing.dmlr.handler.font` | 1 | `src/drawing/dmlr/handler/font/` | P2 | TODO | `dmlr_font_rewriter_plan.md` |
| 115 | `moffice.drawing.dmlr.handler.media` | 6 | `src/drawing/dmlr/handler/media/` | P2 | TODO | `dmlr_media_rewriter_plan.md` |
| 116 | `moffice.drawing.dmlr.handler.shape` | 4 | `src/drawing/dmlr/handler/shape/` | P2 | TODO | `dmlr_shape_rewriter_plan.md` |
| 117 | `moffice.drawing.dmlr.handler.shape.graphicframe` | 2 | `src/drawing/dmlr/handler/shape/graphicframe/` | P2 | TODO | `graphicframe_rewriter_plan.md` |
| 118 | `moffice.drawing.dmlr.handler.shape.nvsppr.cnv` | 6 | `src/drawing/dmlr/handler/shape/nvsppr/cnv/` | P2 | TODO | `cnv_rewriter_plan.md` |
| 119 | `moffice.drawing.dmlr.handler.shape.nvsppr.cnv.hyperlink` | 1 | `src/drawing/dmlr/handler/shape/nvsppr/cnv/hyperlink/` | P2 | TODO | `cnv_hyperlink_rewriter_plan.md` |
| 120 | `moffice.drawing.dmlr.handler.shape.sppr` | 2 | `src/drawing/dmlr/handler/shape/sppr/` | P2 | TODO | `sppr_rewriter_plan.md` |
| 121 | `moffice.drawing.dmlr.handler.shape.sppr.effect` | 30 | `src/drawing/dmlr/handler/shape/sppr/effect/` | P2 | TODO | `sppr_effect_rewriter_plan.md` |
| 122 | `moffice.drawing.dmlr.handler.shape.sppr.fill` | 7 | `src/drawing/dmlr/handler/shape/sppr/fill/` | P2 | TODO | `sppr_fill_rewriter_plan.md` |
| 123 | `moffice.drawing.dmlr.handler.shape.sppr.geometry` | 6 | `src/drawing/dmlr/handler/shape/sppr/geometry/` | P2 | TODO | `sppr_geometry_rewriter_plan.md` |
| 124 | `moffice.drawing.dmlr.handler.shape.sppr.geometry.custom` | 12 | `src/drawing/dmlr/handler/shape/sppr/geometry/custom/` | P2 | TODO | `custom_rewriter_plan.md` |
| 125 | `moffice.drawing.dmlr.handler.shape.sppr.outline` | 1 | `src/drawing/dmlr/handler/shape/sppr/outline/` | P2 | TODO | `sppr_outline_rewriter_plan.md` |
| 126 | `moffice.drawing.dmlr.handler.shape.sppr.threeD` | 8 | `src/drawing/dmlr/handler/shape/sppr/three_d/` | P2 | TODO | `sppr_three_d_rewriter_plan.md` |
| 127 | `moffice.drawing.dmlr.handler.shape.sppr.transform` | 4 | `src/drawing/dmlr/handler/shape/sppr/transform/` | P2 | TODO | `sppr_transform_rewriter_plan.md` |
| 128 | `moffice.drawing.dmlr.handler.shape.style` | 1 | `src/drawing/dmlr/handler/shape/style/` | P2 | TODO | `dmlr_shape_style_rewriter_plan.md` |
| 129 | `moffice.drawing.dmlr.handler.shape.textshape` | 1 | `src/drawing/dmlr/handler/shape/textshape/` | P2 | TODO | `textshape_rewriter_plan.md` |
| 130 | `moffice.drawing.dmlr.handler.shape.textshape.text` | 5 | `src/drawing/dmlr/handler/shape/textshape/text/` | P2 | TODO | `textshape_text_rewriter_plan.md` |
| 131 | `moffice.drawing.dmlr.handler.shape.textshape.text.paragraph` | 5 | `src/drawing/dmlr/handler/shape/textshape/text/paragraph/` | P2 | TODO | `paragraph_rewriter_plan.md` |
| 132 | `moffice.drawing.dmlr.handler.shape.textshape.text.run` | 1 | `src/drawing/dmlr/handler/shape/textshape/text/run/` | P2 | TODO | `run_rewriter_plan.md` |
| 133 | `moffice.drawing.dmlr.handler.style` | 10 | `src/drawing/dmlr/handler/style/` | P2 | TODO | `dmlr_style_rewriter_plan.md` |
| 134 | `moffice.drawing.dmlr.handler.style.list` | 3 | `src/drawing/dmlr/handler/style/list/` | P2 | TODO | `style_list_rewriter_plan.md` |
| 135 | `moffice.drawing.dmlr.handler.table` | 2 | `src/drawing/dmlr/handler/table/` | P2 | TODO | `dmlr_table_rewriter_plan.md` |
| 136 | `moffice.drawing.dmlr.model` | 1 | `src/drawing/dmlr/model/` | P2 | TODO | `dmlr_model_rewriter_plan.md` |
| 137 | `moffice.drawing.dmlr.util` | 2 | `src/drawing/dmlr/util/` | P2 | TODO | `dmlr_util_rewriter_plan.md` |
| 138-156 | `moffice.drawing.dmlw.*` (19 dirs) | ~52 | `src/drawing/dmlw/*/` | P4 | TODO | (各 dmlw_*_plan.md) |

#### 2.1.7 moffice.spreadsheet — 网格控件 (168 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 157 | `spreadsheet.control.grid` (root) | 4 | `src/grid/` | P2 | TODO | |
| 158 | `spreadsheet.control.grid.core` | 9 | `src/grid/core/` | P2 | TODO | |
| 159 | `spreadsheet.control.grid.dataaction` | 1 | `src/grid/dataaction/` | P4 | TODO | |
| 160 | `spreadsheet.control.grid.framework` | 1 | `src/grid/framework/` | P2 | TODO | |
| 161 | `spreadsheet.control.grid.layout` | 7 | `src/grid/layout/` | P2 | TODO | |
| 162 | `spreadsheet.control.grid.layout.allocator` | 1 | `src/grid/layout/allocator/` | P2 | TODO | |
| 163 | `spreadsheet.control.grid.layout.cache` | 5 | `src/grid/layout/cache/` | P2 | TODO | |
| 164 | `spreadsheet.control.grid.layout.data` | 7 | `src/grid/layout/data/` | P2 | TODO | |
| 165 | `spreadsheet.control.grid.layout.layouter` | 1 | `src/grid/layout/layouter/` | P2 | TODO | |
| 166 | `spreadsheet.control.grid.layout.measure` | 1 | `src/grid/layout/measure/` | P2 | TODO | |
| 167 | `spreadsheet.control.grid.layout.typo` | 10 | `src/grid/layout/typo/` | P2 | TODO | |
| 168 | `spreadsheet.control.grid.piper` | 14 | `src/grid/piper/` | P2 | TODO | |
| 169 | `spreadsheet.control.grid.piper.block` | 7 | `src/grid/piper/block/` | P2 | TODO | |
| 170 | `spreadsheet.control.grid.piper.device` | 5 | `src/grid/piper/device/` | P2 | TODO | |
| 171 | `spreadsheet.control.grid.render` | 2 | `src/grid/render/` | P2 | TODO | |
| 172 | `spreadsheet.control.grid.render.animation` | 11 | `src/grid/render/animation/` | P4 | TODO | |
| 173 | `spreadsheet.control.grid.render.animation.interpolator` | 2 | `src/grid/render/animation/interpolator/` | P4 | TODO | |
| 174 | `spreadsheet.control.grid.render.cache` | 3 | `src/grid/render/cache/` | P2 | TODO | |
| 175 | `spreadsheet.control.grid.render.data` | 14 | `src/grid/render/data/` | P2 | TODO | |
| 176 | `spreadsheet.control.grid.render.layer` | 3 | `src/grid/render/layer/` | P2 | TODO | |
| 177 | `spreadsheet.control.grid.render.layer.celllayer` | 2 | `src/grid/render/layer/celllayer/` | P2 | TODO | |
| 178 | `spreadsheet.control.grid.render.layer.celllayer.drawer` | 12 | `src/grid/render/layer/celllayer/drawer/` | P2 | TODO | |
| 179 | `spreadsheet.control.grid.render.layer.objectlayer` | 2 | `src/grid/render/layer/objectlayer/` | P2 | TODO | |
| 180 | `spreadsheet.control.grid.render.layer.objectlayer.drawer` | 5 | `src/grid/render/layer/objectlayer/drawer/` | P2 | TODO | |
| 181 | `spreadsheet.control.grid.render.layer.sheetlayer` | 3 | `src/grid/render/layer/sheetlayer/` | P2 | TODO | |
| 182 | `spreadsheet.control.grid.render.layer.sheetlayer.drawer` | 6 | `src/grid/render/layer/sheetlayer/drawer/` | P2 | TODO | |
| 183 | `spreadsheet.control.grid.render.normal` | 5 | `src/grid/render/normal/` | P2 | TODO | |
| 184 | `spreadsheet.control.grid.render.print` | 12 | `src/grid/render/print/` | P2 | TODO | |
| 185 | `spreadsheet.control.grid.render.print.proto` | 1 | `src/grid/render/print/proto/` | P2 | TODO | |
| 186 | `spreadsheet.control.grid.service` | 1 | `src/grid/service/` | P3 | TODO | |
| 187 | `spreadsheet.control.grid.theme` | 3 | `src/grid/theme/` | P2 | TODO | |
| 188 | `spreadsheet.control.grid.tools` | 15 | `src/grid/tools/` | P2 | TODO | |
| 189 | `spreadsheet.control.uil` | 1 | `src/grid/uil/` | P4 | TODO | |
| 190 | `spreadsheet.control.uil.drawable` | 7 | `src/grid/uil/drawable/` | P4 | TODO | |
| 191 | `spreadsheet.utils` | 5 | `src/spreadsheet_utils/` | P2 | TODO | |

#### 2.1.8 其他 moffice 子模块 (37 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 | 计划文件 |
|---|:-------------------|----------:|:---------|:------:|:----:|:--------|
| 192 | `moffice.kso` (root) | 1 | `src/kso/` | P1 | TODO | |
| 193 | `moffice.kso.filter` | 7 | `src/kso/filter/` | P4 | TODO | |
| 194 | `moffice.kso.filter.impl` | 1 | `src/kso/filter/impl/` | P4 | TODO | |
| 195 | `moffice.kso.framework` | 1 | `src/kso/framework/` | P4 | TODO | |
| 196 | `moffice.kso.kproto` | 4 | `src/kso/kproto/` | P1 | TODO | |
| 197 | `moffice.kso.kproto.internal` | 6 | `src/kso/kproto/internal/` | P1 | TODO | |
| 198 | `moffice.kso.kproto.memory` | 3 | `src/kso/kproto/memory/` | P1 | TODO | |
| 199 | `moffice.kso.kproto.util` | 6 | `src/kso/kproto/util/` | P1 | TODO | |
| 200 | `moffice.kso.msgbox` | 2 | `src/kso/msgbox/` | P4 | TODO | |
| 201 | `moffice.kso.unit` | 1 | `src/kso/unit/` | P1 | TODO | |
| 202 | `moffice.pool` | 1 | `src/pool/` | P1 | TODO | |
| 203 | `moffice.print` | 3 | `src/print/` | P4 | TODO | |
| 204 | `moffice.service.doc` | 1 | `src/service/doc/` | P3 | TODO | |
| 205 | `moffice.text.layout` | 1 | `src/text/layout/` | P2 | TODO | |
| 206 | `moffice.text.layout.typo` | 1 | `src/text/layout/typo/` | P2 | TODO | |
| 207 | `moffice.util` | 1 | `src/moffice_util/` | P1 | TODO | |

#### 2.1.9 其他模块 (43 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 |
|---|:-------------------|----------:|:---------|:------:|:----:|
| 208 | `base.io.css` | 9 | `src/base/io/css/` | P4 | TODO |
| 209 | `base.io.css.model` | 2 | `src/base/io/css/model/` | P4 | TODO |
| 210 | `base.io.css.utils` | 1 | `src/base/io/css/utils/` | P4 | TODO |
| 211 | `coop.net` | 1 | `src/coop/net/` | P4 | TODO |
| 212 | `io.file` | 1 | `src/io_file/` | P4 | TODO |
| 213 | `io.file.parser` | 1 | `src/io_file/parser/` | P4 | TODO |
| 214 | `sparkline` (root) | 6 | `src/sparkline/` | P4 | TODO |
| 215 | `sparkline.app` | 4 | `src/sparkline/app/` | P4 | TODO |
| 216 | `sparkline.base` | 4 | `src/sparkline/base/` | P4 | TODO |
| 217 | `sparkline.constant` | 3 | `src/sparkline/constant/` | P4 | TODO |
| 218 | `sparkline.data` | 2 | `src/sparkline/data/` | P4 | TODO |
| 219 | `sparkline.datasource` | 1 | `src/sparkline/datasource/` | P4 | TODO |
| 220 | `sparkline.layout` | 3 | `src/sparkline/layout/` | P4 | TODO |
| 221 | `sparkline.layout.data` | 1 | `src/sparkline/layout/data/` | P4 | TODO |
| 222 | `sparkline.reader` | 6 | `src/sparkline/reader/` | P4 | TODO |
| 223 | `sparkline.service` | 1 | `src/sparkline/service/` | P4 | TODO |
| 224 | `sparkline.utils` | 3 | `src/sparkline/utils/` | P4 | TODO |
| 225 | `sparkline.writer` | 5 | `src/sparkline/writer/` | P4 | TODO |
| 226 | `org.apache.poi.ss` | 1 | `src/poi/ss/` | P4 | TODO |
| 227 | `org.apache.poi.ss.util` | 10 | `src/poi/ss/util/` | P1 | TODO |
| 228 | `org.apache.poi.ss.usermodel` | 2 | `src/poi/ss/usermodel/` | P1 | TODO |
| 229 | `org.apache.poi.openxml.usermodel.vml.impl` | 2 | `src/poi/openxml/vml/` | P2 | TODO |
| 230 | `org.mozilla.universalchardet` | 3 | `src/chardet/` | P4 | TODO |

---

### 2.2 `koffice/et/et-core-io/src` — 1062 Java files

> 格式读写 (XLSX/XLS/XLSB/CSV/HTML/SXML)

#### 2.2.1 xlsxr — XLSX Reader (297 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 |
|---|:-------------------|----------:|:---------|:------:|:----:|
| 1 | `xlsxr` (root) | 4 | `src/io/xlsxr/` | P2 | TODO |
| 2 | `xlsxr.docProps` | 3 | `src/io/xlsxr/doc_props/` | P2 | TODO |
| 3 | `xlsxr.handler` | 7 | `src/io/xlsxr/handler/` | P2 | TODO |
| 4 | `xlsxr.handler.book` | 4 | `src/io/xlsxr/handler/book/` | P2 | TODO |
| 5 | `xlsxr.handler.comment` | 6 | `src/io/xlsxr/handler/comment/` | P2 | TODO |
| 6 | `xlsxr.handler.customXml` | 10 | `src/io/xlsxr/handler/custom_xml/` | P2 | TODO |
| 7 | `xlsxr.handler.diagrams` | 1 | `src/io/xlsxr/handler/diagrams/` | P4 | TODO |
| 8 | `xlsxr.handler.docProps` | 1 | `src/io/xlsxr/handler/doc_props/` | P2 | TODO |
| 9 | `xlsxr.handler.drawings` | 2 | `src/io/xlsxr/handler/drawings/` | P2 | TODO |
| 10 | `xlsxr.handler.drawings.VmlDrawing` | 10 | `src/io/xlsxr/handler/drawings/vml_drawing/` | P2 | TODO |
| 11 | `xlsxr.handler.drawings.drawing` | 5 | `src/io/xlsxr/handler/drawings/drawing/` | P2 | TODO |
| 12 | `xlsxr.handler.drawings.drawing.Anchor` | 2 | `src/io/xlsxr/handler/drawings/drawing/anchor/` | P2 | TODO |
| 13 | `xlsxr.handler.drawings.drawing.shape` | 8 | `src/io/xlsxr/handler/drawings/drawing/shape/` | P2 | TODO |
| 14 | `xlsxr.handler.drawings.drawing.shape.nv` | 5 | `src/io/xlsxr/handler/drawings/drawing/shape/nv/` | P2 | TODO |
| 15 | `xlsxr.handler.drawings.drawing.shape.spPr.*` (8 dirs) | 12 | `src/io/xlsxr/handler/drawings/drawing/shape/sp_pr/*/` | P2 | TODO |
| 16 | `xlsxr.handler.drawings.drawing.shape.style` | 1 | `src/io/xlsxr/handler/drawings/drawing/shape/style/` | P2 | TODO |
| 17 | `xlsxr.handler.drawings.drawing.shape.txBody` | 8 | `src/io/xlsxr/handler/drawings/drawing/shape/tx_body/` | P2 | TODO |
| 18 | `xlsxr.handler.externalLink` | 5 | `src/io/xlsxr/handler/external_link/` | P2 | TODO |
| 19 | `xlsxr.handler.ole` | 6 | `src/io/xlsxr/handler/ole/` | P2 | TODO |
| 20 | `xlsxr.handler.pivot` | 23 | `src/io/xlsxr/handler/pivot/` | P4 | TODO |
| 21 | `xlsxr.handler.property.color` | 14 | `src/io/xlsxr/handler/property/color/` | P2 | TODO |
| 22 | `xlsxr.handler.property.fill` | 4 | `src/io/xlsxr/handler/property/fill/` | P2 | TODO |
| 23 | `xlsxr.handler.sheet` | 42 | `src/io/xlsxr/handler/sheet/` | P2 | TODO |
| 24 | `xlsxr.handler.sheet.autoFilter` | 14 | `src/io/xlsxr/handler/sheet/auto_filter/` | P2 | TODO |
| 25 | `xlsxr.handler.sheet.condfmt` | 5 | `src/io/xlsxr/handler/sheet/condfmt/` | P2 | TODO |
| 26 | `xlsxr.handler.sheet.table` | 8 | `src/io/xlsxr/handler/sheet/table/` | P2 | TODO |
| 27 | `xlsxr.handler.style` | 16 | `src/io/xlsxr/handler/style/` | P2 | TODO |
| 28 | `xlsxr.handler.theme` | 3 | `src/io/xlsxr/handler/theme/` | P2 | TODO |
| 29 | `xlsxr.handler.theme.color` | 4 | `src/io/xlsxr/handler/theme/color/` | P2 | TODO |
| 30 | `xlsxr.model` | 11 | `src/io/xlsxr/model/` | P2 | TODO |
| 31 | `xlsxr.util` | 11 | `src/io/xlsxr/util/` | P2 | TODO |
| 32 | `xlsxr.util.datatype` | 6 | `src/io/xlsxr/util/datatype/` | P2 | TODO |
| 33 | `xlsxr.xl` (root) | 10 | `src/io/xlsxr/xl/` | P2 | TODO |
| 34 | `xlsxr.xl.cellimages` | 1 | `src/io/xlsxr/xl/cellimages/` | P2 | TODO |
| 35 | `xlsxr.xl.charts` | 3 | `src/io/xlsxr/xl/charts/` | P2 | TODO |
| 36 | `xlsxr.xl.customXml` | 3 | `src/io/xlsxr/xl/custom_xml/` | P2 | TODO |
| 37 | `xlsxr.xl.diagrams` | 1 | `src/io/xlsxr/xl/diagrams/` | P4 | TODO |
| 38 | `xlsxr.xl.drawings` | 3 | `src/io/xlsxr/xl/drawings/` | P2 | TODO |
| 39 | `xlsxr.xl.externalLinks` | 1 | `src/io/xlsxr/xl/external_links/` | P2 | TODO |
| 40 | `xlsxr.xl.ink` | 1 | `src/io/xlsxr/xl/ink/` | P4 | TODO |
| 41 | `xlsxr.xl.media` | 2 | `src/io/xlsxr/xl/media/` | P2 | TODO |
| 42 | `xlsxr.xl.theme` | 2 | `src/io/xlsxr/xl/theme/` | P2 | TODO |
| 43 | `xlsxr.xl.webExtensions` | 1 | `src/io/xlsxr/xl/web_extensions/` | P4 | TODO |
| 44 | `xlsxr.xl.worksheets` | 5 | `src/io/xlsxr/xl/worksheets/` | P2 | TODO |
| 45 | `xlsxr.xl.worksheets.block` | 4 | `src/io/xlsxr/xl/worksheets/block/` | P2 | TODO |

#### 2.2.2 filefmt.biff8 — XLS Binary Format (365 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 |
|---|:-------------------|----------:|:---------|:------:|:----:|
| 46 | `filefmt.biff8` (root) | 1 | `src/io/filefmt/biff8/` | P4 | TODO |
| 47 | `filefmt.biff8.dev` | 1 | `src/io/filefmt/biff8/dev/` | P4 | TODO |
| 48 | `filefmt.biff8.record` | 155 | `src/io/filefmt/biff8/record/` | P4 | TODO |
| 49 | `filefmt.biff8.record.cf` | 18 | `src/io/filefmt/biff8/record/cf/` | P4 | TODO |
| 50 | `filefmt.biff8.record.common` | 13 | `src/io/filefmt/biff8/record/common/` | P4 | TODO |
| 51 | `filefmt.biff8.record.cont` | 4 | `src/io/filefmt/biff8/record/cont/` | P4 | TODO |
| 52 | `filefmt.biff8.record.drawing` | 2 | `src/io/filefmt/biff8/record/drawing/` | P4 | TODO |
| 53 | `filefmt.biff8.record.dxf` | 16 | `src/io/filefmt/biff8/record/dxf/` | P4 | TODO |
| 54 | `filefmt.biff8.record.etextdata` | 3 | `src/io/filefmt/biff8/record/etextdata/` | P4 | TODO |
| 55 | `filefmt.biff8.record.pivottable` | 28 | `src/io/filefmt/biff8/record/pivottable/` | P4 | TODO |
| 56 | `filefmt.biff8.record.pivottable.cache` | 18 | `src/io/filefmt/biff8/record/pivottable/cache/` | P4 | TODO |
| 57 | `filefmt.biff8.record.pivottable.cache.define` | 9 | `src/io/filefmt/biff8/record/pivottable/cache/define/` | P4 | TODO |
| 58 | `filefmt.biff8.record.pivottable.common` | 7 | `src/io/filefmt/biff8/record/pivottable/common/` | P4 | TODO |
| 59 | `filefmt.biff8.record.pivottable.sxds` | 51 | `src/io/filefmt/biff8/record/pivottable/sxds/` | P4 | TODO |
| 60 | `filefmt.biff8.record.sort` | 6 | `src/io/filefmt/biff8/record/sort/` | P4 | TODO |
| 61 | `filefmt.biff8.record.table` | 16 | `src/io/filefmt/biff8/record/table/` | P4 | TODO |
| 62 | `filefmt.biff8.record.table.unsupport` | 6 | `src/io/filefmt/biff8/record/table/unsupport/` | P4 | TODO |
| 63 | `filefmt.biff8.record.tableStyles` | 3 | `src/io/filefmt/biff8/record/table_styles/` | P4 | TODO |
| 64 | `filefmt.biff8.util` | 8 | `src/io/filefmt/biff8/util/` | P4 | TODO |

#### 2.2.3 其他 IO 模块 (400 files)

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 |
|---|:-------------------|----------:|:---------|:------:|:----:|
| 65 | `xlsr.*` (XLS reader, 12 dirs) | ~67 | `src/io/xlsr/*/` | P4 | TODO |
| 66 | `xlsw.*` (XLS writer, 12 dirs) | ~40 | `src/io/xlsw/*/` | P4 | TODO |
| 67 | `xlsxw.*` (XLSX writer, 20+ dirs) | ~93 | `src/io/xlsxw/*/` | P4 | TODO |
| 68 | `xlsbr.*` (XLSB reader, 5 dirs) | ~21 | `src/io/xlsbr/*/` | P4 | TODO |
| 69 | `htmlr.*` (HTML reader, 9 dirs) | ~69 | `src/io/htmlr/*/` | P4 | TODO |
| 70 | `sxmlr.*` (SXML reader, 8 dirs) | ~34 | `src/io/sxmlr/*/` | P4 | TODO |
| 71 | `sxmlw` | 7 | `src/io/sxmlw/` | P4 | TODO |
| 72 | `plainr.*` (plain reader, 3 dirs) | ~16 | `src/io/plainr/*/` | P4 | TODO |
| 73 | `plainw` | 2 | `src/io/plainw/` | P4 | TODO |
| 74 | `gvmlr.*` (GVM reader) | ~5 | `src/io/gvmlr/*/` | P4 | TODO |
| 75 | `gvmlw` | 3 | `src/io/gvmlw/` | P4 | TODO |
| 76 | `crtx` | 2 | `src/io/crtx/` | P4 | TODO |

---

### 2.3 `koffice/et/et-core-chart/src` — 796 Java files

> 图表引擎

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 |
|---|:-------------------|----------:|:---------|:------:|:----:|
| 1 | `chart` (root) | 2 | `src/chart/` | P1 | TODO |
| 2 | `chart.data` (root) | 34 | `src/chart/data/` | P1 | TODO |
| 3 | `chart.data.axis` | 3 | `src/chart/data/axis/` | P1 | TODO |
| 4 | `chart.data.base` | 7 | `src/chart/data/base/` | P1 | TODO |
| 5 | `chart.data.chart` | 7 | `src/chart/data/chart/` | P1 | TODO |
| 6 | `chart.data.chartspace` | 3 | `src/chart/data/chartspace/` | P1 | TODO |
| 7 | `chart.data.d3` | 2 | `src/chart/data/d3/` | P1 | TODO |
| 8 | `chart.data.dispunits` | 2 | `src/chart/data/dispunits/` | P1 | TODO |
| 9 | `chart.data.label` | 5 | `src/chart/data/label/` | P1 | TODO |
| 10 | `chart.data.layout` | 2 | `src/chart/data/layout/` | P1 | TODO |
| 11 | `chart.data.legend` | 2 | `src/chart/data/legend/` | P1 | TODO |
| 12 | `chart.data.pivot` | 3 | `src/chart/data/pivot/` | P1 | TODO |
| 13 | `chart.data.print` | 4 | `src/chart/data/print/` | P1 | TODO |
| 14 | `chart.data.series` | 7 | `src/chart/data/series/` | P1 | TODO |
| 15 | `chart.data.shape` | 4 | `src/chart/data/shape/` | P1 | TODO |
| 16 | `chart.data.source` | 10 | `src/chart/data/source/` | P1 | TODO |
| 17 | `chart.data.str` | 7 | `src/chart/data/str/` | P1 | TODO |
| 18 | `chart.data.trend` | 2 | `src/chart/data/trend/` | P1 | TODO |
| 19 | `chart.msenum` | 4 | `src/chart/msenum/` | P1 | TODO |
| 20 | `chart.common` | 2 | `src/chart/common/` | P1 | TODO |
| 21 | `chart.store` | 4 | `src/chart/store/` | P1 | TODO |
| 22 | `chart.objpool` | 5 | `src/chart/objpool/` | P1 | TODO |
| 23 | `chart.style` | 9 | `src/chart/style/` | P1 | TODO |
| 24 | `chart.format` | 1 | `src/chart/format/` | P1 | TODO |
| 25 | `chart.description` | 3 | `src/chart/description/` | P1 | TODO |
| 26 | `chart.event` | 2 | `src/chart/event/` | P1 | TODO |
| 27 | `chart.datasource` | 13 | `src/chart/datasource/` | P2 | TODO |
| 28 | `chart.io` (root) | 1 | `src/chart/io/` | P2 | TODO |
| 29 | `chart.creator` | 3 | `src/chart/creator/` | P2 | TODO |
| 30 | `chart.amend` | 3 | `src/chart/amend/` | P4 | TODO |
| 31-40 | `chart.app.*` (10 dirs) | ~41 | `src/chart/app/*/` | P2 | TODO |
| 41-55 | `chart.chartxml.reader.*` (15 dirs) | ~92 | `src/chart/chartxml/reader/*/` | P2 | TODO |
| 56-70 | `chart.chartxml.writer.*` (15 dirs) | ~88 | `src/chart/chartxml/writer/*/` | P4 | TODO |
| 71-78 | `chart.chartole.*` (8 dirs) | ~177 | `src/chart/chartole/*/` | P4 | TODO |
| 79 | `chart.text` | 16 | `src/chart/text/` | P2 | TODO |
| 80 | `chart.typo` (root) | 13 | `src/chart/typo/` | P2 | TODO |
| 81 | `chart.typo.data` | 9 | `src/chart/typo/data/` | P2 | TODO |
| 82 | `chart.typo.service` | 2 | `src/chart/typo/service/` | P2 | TODO |
| 83-88 | `chart.render.*` (6 dirs) | ~50 | `src/chart/render/*/` | P2 | TODO |
| 89 | `chart.service` | 4 | `src/chart/service/` | P3 | TODO |
| 90 | `chart.util` | 11 | `src/chart/util/` | P2 | TODO |
| 91 | `chart.utils` | 14 | `src/chart/utils/` | P2 | TODO |

---

### 2.4 `koffice/et/et-core-formula/src` — 431 Java files

> 公式引擎

| # | Java 包 (叶子目录) | Java 文件 | Rust 目标 | 优先级 | 状态 |
|---|:-------------------|----------:|:---------|:------:|:----:|
| 1 | `ss.constant` | 7 | `src/formula/constant/` | P1 | TODO |
| 2 | `ss.interfaces` | 11 | `src/formula/interfaces/` | P1 | TODO |
| 3 | `ss.formula` (root) | 11 | `src/formula/` | P1 | TODO |
| 4 | `ss.formula.constant` | 2 | `src/formula/formula_constant/` | P1 | TODO |
| 5 | `ss.formula.ptg` | 76 | `src/formula/ptg/` | P1 | TODO |
| 6 | `ss.formula.ptg.external` | 6 | `src/formula/ptg/external/` | P1 | TODO |
| 7 | `ss.formula.parser` | 5 | `src/formula/parser/` | P1 | TODO |
| 8 | `ss.formula.parser.rav` | 2 | `src/formula/parser/rav/` | P1 | TODO |
| 9 | `ss.formula.eval` | 18 | `src/formula/eval/` | P1 | TODO |
| 10 | `ss.formula.eval.advance` | 3 | `src/formula/eval/advance/` | P1 | TODO |
| 11 | `ss.formula.evalcache` | 7 | `src/formula/evalcache/` | P1 | TODO |
| 12 | `ss.formula.evaluator` | 9 | `src/formula/evaluator/` | P1 | TODO |
| 13 | `ss.formula.function` | 3 | `src/formula/function/` | P1 | TODO |
| 14 | `ss.formula.functions` (root) | 56 | `src/formula/functions/` | P1 | TODO |
| 15 | `ss.formula.functions.base` | 4 | `src/formula/functions/base/` | P1 | TODO |
| 16 | `ss.formula.functions.builtin` | 8 | `src/formula/functions/builtin/` | P1 | TODO |
| 17 | `ss.formula.functions.calendar` | 9 | `src/formula/functions/calendar/` | P1 | TODO |
| 18 | `ss.formula.functions.db` | 4 | `src/formula/functions/db/` | P1 | TODO |
| 19 | `ss.formula.functions.financial` | 14 | `src/formula/functions/financial/` | P1 | TODO |
| 20 | `ss.formula.functions.matrix` | 5 | `src/formula/functions/matrix/` | P1 | TODO |
| 21 | `ss.formula.functions.statistics` | 38 | `src/formula/functions/statistics/` | P1 | TODO |
| 22 | `ss.formula.functions.util` | 4 | `src/formula/functions/util/` | P1 | TODO |
| 23 | `ss.formula.functions.util.db` | 4 | `src/formula/functions/util/db/` | P1 | TODO |
| 24 | `ss.formula.functions.util.financial` | 2 | `src/formula/functions/util/financial/` | P1 | TODO |
| 25 | `ss.formula.functions.util.iteration` | 11 | `src/formula/functions/util/iteration/` | P1 | TODO |
| 26 | `ss.formula.functions.util.matcher` | 4 | `src/formula/functions/util/matcher/` | P1 | TODO |
| 27 | `ss.formula.functions.util.numbercollector` | 5 | `src/formula/functions/util/numbercollector/` | P1 | TODO |
| 28 | `ss.formula.functions.util.statistics` | 3 | `src/formula/functions/util/statistics/` | P1 | TODO |
| 29 | `ss.formula.functions.util.statistics.toms708` | 3 | `src/formula/functions/util/statistics/toms708/` | P1 | TODO |
| 30 | `ss.formula.renderer` | 2 | `src/formula/renderer/` | P2 | TODO |
| 31 | `ss.formula.udf` | 2 | `src/formula/udf/` | P4 | TODO |
| 32 | `ss.formula.util` | 2 | `src/formula/util/` | P1 | TODO |
| 33 | `ss.formula.atp` | 36 | `src/formula/atp/` | P1 | TODO |
| 34 | `ss.calcchain` | 6 | `src/formula/calcchain/` | P1 | TODO |
| 35 | `ss.calcchain.listener` | 3 | `src/formula/calcchain/listener/` | P1 | TODO |
| 36 | `ss.calcchain.marktype` | 7 | `src/formula/calcchain/marktype/` | P1 | TODO |
| 37 | `ss.calcchain.threadpool` | 1 | `src/formula/calcchain/threadpool/` | P1 | TODO |
| 38 | `ss.dependent` | 3 | `src/formula/dependent/` | P1 | TODO |
| 39 | `ss.dependent.refer` | 3 | `src/formula/dependent/refer/` | P1 | TODO |
| 40 | `ss.dependent.refer.base` | 3 | `src/formula/dependent/refer/base/` | P1 | TODO |
| 41 | `ss.statemachine` | 3 | `src/formula/statemachine/` | P4 | TODO |
| 42 | `ss.manual` | 7 | `src/formula/manual/` | P4 | TODO |
| 43 | `ss.multlang` | 1 | `src/formula/multlang/` | P1 | TODO |
| 44 | `ss.multlang.res` | 5 | `src/formula/multlang/res/` | P1 | TODO |
| 45 | `ss.debugtool` | 2 | `src/formula/debugtool/` | P4 | TODO |
| 46 | `ss.util` | 4 | `src/formula/ss_util/` | P1 | TODO |
| 47 | `ss.util.indexmap` | 2 | `src/formula/ss_util/indexmap/` | P1 | TODO |
| 48 | `ss.util.level` | 3 | `src/formula/ss_util/level/` | P1 | TODO |
| 49 | `ss.util.primeval` | 0 | - | - | - |
| 50 | `ss.util.primeval.objmap` | 2 | `src/formula/ss_util/primeval/objmap/` | P1 | TODO |

---

## 3. 功能优先级分类

| 优先级 | 功能分类 | 对应目录范围 | 说明 |
|:------:|---------|------------|------|
| P1 | 基础数据模型 | `moss.core.*`, `moss.util.*`, `moss.framework`, `moss.engine.*`, `drawing.data.*`, `chart.data.*`, `formula.*`, `poi.*` | 前置依赖，所有模块都依赖 |
| P2 | 核心渲染管线 | `xlsxr.*` (IO reader), `grid.layout.*`, `grid.render.*`, `grid.piper.*`, `chart.render.*`, `chart.typo.*`, `drawing.dmlr.*`, `moss.io.*` | IO→解析→Layout→RenderCommand |
| P3 | HitTest 等基础功能 | `grid.service`, `chart.service`, `moffice.service.*` | 交互定位能力 |
| P4 | 其他功能 | `moss.app.*` (大量应用逻辑), `filefmt.biff8.*` (XLS format), `xlsxw.*` (写入), `dmlw.*` (writing), `sparkline.*`, `htmlr.*` 等 | 编辑/导出/协同等辅助功能 |

---

## 4. 依赖关系

```
formula (ptg/eval/functions) ──┐
                                ↓
moss.core (book/sheet/table) → moss.app (API) → grid.layout → grid.render
         ↑                                           ↑
moss.engine (grid/eval/linktb) ──────────────────────┘
         ↑
moss.util (data structures)
         ↑
drawing.data → drawing.dmlr (IO) ──→ grid.render.layer.objectlayer
         ↑
chart.data → chart.chartxml.reader → chart.render → grid.render
         ↑
io.xlsxr (XLSX reader) ─────→ moss.core (build model)
```

---

## 5. 分阶段实施计划

### Phase 1: 基础数据模型 (P1) — ~680 files

> 所有模块的数据结构、常量、接口定义

| 批次 | 目录 | Java 文件 | 状态 |
|------|------|----------:|:----:|
| 1.1 | `moss.util` | 28 | ✅ 完成 (442✅/0❌/449⚠️) |
| 1.2 | `moss.util.binarytree` | 3 | ✅ 完成 (55✅/0❌/17⚠️) |
| 1.3 | `moss.util.binarytree.node` | 3 | ✅ 完成 (27✅/0❌/0⚠️) |
| 1.4 | `moss.util.cache` | 2 | ✅ 完成 (44✅/0❌/20⚠️) |
| 1.5 | `moss.util.checksum` | 4 | ✅ 完成 (371✅/0❌/9⚠️) |
| 1.6 | `moss.util.deqlist` | 4 | ✅ 完成 (89✅/0❌/55⚠️) |
| 1.7 | `moss.util.objpool` | 4 | ✅ 完成 (38✅/0❌/10⚠️) |
| 1.8 | `moss.util.primitive` | 3 | ✅ 完成 (38✅/0❌/4⚠️) |
| 1.9 | `moss.util.rangelist` | 7 | ✅ 完成 (120✅/0❌/142⚠️) |
| 1.10 | `moss` (root) | 3 | ✅ 完成 (70✅/0❌/16⚠️) |
| 1.11 | `moss.framework` | 14 | ✅ 完成 (238✅/0❌/127⚠️) |
| 1.12 | `moss.engine.grid` | 32 | ✅ DONE |
| 1.13 | `moss.engine.grid.griditerator` | 3 | ✅ 完成 (57✅/0❌/20⚠️) |
| 1.14 | `moss.engine.eval` | 14 | TODO |
| 1.15 | `moss.engine.eval.sup` | 3 | TODO |
| 1.16 | `moss.engine.linktb` | 11 | TODO |
| 1.17 | `moss.engine.areaevent` | 3 | TODO |
| 1.18 | `moss.core.book` | 13 | TODO |
| 1.19 | `moss.core.sheet` | 18 | TODO |
| 1.20 | `moss.core.table` | 2 | TODO |
| 1.21 | `moss.core.theme` | 11 | TODO |
| 1.22 | `moss.core.drawing` | 34 | TODO |
| 1.23 | `moss.core.drawing.tools` | 8 | TODO |
| 1.24 | `moss.app` (root) | 40 | TODO |
| 1.25 | `moss.app.event` | 8 | TODO |
| 1.26 | `moss.app.except` | 15 | TODO |
| 1.27 | `moss.app.xf` | 11 | TODO |
| 1.28 | `moffice.drawing.data.basic` | 9 | TODO |
| 1.29 | `moffice.drawing.data.blip` | 1 | TODO |
| 1.30 | `moffice.drawing.data.color` | 3 | TODO |
| 1.31 | `moffice.drawing.data.effect` | 30 | TODO |
| 1.32 | `moffice.drawing.data.fill` | 9 | TODO |
| 1.33 | `moffice.drawing.data.font` | 1 | TODO |
| 1.34 | `moffice.drawing.data.geometry` | 7 | TODO |
| 1.35 | `moffice.drawing.data.hyperlink` | 1 | TODO |
| 1.36 | `moffice.drawing.data.media` | 1 | TODO |
| 1.37 | `moffice.drawing.data.outline` | 1 | TODO |
| 1.38 | `moffice.drawing.data.shape` | 3 | TODO |
| 1.39 | `moffice.drawing.data.style` | 10 | TODO |
| 1.40 | `moffice.drawing.data.table` | 11 | TODO |
| 1.41 | `moffice.drawing.data.text` | 8 | TODO |
| 1.42 | `moffice.drawing.data.threeD` | 9 | TODO |
| 1.43 | `moffice.drawing` (root) | 1 | TODO |
| 1.44 | `moffice.kso.kproto` | 4 | TODO |
| 1.45 | `moffice.kso.kproto.internal` | 6 | TODO |
| 1.46 | `moffice.kso.kproto.memory` | 3 | TODO |
| 1.47 | `moffice.kso.kproto.util` | 6 | TODO |
| 1.48 | `moffice.kso` (root) | 1 | TODO |
| 1.49 | `moffice.kso.unit` | 1 | TODO |
| 1.50 | `moffice.pool` | 1 | TODO |
| 1.51 | `moffice.util` | 1 | TODO |
| 1.52 | `poi.ss` | 1 | TODO |
| 1.53 | `poi.ss.util` | 10 | TODO |
| 1.54 | `poi.ss.usermodel` | 2 | TODO |
| 1.55 | `chart` (root) | 2 | TODO |
| 1.56 | `chart.data` (root) | 34 | TODO |
| 1.57 | `chart.data.axis` | 3 | TODO |
| 1.58 | `chart.data.base` | 7 | TODO |
| 1.59 | `chart.data.chart` | 7 | TODO |
| 1.60 | `chart.data.chartspace` | 3 | TODO |
| 1.61 | `chart.data.d3` | 2 | TODO |
| 1.62 | `chart.data.dispunits` | 2 | TODO |
| 1.63 | `chart.data.label` | 5 | TODO |
| 1.64 | `chart.data.layout` | 2 | TODO |
| 1.65 | `chart.data.legend` | 2 | TODO |
| 1.66 | `chart.data.pivot` | 3 | TODO |
| 1.67 | `chart.data.print` | 4 | TODO |
| 1.68 | `chart.data.series` | 7 | TODO |
| 1.69 | `chart.data.shape` | 4 | TODO |
| 1.70 | `chart.data.source` | 10 | TODO |
| 1.71 | `chart.data.str` | 7 | TODO |
| 1.72 | `chart.data.trend` | 2 | TODO |
| 1.73 | `chart.msenum` | 4 | TODO |
| 1.74 | `chart.common` | 2 | TODO |
| 1.75 | `chart.store` | 4 | TODO |
| 1.76 | `chart.objpool` | 5 | TODO |
| 1.77 | `chart.style` | 9 | TODO |
| 1.78 | `chart.format` | 1 | TODO |
| 1.79 | `chart.description` | 3 | TODO |
| 1.80 | `chart.event` | 2 | TODO |
| 1.81 | `ss.constant` | 7 | TODO |
| 1.82 | `ss.interfaces` | 11 | TODO |
| 1.83 | `ss.formula` (root) | 11 | TODO |
| 1.84 | `ss.formula.constant` | 2 | TODO |
| 1.85 | `ss.formula.ptg` | 76 | TODO |
| 1.86 | `ss.formula.ptg.external` | 6 | TODO |
| 1.87 | `ss.formula.parser` | 5 | TODO |
| 1.88 | `ss.formula.parser.rav` | 2 | TODO |
| 1.89 | `ss.formula.eval` | 18 | TODO |
| 1.90 | `ss.formula.eval.advance` | 3 | TODO |
| 1.91 | `ss.formula.evalcache` | 7 | TODO |
| 1.92 | `ss.formula.evaluator` | 9 | TODO |
| 1.93 | `ss.formula.function` | 3 | TODO |
| 1.94 | `ss.formula.functions` (root) | 56 | TODO |
| 1.95 | `ss.formula.functions.base` | 4 | TODO |
| 1.96 | `ss.formula.functions.builtin` | 8 | TODO |
| 1.97 | `ss.formula.functions.calendar` | 9 | TODO |
| 1.98 | `ss.formula.functions.db` | 4 | TODO |
| 1.99 | `ss.formula.functions.financial` | 14 | TODO |
| 1.100 | `ss.formula.functions.matrix` | 5 | TODO |
| 1.101 | `ss.formula.functions.statistics` | 38 | TODO |
| 1.102 | `ss.formula.functions.util` | 4 | TODO |
| 1.103 | `ss.formula.functions.util.*` (7 dirs) | ~32 | TODO |
| 1.104 | `ss.formula.atp` | 36 | TODO |
| 1.105 | `ss.formula.util` | 2 | TODO |
| 1.106 | `ss.calcchain` | 6 | TODO |
| 1.107 | `ss.calcchain.*` (3 dirs) | ~11 | TODO |
| 1.108 | `ss.dependent` | 3 | TODO |
| 1.109 | `ss.dependent.refer` | 3 | TODO |
| 1.110 | `ss.dependent.refer.base` | 3 | TODO |
| 1.111 | `ss.multlang` | 1 | TODO |
| 1.112 | `ss.multlang.res` | 5 | TODO |
| 1.113 | `ss.util` | 4 | TODO |
| 1.114 | `ss.util.indexmap` | 2 | TODO |
| 1.115 | `ss.util.level` | 3 | TODO |
| 1.116 | `ss.util.primeval.objmap` | 2 | TODO |

### Phase 2: 核心渲染管线 (P2) — ~900 files

| 批次 | 目录 | Java 文件 | 状态 |
|------|------|----------:|:----:|
| 2.1 | `moss.io` + `moss.io.except` + `moss.io.ole` | 20 | TODO |
| 2.2 | `xlsxr.*` (全部 XLSX reader, ~45 dirs) | 297 | TODO |
| 2.3 | `moss.app.condfmt` + `condfmt.rule` + `condfmt.valuebased` | 27 | TODO |
| 2.4 | `moffice.drawing.dmlr.*` (all reader dirs) | ~99 | TODO |
| 2.5 | `moffice.drawing.app.*` | 15 | TODO |
| 2.6 | `moffice.text.layout.*` | 2 | TODO |
| 2.7 | `spreadsheet.control.grid.*` (layout+render+piper) | ~138 | TODO |
| 2.8 | `spreadsheet.utils` | 5 | TODO |
| 2.9 | `chart.datasource` | 13 | TODO |
| 2.10 | `chart.io` + `chart.creator` | 4 | TODO |
| 2.11 | `chart.app.*` (all app dirs) | 41 | TODO |
| 2.12 | `chart.chartxml.reader.*` (all reader dirs) | 92 | TODO |
| 2.13 | `chart.text` | 16 | TODO |
| 2.14 | `chart.typo.*` | 24 | TODO |
| 2.15 | `chart.render.*` | 50 | TODO |
| 2.16 | `chart.util` + `chart.utils` | 25 | TODO |
| 2.17 | `ss.formula.renderer` | 2 | TODO |
| 2.18 | `poi.openxml.vml` | 2 | TODO |

### Phase 3: HitTest + 基础功能 (P3) — ~6 files

| 批次 | 目录 | Java 文件 | 状态 |
|------|------|----------:|:----:|
| 3.1 | `grid.service` | 1 | TODO |
| 3.2 | `chart.service` | 4 | TODO |
| 3.3 | `moffice.service.doc` | 1 | TODO |

### Phase 4: 其他功能 (P4) — ~1966 files

| 批次 | 目录 | Java 文件 | 状态 |
|------|------|----------:|:----:|
| 4.1 | `moss.app.*` (剩余应用逻辑) | ~160 | TODO |
| 4.2 | `moss.core.pivottable.*` | 25 | TODO |
| 4.3 | `moss.engine.sort` | 14 | TODO |
| 4.4 | `moss.compiler.edithint` | 7 | TODO |
| 4.5 | `moss.external.text` | 3 | TODO |
| 4.6 | `moss.sparkline.*` | 8 | TODO |
| 4.7 | `moss.chart.*` (in core) | 35 | TODO |
| 4.8 | `moffice.drawing.dmlw.*` (all writer dirs) | ~52 | TODO |
| 4.9 | `sparkline.*` | ~39 | TODO |
| 4.10 | `base.io.css.*` | 12 | TODO |
| 4.11 | `coop.net` + `io.file.*` | 3 | TODO |
| 4.12 | `moffice.kso.filter.*` + `kso.framework` + `kso.msgbox` | 12 | TODO |
| 4.13 | `moffice.print` | 3 | TODO |
| 4.14 | `grid.render.animation.*` | 13 | TODO |
| 4.15 | `grid.dataaction` + `grid.uil.*` | 9 | TODO |
| 4.16 | `chardet` | 3 | TODO |
| 4.17 | `filefmt.biff8.*` (XLS format) | 365 | TODO |
| 4.18 | `xlsr.*` (XLS reader) | 67 | TODO |
| 4.19 | `xlsw.*` (XLS writer) | 40 | TODO |
| 4.20 | `xlsxw.*` (XLSX writer) | 93 | TODO |
| 4.21 | `xlsbr.*` (XLSB reader) | 21 | TODO |
| 4.22 | `htmlr.*` (HTML reader) | 69 | TODO |
| 4.23 | `sxmlr.*` + `sxmlw` (SXML) | 41 | TODO |
| 4.24 | `plainr.*` + `plainw` (plain text) | 18 | TODO |
| 4.25 | `gvmlr.*` + `gvmlw` + `crtx` (GVM) | 10 | TODO |
| 4.26 | `xlsxr.handler.pivot` | 23 | TODO |
| 4.27 | `chart.chartxml.writer.*` (all writer dirs) | 88 | TODO |
| 4.28 | `chart.chartole.*` (OLE chart) | 177 | TODO |
| 4.29 | `chart.amend` | 3 | TODO |
| 4.30 | `ss.formula.udf` | 2 | TODO |
| 4.31 | `ss.statemachine` | 3 | TODO |
| 4.32 | `ss.manual` | 7 | TODO |
| 4.33 | `ss.debugtool` | 2 | TODO |
| 4.34 | `moss.io.clipx` | 2 | TODO |

---

## 6. 已创建的目录级计划

| 批次 | Java 包 | Rust 目录 | 计划文件 | 状态 |
|------|---------|----------|---------|:----:|
| 1.1 | `moss.util` | `src/util/` | `util_rewriter_plan.md` | ✅ 完成 (28 files, 442✅/0❌/449⚠️) |
| 1.2 | `moss.util.binarytree` | `src/util/binarytree/` | `binarytree_rewriter_plan.md` | ✅ 完成 (55✅/0❌/17⚠️, 2026-05-08) |
| 1.3 | `moss.util.binarytree.node` | `src/util/binarytree/node/` | `node_rewriter_plan.md` | ✅ 完成 (27✅/0❌/0⚠️, 2026-05-08) |
| 1.4 | `moss.util.cache` | `src/util/cache/` | `cache_rewriter_plan.md` | ✅ 完成 (44✅/0❌/20⚠️, 2026-05-08) |
| 1.5 | `moss.util.checksum` | `src/util/checksum/` | `checksum_rewriter_plan.md` | ✅ 完成 (371✅/0❌/9⚠️, 2026-05-08) |
| 1.6 | `moss.util.deqlist` | `src/util/deqlist/` | `deqlist_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.7 | `moss.util.objpool` | `src/util/objpool/` | `objpool_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.8 | `moss.util.primitive` | `src/util/primitive/` | `primitive_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.9 | `moss.util.rangelist` | `src/util/rangelist/` | `rangelist_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.10 | `moss` | `src/moss/` | `moss_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.11 | `moss.framework` | `src/framework/` | `framework_rewriter_plan.md` | ⬜ 待重写 (14 files) |
| 1.12 | `moss.engine.grid` | `src/engine/grid/` | `grid_rewriter_plan.md` | ✅ 已完成 (344 pass / 0 fail) |
| 1.13 | `moss.engine.grid.griditerator` | `src/engine/grid/griditerator/` | `griditerator_rewriter_plan.md` | ✅ 完成 (57✅/0❌/20⚠️, 2026-05-09) |
| 1.14 | `moss.engine.eval` | `src/engine/eval/` | `eval_rewriter_plan.md` | ⬜ 待重写 (14 files) |
| 1.15 | `moss.engine.eval.sup` | `src/engine/eval/sup/` | `sup_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.16 | `moss.engine.linktb` | `src/engine/linktb/` | `linktb_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.17 | `moss.engine.areaevent` | `src/engine/areaevent/` | `areaevent_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.18 | `moss.core.book` | `src/core/book/` | `book_rewriter_plan.md` | ⬜ 待重写 (13 files) |
| 1.19 | `moss.core.sheet` | `src/core/sheet/` | `sheet_rewriter_plan.md` | ⬜ 待重写 (18 files) |
| 1.20 | `moss.core.table` | `src/core/table/` | `table_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.21 | `moss.core.theme` | `src/core/theme/` | `theme_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.22 | `moss.core.drawing` | `src/core/drawing/` | `drawing_rewriter_plan.md` | ⬜ 待重写 (34 files) |
| 1.23 | `moss.core.drawing.tools` | `src/core/drawing/tools/` | `tools_rewriter_plan.md` | ⬜ 待重写 (8 files) |
| 1.24 | `moss.app` | `src/app/` | `app_rewriter_plan.md` | ⬜ 待重写 (40 files) |
| 1.25 | `moss.app.event` | `src/app/event/` | `event_rewriter_plan.md` | ⬜ 待重写 (8 files) |
| 1.26 | `moss.app.except` | `src/app/except/` | `except_rewriter_plan.md` | ⬜ 待重写 (15 files) |
| 1.27 | `moss.app.xf` | `src/app/xf/` | `xf_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.28 | `moffice.drawing.data.basic` | `src/drawing/data/basic/` | `basic_rewriter_plan.md` | ⬜ 待重写 (9 files) |
| 1.29 | `moffice.drawing.data.blip` | `src/drawing/data/blip/` | `blip_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.30 | `moffice.drawing.data.color` | `src/drawing/data/color/` | `color_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.31 | `moffice.drawing.data.effect` | `src/drawing/data/effect/` | `effect_rewriter_plan.md` | ⬜ 待重写 (30 files) |
| 1.32 | `moffice.drawing.data.fill` | `src/drawing/data/fill/` | `fill_rewriter_plan.md` | ⬜ 待重写 (9 files) |
| 1.33 | `moffice.drawing.data.font` | `src/drawing/data/font/` | `font_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.34 | `moffice.drawing.data.geometry` | `src/drawing/data/geometry/` | `geometry_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.35 | `moffice.drawing.data.hyperlink` | `src/drawing/data/hyperlink/` | `hyperlink_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.36 | `moffice.drawing.data.media` | `src/drawing/data/media/` | `media_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.37 | `moffice.drawing.data.outline` | `src/drawing/data/outline/` | `outline_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.38 | `moffice.drawing.data.shape` | `src/drawing/data/shape/` | `shape_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.39 | `moffice.drawing.data.style` | `src/drawing/data/style/` | `style_rewriter_plan.md` | ⬜ 待重写 (10 files) |
| 1.40 | `moffice.drawing.data.table` | `src/drawing/data/table/` | `drawing_table_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.41 | `moffice.drawing.data.text` | `src/drawing/data/text/` | `drawing_data_text_rewriter_plan.md` | ⬜ 待重写 (8 files) |
| 1.42 | `moffice.drawing.data.threeD` | `src/drawing/data/three_d/` | `three_d_rewriter_plan.md` | ⬜ 待重写 (9 files) |
| 1.43 | `moffice.drawing` | `src/drawing/` | `drawing_root_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.44 | `moffice.kso.kproto` | `src/kso/kproto/` | `kproto_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.45 | `moffice.kso.kproto.internal` | `src/kso/kproto/internal/` | `internal_rewriter_plan.md` | ⬜ 待重写 (6 files) |
| 1.46 | `moffice.kso.kproto.memory` | `src/kso/kproto/memory/` | `memory_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.47 | `moffice.kso.kproto.util` | `src/kso/kproto/util/` | `kproto_util_rewriter_plan.md` | ⬜ 待重写 (6 files) |
| 1.48 | `moffice.kso` | `src/kso/` | `kso_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.49 | `moffice.kso.unit` | `src/kso/unit/` | `unit_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.50 | `moffice.pool` | `src/pool/` | `pool_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.51 | `moffice.util` | `src/moffice_util/` | `moffice_util_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.52 | `poi.ss` | `src/poi/ss/` | `ss_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.53 | `poi.ss.util` | `src/poi/ss/util/` | `poi_util_rewriter_plan.md` | ⬜ 待重写 (10 files) |
| 1.54 | `poi.ss.usermodel` | `src/poi/ss/usermodel/` | `usermodel_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.55 | `chart` | `src/chart/` | `chart_root_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.56 | `chart.data` | `src/chart/data/` | `chart_data_rewriter_plan.md` | ⬜ 待重写 (34 files) |
| 1.57 | `chart.data.axis` | `src/chart/data/axis/` | `axis_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.58 | `chart.data.base` | `src/chart/data/base/` | `base_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.59 | `chart.data.chart` | `src/chart/data/chart/` | `chart_chart_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.60 | `chart.data.chartspace` | `src/chart/data/chartspace/` | `chartspace_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.61 | `chart.data.d3` | `src/chart/data/d3/` | `d3_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.62 | `chart.data.dispunits` | `src/chart/data/dispunits/` | `dispunits_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.63 | `chart.data.label` | `src/chart/data/label/` | `label_rewriter_plan.md` | ⬜ 待重写 (5 files) |
| 1.64 | `chart.data.layout` | `src/chart/data/layout/` | `layout_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.65 | `chart.data.legend` | `src/chart/data/legend/` | `legend_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.66 | `chart.data.pivot` | `src/chart/data/pivot/` | `pivot_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.67 | `chart.data.print` | `src/chart/data/print/` | `print_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.68 | `chart.data.series` | `src/chart/data/series/` | `series_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.69 | `chart.data.shape` | `src/chart/data/shape/` | `chart_shape_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.70 | `chart.data.source` | `src/chart/data/source/` | `source_rewriter_plan.md` | ⬜ 待重写 (10 files) |
| 1.71 | `chart.data.str` | `src/chart/data/str/` | `str_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.72 | `chart.data.trend` | `src/chart/data/trend/` | `trend_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.73 | `chart.msenum` | `src/chart/msenum/` | `msenum_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.74 | `chart.common` | `src/chart/common/` | `common_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.75 | `chart.store` | `src/chart/store/` | `store_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.76 | `chart.objpool` | `src/chart/objpool/` | `objpool_rewriter_plan.md` | ⬜ 待重写 (5 files) |
| 1.77 | `chart.style` | `src/chart/style/` | `chart_style_rewriter_plan.md` | ⬜ 待重写 (9 files) |
| 1.78 | `chart.format` | `src/chart/format/` | `format_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.79 | `chart.description` | `src/chart/description/` | `description_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.80 | `chart.event` | `src/chart/event/` | `chart_event_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.81 | `ss.constant` | `src/formula/constant/` | `constant_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.82 | `ss.interfaces` | `src/formula/interfaces/` | `interfaces_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.83 | `ss.formula` | `src/formula/` | `formula_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.84 | `ss.formula.constant` | `src/formula/formula_constant/` | `formula_constant_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.85 | `ss.formula.ptg` | `src/formula/ptg/` | `ptg_rewriter_plan.md` | ⬜ 待重写 (76 files) |
| 1.86 | `ss.formula.ptg.external` | `src/formula/ptg/external/` | `external_rewriter_plan.md` | ⬜ 待重写 (6 files) |
| 1.87 | `ss.formula.parser` | `src/formula/parser/` | `parser_rewriter_plan.md` | ⬜ 待重写 (5 files) |
| 1.88 | `ss.formula.parser.rav` | `src/formula/parser/rav/` | `rav_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.89 | `ss.formula.eval` | `src/formula/eval/` | `eval_rewriter_plan.md` | ⬜ 待重写 (18 files) |
| 1.90 | `ss.formula.eval.advance` | `src/formula/eval/advance/` | `advance_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.91 | `ss.formula.evalcache` | `src/formula/evalcache/` | `evalcache_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.92 | `ss.formula.evaluator` | `src/formula/evaluator/` | `evaluator_rewriter_plan.md` | ⬜ 待重写 (9 files) |
| 1.93 | `ss.formula.function` | `src/formula/function/` | `function_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.94 | `ss.formula.functions` | `src/formula/functions/` | `functions_rewriter_plan.md` | ⬜ 待重写 (56 files) |
| 1.95 | `ss.formula.functions.base` | `src/formula/functions/base/` | `functions_base_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.96 | `ss.formula.functions.builtin` | `src/formula/functions/builtin/` | `builtin_rewriter_plan.md` | ⬜ 待重写 (8 files) |
| 1.97 | `ss.formula.functions.calendar` | `src/formula/functions/calendar/` | `calendar_rewriter_plan.md` | ⬜ 待重写 (9 files) |
| 1.98 | `ss.formula.functions.db` | `src/formula/functions/db/` | `db_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.99 | `ss.formula.functions.financial` | `src/formula/functions/financial/` | `financial_rewriter_plan.md` | ⬜ 待重写 (14 files) |
| 1.100 | `ss.formula.functions.matrix` | `src/formula/functions/matrix/` | `matrix_rewriter_plan.md` | ⬜ 待重写 (5 files) |
| 1.101 | `ss.formula.functions.statistics` | `src/formula/functions/statistics/` | `statistics_rewriter_plan.md` | ⬜ 待重写 (38 files) |
| 1.102 | `ss.formula.functions.util` | `src/formula/functions/util/` | `functions_util_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.103a | `ss.formula.functions.util.db` | `src/formula/functions/util/db/` | `util_db_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.103b | `ss.formula.functions.util.financial` | `src/formula/functions/util/financial/` | `util_financial_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.103c | `ss.formula.functions.util.iteration` | `src/formula/functions/util/iteration/` | `util_iteration_rewriter_plan.md` | ⬜ 待重写 (11 files) |
| 1.103d | `ss.formula.functions.util.matcher` | `src/formula/functions/util/matcher/` | `util_matcher_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.103e | `ss.formula.functions.util.numbercollector` | `src/formula/functions/util/numbercollector/` | `util_numbercollector_rewriter_plan.md` | ⬜ 待重写 (5 files) |
| 1.103f | `ss.formula.functions.util.statistics` | `src/formula/functions/util/statistics/` | `util_statistics_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.103g | `ss.formula.functions.util.statistics.toms708` | `src/formula/functions/util/statistics/toms708/` | `toms708_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.104 | `ss.formula.atp` | `src/formula/atp/` | `atp_rewriter_plan.md` | ⬜ 待重写 (36 files) |
| 1.105 | `ss.formula.util` | `src/formula/util/` | `formula_util_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.106 | `ss.calcchain` | `src/formula/calcchain/` | `calcchain_rewriter_plan.md` | ⬜ 待重写 (6 files) |
| 1.107a | `ss.calcchain.listener` | `src/formula/calcchain/listener/` | `listener_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.107b | `ss.calcchain.marktype` | `src/formula/calcchain/marktype/` | `marktype_rewriter_plan.md` | ⬜ 待重写 (7 files) |
| 1.107c | `ss.calcchain.threadpool` | `src/formula/calcchain/threadpool/` | `threadpool_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.108 | `ss.dependent` | `src/formula/dependent/` | `dependent_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.109 | `ss.dependent.refer` | `src/formula/dependent/refer/` | `refer_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.110 | `ss.dependent.refer.base` | `src/formula/dependent/refer/base/` | `refer_base_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.111 | `ss.multlang` | `src/formula/multlang/` | `multlang_rewriter_plan.md` | ⬜ 待重写 (1 files) |
| 1.112 | `ss.multlang.res` | `src/formula/multlang/res/` | `res_rewriter_plan.md` | ⬜ 待重写 (5 files) |
| 1.113 | `ss.util` | `src/formula/ss_util/` | `ss_util_rewriter_plan.md` | ⬜ 待重写 (4 files) |
| 1.114 | `ss.util.indexmap` | `src/formula/ss_util/indexmap/` | `indexmap_rewriter_plan.md` | ⬜ 待重写 (2 files) |
| 1.115 | `ss.util.level` | `src/formula/ss_util/level/` | `level_rewriter_plan.md` | ⬜ 待重写 (3 files) |
| 1.116 | `ss.util.primeval.objmap` | `src/formula/ss_util/primeval/objmap/` | `objmap_rewriter_plan.md` | ⬜ 待重写 (2 files) |

**总计**: 124 个目录级计划已创建，覆盖全部 Phase 1 批次

---

## 文档日期
- **编写日期**: 2026-05-08
- **更新日期**: 2026-05-08
- **状态**: Phase 1 Batch 1.6 (`moss.util.deqlist`) ✅ 已完成 (89✅/0❌/55⚠️, 2026-05-08)
- **状态**: Phase 1 Batch 1.7 (`moss.util.objpool`) ✅ 已完成 (38✅/0❌/10⚠️, 2026-05-08)
- **状态**: Phase 1 Batch 1.8 (`moss.util.primitive`) ✅ 已完成 (38✅/0❌/4⚠️, 2026-05-08)
- **状态**: Phase 1 Batch 1.9 (`moss.util.rangelist`) ✅ 已完成 (120✅/0❌/142⚠️, 2026-05-08)
- **下一步**: Phase 1 Batch 1.10 (`moss` root) 开始实施

2026-10-30 / R0b-40 实际 styles 调用暴露共享 KFC NumberFormatter.compile → NumFmtAnalyzer/VariantToken/VariantPieceDic/AnalyzerFactory/AnalyzerBase 旧占位。R0b-41 改写中：按原资源、piece、analyzer、holder compile 的原依赖逐类恢复，不将 compile 固定为 true/false，不跳过 TableStylesLabel 的转换调用。R40 新真机金文件已采集，等待完整原调用闭环复核。

2026-10-30 / R0b-42 改写中：继续原跨工作簿 SupBookRecord → CRNCountRecord/CRNRecord → LinkTable.CRNBlock/ExternalBookBlock → SupEvaluationCell/Sheet/Workbook，接回真实外部缓存及其求值对象。首先恢复原模型/工厂/URL 与二进制字符串叶子；不将旧简化 SupBook 标志容器伪称完整 ExternalBookBlock，不以空外链返回绕过原依赖。R39～41 固定源回归已完成，256/0/79；下一段另立指纹。

2026-10-30 / R0b-43～44 测试中：原 XlsxwExternalLinks 实际 XML/KFS ZIP、原 ExternalLinkHandler 内部类、XlsxrExternalLinks 及 LinkTable 可变 List 引用真机零差；详见 rewrite-review/milestones/m4-edit-save/12-20261030-r0b43-r0b44-external-links-opc.md。完整回归按 R44 固定源复跑中。R45 依赖分析及真机采集已完成：HLElementByteParser → StringBlock/StringsTraversal → XlsxrSharedStrings before/after 大块时序；原 workbook 完成回调接线不能跳过此链。
