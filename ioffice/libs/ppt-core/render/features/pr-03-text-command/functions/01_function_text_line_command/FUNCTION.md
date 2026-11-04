# 01_function_text_line_command — 排版行到 DrawText

> feature: pr-03-text-command ｜ 状态：**pass** ｜ Java 管线环节：S5（`ShapeTextRender._render`/`TextRender.render`）

## 0. 管线能力解锁

实现后：一个文本框的 typo 行变成 `DrawText{text,x,y}` 序列——文本第一次「可上屏」。没有它，管线断在 S5：pl-05 的行几何只是排版结果，没有指令出口（主文档：**pr-03 之前文本只存在于 typo 结果中，不上屏**）。

## 1. 功能说明

`文本形状（LayeredTypoShape）→ Vec<DrawText{text,x,y}>`。入口语义 = `ShapeTextRender._render` 的过滤 + `TextRender.render` 的行枚举（Rust 侧等价 `translate(typoRect.left/top) + renderText` 为「行坐标 + 文本区原点」相加）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 文本形状（含 `ShapeText`，`hasTypoText()`=hasRealText\|\|hasBullets）、shape 的文本区几何（`renderModel.getTextRect(w,h)`）、pl-05 行列表 `{text,x,y,w,h}`（布局单位）、drawFlags（**&16 = NOT_DRAW_PLACEHOLDER_TEXT → 占位符不画**）。 |
| **输出** | `Vec<DrawText{text,x,y,font:None,size:None,color:None,...}>`；原点 = `typoRect.left/top`（`RenderModel.getTextRect` 的 inset 结果 + 形状 rect 平移）+ 行 x/y；坐标经 `layoutRenderScaleX/Y` 换算为 render px。样式字段由 02 附着。 |
| **状态与副作用** | Java `TextRender.render` 有三个 canvas 语义：`canvas.scale(layoutRenderScale)`（Rust 换算进坐标）、`rotateCanvas`（artTransform/垂直文本，MVP 延后）、`translate(typoRect.left/top)+renderText`（Rust 指令平移）。flip：`ShapeUtil.calcShapeFlipEx` 修正后 `bFlip = flipH xor flipV` → 命令 `flip_h/flip_v` 元数据（镜像在 painter 层，内核不做坐标变换）。slide 缓存：`typoText()` 的 cache/add/remove 语义归 pl-05，02 不重复。 |
| **错误语义** | 无 Java 异常路径（`typoText` 返回 null → 直接 return 不画）；Rust 侧文字为空/无行 → 空序列，不许 panic。 |

