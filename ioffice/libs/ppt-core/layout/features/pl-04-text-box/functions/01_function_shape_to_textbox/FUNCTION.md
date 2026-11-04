# 01_function_shape_to_textbox — 段与 run → TextBoxData 文本

> feature: pl-04-text-box ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后：`KmoShowShape`（有 `txBody`）的文本从 parse 模型进入内部排版模型——每段一个 `TextData`（`mSourceText` + `ParaInfo` + `SpanInfo[]`，span 带 `mStartPos/mEndPos` 区间与最小字体样式），`phType` 就位。pl-05 断行、pr-03 画字的输入由此就绪（此前只有 `KmoTextBody` 的 XML 结构，pl-05 无从下口）。

没有它管线断在哪：**有字 slide** 的文本框永远没有可排文本（pl-05 空转、pr-03 无 `DrawText`）。空 body / 无 txBody 时本 function 返回 empty——这是合法结果，不是失败（Java `ShowTextLayout.layout` 对 `isEmpty()` 返回 null）。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `(KmoShowShape, KmoContext)`：`shape.hasTextBody()` 或其 `ph().type()==15`；`ctx` 仅在取 presentation（bullet 展示）时使用，MVP 可为 None |
| **输出** | `TextBoxData`：`textDatas: Vec<TextData>`（每段 `mSourceText`、`mPara`、`mFirstSpan`/`mOtherSpans`）+ `phType` + `mTextLength`；空 body → `TextBoxData::empty()` |
| **状态与副作用** | 只读输入、无上下文外写；Java 有 `LayoutSharedObjects.showTextDataPool` 借用复用（Rust 侧先持有 `Vec`，不实现池）；`IHolder.setTextRect` 回写 shape 缓存 MVP 不做 |
| **错误语义** | 无 `textOp`（无 body 且非 note ph）→ 返回 empty；跨度数据缺失等 Java 会抛 `SpanNullPointerException` 的场景 → MVP 先行 `Result` 返回 Err 或夹紧区间，具体 待 loop 确认 |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `shape_to_textbox` | `KmoShowShape`（`hasTextBody/getTextOperator/ph().type()/parent()`）+ `KmoContext` | `ShapeDataTransform.getTextOperator`（`ShapeDataTransform.java:17`）；`DataTransform.exToInternal`（`DataTransform.java:75`） |

### 1.3 输出
| 输出 | 说明 | Java 对标 |
|------|------|-----------|
| `TextData[]` | 每段：`mSourceText`（拼接后的可见文本）、`mPara`（对齐/缩进/段距/行距/方向）、`mFirstSpan`+`mOtherSpans` | `DataTransform.fetchSpanInfo`（`DataTransform.java:752`） |
| `SpanInfo` 最小集 | `mFontName/mFontSize/mFontColor/mIsBold/mIsItalic/mStartPos/mEndPos` | `setSpanInfo`（`DataTransform.java:291`）的字体/字号/颜色子集 |
| `phType` | `DefineConverter.convertPhHolder(typePh)`（`DefineConverter.java:4`） | `convertToBoxData`（`DataTransform.java:87`） |
| empty 标志 | `textDatas.is_empty()`；Java 另判单段 `"\r"`/空串 | `TextBoxData.isEmpty`（`TextBoxData.java:178`） |

### 1.4 上游
pp-11（`KmoTextBody`，真实文件）；pl-03（ph 合并，fixture 可绕）；pl-01（单位语义，layout 档）。

### 1.5 下游
02 落框属性；pl-05 断行（`textDatas`/`phType`/spans）。

