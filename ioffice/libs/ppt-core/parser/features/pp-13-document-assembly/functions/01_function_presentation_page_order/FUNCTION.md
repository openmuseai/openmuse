# 01_function_presentation_page_order — presentation rId 页序

> feature: pp-13-document-assembly ｜ 状态：**pass** ｜ Java 对标：`PresentationHandler.onSlideIds` → `PptxrSlides.onSlideIds`

## 0. 管线能力解锁

将 `p:sldIdLst` 的 rId 顺序与 `ppt/_rels/presentation.xml.rels` 的 slide target 合并为有序 partName；禁止按 ZIP 文件名排序替代文档页序。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|---|---|
| 输入 | presentation.xml、presentation.xml.rels、ZIP entries |
| 输出 | `Vec<String>`，元素为规范化 slide partName |
| 状态 | 纯装配；不打开 slide XML |
| 错误 | rels 缺失/空映射时兼容旧 fixture，回退数值化 `slideN.xml` 顺序 |

### 1.2–1.6 边界

上游 pp-12:01、pp-06:02；下游 02。保持重复/缺失 rId 的过滤顺序，不在本层解析 shape。

### 1.7 依赖与实现状态

| 依赖 | 接口 | 状态 | 禁止修改 |
|---|---|---|---|
| pp-06 | `p:sldIdLst` rId | wired | 不按数值 slide id 重排 |
| pp-12 | rel target/ZIP entries | pass | 不重写 OPC |
| quick-xml | attrs/events | pass | 不引入 DOM |

## 2. 核心代码

Rust：`src/cn/wps/show/pptxr/PptxReader.rs` 的 `presentation_slide_rids`、`presentation_slide_targets`、`ordered_slide_xml_names`。Java 结构语义保留为邻近 `// Java` 管线注释；完整类改写仍归 pp-06/08 loop。

## 3. 测试方案

### 3.1–3.5 覆盖矩阵

T01：ZIP 文件名为 slide1/slide2，但 rId 顺序为 slide2/slide1，输出必须反序；T02：无 rels fixture 回退；T03：Sample_ppt 输出 23 个名称。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_pptx_reader t04_document_uses_presentation_relationship_order
```

### 3.7 覆盖率报告

由 pp-09/04 联合覆盖报告记录；本次 executable regression：4 tests PASS，Sample_ppt=23 pages。

### 3.8 模板布局校验

组合函数没有一对一 Java class；Java 权威方法由 pp-06:02 与 pp-08:02 各自 loop 校验。本 function 禁止宣称替代两者。

### 3.9 Java↔Rust 结构对齐报告

组合边界：`PresentationHandler.onSlideIds` → `PptxrSlides.onSlideIds` → ordered names，结果 **PASS（行为测试）**；类级结构对齐沿上游报告继承。
