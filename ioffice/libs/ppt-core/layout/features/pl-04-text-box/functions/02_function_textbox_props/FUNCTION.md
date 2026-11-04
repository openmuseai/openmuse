# 02_function_textbox_props — 框属性（insets/anchor/vertical/wrap/autofit）

> feature: pl-04-text-box ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后：文本框的排版几何输入就位——`isWrapTextInShape`（决定 pl-05 是否折行）、四边 inset（决定断行可用宽与行首 x/y）、`anchor`/`isAnchorCenter`（垂直归属）、`textVerticalType`（竖排开关）、`textAutofit`+`fontScale`+`lnSpcReduction`（记录，连锁在 pl-05 之外）。pl-05-02「文本区内缩几何」直接吃这里的 insets。

没有它管线断在哪：行会画在框的物理边界上（insets 没扣）、长文本按不折行处理（wrap 没判定）、垂直铰链缺失。**属性解析错 → 画面整体偏移**，比丢一两个 span 更致命。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `TextBoxData`（01 产物）+ `TextBodyProperties`（`textOp.textBodyProperties(true)` 的取值，含 parent 链继承语义） |
| **输出** | 在同一 `TextBoxData` 上落字段：`leftInset/rightInset/topInset/bottomInset`（**layout 单位**）、`anchor`（`DefineConverter.convertTxtAnchor` 映射后 0=t/1=b/2=ctr/…）、`isAnchorCenter`、`textVerticalType`、`isWrapTextInShape`、`textAutofit`、`fontScale`、`lnSpcReduction`、`presetTextWarp`、`oldTextAutoFit/oldAnchor` 快照 |
| **状态与副作用** | 只写 `TextBoxData` 实例字段；`shape.getOriginalAutoFit()/getOriginalAnchor()` 快照后调用 `resetOriginalProperty()`（Java 有把 `originalAutoFit/originalAnchor` 归位的函数外副作用——Rust 侧若 shape 不可变，快照从 shape 读一次即可，**reset 语义 待 loop 确认**） |
| **错误语义** | `TextBodyProperties` 无对应 key → 用 Java 默认值（`anchor()==2`、left/right inset 默认 91440 EMU、top/bottom 默认 45720 EMU、`verticalText()==1`、wrap 默认 true）；不 panic |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `textbox_props` | `TextBoxData` + `TextBodyProperties` | `DataTransform.covertTextBodyProperties`（`DataTransform.java:222`）+ `covertTextProperties`（`DataTransform.java:252`）；`ShapeDataTransform.covertTextBodyProperties`（`ShapeDataTransform.java:40`，oldAutoFit/oldAnchor） |

### 1.3 输出
| 输出 | 说明 | Java 对标 |
|------|------|-----------|
| `leftInset/rightInset` 默认 91440 EMU → layout | `emuToLayoutUnits(91440.0F)` | `covertTextBodyProperties`（`DataTransform.java:229`） |
| `topInset/bottomInset` 默认 45720 EMU → layout | 同上 | 同上 |
| `anchor` | `DefineConverter.convertTxtAnchor(txBodyProp.anchor())`（`DefineConverter.java:59`） | `covertTextProperties`（`DataTransform.java:252`） |
| `isAnchorCenter` | `hasAnchorCenter() && isAnchorCenter()` | 同上 |
| `textVerticalType` | `hasVerticalText()` → `verticalText()` | 同上 |
| `isWrapTextInShape=false` | 仅当 `hasTextWrappingType() && textWrappingType()==0` | `covertTextBodyProperties`（`DataTransform.java:224`） |
| `textAutofit/fontScale/lnSpcReduction` | `hasTextAutofit()` → `autofit.type()/fontScale()/lnSpcReduction()` | 同上（`DataTransform.java:238`） |
| `presetTextWarp` | `hasPresetTextWarp()` → `presetTextWarp().prst()` | 同上 |
| `oldTextAutoFit/oldAnchor` | `shape.getOriginalAutoFit()/getOriginalAnchor()` ≠ -1 才写 | `ShapeDataTransform.java:42/46` |

### 1.4 上游
01 的 `TextBoxData`；`TextBodyProperties`（pp-11 解析产物，`koffice/presentation/core/cn/wps/drawing/data/text/TextBodyProperties.java`——注意 `anchor()` 默认 2、insets 默认 91440/45720 EMU、`verticalText()` 默认 1）。

### 1.5 下游
pl-05-02（insets/anchor/vertical 进入文本区内缩几何）；pl-05-01（wrap 开关、autofit 记录）。

