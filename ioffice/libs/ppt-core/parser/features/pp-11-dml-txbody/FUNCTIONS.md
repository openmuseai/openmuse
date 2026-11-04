# pp-11-dml-txbody — 文本框 txBody

> status: **pass** ｜ depends_on: `pp-01-pptx-parse`; `pp-02-shape-tree-parse`; `pp-10-dml-sppr`
>
> Java 管线环节：**S0 parse（txBody → KmoTextBody）→ S1 convert 输入的文本侧**。有字才必要；
> 本 feature 吃不排版、不渲染。

## §0 管线位置

```text
pp-01: token maps + HandlerCallback（pass）
  -> pp-02: p:spTree 事件 -> KmoShowShape/DgStore 行（pass；txBody 为 constructor fixture）
  -> pp-10: spPr -> ShapePr（形状可画）
  -> pp-11: p:txBody
       -> 01 text_body  段落/run 链 -> TextBody.paraLst（纯文本 + rPr 最小集）
       -> 02 text_body_props  bodyPr -> TextBodyProperties（insets/anchor/autofit/vert）
       -> pl-04 shape->TextBoxData / pl-05 框内断行 / pr-03 DrawText
```

## §1 function 清单与依赖序

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | `01_function_text_body` | `p:txBody` 子事件 + `TextBody` | `paraLst`（r/br/fld run 链 + `a:t` + rPr 最小集） | pp-02:02/03/04 | 02;pl-04-text-box | pass |
| 2 | `02_function_text_body_props` | `a:bodyPr` attrs + `TextBodyProperties` | insets/anchor/vert/wrap/autofit/rot（二十项） | 01 | pl-04-text-box;pr-03-text-command | pass |

01 是唯一 txBody 编排（bodyPr 分支转交 02 的 `BodyPropertiesHandler`）；lstStyle 分支（1114449）
回 pp-03 的 `TextListStyleHandler` 契约（本 feature 不实现，构造时不伪造）。

## §2 数据流

```text
p:txBody (TextBodyHandler)
  ├─ bodyPr(1114448) ─────> BodyPropertiesHandler ──> TextBodyProperties  ← 02 function
  ├─ lstStyle(1114449) ───> 归 pp-03（不实现，不进本层数据流）
  └─ p(1114562) 循环复用 ParagraphHandler
       ├─ pPr(1114531) ─────> 壳（对齐/缩进最小集随 pl-05）
       ├─ r(1114193)/br(1114618)/fld(1114619) ─> TextRunsHandler（mode 0/1/2）
       │    ├─ t(1114214) ──> putText + putLength
       │    └─ rPr(1114603) ─> TextCharacterPropertiesHandler（sz/b/i/u/strike/lang…）
       └─ endParaRPr(1114532) ─> 同 rPr 模板（壳）
  endElement：paraLst().forceWrite；GVML -> callback.onMergeThemeTypeface(run)
```

写入目标模型：`data/text/TextBody.rs`（953 行）、`data/text/TextParagraph.rs`（953 行）、
`data/text/TextRun.rs`（2089 行）、`data/text/TextBodyProperties.rs`（3209 行）、
`data/text/TextCharacterProperties.rs`（2922 行）——均在，测试直接断言。

## §3 测试面

- crate：`libs/ppt-core/Cargo.toml`；测试镜像根 `libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/textshape/text/`。
- 每个 function 一个 `[[test]]` name（`01_function_text_body` / `02_function_text_body_props`），
  与 [functions.csv](./functions.csv) 对齐。
- 测试输入全部内存 attrs/events（contract fixture）。
- **Rust 现状（审计表 `ppt_core_mvp_audit.csv`）**：textshape 族 handler 全是 `rewrite` 级垃圾模板——
  `TextBodyHandler.rs` 160 行（**10 个 todo! stub**，「local stub text types; 10 stubs」）、
  `ParagraphHandler.rs` 415 行（108 stub，「local stub TextParagraph/TextRun; **runs broken**」）、
  `TextCharacterPropertiesHandler.rs` 452 行（**49 stubs**，依赖全部本地 stub todo!）。
  **当前工作树这些 handler 已从模块接线注释**（`textshape/text/mod.rs` 等仅 `// pub mod …`），
  默认 crate 不编译。**loop 需重做而非增量实现**：`template --recursive` 重新生成 Java 结构模板，
  删本地 stub，改用 `data/text/*.rs` 真实模型。
- 覆盖/对齐数字：FUNCTION.md §3.7/§3.9 保持占位，本 split 不填。

## §4 串联验收

1. 01 用内存 events 构造含 2-3 段、每段多 run（r/br/fld 混合）的 `txBody` fixture，验证
   `paraLst` 顺序、run mode、`t` 文本与 rPr 最小集；GVML 回调恰好一次。
2. 02 用 bodyPr fixture 验证 insets/anchor/vert/wrap/autofit（三型 type）写入。
3. 01+02 组合 fixture 的 `TextBody` 可被 pl-04 `ShapeDataTransform.exToInternal` 的 contract 消费。
4. 不打开 part、不装配 slide；`cargo test` 绿。

## §5 推荐 loop 顺序

`01_function_text_body → 02_function_text_body_props`

（01 先行交付段落/run 链；02 的 bodyPr 由 01 的 `TextBodyHandler.enterSubElement` 分发接入，
两 function 可并行实现、01 提供 dispatch 契约。）

## §6 类级范围

| Java 类 | function | Rust 现状（审计表） |
|---|---|---|
| `TextBodyHandler`（`…/textshape/text/TextBodyHandler.java`） | 01 | `TextBodyHandler.rs` 160 行、**10 todo stub**（rewrite）→ **重做** |
| `ParagraphHandler` + nested `TextRunsHandler`/`RegularTextHandler`/`FieldTextHandler`（`…/paragraph/ParagraphHandler.java`） | 01 | `ParagraphHandler.rs` 415 行（108 stub，**runs broken**）→ **重做** |
| `TextCharacterPropertiesHandler` + nested `RightToLeftHandler`（`…/run/TextCharacterPropertiesHandler.java`） | 01（rPr 最小集） | `TextCharacterPropertiesHandler.rs` 452 行（**49 stubs**）→ **重做** |
| `BodyPropertiesHandler` + nested `NormalAutoFitHandler`/`PresetTextWrapHandler`/`FlatTextHandler`（`…/textshape/text/BodyPropertiesHandler.java`；写入 `data/text/TextBodyProperties.java`） | 02 | 无对应 .rs（Rust `data/text/TextBodyProperties.rs` 3209 行已是完整模型）→ 新建 handler |
| `TextParagraphPropertiesHandler`（`…/paragraph/`） | 01（壳） | paraPr 模型在 `data/text/TextParagraphProperties.rs`；handler 最小集随 pl-05 |
| `TextListStyleHandler` | —（pp-03） | 归 pp-03 `02_function_text_list_style`，本 feature 不实现 |
| `HandlerCallback.onMergeThemeTypeface` | 01 | pp-01:02 pass；GVML 分支按 null 判空 |

Rust 根路径前缀：`libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/`。元素/attr id
以 `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/xsd/dml/A.java` 为准（已逐项查证，见各
FUNCTION.md §2）。

## 文档

- 机器状态：[functions.csv](./functions.csv)
- function 方案：`functions/<function_id>/FUNCTION.md`
- 上游：pp-02（[FEATURE.md](../pp-02-shape-tree-parse/FEATURE.md)）、pp-10（[FEATURE.md](../pp-10-dml-sppr/FEATURE.md)）；
  下游消费方：layout [pl-04](../../../layout/features/pl-04-text-box/FEATURE.md)。