# 04_function_change_layout — 空 slide 套 layout

> feature: pp-08-slides-walk ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：空列表路径把 layout 的 placeholder 复制到 `mPres.slideAt(0)`（ph 4/5/6 依 master HeaderFooter 判定），空文档也有可读内容，并补 `slideDidFinishReadingEverything(0)`。缺失时断在哪：空/异常文档打开后 slide 空白无版式内容，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

空文档/空列表路径：`PptxrSlides.open()` 在 `mSlidesReader.size() == 0` 时对
`mPres.slideAt(0)` 调用 static `changeLayout(KmoSlide, KmoSlideLayout)`，把布局的
placeholder shape 复制到空 slide 上；copy 判定与文本清洗分别由 static
`shapeNeedCopyFromLayout` / `shapeNeedClearContentText` 决定。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `changeLayout(KmoSlide emptySlide, KmoSlideLayout layout)`（static）；`layout.spTree().getAllPh()` 返回布局占位符列表；每个 `layoutPh`（`KmoShowShape`）需要 `type()`、`isPlaceHolder()`、`ph()`、`getTextOperator().getTextBody().protocol()`；hf 判定需要 `layout.master().hasHeaderFooter()` 与 `layout.master().headerFooter()`（`HeaderFooter`：`hasDateTime/isDateTime`、`hasSlideNum/isSlideNum`、`hasFooter/isFooter`）。 |
| **输出** | `emptySlide.spTree().insertShapes(addShapes, 0)`：每个被 copy 的 layoutPh 生成一个 `new KmoShowShape(emptySlide, layoutPh.type())`（`setNoGrp(true)`、`writePh(layoutPh.ph())`）；ph 类型 4/5/6 的副本 `getTextBody().updateProtocol(layoutPh 的 protocol)` + `textOperator.save(newShape)`，其余类型 `clearTextContents()`。 |
| **状态与副作用** | `emptySlide.setLayout(layout)` + `emptySlide.spTree().initSpTreeGroupTransform()`；copy 判定 `shapeNeedCopyFromLayout(layout, shape)`（static）：`shape.type() != 1 → false`，`!shape.isPlaceHolder() → false`，随后按 `shape.ph().type()` 分支：`case 4` `!layout.master().hasHeaderFooter() → false`，否则 `!hf.hasDateTime() || hf.isDateTime()`；`case 5` `!hf.hasSlideNum() || hf.isSlideNum()`；`case 6` `!hf.hasFooter() || hf.isFooter()`；`default → true`。文本清洗 `shapeNeedClearContentText(shape)`：`shape.ph().type()` 为 4/5/6 → false（保留并走 protocol 复制），其余 → true（清文本）。 |
| **错误语义** | 无受检异常；`getAllPh()` 空 → 不 insert；type!=1 或非 placeholder 的 ph 静默跳过（不 copy、不算失败）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| ph 元数据（`type/isPlaceHolder/ph`） | pp-02-shape-tree-parse | layoutPh 判定入参 | 编译面 `KmoShowShape` 可测 | 不改 pp-02 解析 | 不改已 pass 的 pp-01/pp-02 语义 |
| `spTree()/getAllPh/initSpTreeGroupTransform/insertShapes` | pp-02 形状树 API | 复制载体 | 编译面 `KmoShapeTree` 已接 | 随 pp-02 真树接线 | 不在本文件伪造第二套类型名 |
| `master().headerFooter()`（HeaderFooter 状态） | pp-07（master part 数据） | hf 4/5/6 判定 | 编译面 `HeaderFooter` 字段可测 | 随 master part 数据接线 | 不默认造 KoBin HeaderFooter |
| 文本 protocol 复制（`KmoTextOperator`/`TextBody.updateProtocol`） | 文本层模型 | 4/5/6 副本内容 | `updateProtocol`+`save` 已接 | 全文协议仍可加深 | 不在这里实现 KoBin 文本协议 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` |
| Rust 现状 | 目标 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs`；当前 `libs/ppt-core/src/cn/wps/show/pptxr/ppt/` 目录尚未生成 |
| 直接调用方 | `PptxrSlides.open()` 空列表分支（Java 第 151–163 行：无 master → `buildDefaultSlideMaster.layoutAt(0)`，有 → `masterAt(0).layoutAt(0)`，随后 `changeLayout(slide, layout)` + `slideDidFinishReadingEverything(0)`） |

关键方法/字段表（方法名与行号以 Java 源码为准）：