### 1.6 不变量
1. span 区间 `[mStartPos, mEndPos)` 连续覆盖 `mSourceText`，跨 span 不重不漏。
2. 空 body（无 textOp）→ 返回 empty，不 panic；`ph().type()==15` 的 note 文本同样可排。
3. 所有长度类字段（`mMarginL/mIndent/…`）已是 **layout 单位**（`emuToLayoutUnits = units.fromEmuY`），禁止在输出里留 EMU。
4. 域（页码/日期）：`resetDateFieldTextRuns`/`resetSlideNumFieldTextRuns` 不执行，`canRenderCache` 固定 true。
5. bullet 只落类型号/布尔（`buNone/buChar/buAutonum/buBlip` 最小判定），不跑 `convertBullet` 全量。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `KmoShowShape.hasTextBody/getTextOperator` | layout host `layout_shape.rs` | `hasTextBody()`、`getTextOperator()` | 本 loop 已接默认 crate | note ph 已测 TB4 | 禁止在本包 new 假 textOp 冒充 parser |
| `KmoTextOperator`（seek/next/format/text/range） | pp-11 | 逐段/run 迭代与格式查询 | 否（待 pp-11） | 格式拼接细节 | 禁止绕过遍历直接拿整段文本假装对齐 |
| `TextBoxData/TextData/ParaInfo/SpanInfo` | core-libs/wpslayout | 字段齐备的构造 | 有 158/110/… 行结构模板，方法多为空 | 池化/复用（pool） | 禁止发明第二套文本类型 |
| `DefineConverter.convertPhHolder` | 本 dataaction 包 | swType→phType 映射表 | Java 有（`DefineConverter.java:4`）；Rust 待 loop | — | 禁止手改映射数值 |
| bullet 全量 / effect 分支 | DataTransform | — | 否 | 是（DEFERRED_NON_FUNCTION） | 禁止顺手实现 autofit 连锁 |

## 2. 核心代码与函数改写列表

Java→Rust 表（方法名均已查证；行号 = Java 源）：

| Java（`DataTransform`/`ShapeDataTransform`） | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `exToInternal(T shp, KmoContext ctx)`（`DataTransform.java:75`，final 入口） | `ShapeDataTransform.rs: ex_to_internal` | 01 门面 |
| `convertToBoxData`（`DataTransform.java:84`） | 同上（private→内部函数） | 01 主路径 |
| `getTextOperator`（`ShapeDataTransform.java:17`） | 同上 | 01 分支（hasTextBody / ph 15） |
| `getPresentation`/`setHolder`（`ShapeDataTransform.java:54/10`） | 同上 | 01（ShapeHolder MVP 只记 shape 引用） |
| `fetchSpanInfo`（`DataTransform.java:752`） | 同上 | 01（span 区间+字体样式最小集） |
| `getHAlign`（`DataTransform.java:871`）、`getLineSpacing`（`DataTransform.java:1218`） | 同上 | 01（对齐/行距映射） |
| `DefineConverter.convertPhHolder`（`DefineConverter.java:4`） | `DefineConverter.rs` | 01（phType 映射） |
| `TextParagraphProperties`（`lvl/marginLeft/indent/spcBef/spcAft/options`） | drawing/data/text 已有结构 | 01 读取端 |
| `ShowShape.Placeholder.type()` | show/data/shape 已有结构 | 01（ph==15 判定） |

不在改写列表的方法（`todo!("DEFERRED_NON_FUNCTION")` 停车）：`convertBullet` 全量、`setSpanInfo` 的 fill/effect/outline/shadow/reflection 分支、`getPrstShdwAttr`、`fetchHyperLink` 等。

## 3. 测试方案

### 3.1 审计面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `shape_to_textbox` | 有 body | fixture shape（2 段、每段 2 run） | TextData×2、span 区间精确 | seek/next 遍历 |
| `shape_to_textbox` | 空 body / 无 body | 无 textOp 的 shape | `empty()` | 空分支 |
| `shape_to_textbox` | note ph | `ph().type()==15` 且无 body | 可排（Java 语义） | ph 分支 |
| `shape_to_textbox` | 单段空串/`\r` | 一段空文本 | `isEmpty()==true` | Java `isEmpty` 语义 |
| `phType` 映射 | `convertPhHolder` | swType 0/1/2/10/15… | 与 `DefineConverter.java:4` 表一致 | 映射表 |
| 单位 | `emuToLayoutUnits` | 91440/45720 EMU | layout 单位值 | 换算 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| TB-MULTI | shape_to_textbox | 多段多 run 区间 | 是 | TB1 |
| TB-EMPTY | shape_to_textbox | 无 body → empty | 是 | TB2 |
| TB-SINGLE | shape_to_textbox | 单段空文本/`\r` → isEmpty | 是 | TB3 |
| TB-NOTE | shape_to_textbox | ph 15 可排 | 是 | TB4 |
| TB-PHTYPE | convertPhHolder | swType 样本 | 是 | TB5 |
| TB-SPAN | shape_to_textbox | span 属性最小集 | 是 | TB6 |
| TB-UNITS | emuToLayoutUnits | inset 换算 | 是（与 02 共用口径） | TB7 |

