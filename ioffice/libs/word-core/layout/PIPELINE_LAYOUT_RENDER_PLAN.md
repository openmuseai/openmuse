# Parser → Layout → RenderCommand → Flutter 开发方案与计划

> 日期：2026-08-21（相对初稿：已按 Java 源码复核；parse wc-05 已 7/7 pass）  
> 主技术文档：[JAVA_RUST_PIPELINE.md](./JAVA_RUST_PIPELINE.md)  
> 像素对齐 / Android 依赖对照：[PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md](./PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md)  
> 测宽双引擎与断行补救：[MEASURE_DRAW_INSTRUCTION.md](./MEASURE_DRAW_INSTRUCTION.md)  
> 拆分原理：[SPLIT_PRINCIPLES.md](./SPLIT_PRINCIPLES.md)  
> Feature 飞地：`layout/features/`（**不**写入 `parser/features/manifest.csv`）  
> 字体引擎（FontHost 注入表）：[FONT_HOST_DEV_PLAN.md](./FONT_HOST_DEV_PLAN.md)

目标：`TextDocument → TypoSnapshot → RenderCommand[] → Flutter` 最小闭环，再扩展完整排版。  
方法：feature（wr-0X）→ I/O function → feature-function loop（§3.6/3.7/3.8）。

---

## 1. 管线原理（复核后）

```text
.docx
  → [解析]  DocxReader + DocumentImporter（wc-05 pass）     → TextDocument
  → [装配]  LayoutManager（Java）/ 同步 layout()（Rust MVP）
            ViewEnv + FlowLayout → PageViewLayouter
  → [排版]  TextLayouter.flow → LineLayouter
            度量：FontStyleFactory → ITypeface → FreeType（正文不走 RichTextLayout）
                                                                      → TypoSnapshot（页=TypoLayoutPage，行=TypoTextLine）
  → [渲染]  Java: LayoutRender → IRenderDrawer → android.graphics.Canvas
            Rust: 同一遍历 → RenderCommand[] → Flutter
```

Java 类职责与 Android 切断点见 [JAVA_RUST_PIPELINE.md](./JAVA_RUST_PIPELINE.md) §2–§3。

**架构决策（钉死）：**

1. 接缝不变：cp 锚定 → ViewEnv → 页流 → Snapshot → 指令。
2. TypoSnapshot **语义对齐、不复刻 pointer arena**。
3. 度量唯一源：`core_libs_font` FreeType；内核禁止 `android.graphics.Paint`。这与「复制现网 Android 每一条 measure 回退」不是一回事——现网 `AndroidTypeface` 是 FT + Paint sticky 回退；要对齐现网断行必须 Stage2 双跑，见像素对齐文档。
4. 默认页：`PageSetup(11906, 16838, 1800, 1800, 1440, 1440)`，与 wc-05 06 默认节一致。
5. 内核无 `android.*`；`IRenderDrawer`/`InstractionBuilder` 是指令语义源。
6. MVP 同步 `layout()`，defer `CoreThread`。
7. 行类型名：**TypoTextLine**（Java 无 TypoLine）。

---

## 2. Java ↔ Rust 现状（2026-08-21）