| Java symbol | Java 位置 | 语义要点 | Rust 目标 |
|---|---|---|---|
| `public static void changeLayout(KmoSlide emptySlide, KmoSlideLayout layout)` | 第 337–362 行 | setLayout + initSpTreeGroupTransform；逐 layoutPh 按 copy 判定收集 `addShapes` 后 `insertShapes(addShapes, 0)` | `fn change_layout` |
| `private static boolean shapeNeedCopyFromLayout(KmoSlideLayout layout, KmoShowShape shape)` | 第 365–399 行 | type==1 && isPlaceHolder；ph 4/5/6 依赖 master HeaderFooter 双条件 | `fn shape_need_copy_from_layout` |
| `private static boolean shapeNeedClearContentText(KmoShowShape shape)` | 第 401–404 行 | `phType != 4 && phType != 5 && phType != 6` → 清文本 | `fn shape_need_clear_content_text` |
| 关联模型 `KmoShowShape`/`KmoTextOperator`/`HeaderFooter` | 模型层（pp-02 与文本层） | 复制载体与 hf 数据 | 依赖 pp-02/pp-07 模型 API |

改写列表：`changeLayout`、`shapeNeedCopyFromLayout`、`shapeNeedClearContentText`。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。

| symbol | Java 位置 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `changeLayout` | 337–362 | 空 getAllPh / 可 copy 项 | 不 insert / 收集 addShapes 后 `insertShapes(..., 0)` | 循环 + clear 与 protocol 二选一 |
| `shapeNeedCopyFromLayout` | 365–399 | type!=1；非 placeholder；ph 4/5/6；default | false / hf 双条件 / true | 短路逐级；switch 4/5/6 |
| `shapeNeedClearContentText` | 401–404 | phType 4/5/6 / 其它 | false / true | 三值排除 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | `changeLayout` type!=1 不 copy | 失败 | 是 | T01 |
| C02 | `shapeNeedCopyFromLayout` ph 4/5/6 看 hf | 分支 | 是 | T02 |
| C03 | 其它 ph copy 并清文本 | 正常 | 是 | T03 |

### 3.3 可测试性

内存构造空 `KmoSlide` + fake layout（`getAllPh` 返回预置的 `KmoShowShape` 列表，shape 提供
`type/isPlaceHolder/ph/headerFooter`）；Recording 形状树验证 `insertShapes(addShapes, 0)` 的内容
与顺序。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | layoutPh `type()=2` 或 `isPlaceHolder()=false` | 不 copy、`addShapes` 为空、不调用 insertShapes（或空插入） | 红测停在本 function |
| T02 | C02 | ph `type()=4`：master 无 `hasHeaderFooter` → false；有但 `hasDateTime=false, isDateTime=true` → true；`hasDateTime=true, isDateTime=false` → false；`type()=5`/`6` 按 slideNum/footer 同构 | 与 Java 一致：`!hasX || isX` 双条件 | 红测停在本 function |
| T03 | C03 | ph `type()=1` 普通占位符（非 4/5/6）且 placeholder | copy：`setNoGrp(true)`、`writePh`、`clearTextContents`；空 getAllPh → 不 insert | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "04_function_change_layout"
```

测试入口：`libs/ppt-core/tests/cn/wps/show/pptxr/ppt/slides/PptxrSlides_change_layout.rs`（`[[test]] name = "04_function_change_layout"`）。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_change_layout

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_change_layout \
  --src src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/04_function_change_layout/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_change_layout --json`)
date: 2026-09-01 16:21

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | 51/632 | 8.1% | 103/992 | 10.4% |
| **function total** | **51/632** | **8.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs \
  --report libs/ppt-core/parser/features/pp-08-slides-walk/functions/04_function_change_layout/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 08:21

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `initialize`, `getSlideIdByFileName`, `getRelationShipTargetFileName`, `PptxrSlides`, `open`, `delayCreateSlides`, `onSlideIds`, `onSlideMasterIds`, `getMasterReader`, `changeLayout`, `shapeNeedCopyFromLayout`, `shapeNeedClearContentText` | `getSlideIdByFileNameNotAdd`, `createSlidesByFileName`, `onNotesMasterIds`, `onHandoutMasterIds`, `getNotesMasterReader` | — | **PASS** |
| `UnReadSlides` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/slides/PptxrSlides.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/slides/PptxrSlides.rs` | **PASS** | `UnReadSlides`, `resetSlides`, `next`, `getCreateSlideCount` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->