### 3.3 可测试性
只喂 fixture `KmoShowShape`/`KmoTextBody`（不跑 parser、不跑 `ShowTextLayout.layout`）。`KmoContext` MVP 传 None。禁止启动 `RichTextLayout.typo`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| TB1 | TB-MULTI | fixture shape 2 段×2 run | ex_to_internal | 2 个 TextData，span 区间连续覆盖 | 区间错位/丢段 |
| TB2 | TB-EMPTY | 无 body | ex_to_internal | empty，无 panic | 空 body 返回非 empty |
| TB3 | TB-SINGLE | 一段 `""`/`"\r"` | ex_to_internal | isEmpty=true | 空格也算有字 |
| TB4 | TB-NOTE | ph=15 无 body | ex_to_internal | 有 TextData（Java 语义） | 误判 empty |
| TB5 | TB-PHTYPE | swType∈{0,1,2,3,4,5,10,15} | convertPhHolder | 与 Java 表逐项相等 | 映射漂移 |
| TB6 | TB-SPAN | 2 run 不同字号/颜色/bold | span 输出 | mFontSize/mFontColor/mIsBold 齐 | 字体样式丢失 |
| TB7 | TB-UNITS | 91440/45720 EMU | emuToLayoutUnits | `units.fromEmuY` 同公式 | 混入 EMU 原值 |

### 3.5 位置
crate：`libs/ppt-core/Cargo.toml`
src：`libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs`（loop 生成，当前不存在）
test：`libs/ppt-core/tests/cn/wps/show/layout/dataaction/ShapeDataTransform.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_shape_to_textbox

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_shape_to_textbox \
  --src src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs \
  --src src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --src src/cn/wps/show/layout/dataaction/DefineConverter.rs \
  --report libs/ppt-core/layout/features/pl-04-text-box/functions/01_function_shape_to_textbox/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_shape_to_textbox --json`)
date: 2026-08-30 11:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/dataaction/DataTransform.rs` | 339/730 | 46.4% | 628/1043 | 60.2% |
| `src/cn/wps/show/layout/dataaction/DefineConverter.rs` | 39/98 | 39.8% | 16/37 | 43.2% |
| `src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | 6/69 | 8.7% | 6/102 | 5.9% |
| **function total** | **384/897** | **42.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（loop 必跑；PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DefineConverter.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DefineConverter.java
```
当前 **PASS**（loop L5b）。

### 3.9 Java↔Rust 结构对齐报告（loop 必跑；overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DefineConverter.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DefineConverter.rs \
  --report libs/ppt-core/layout/features/pl-04-text-box/functions/01_function_shape_to_textbox/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 03:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapeDataTransform` | `koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | **PASS** | `setHolder`, `getTextOperator`, `covertTextBodyProperties`, `getPresentation` | — | — | **PASS** |
| `ShapeHolder` | `koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | **PASS** | `ShapeHolder` | `setTextRect`, `hasFontSizeFit`, `clearFontSizeFit`, `setFontScale`, `getHolder` | — | **PASS** |
| `DataTransform` | `koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs` | **PASS** | `setHolder`, `getTextOperator`, `exToInternal`, `convertToBoxData`, `covertTextBodyProperties`, `covertTextProperties`, `updateColorConvContext`, `setSpanInfo`, `fetchSpanInfo`, `getHAlign`, `convertBullet`, `getBulletType`, `fetchHyperLink`, `getStrikethroughline`, `getUnderline`, `getLineSpacing`, `adjustPos`, `emuToLayoutUnits`, `pointToLayoutUnits`, `point100ToLayoutUnits`, `getPresentation`, `getFontName`, `getColorAlpha` | `getParaDirection`, `getPrstShdwAttr`, `fetchBulletColor`, `fetchBulletFont`, `fetchBulletSize`, `setBulletContent`, `getTextSpacing`, `getColor`, `getGradientFill`, `convertToBitmap`, `getLangID` | — | **PASS** |
| `BulletNumberItem` | `koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs` | **PASS** | `index`, `setParagraphIndex`, `type`, `setBulletType`, `level`, `setParagraphLevel`, `startAt`, `setStartAt`, `isEmptyParagraphText`, `setEmptyParagraphText` | — | — | **PASS** |
| `DefineConverter` | `koffice/presentation/core/cn/wps/show/layout/dataaction/DefineConverter.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/DefineConverter.rs` | **PASS** | `convertPhHolder`, `convertTxtAnchor` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->