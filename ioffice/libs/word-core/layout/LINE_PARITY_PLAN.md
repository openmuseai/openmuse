# 纯文本行级对齐 — wr-09..20 计划（已落地）

> **现行下一波**（产品接到 Java `TextLayouter`/`ParaLayouter`，实排 Stage2）：[state-java-l2-parity/NEXT_PLAN.md](../state-java-l2-parity/NEXT_PLAN.md)（M11–M15）。本文件保留 wr-09..16 行级叶子 + wr-18..20 session 的历史约束。  
> 日期：2026-08-23（正文未改口径；产品路径现状以 state 目录 PIPELINE_OVERVIEW 为准）  
> 对照蓝皮书：[JAVA_RUST_PIPELINE.md](./JAVA_RUST_PIPELINE.md)  
> 拆分原理：[SPLIT_PRINCIPLES.md](./SPLIT_PRINCIPLES.md)  
> 上一波（macOS 可画字）：wr-01..wr-08 + `layout_document` MVP  
> Feature 飞地：`layout/features/wr-09-..`（function 已拆且 pass）

macOS 样例已经能构建并画出真实字形。那条路径证明了 **FontHost + 指令绘制** 可工作，**没有**证明断行/分页与 Java 相等。下一阶段把产品验收从「能看见字」换成「打印出来的行、页与 Java 一致」。

---

## 0. 目标（锁死）

| # | 约束 | 含义 |
|---|------|------|
| 1 | **元素范围** | 主文档纯文本。含标题、加粗、斜体、字号、字体族、段对齐/缩进/段前段后/行距。 **不含** 图片、表格、形状、文本框、页眉页脚、脚注、域结果对象、修订气球。 |
| 2 | **核心验收** | parse 产出的 TextDocument + 度量 + layout，Rust 与 Java **严格一致**。尤其 **分行、分页**。目标是 **行级一致**（同一页、同一行、同一 cp 区间），从而打印分页一致。 **仅允许行内字符像素级差异**（L3 光栅，非 CI）。 |
| 3 | **之后** | 同一条 Java layout 热路径继续补齐（仍限纯文本）；再在 TypoSnapshot 上做光标 / 选区 / 编辑回环。 |

本计划 **不** 改 parse 热路径，除非 Stage1 金文件证明缺字段（[JAVA_RUST_PIPELINE.md](./JAVA_RUST_PIPELINE.md) §6）。缺样式时修 wc-03/wc-02，不在 layout feature 里重写 OOXML。

---

## 1. 验收口径：L2-line（行级）

相对现有 Stage2/3，本波把门闩从「整条 `DrawText` 序列 + `xpos[]`」收成 **行几何**：

| 字段 | 必须与 Java 相等 | 容差 |
|------|------------------|------|
| `page_index` | 是 | 精确 |
| `line_index`（页内） | 是 | 精确 |
| `start_cp` / `end_cp` | 是 | 精确（哪些字在哪一行） |
| 行框 `y` / `height`（twip） | 是 | Stage2：±0.01px（约 ±1 twip 量级） |
| 行起始 `x`（对齐后） | 是 | 同上 |
| 行内各字 `xpos[i]` | **否** | 允许像素级差；不进 CI |
| 字形光栅 / AA / hinting | **否** | L3，不做门闩 |

**一句话：** 换行点、分页点、行框必须和 Java 一样；行内字与字之间可以差几个像素。

金文件形态（wr-16）：每行一条记录，不是完整 Stage3 `InstractionBuilder` 序列。P4 拉丁 Stage3 金文件继续守住 Hello 回归，**不**当作中文标题文档的行级门闩。

---

## 2. Java 管线 vs 当前 Rust MVP

Java 热路径（正文，**不**走 `RichTextLayout`）：

```text
.docx
  → DocxReader / DocumentImporter          → TextDocument（rope + CHPX + PAPX + KStyles + PLCSection）
  → LayoutManager 装配                       ViewEnv + FlowLayout + TypoDocument
  → FontStyleFactory（PropertySet → TypoFontStyle + ITypeface）
  → TextLayouter.flow → LineLayouter.flowLine
  → processPagination（页满 / \f / 寡妇孤儿 197）
  → TypoSnapshot（TypoLayoutPage / TypoTextLine）
  → LayoutRender → IRenderDrawer（InstractionBuilder）
```