### 1.6 不变量
1. insets 一律 layout 单位；EMU 只在输入端出现。
2. `textWrappingType()!=0` 或未设置 → wrap 保持默认 true。
3. `anchor` 走 `convertTxtAnchor` 映射（0→0、1→1、2→2、3→3、4→4，与 `DefineConverter.java:59` 一致），不落 `TextBodyProperties` 原值。
4. autofit/warp 只解析落字段，**不触发缩放**（缩放是 pl-05/Java RichTextLayout 的 `doTypo` 连锁，本波不做）。
5. `oldTextAutoFit/oldAnchor` 在原始值≠-1 时写入；写后 Java 会复位 shape 的 original 属性，Rust 侧该复位语义 待 loop 确认。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `TextBodyProperties` 读方法 | drawing/data/text（Rust `libs/ppt-core/src/cn/wps/drawing/data/text/TextBodyProperties.rs`，3209 行） | `anchor/leftInset/rightInset/topInset/bottomInset/textWrappingType/textAutofit/presetTextWarp/anchorCenter/verticalText/hasXxx` | 结构已整文件 keep/rewrite；具体读方法状态 待 loop 确认 | 继承链（parent follow）取值为 MVP 直接读值 | 禁止只读 map 绕过继承语义 |
| `DefineConverter.convertTxtAnchor` | show/layout/dataaction | 映射表 | Java 有；Rust 待 loop | — | 禁止改映射数值 |
| `TextBoxData.setXxx` | core-libs/wpslayout | 字段 setter | 模板结构有，实现待 loop | — | 禁止发明第二套框属性类型 |
| 默认值常量 | TextBodyProperties | 91440/45720 EMU、anchor=2、vertical=1 | Java 已查证 | — | 禁止「好心」换成别的默认 |

## 2. 核心代码与函数改写列表

Java→Rust 表（方法名均已查证）：

| Java | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `covertTextBodyProperties(T, KmoTextOperator, TextBoxData)`（`DataTransform.java:222`） | dataaction `TextboxProps.rs`（或 `ShapeDataTransform.rs` 内） | 02 门面 |
| `covertTextProperties`（`DataTransform.java:252`） | 同上 | 02（anchor/anchorCenter/verticalText/insets） |
| `ShapeDataTransform.covertTextBodyProperties`（`ShapeDataTransform.java:40`） | 同上 | 02（oldAutoFit/oldAnchor/resetOriginalProperty） |
| `DefineConverter.convertTxtAnchor`（`DefineConverter.java:59`） | `DefineConverter.rs` | 02 映射 |
| `TextBodyProperties.anchor()`（默认 2）、`leftInset/rightInset`（默认 91440）、`topInset/bottomInset`（默认 45720）、`verticalText`（默认 1） | 已有 Rust 结构 | 02 读取端 |
| `KmoShowShape.getOriginalAutoFit/getOriginalAnchor/resetOriginalProperty` | `KmoShowShape.rs`（扩） | 02 快照（reset 语义 待 loop 确认） |

不在改写列表：`updateAuotfit`/`checkWordArt`（那是 typo 期连锁）、`textShape3d/scene3d` 字段、warp 几何变换。

## 3. 测试方案

### 3.1 审计面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `textbox_props` | defaults | 空 `TextBodyProperties`（无 key） | left/right=91440→layout、top/bottom=45720→layout、anchor=2、vertical=1、wrap=true | 默认值 |
| `textbox_props` | wrap=0 | `textWrappingType()==0` | `isWrapTextInShape=false` | wrap 分支 |
| `textbox_props` | anchor 映射 | anchor=0/1/2/3/4 | convertTxtAnchor 各值 | 映射 |
| `textbox_props` | insets 覆盖 | 显式 left/right/top/bottom | 覆盖默认值 | 覆盖优先级 |
| `textbox_props` | autofit/warp | `textAutofit` none/norm + `presetTextWarp` | 字段落值但不缩放 | 解析不连锁 |
| `textbox_props` | old 快照 | shape original=-1 vs ≠-1 | 不写 vs 写 old TextAutoFit/Anchor | 快照分支 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| TP-DEF | textbox_props | 空属性对象 | 是 | TP1 |
| TP-WRAP | textbox_props | wrappingType 0/非 0 | 是 | TP2 |
| TP-ANCH | textbox_props | anchor 0-4 | 是 | TP3 |
| TP-INSET | textbox_props | 显式 inset 覆盖 | 是 | TP4 |
| TP-AUTOFIT | textbox_props | autofit type+fontScale+lnSpcReduction | 是 | TP5 |
| TP-VERT | textbox_props | verticalText 1/2/3/4 | 是 | TP6 |
| TP-OLD | textbox_props | original=-1 / ≠-1 | 是 | TP7 |
| TP-UNITS | emuToLayoutUnits | 91440/45720 | 与 01 口径一致 | TP8 |

