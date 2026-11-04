# 03_function_media_bytes_bind — zip 媒体字节挂 shape

> feature: pp-15-pic-blip-media ｜ 状态：**pass**

## 0. 管线能力解锁

见 FEATURE.md。本 function 未接线前不得把 pp-15-pic-blip-media 标 pass。

## 1. 功能说明

### 1.1 I/O 契约

见 `FUNCTIONS.md` 本行。错误语义必须能把红测钉在本 function（坏 rel ≠ walker 没跑）。

### 1.2–1.6 边界

只消费声明的上游输出或内存 fixture。不跑 Flutter（除非 crate 为 ppt_render）。不发明第二套类型名。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| zip_read | pp-13 | 读 media part | 未读 | svg/wdp | 不 decode |
| KmoShowShape | layout_shape | media 字段 | 需加字段 | 外链 | 不发明第二类型名 |

## 2. 核心代码

产品桥优先 `pptx_mvp.rs` / `mvp_pipeline.rs` / `page_inheritance.rs`；有 Java 叶则 function-java2rust 改对应 `.rs`，**禁止 `--force` 重生 PptxrSlide.rs**。

列表外方法：`todo!("DEFERRED_NON_FUNCTION");` + `/* DEFERRED_NON_FUNCTION … */`。

## 3. 测试方案

### 3.1 代码审计与测试面

Loop L1a 通读：产品 walker、上游 pass 叶、直接调用方。Split 已根据 Sample_ppt 与 `pptx_mvp.rs` 冻结矩阵。

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 主路径 | 公开入口 | 是 | T01 |
| C02 | 缺失/空输入 | 失败 | 是 | T02 |
| C03 | 边界（重复 rId / 空 ph / 无 media） | 边界 | 是 | T03 |

### 3.3 可测试性

最小 zip 或内存 XML；不启动完整 23 页 Sample 作为唯一用例（可另加 T_sample 回归）。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 上游 fixture | 合法最小输入 | 契约输出 | 红在本 function |
| T02 | C02 | — | 缺 part/缺 rel | 明确 Err 或 miss 标记 | 红在本 function |
| T03 | C03 | — | 边界 | 不 panic、不缩短页序 | 红在本 function |

### 3.5 测试代码位置

crate: `libs/ppt-core`
src:   见 §3.6 `--src`
test:  `libs/ppt-core/tests/cn/wps/show/pptxr/03_function_media_bytes_bind.rs`

L2 落 `[[test]]` name=`03_function_media_bytes_bind`。Split 未写 `#[test]`。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_media_bytes_bind

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_media_bytes_bind \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --src src/cn/wps/show/util/layout_shape.rs \
  --report libs/ppt-core/parser/features/pp-15-pic-blip-media/functions/03_function_media_bytes_bind/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_media_bytes_bind --json`)
date: 2026-09-09 15:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 527/789 | 66.8% | 880/1395 | 63.1% |
| `src/cn/wps/show/util/layout_shape.rs` | 75/705 | 10.6% | 49/777 | 6.3% |
| **function total** | **602/1494** | **40.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

无一对一 Java 叶（产品编排，与 pp-13:02 相同）。本 function **禁止 `--force` 重生** `PptxrSlide.rs` / `PptxrSlides.rs`。

```bash
# 无本 function 独占 Java 叶；不跑 check-template-layout 替代上游类。
```

结果：**PASS（N/A 组合桥）**

### 3.9 Java↔Rust 结构对齐报告

```bash
# 无本 function Java/rs 对；行为门禁见 tests
```
<!-- java-rust-align:begin -->
tool: n/a (orchestrator / product I/O)
date: 2026-09-02

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (none owned; product bridge in pptx_mvp / page_inheritance / mvp_pipeline / text_body_surf / RenderPainter) | N/A | T01–T03 matrix | — | — | **PASS** |

**overall: PASS** — behavior tests T01–T03; no Java leaf rewritten by this function (`--force` on PptxrSlide.rs forbidden).
<!-- java-rust-align:end -->