当前 `layout_document`（产品入口）实际走的是：

```text
.docx zip
  → 抽出 word/document.xml
  → 正则拼接 <w:t>                         ← 丢掉 CHPX/PAPX/pStyle/标题/加粗
  → 合成一段默认字号 run
  → FontHost + measure_run
  → greedy wrap（空格优先；CJK 无空格则按字切）
  → 内容区高度累加分页                     ← 无 LASTLINEOFPAGE / 无寡妇孤儿
  → LayoutFrame / RenderCommand
```

| Java 环节 | Java I → O | 当前 Rust | 差距 |
|-----------|------------|-----------|------|
| S06 文档模型 | `.docx` → `TextDocument` | 产品 SAX：XWPF parse → DocumentImporter | `DocxReader` 仍 RecXwpf；styles.xml 未进 run |
| S07 装配 | TextDocument + PageSetup → ViewEnv | wr-02 默认 A4 常量 | 未读节 `PLCSection`；视图用 `ViewParams` |
| S08 样式 | CHPX+KStyle → `TypoFontStyle` | StyleMerge + 直接 rPr；view 字号仅在无 sz 时 | 主题字体；完整 styles 层叠边角 |
| S08 度量 | FreeType hmtx | FontHost + ttf-parser（wr-03） | 引擎可对齐；**输入样式不对** |
| S09 断行 | `LineLayouter` | `Wrap.rs` 贪心空格 | **不是 Java 断点**；CJK/标题会漂 |
| S10 行框 | 行高/基线/jc/段距 | `line_height` + `align_x` 简化 | 未接 PAPX 191/193/208… |
| S11 分页 | `processPagination` + `\f` | 高度累加 | **页断点会漂**；寡妇孤儿是 stub |
| S12 快照 | `TypoSnapshot` 为排版真相 | Typo* 结构在；产品 IR 是 `LayoutFrame` | 行级金文件应对 Snapshot 行表 |
| S13 命中 | cp ↔ 页/行/x | 未做 | 下一波 wr-17 |
| S14 指令 | 按 snapshot 走 `TypoTextRender` | 按 greedy 行 emit | 样式指令不完整 |
| S17 光栅 | Android Canvas | Flutter Skia | **允许差**（本波明确不管） |

wr-01..wr-08 的 function「pass」= **MVP 捷径可编译可画**，≠ Java `LineLayouter` / `processPagination` 相等。本波 **不** 在 wr-04 上继续堆功能假装已经对齐。

---

## 3. 元素与属性范围（本波 in / out）

**in（必须影响断行或行框的）：**

- 正文 Unicode 字符流（含中文）
- 段落样式：`pStyle` → Heading1..9（字号/加粗/对齐来自 KStyles 解析结果）
- 直接格式：加粗、斜体、字号、ascii/eastAsia 字体名
- 段属性：`jc` 左/中/右（不含两端对齐）、首行/左/右缩进、段前段后、行距
- 硬换段 `\r`、分页符 `\f`
- Java 默认寡妇/孤儿（PAPX 197）：为打印分页一致而必须做

**in（画出，但不作为换行输入）：**

- 下划线、字体颜色（emit deco / color；断行仍只看 advance + 行宽）

**out（本波禁止扩展）：**

- 表、图、drawing wrap、文本框、艺术字、`RichTextLayout`
- 页眉页脚、脚注尾注、批注、修订
- 分栏、多节换页尺寸、节首页不同页边距
- 两端对齐 / 分散对齐、tab、bidi、复杂脚本
- 主题字体、kerning、ligature、东亚标点挤压（若 Java 金文件证明必须，单开 feature，不塞进 wr-12）
- Flutter `TextPainter` 测宽或断行
- cosmic-text / 第二套 layouter
- L3 截图 CI

---

## 4. Feature 拆分（只拆 feature）

切在 **I/O**，不切在 Java 类，不在本波写 `FUNCTIONS.md`。

