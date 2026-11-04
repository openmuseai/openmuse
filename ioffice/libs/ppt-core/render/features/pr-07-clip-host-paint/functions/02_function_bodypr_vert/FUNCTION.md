# 02_function_bodypr_vert — bodyPr vert 竖排

> feature: pr-07-clip-host-paint ｜ 状态：**pass**

## 1. I/O 契约

| | 契约 |
|---|---|
| **输入** | `a:bodyPr vert="eaVert"` 或 `"vert"`（Sample 目录页 CONTENTS）。Java `DGMap.textVerticalTypeMap`：`eaVert=4`。属性 id **1114480**。 |
| **输出** | `TextBodySurf`/`bodyProperties.verticalText=Some(4)`；`emit_text` 在框内竖排（逐字沿框高，骨架级）。 |
| **错误** | 无 vert → 横排。未知 vert 字符串 → 横排，不 panic。 |

已读：`BodyPropertiesHandler.rs` 104-109 已有 1114480；`attr_id("vert")==1114480`；`element_id("bodyPr")=1114448`。`emit_text` 对 `vertical_text==4` 骨架竖排，不走 RichTextLayout。

### 1.7

| 依赖 | 已实现 | 禁止改 |
|---|---|---|
| BodyPropertiesHandler | Java 对齐分支在 | 不改 DGMap 其它表 |
| attr_id("vert") | 1114480 | PptxrSlide |
| emit_text | eaVert 骨架竖排 | 不重写 RichTextLayout |

## 3. 测试

| coverage_id | test_id |
|---|---|
| C01 attr_id("vert")==1114480 | T01 |
| C02 eaVert → verticalText=4 | T02 |
| C03 无 vert → None，横排 DrawText | T03 |

`test: tests/cn/wps/show/pptxr/02_function_bodypr_vert.rs`

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_bodypr_vert

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_bodypr_vert \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs \
  --report libs/ppt-core/render/features/pr-07-clip-host-paint/functions/02_function_bodypr_vert/FUNCTION.md
```

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_bodypr_vert --json`)
date: 2026-09-09 21:25

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs` | 18/159 | 11.3% | 34/211 | 16.1% |
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 544/900 | 60.4% | 884/1617 | 54.7% |
| **function total** | **562/1059** | **53.1%** |  |  |
<!-- coverage-report:end -->

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.java
```

Loop 后布局：**PASS**。禁 `--force` PptxrSlide。

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs \
  --report libs/ppt-core/render/features/pr-07-clip-host-paint/functions/02_function_bodypr_vert/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-09 13:24

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `BodyPropertiesHandler` | `../../../../koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.java` | `../../../../libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/textshape/text/BodyPropertiesHandler.rs` | **PASS** | `BodyPropertiesHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
