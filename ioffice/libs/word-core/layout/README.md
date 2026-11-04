# Writer Layout → RenderCommand → Flutter

> 日期：2026-08-21  
> 目标：在 **parse MVP（wc-01..05 已 pass）** 之上，把 Java Android 排版/渲染管线改写成 **Rust 内核 + 平台无关 RenderCommand + Flutter 绘制** 的最小闭环。  
> 方法：与 parse 相同——先拆 **feature（wr-0X）**，再按 **I/O 边界**拆 **function**，每个 function 走 feature-function loop。

## 读这个目录的顺序

| 文档 | 用途 |
|------|------|
| [JAVA_RUST_PIPELINE.md](./JAVA_RUST_PIPELINE.md) | Java 全链类图/流程图/依赖闭合 + Rust 现状对照（本目录主技术文档） |
| [MEASURE_DRAW_INSTRUCTION.md](./MEASURE_DRAW_INSTRUCTION.md) | 测宽双引擎（FT vs Paint）原理、绘制/指令展开、断行连锁补救 |
| [SPLIT_PRINCIPLES.md](./SPLIT_PRINCIPLES.md) | 为何这样切 feature / function（对照 parse wc-0X） |
| [features/manifest.csv](./features/manifest.csv) | wr-01..wr-28 清单 |
| [state-java-l2-parity](../state-java-l2-parity/README.md) | **现行**下一波：Java 热路径 + 实排 L2（M11–M15） |
| `features/wr-0X-*/STRUCTURE_ANALYSIS.md` | 该类 feature 的类图、流程、must-write / stub / defer |
| `features/wr-0X-*/FUNCTIONS.md` | I/O function 组合图 |
| `features/wr-0X-*/functions/<id>/FUNCTION.md` | 单 function 契约 + 覆盖矩阵 + §3.6/3.7/3.8 |

## 一句话管线

```text
.docx
  → [parse  wc-01..05]  TextDocument（rope / CHPX / PAPX / KStyles / PLCSection）
  → [layout wr-01..06]  ViewEnv + 度量/断行/分页 → TypoSnapshot（页/行/run + cp）
  → [emit   wr-07]      RenderCommand[]（平台无关）
  → [paint  wr-08]      Flutter Canvas / TextPainter
```

内核 **禁止** 依赖 `android.graphics.Canvas` / `Paint` / `View`。Java 的 `IRenderDrawer` + `InstractionBuilder` 是 RenderCommand 的语义来源；Flutter 只消费指令。与 Android **截图像素级一致不是硬保证**，见 [PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md](./PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md)。

## 当前状态（2026-08-21 复核）

| 层 | 状态 |
|----|------|
| parse wc-05 | **7/7 pass**（styles 子链缺口 C 仍 ignore，不影响纯文本正文） |
| Typo* / ViewEnv / Layouter / LayoutRender | **Rust 不存在**（word-core `layout/base` 空目录且未进 `lib.rs`） |
| `core_libs_font` | **可编译**（FreeTypeJNI 已转写） |
| `core_libs_wpslayout` | **不可编译**；RichTextLayout 等为壳。正文度量 MVP **不走** RichTextLayout，走 FontStyleFactory → ITypeface |
| RenderCommand / UniFFI | **不存在** |