```text
wc-05 TextDocument（已有，只消费）
        │
        ▼
 wr-09  run-stream          主故事 run 流（cp + 文本 + CHPX/PAPX 引用）
        │
        ▼
 wr-10  font-style          PropertySet → 度量用 TypoFontStyle
        │
        ▼
 wr-11  run-metrics-parity  同 TTF：逐 cp advance / 行格
        │
        ▼
 wr-12  line-layouter       段内 → TypoTextLine[]（断点=Java）
        │
        ▼
 wr-13  page-layouter       行流 → 页（分页点=Java）
        │
        ▼
 wr-14  typo-stage2         提交 TypoSnapshot；Stage2 行表
        │
        ├──────────────────► wr-15 style-commands   带样式的 RenderCommand（行级门闩仍是 wr-16）
        │
        ▼
 wr-16  line-golden         双跑：Java 行表 vs Rust 行表  ← 本波出口

下一波（只锁 I/O，本波不实现）：
 wr-17  hit-test            Snapshot + (x,y) → cp
```

| feature_id | 目标（一句话） | I | O | 明确不做 |
|------------|----------------|---|---|----------|
| [wr-09](features/wr-09-run-stream/FEATURE.md) | 布局改吃 TextDocument，不再 zip 抽 `<w:t>` | `TextDocument` 主故事 | 有序 run：`start_cp/end_cp/text` + CHPX/PAPX 入口 | 表单元格、drawing、域展开、改 parse |
| [wr-10](features/wr-10-font-style/FEATURE.md) | 标题/加粗等进入度量样式 | CHPX + 段 `pStyle` + KStyles + rPrDefault | `TypoFontStyle`：族名、size、bold/italic | 主题字体、复杂脚本、高亮 |
| [wr-11](features/wr-11-run-metrics-parity/FEATURE.md) | 同一 TTF 上 advance 与 Java 可对 | wr-09 runs + wr-10 样式 + FontHost 字节 | 逐 cp width + run ascent/descent（twip） | 探盘、Paint sticky、假宽度断行 |
| [wr-12](features/wr-12-line-layouter/FEATURE.md) | **分行与 Java 一致** | 已度量 runs + `content_width` + PAPX 段框 | `TypoTextLine[]`：cp 区间 + 行宽 + 行高 + 对齐 x | 表/图绕排、bidi、justify、tab |
| [wr-13](features/wr-13-page-layouter/FEATURE.md) | **分页与 Java 一致** | 行流 + 内容区高 + `\f` + widowControl | `TypoLayoutPage[]` + `LASTLINEOFPAGE` | 多节换尺寸、分栏、表跨页 |
| [wr-14](features/wr-14-typo-stage2/FEATURE.md) | 排版真相回到 Snapshot | wr-13 页流 | `TypoSnapshot`；Stage2 行表 JSON | 指针竞技场、增量 `updatePageRanges` |
| [wr-15](features/wr-15-style-commands/FEATURE.md) | 把样式画出来（加粗/标题字号） | Snapshot | `RenderCommand[]`（bold/size/font_id） | 以 xpos 序列当本波门闩；表/图指令 |
| [wr-16](features/wr-16-line-golden/FEATURE.md) | **本波出口：行级双跑 CI** | 纯文本 corpus + 同一 TTF | pass/fail：行表 vs Java dump | 截图 CI；把表/图 fixture 塞进来 |
| [wr-17](features/wr-17-hit-test/FEATURE.md) | 光标基础（下一波） | Snapshot + 点 twip | `(page, line, cp)` | 选区绘制、编辑、CoreThread |
| [wr-18](features/wr-18-session-caret/FEATURE.md) | 编辑会话 + caret | session / Frame + 点 | `DocumentSession`；`Option<cp>` | 选区 FillRect、IME |
| [wr-19](features/wr-19-selection/FEATURE.md) | 选区反色 | Frame + cp 区间 | `DrawRect` `kind=selection` | 编辑、IME |

依赖序：`wr-09 → wr-10 → wr-11 → wr-12 → wr-13 → wr-14 → wr-15`；`wr-16` 从 wr-12 起即可用 fixture 行表，**整波出口以 wr-16 全绿为准**。wr-17 不阻塞 wr-16。