### 3.3 可测试性
纯映射函数：喂 `TextBodyProperties` fixture + 空 `TextBoxData`，读回字段断言。不跑 `getTextOperator`、不跑 typo。测 N 只构造属性对象，值与 Java 默认对照。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| TP1 | TP-DEF | 空 props | textbox_props | left/right=91440→layout，top/bottom=45720→layout，anchor=2，vertical=1，wrap=true | 默认值漂移 |
| TP2 | TP-WRAP | wrappingType=0 | textbox_props | wrap=false；非 0 → true | 折行开关反 |
| TP3 | TP-ANCH | anchor=0..4 | convertTxtAnchor | 逐项与 `DefineConverter.java:59` 相等 | 映射错位 |
| TP4 | TP-INSET | 显式 inset 各 1 值 | textbox_props | 覆盖默认 | 默认覆盖显式 |
| TP5 | TP-AUTOFIT | autofit=norm+fontScale | textbox_props | textAutofit=1、fontScale 落值；无副作用 | 解析触发缩放 |
| TP6 | TP-VERT | verticalText=3 | textbox_props | textVerticalType=3 | 竖排开关丢失 |
| TP7 | TP-OLD | original=-1 / 5 | textbox_props | -1 不写 old；5 写 old | 快照条件反 |
| TP8 | TP-UNITS | 91440/45720 | emuToLayoutUnits | 与 `units.fromEmuY` 公式相等 | 混入 EMU 原值 |

### 3.5 位置
crate：`libs/ppt-core/Cargo.toml`
src：`libs/ppt-core/src/cn/wps/show/layout/dataaction/{ShapeDataTransform,DataTransform,DefineConverter}.rs`
test：`libs/ppt-core/tests/cn/wps/show/layout/dataaction/textbox_props.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_textbox_props

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_textbox_props \
  --src src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs \
  --src src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --src src/cn/wps/show/layout/dataaction/DefineConverter.rs \
  --report libs/ppt-core/layout/features/pl-04-text-box/functions/02_function_textbox_props/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_textbox_props --json`)
date: 2026-08-30 12:00

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/dataaction/DataTransform.rs` | 63/730 | 8.6% | 119/1043 | 11.4% |
| `src/cn/wps/show/layout/dataaction/DefineConverter.rs` | 27/98 | 27.6% | 13/37 | 35.1% |
| `src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | 15/69 | 21.7% | 22/102 | 21.6% |
| **function total** | **105/897** | **11.7%** |  |  |
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
当前 **PASS**。

### 3.9 Java↔Rust 结构对齐报告（loop 必跑；overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DefineConverter.java \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DefineConverter.rs \
  --report libs/ppt-core/layout/features/pl-04-text-box/functions/02_function_textbox_props/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 04:00

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapeDataTransform` | `koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | **PASS** | `setHolder`, `getTextOperator`, `covertTextBodyProperties`, `getPresentation` | — | — | **PASS** |
| `ShapeHolder` | `koffice/presentation/core/cn/wps/show/layout/dataaction/ShapeDataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | **PASS** | `ShapeHolder` | `setTextRect`, `hasFontSizeFit`, `clearFontSizeFit`, `setFontScale`, `getHolder` | — | **PASS** |
| `DataTransform` | `koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs` | **PASS** | `setHolder`, `getTextOperator`, `exToInternal`, `convertToBoxData`, `covertTextBodyProperties`, `covertTextProperties`, `updateColorConvContext`, `setSpanInfo`, `fetchSpanInfo`, `getHAlign`, `convertBullet`, `getBulletType`, `fetchHyperLink`, `getStrikethroughline`, `getUnderline`, `getLineSpacing`, `adjustPos`, `emuToLayoutUnits`, `pointToLayoutUnits`, `point100ToLayoutUnits`, `getPresentation`, `getFontName`, `getColorAlpha` | `getParaDirection`, `getPrstShdwAttr`, `fetchBulletColor`, `fetchBulletFont`, `fetchBulletSize`, `setBulletContent`, `getTextSpacing`, `getColor`, `getGradientFill`, `convertToBitmap`, `getLangID` | — | **PASS** |
| `BulletNumberItem` | `koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs` | **PASS** | `index`, `setParagraphIndex`, `type`, `setBulletType`, `level`, `setParagraphLevel`, `startAt`, `setStartAt`, `isEmptyParagraphText`, `setEmptyParagraphText` | — | — | **PASS** |
| `DefineConverter` | `koffice/presentation/core/cn/wps/show/layout/dataaction/DefineConverter.java` | `libs/ppt-core/src/cn/wps/show/layout/dataaction/DefineConverter.rs` | **PASS** | `convertPhHolder`, `convertTxtAnchor` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->