| 层 | Java | Rust | Feature |
|----|------|------|---------|
| 解析 | DocxReader 全链 | **wc-05 7/7 pass** | 已完成 |
| TextDocument / PageSetup | core | **已在 lib.rs** | call-only |
| Typo* | cache/* | **无** | wr-01/06 |
| ViewEnv | layout/base/env | **无** | wr-02 |
| 度量 | FontHost + FreeTypeJNI + AndroidTypeface | FontStack + FontHost（注入表）；ttf-parser 度量 | wr-03 |
| 断行/分页 | Line/Text/MainLayouter | **无**（空目录） | wr-04/05 |
| LayoutRender | Canvas | **无** | wr-07 |
| Flutter | — | **无** | wr-08 |
| wpslayout RichTextLayout | 形状文本 | 壳且 **cargo 红** | 不阻塞正文 MVP |

---

## 3. 最小闭环（第一张 Flutter 页）

```text
fixture 或 wc-05 最小 docx
  → wr-02 ViewEnv A4
  → wr-03 FreeType 度量
  → wr-04 空格断行
  → wr-05 单节分页（可先单页）
  → wr-01+wr-06 TypoSnapshot
  → wr-07 DrawText + 页框
  → wr-08 Flutter CustomPainter（demo 可用 TextPainter；Stage3 对齐须按指令 x 逐字/逐 run 画，禁止 Paragraph 二次 shaping）
```

---

## 4. Feature 清单

见 [features/manifest.csv](./features/manifest.csv)。每个 feature 的类图/function 在 `features/wr-0X-*/`。

依赖序：`wr-01 ∥ wr-02 → wr-03 → wr-04 → wr-05 → wr-06 → wr-07 → wr-08`。

---

## 5. 验收

| Stage | 对比 | 容差 |
|-------|------|------|
| 1 | DocumentModel JSON | 严格相等 |
| 2 | TypoSnapshot JSON | x/y ±0.01px |
| 3 | RenderCommand 列表 | 序列相等 |
| 像素（L3） | 与 Android 设备截图 | **不做 CI 门闩**。几何用 Stage2/3；光栅差见 [PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md](./PIXEL_ALIGNMENT_ANDROID_VS_FLUTTER.md) |

每 function：矩阵非 ignore 全绿、`cargo test --test <id>`、coverage 脚本、模板布局 PASS。

---

## 6. 里程碑

| M | 内容 | 出口 |
|---|------|------|
| M0 | parse 已完成 | wc-05 pass（**已达成**） |
| M0.5 | wpslayout 审计（不阻塞 FreeType 路径） | 报告：壳分类 |
| M1 | wr-01/02 + wr-07 文本指令 + wr-08 骨架 | Flutter 画出第一行 |
| M2 | wr-03/04/05/06 | 多段分页；Stage2 |
| M3 | wr-07 装饰 + 分页符 | Stage3 |
| M4 | wr-08 滚动缩放 | 交互 |

---

## 7. 开发顺序建议（执行）

1. Split 文档（本目录，本提交）→ 各 wr 的 STRUCTURE / FUNCTIONS / FUNCTION.md 均为 `spec`。
2. **先 loop wr-01**（纯结构、无 font），同时可并行 wr-02、wr-07 enum + wr-08 fixture painter。
3. wr-03 接通 `core_libs_font` 到 word-core compile surface。
4. wr-04 → wr-05 → wr-06 串成 `layout_document(&TextDocument, &ViewEnv) -> TypoSnapshot`。
5. wr-07 emit；用 Java InstractionBuilder 日志或 Stage3 JSON 对齐。
6. wr-08 UniFFI/`cxx` 单入口 `render_docx(bytes) -> commands`。

---

## 8. 下一阶段：纯文本行级对齐（2026-08-23）

MVP（M1–M4 产品捷径）已在 macOS 画出真实字形。那 **不是** Java `LineLayouter` / `processPagination` 相等。

完整计划、I/O 锁与 feature 清单：[LINE_PARITY_PLAN.md](./LINE_PARITY_PLAN.md)。

| M | 出口 |
|---|------|
| M5 | wr-09/10：布局吃 TextDocument；标题/加粗进样式 |
| M6 | wr-11：同 TTF 度量可对 |
| M7 | wr-12：行 cp 与 Java 相等 |
| M8 | wr-13/14：分页 + Snapshot 行表 |
| M9 | wr-15/16：样式指令 + **行级 CI 全绿**（本波结束） |
| M10 | wr-17 起：光标 → 选区 → 编辑（不在本波） |

元素范围：纯文本 + 核心样式。禁止表/图。 wr-04/05 贪心实现保留作 Hello 回归。

## 9. 再下一波：Java 热路径 + 实排 Stage2（2026-09-03）

wr-16 比较器绿之后，产品仍未走 `TextLayouter.flow`。权威计划：[state-java-l2-parity/NEXT_PLAN.md](../state-java-l2-parity/NEXT_PLAN.md)。

| M | 出口 |
|---|------|
| M11 | wc-06 Stage1 实文档 + wc-07 几何 PAPX |
| M12 | wr-21 ParaLayouter 段框 |
| M13 | wr-22 TextLayouter.flow 正文 |
| M14 | wr-23 PageView MODE_PAGE 切换 `layout_document` |
| M15 | wr-24 当次实排 Stage2 vs Java（本波结束；忽略 xpos） |
