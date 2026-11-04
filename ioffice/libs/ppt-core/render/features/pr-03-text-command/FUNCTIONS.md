# pr-03-text-command — Function 组合图

> depends_on: `pl-05` text-typo（排版行）；`pl-01` units-view；`pr-01` slide-layers  
> 状态：**pass**  
> 排版行 → DrawText（MVP：纯文本行；bullet/动画 timing 不做）。

## §0 管线位置

S5 文本绘制（见 [PIPELINE_LAYOUT_RENDER_PLAN.md](../../../PIPELINE_LAYOUT_RENDER_PLAN.md) 热路径 S5 段）。上游 pl-05 给 typo 行几何，pl-06 给 `LayeredTypoShape`；下游 pr-04 消费 DrawText 指令。对齐 writer wr-07 `DrawText{字体/字号/色/坐标}`。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_text_line_command` | typo 行（pl-05）+ `renderModel.getTextRect(w,h)` + `_render` 过滤（drawPHolderText） | `DrawText{text,x,y}` 序列 | pl-05；pr-01 | 02 |
| 2 | `02_function_run_style_command` | `SpanInfo`（fontName/fontSize/color/bold/italic）+ 行定位 | 带样式的 `DrawText` | 01 | pr-04-ffi-flutter |

指令模型（MVP 稳定版）：

```text
DrawText { text, x, y, font: String, size: f32, color: Rgba255, bold: bool, italic: bool, flip_h: bool, flip_v: bool }
```

## §2 数据流

```text
LayeredTypoShape（文本形状）
  ShapeTextRender._render 语义：
    DrawFlags.drawPHolderText(flags)=false && isPlaceHolder → 不画
    shapeText = renderModel.getTextObj()（ShapeText）
    isPlaceHolder || isTextBox || shapeText.hasTypoText() → 继续
    typoRect = shapeRect 平移至原点 → renderModel.getTextRect(w,h)（inset 文本区）
    flipH/flipV 计算（ShapeUtil.calcShapeFlipEx），bFlip=flipH xor flipV → 命令元数据
  TextRender.render 语义：
    typoRect /= layoutRenderScaleX/Y（布局单位）
    IShowTextLayout = pl-05 快照（cache 或 typoText()）
    drawText = translate(typoRect.left/top) + textLayout.renderText(canvas) —— Rust 侧等价：
      逐行/逐 span 产出 DrawText{x=typoRect.left+line.x, y=typoRect.top+line.y, ...}
  02 附着 span 样式（fontName/fontSize/color/bold/italic）→ DrawText
```

## §3 测试面

- fixture：一行纯文本 TextBoxData（pl-04/05 或手写行几何）+ 两个 run 的样式。
- 断言：DrawText 数量=行数（或 run 数）、坐标、font/size/color 附着、ph 过滤、flip 元数据。
- 不跑真实 `PptxReader`；绘制不经 canvas（就算有 Flutter，坐标对齐另开像素门闩）。

## §4 验收

1. 单行纯文本 → 恰好 1 个 DrawText（text/坐标正确，layout→render px）。
2. `getTextRect(w,h)` inset 生效（文本区 ≠ 形状框）。
3. run 样式附着：fontName/size/color/bold/italic。
4. ph 占位符 + `drawPHolderText` 关闭（&16）→ 不产 DrawText。
5. flipH（xor flipV）→ flip 元数据随命令（镜像在 painter 层）。

## §5 loop

`01 → 02`（02 依赖 01 的行定位）。pr-01 → pr-02 → pr-03 → pr-04。**建议 pass 口径**：01/02 各必测 3 条（坐标、过滤、样式）；bullet/timing/编辑选区从必测移附录。

## §6 Java/Rust 范围

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `render/ShapeTextRender.java` | `src/cn/wps/show/render/ShapeTextRender.rs`（新建） | `_render` 的过滤 + getTextRect + flip 计算必写；`_createTextInfo`（TimingInfo）停车；`getTextRect(env,typoShape)`/`isTextDrawExist` 边缘接口待 loop |
| `render/TextRender.java` | `src/cn/wps/show/render/TextRender.rs`（新建） | 无参 `render(canvas, rcShape, typoRect, typoShape, shapeText, drawFlags)` 语义必写（行枚举）；startPara/endPara 超载与 `getTextRects`/`typo`/`getFirstSpan` 停车 |
| `render/ShapeText.java` | `src/cn/wps/show/render/ShapeText.rs`（已有 168 行） | 复用（keep_refactor）；`hasTypoText`/`hasRealText`/`hasBullets` 已实现 |
| `core-libs/drawing` RenderModel/Shape | `drawing::cn::wps::moffice::drawing::Shape` | `getTextRect`/`getTextObj` 判读（getTextRect 的 DataCal 语义内置或待 loop） |
| `show/layout/ShowTextLayout.java` | pl-05 | `layout(...)` 语义在 pl-05 落地（MVP 贪心）；本 feature 只消费行几何 |
| `render/util/ShowShapeEnv.java` | —（语义源） | `setTextRender(new ShapeTextRender(slide, drawFlags))` 装配点入 01 |
| `UnitsConverter.layoutRenderScaleX/Y` | `base/converter/UnitsConverter.rs`（已有） | 换算复用 |

## 文档

- [functions.csv](./functions.csv)