### 1.2-1.6 已并入 1.1（行枚举纯函数）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 排版行 | pl-05 | 行列表 {text,x,y,w,h} | spec（MVP 贪心） | Java 一致版 | 不在此重排 |
| 文本区 | pl-05/02 | `renderModel.getTextRect` 语义（inset） | getTextRect 的 DataCal 语义未落地 | 预置几何文本框待 loop | 不改几何层 |
| ShapeText | render | hasTypoText/hasRealText/hasBullets | keep_refactor（`ShapeText.rs` 168 行） | — | 不改既有文件 |
| UnitsConverter | pl-01 | layoutRenderScaleX/Y | keep | — | — |
| drawFlags | pr-01/02 | &16（NOT_DRAW_PLACEHOLDER_TEXT） | 无 DrawFlags.rs | loop 确认 | — |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `render/ShapeTextRender.java`（`_render`）+ `render/TextRender.java`（`render(Canvas, RectF, RectF, ShowTypoShape, ShapeText, int)`） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/ShapeTextRender.rs` + `TextRender.rs`（均新建，loop 模板生成） |

关键方法（对照 Java，均已查证）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| `ShapeTextRender._render(ITypoShape)` | `DrawFlags.drawPHolderText(mDrawFlags) \|\| !isPlaceHolder` 才继续；`ShapeText shapeText = (ShapeText)renderModel.getTextObj()`；`isPlaceHolder \|\| showShape.isTextBox() \|\| (shapeText!=null && shapeText.hasTypoText())` 才画 | 过滤门（&16 + 内容判定） |
| `renderModel.getTextRect(w,h)` | `DataCal.getTextRect`：预置几何→pathData.getTextBoxRect()；自定义几何→geoTextRect 缩放；默认全框 `(0,0,w,h)` | 文本区 rect |
| flip | `ShapeUtil.calcShapeFlipEx(shape, flip, false)`；`flipH = flip.x!=0 ? !flipH : flipH`；`bFlip = (flipH&&!flipV)\|\|(flipV&&!flipH)` | flip_h/flip_v 元数据 |
| `TextRender.render(Canvas, RectF rcShape, RectF typoRect, ShowTypoShape, ShapeText, int drawFlags)` | `mTypoRect = typoRect / layoutRenderScale`；`getCache=drawThumbnail(flags)`；`textLayout = cache \|\| typoText(...)`；null→return；`translate(mTypoRect.left/top)` + `textLayout.renderText(canvas)` | `typoRect.left+line.x, typoRect.top+line.y` 行序列 |
| `TextRender.drawText(Canvas, IShowTextLayout, RectF)` | `canvas.translate(typoRect.left, typoRect.top); textLayout.renderText(canvas)` | 等价换算（无 canvas） |
| `typoText(...)` | `mShowTextLayout.layout(mTypoRect, shape, ctx, typoResize, callback)` + cache 语义（state 1/2） | pl-05 接口调用点 |
| `ShowTextLayout.layout` | `exToInternal`→`TextBoxData`→`ShowRichTextLayout().typo(...)` | pl-05（贪心） |

改写列表：`ShapeTextRender._render` 的过滤与文本区计算、`TextRender.render` 的坐标换算与行枚举；`typoText`/`_layout` 为 pl-05 调用点。

停车列表：`_createTextInfo`（TimingInfo）/`beginAnim`/`endAnim`、带 `stPara/endPara/offset` 的编辑超载、`getTextRects`、`typo(String[])`、`getFirstSpan`、`rotateCanvas`（warp/垂直）、`PromptResource` 提示文本、`modular` 子文本判定 `isModularGroupTextChild`。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `ShapeTextRender._render` 与 `TextRender.render(6 参)` 全量（重点：过滤门的三段式、`typoRect` 的 scale 前后语义、null textLayout 直接 return）。当前 Rust 无 `ShapeTextRender.rs`/`TextRender.rs`，以 Java 为准。

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| _render 门 | ph/&16/textbox/hasTypoText | 画/不画 | 三段 |
| getTextRect | 预置/自定义/默认 | inset rect | — |
| render 行枚举 | 行列表 | DrawText[] | textLayout null |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| T01 | 单行 → 1×DrawText（text/坐标） | 主路径 | 是 | T01 |
| T02 | 多行 → 每行一个 DrawText（坐标=typoRect+line） | 主路径 | 是 | T02 |
| T03 | getTextRect inset 生效 | 边界 | 是 | T03 |
| T04 | ph + &16 → 不产 DrawText | 过滤 | 是 | T04 |
| T05 | flipH（xor flipV）→ flip 元数据 | 元数据 | 是 | T05 |
| T06 | textLayout null / 空行 | 边界 | 是 | T06 |

### 3.3 可测试性

fixture：手写行几何（替 pl-05）+ `ShapeText`（fixture 真/假 hasTypoText）+ 表驱动 flags；断言 DrawText 数组与坐标；无 canvas。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | TA1 | 1 行 "Hello" + typoRect(10,10)（render px） | DrawText{text:"Hello", x,y=行原点} | 红测停在本 function |
| T02 | TA2 | 2 行 | 2×DrawText，y 按行高递进 | 红测停在本 function |
| T03 | TA3 | full rect vs inset | 文本区 ≠ 形状框 | 红测停在本 function |
| T04 | TA4 | flags&16 + isPlaceHolder | 无 DrawText | 红测停在本 function |
| T05 | TA5 | flipH=true, flipV=false | flip_h=true 元数据 | 红测停在本 function |
| T06 | TA6 | 行列表空 | 空序列 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/ShapeTextRender.rs（+ TextRender.rs 行枚举）
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_text_line_command"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_text_line_command

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_text_line_command \
  --src src/cn/wps/show/render/TextRender.rs \
  --report libs/ppt-core/render/features/pr-03-text-command/functions/01_function_text_line_command/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_text_line_command --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 27/113 | 23.9% | 25/106 | 23.6% |
| **function total** | **27/113** | **23.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/TextRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/TextRender.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/TextRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/TextRender.rs \
  --report libs/ppt-core/render/features/pr-03-text-command/functions/01_function_text_line_command/FUNCTION.md
```

<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
| N/A (`rendercommand.rs`; Canvas not ported) | PASS (`--rs` only / no Java twin) | command I/O in this function | none | none | PASS |
| **overall** | | | | | **PASS** |
<!-- java-rust-align:end -->