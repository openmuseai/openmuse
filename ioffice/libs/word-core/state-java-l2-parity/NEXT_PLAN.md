# 下一步：Java 管线 L2 对齐（仅允许行内像素偏差）

> 基线：[PIPELINE_OVERVIEW.md](./PIPELINE_OVERVIEW.md)  
> 拆分：[features.csv](./features.csv) + 各 `FEATURE.md` / `FUNCTIONS.md`  
> 对照蓝皮书：[layout/JAVA_RUST_PIPELINE.md](../layout/JAVA_RUST_PIPELINE.md) · Java 环节 [layout/docs/java/](../layout/docs/java/)

## 0. 目标（锁死）

1. **元素范围（本波）：** 主故事纯文本。标题/加粗/字号/字体族、jc 左中右、首行+左右缩进、段前段后塌缩、行距、`\r`/`\f`、寡妇孤儿最小集。与 [LINE_PARITY_PLAN.md](../layout/LINE_PARITY_PLAN.md) §3 in 相同，并补上 **左右缩进进内容宽** 与 **ParaLayouter 段距规则**。
2. **核心验收：** `layout_document`（及 session layout）对 corpus **实排** 出的行表，与 Java Stage2 dump **L2 相等**。不再用「预导出 Rust JSON vs 预导出 Java JSON」冒充产品路径。
3. **行内：** `xpos[]`、Skia/Impeller 光栅允许像素差；CI 不比。

Parse **只**修 Stage1 里会影响 L2 的桶（PAPX/CHPX/pStyle）。缺字段修 wc-03/SAX，不在 layout 里重写 OOXML。

## 1. 为何还要一波

| 已绿 | 仍不等于 Java |
|------|----------------|
| wr-09..16 function | 产品编排是 `LayoutDocument` 壳；`TextLayouter.java` 未接线 |
| wr-16 corpus pass | 比较器吃静态 JSON，不调用当前 layout |
| sample2 可画、jc 已进 PAPX | 左右缩进 210/212、段距塌缩、行距 220 对象未按 ParaLayouter |
| `LineLayouter.rs` 模板在 | `j2r_ty` 桩；产品走 `env::LineLayout` |

## 2. Feature 依赖（I/O）

```text
wc-05 TextDocument
   │
   ├─ wc-06-stage1-live     实文档 Stage1 dump vs Java（门闩，先于改断行）
   └─ wc-07-papx-geometry   SAX/convert 补齐 208–213 / 214/217/220/197
            │
            ▼
     wr-21-para-layouter    PAPX → ParaEnv（段距/行距/缩进/jc）
            │
            ▼
     wr-22-text-flow        TextLayouter.flow / doFlowLine / postFlow
            │
            ▼
     wr-23-pageview-main    MODE_PAGE=0：flowToPage → 提交页
            │
            ▼
     wr-24-live-stage2      产品实排 Stage2 vs Java dump  ← 本波出口

后波（只锁 I/O，本波不 loop）：
  wr-25-line-compress   整行压缩后再 rollback
  wr-26-kinsoku         标点禁则（金文件证明必须才 loop）
  wr-27-table-layout    TableLayouter
  wr-28-drawing-wrap    DrawingLayouter
  wc-08-numbering-apply AutoNum 串 + 悬挂
  wc-09-hdrftr-story    页眉页脚故事
```

禁止：在 wr-04 `Wrap.rs` 上继续堆 Java；用 Flutter 测宽冒充 wr-22；为过 CI 改 Java dump 的 cp 切分。

## 3. 里程碑

| M | 出口 | 未完成则不算进入下一 M |
|---|------|------------------------|
| **M11** | wc-06 + wc-07 | sample2/heading corpus：Stage1 含 193/208/210/217/214；缺 sprm 先修 SAX/convert |
| **M12** | wr-21 | 同一 PAPX：段前/段后/行距/左右+首行缩进与 Java ParaLayouter 单测一致 |
| **M13** | wr-22 | `flow` 只走正文 `flowTextLine`；table/frame 分支明确 defer |
| **M14** | wr-23 | `layout_document` 切到 PageView 热路径；env 壳仅回归 |
| **M15** | wr-24 | 实排 L2 corpus 全绿（含 sample2 标题居中行框，**不**比 xpos） |

执行顺序必须 M11→M15。禁止 wr-24 用「看起来居中」的截图代替行 `x`。

## 4. 金文件

1. **Stage1（wc-06）：** rope 文本 + 每段 PAPX 关键 sprm + 每 run CHPX sz/b/eastAsia。Java 从 koffice `TextDocument` dump。
2. **Stage2（wr-24）：** 每行 `page,line,start_cp,end_cp,x,y,width,height`（twip）。沿用 wr-16 schema；**Rust 侧必须来自当次 `layout_document`/`layout_text_document`**。
3. **字体：** 与 Java 同一 TTF 字节注入。CJK 禁止 MvpLatin。
4. **Corpus 至少：** wr-16 五类 + `samples/.../sample2.docx`（标题 18pt 居中加粗、正文首行缩进）。

## 5. Loop 约定

每个 function：`FUNCTION.md` §1.7 / §3.6–§3.9；`tests/` 镜像；`[[test]]` 在 **进入该 function 的 feature-function loop** 时再挂 Cargo.toml（Split 已写方案与路径，避免未实现 src 把 workspace 测红）。

改写走 **function-java2rust**：完整交错模板；列表外 `todo!("DEFERRED_NON_FUNCTION")`；`check-template-layout.mjs` PASS。

**第一个 loop（用户点名后再开）：** `wc-06-stage1-live` / `01_function_rope_dump`。顺序：wc-06 全部 → wc-07 → wr-21 → wr-22 → wr-23 → wr-24。禁止跳过 Stage1 金文件去改断行。

话术：`loop 01_function_rope_dump` 或「开始 loop wc-06」。

## 6. 明确不做（整波）

- L3 截图 CI、承诺 Android 帧缓冲一致
- 表/图/页眉排版（wr-27/28、wc-09）
- 把 `LineLayouter.rs` 的 `j2r_ty` 桩当产品类型
- 改 `.java` 或删 `// java` 行