**产品入口切换：** `layout_document` 经 XWPF SAX + `DocumentImporter` 写入 `TextDocument`，再 `RunStream` → wr-10..15（2026-08-26）。`DocxReader` 测试仍 `RecXwpf`。wr-04 greedy wrap 只留回归。

---

## 5. 与 wr-01..08 的关系

| 已有 | 本波怎么用 |
|------|------------|
| wr-01 Typo 结构 | 复用；行/页写入 wr-14 |
| wr-02 PageSetup / ViewEnv | 复用 A4 默认；节几何仍单节 |
| wr-03 FontHost / FontStack | 度量引擎；**不**再探盘 |
| wr-04 greedy wrap | **保留作 MVP 回归**；行级对齐由 wr-12 替换，不在 wr-04 上加 Java 语义 |
| wr-05 高度分页 | 同上，由 wr-13 替换 |
| wr-06 snapshot 组装 | wr-14 接到真实行流；不重做竞技场 |
| wr-07/08 指令与 Flutter | wr-15 只加样式字段；Flutter 仍禁止测宽 |
| P4 Stage3 拉丁金 | 回归锁；不是中文标题的行级门闩 |

---

## 6. 金文件与双跑

1. **Corpus（纯文本，本波只加这类）：** 单段拉丁；多段；空格换行；无空格中文；Heading1/2 + 正文；段内加粗；窄页强制切字；含 `\f`；段前距导致换页；widowControl 打开时的段末两行。
2. **Java dump：** 每行 `page, line, start_cp, end_cp, x, y, width, height`（twip）。从现有 `TypoTextLine` / Stage2 导出，不必先对齐完整 Stage3。
3. **Rust dump：** 同一 schema，来自 wr-14 Snapshot。
4. **CI：** wr-16 比较上表；xpos / 截图不比。
5. **字体：** 与 Java 双跑同一 TTF 字节（FontHost 注入）。中文 fixture 必须有 cmap，禁止用 MvpLatin 冒充宋体去比断行。

---

## 7. 里程碑

| M | 出口 | 未完成则不算进入下一 M |
|---|------|------------------------|
| **M5** | wr-09 + wr-10 | layout 能列出带 pStyle/bold 的 run，仍可用假宽度 |
| **M6** | wr-11 | 同 TTF 下 run 宽与 Java ≤0.5 twip（沿用 S08） |
| **M7** | wr-12 | 至少拉丁+中文+标题 corpus 的 **行 cp 区间** 与 Java 相等 |
| **M8** | wr-13 + wr-14 | **页断点** + 行框 y 与 Java 相等 |
| **M9** | wr-15 + wr-16 | 样例画出标题/加粗；行级 CI 全绿；`layout_document` 切到新路径 |
| **M10** | wr-17 起 | 光标；再选区；再同步全量重排（编辑）。**不在本波实施** |

建议执行顺序与表格一致。禁止 wr-12 未绿时用 wr-15 的「看起来像标题」当完成。

---

## 8. 明确不做（整波）

- 拆 function / 写 `FUNCTION.md`（下一轮再按 I/O 切）
- 在 `libs/core-libs/stubs` 为 `cn.wps.*` 补工程 stub
- 为了 cargo 绿而空实现 `LineLayouter` 却宣称行级对齐
- 改 Flutter 去「修」断行
- 把表、图、页眉塞进 corpus「顺便做掉」
- 承诺与 Android 截图像素一致

---

## 9. 下一波预告（光标 / 选区 / 编辑）

在 wr-16 行级绿之后，才允许：

| 预告 | I | O | 仍不做 |
|------|---|---|--------|
| wr-17 命中 | Snapshot + 点 | cp | 选区反色 |
| wr-18 会话/光标 | `.docx` / session + 点 | heap session；caret cp | FillRect、IME |
| wr-19 选区 | Snapshot + cp 区间 | `FillRect` 指令 | 编辑 |
| wr-20 编辑重排 | `DocumentEvent` | 新 Snapshot | CoreThread / 增量缓存 |

编辑路径必须 **先** 改 TextDocument（parse/storage 已有），**再** 全量走 wr-09..14。禁止在 `LayoutFrame` 上直接改字。
