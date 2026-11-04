# wr-29-java-pipeline-cutover — Function 组合图

> 切分依据：**I/O 边界**，顺序 = 管线依赖（图节点 → prepareGraphs → flowTextLine → 产品切线 → 实排/App）。  
> 不按类切。复杂叶（Java >700 行）单独成 function，一批只 loop 一个。  
> 上游：P0 `BrHandler`；wr-21 `ParaLayouter.start`；wr-22 flow 骨架（`flowTextLine` 仍 DEFERRED）。  
> 下游：`samples/docx-mvp-render` FFI / Flutter。  
> 权威计划：`rewrite-review/milestones/layout-java-pipeline/03-file-rewrite-plan.md`

---

## 0. 一句话管线

```text
parse:  已通（SAX → TextDocument；P0 写入 '\f'）
layout: SrcNode/CharsNode/Graphs
        → GraphBuff.prepareGraphs → GraphReader.doPrepareGraphs → GraphFactory
        → TextLayouter.flowTextLine → LineLayouter.flow
        → PageViewLayouter.flowToPage → TypoSnapshot
render: snapshot 行 → RenderCommand → FFI → Flutter
```

---

## 1. Function 清单

| order | function_id | 阶段 | I | O | 上游 | 下游 |
|------:|-------------|------|---|---|------|------|
| 1 | `01_function_graph_nodes` | layout | LayoutState 列表槽位 | 真实 `SrcNode`/`CharsNode` 模块；`Graphs`/`LayoutState` 去掉 Copy 影子 | P0；Graphs 模板 | 02 |
| 2 | `02_function_graph_buff` | layout | `hasContents` 的 LayoutState | `GraphBuff.prepareGraphs` 调 `doPrepareGraphs`；`initFilter`/`clear` 不 panic | 01 | 03 |
| 3 | `03_function_graph_reader` | layout | hello rope+CHPX | `doPrepareGraphs` 填 `srcList`/`charsList`/`graphList`（**复杂叶** ~1560） | 02 | 04 |
| 4 | `04_function_graph_factory` | layout | graphs + FontHost | 字宽/基线写入 Graphs（**复杂叶** ~1160） | 03；wr-11 | 05 |
| 5 | `05_function_flow_text_line` | layout | 非空 graphList | 解开 `flowTextLine`/`LineLayouter.flow(&mut LayoutState)`；hello ≥1 `TypoTextLine` | 04；wr-12；wr-21 | 06 |
| 6 | `06_function_product_switch` | layout | `layout_text_document(docx)` | 删除 `attach_pageview_hot_path`；默认不调 `env::layout_lines`；Frame 来自 snapshot | 05 | 07 |
| 7 | `07_function_e2e_render` | render | 同 sha256 语料 | wr-24 实排 L2；FFI 指令；Flutter 画真实 docx（hello/sample2/multipage） | 06；wr-24 | 08 |
| 8 | `08_function_wrap_para_env` | layout | sample2.docx + WPS 全页 PNG | 折行 / 首行缩进 / 1.5 行距 / 页溢出续页 | 07 | 09 |
| 9 | `09_function_section_grid` | layout | sample2 sectPr `docGrid` | `gridHeight=312`；行高 468；WPS p1 不含「三、」 | 08 | App |

`status` 以 `functions.csv` 为准。`§3.6–§3.11` 在 loop 该 id 时写入 `FUNCTION.md`。

---

## 2. Java ↔ Rust 门面对表（D1）

| Java | Rust |
|------|------|
| `layout/base/graph/SrcNode.java` | `layout/base/graph/SrcNode.rs` |
| `layout/base/graph/CharsNode.java` | `layout/base/graph/CharsNode.rs` |
| `layout/base/graph/Graphs.java` | `layout/base/graph/Graphs.rs` |
| `layout/base/graph/GraphBuff.java` | `layout/base/graph/GraphBuff.rs` |
| `layout/base/graph/GraphReader.java` | `layout/base/graph/GraphReader.rs` |
| `layout/base/graph/GraphFactory.java` | `layout/base/graph/GraphFactory.rs` |
| `layout/base/TextLayouter.java` | `layout/base/TextLayouter.rs` |
| `layout/base/line/LineLayouter.java` | `layout/base/line/LineLayouter.rs` |
| `layout/pageview/PageViewLayouter.java` | `layout/pageview/PageViewLayouter.rs` |

禁止第二套类型名。`LayoutState` 内 `pub struct SrcNode;` 必须改为 `use` 上表模块。

---

## 3. 编译面

`CRATE_DIR` = `libs/word-core`。每个 function 一条 `[[test]]`，`name` = `function_id`，`path` = 入口类型的 `tests/` 镜像。

---

## 4. 验收顺序

01 → 02 → 03（复杂停）或 03 pass 后 04（复杂停）→ 05 → 06 → 07。  
07 必须跑 `samples/docx-mvp-render` 真实 docx UI（screenshot / FFI），不是只 `cargo test`。  
08 折行后 09 灌 `docGrid`，使 WPS p1 分页与 `page_count=8` 对齐。
