# 04_function_no_layout_geom_in_render_slide — render_slide 热路径不读 layout geom

> feature: pr-08-render-from-public-shape ｜ 状态：**spec** ｜ Java 管线：S9 几何/填线（输入必须是 S5 公开 Shape）

## 0. 管线能力解锁

结构对齐总闸：禁止并行维护第二套几何。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | render_slide 源 |
| **输出** | 不读 KmoShowShape.sp_pr.geom_obj（grep 门闩） |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| render_slide | mvp_pipeline:290 | 产品出图 | 仍混 layout 字段 | — | 禁止第二套 geom |
| pp-18 DrawImage | 回归 | media 指令 | pass | 解码是 pr-12 | 不得改回 layout 捷径 |

## 2. 核心代码与函数改写列表

**本 function：** grep 门闩 + 回归 pr-02/pr-06/pp-18。media 走 Shape 上 blip，不得改回 layout 捷径标 pass。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 `render_slide` pic 分支仍 `s.shape_type==2 || media_bytes`（layout_shape 字段）。本测要求热路径改为公开 Shape / app Kmo。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| NG-GREP | mvp_pipeline+rendercommand | rg sp_pr.geom_obj | 热路径零命中 | 门闩 |
| NG-REG06 | 02_function_prst_path | prst 页 | 仍绿且 path 来自 getShape | 回归 |
| NG-REG18 | 02_function_sp_blip_drawimage | sp 图 | 仍有 DrawImage | 回归 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| geom_obj 门闩 | mvp_pipeline+rendercommand | 门闩 | 是 | NG-GREP |
| 回归 pr-06 | 02_function_prst_path | 回归 | 是 | NG-REG06 |
| 回归 pp-18 | 02_function_sp_blip_drawimage | 回归 | 是 | NG-REG18 |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| NG-GREP | geom_obj 门闩 | — | rg sp_pr.geom_obj | 热路径零命中 | 仍读 fixture geom |
| NG-REG06 | 回归 pr-06 | — | prst 页 | 仍绿且 path 来自 getShape | 骨架测改断言 |
| NG-REG18 | 回归 pp-18 | — | sp 图 | 仍有 DrawImage | 丢图 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/render/mvp_pipeline.rs, src/cn/wps/show/render/rendercommand.rs
test:  tests/cn/wps/show/render/04_function_no_layout_geom_in_render_slide.rs
[[test]] name = "04_function_no_layout_geom_in_render_slide"
```

`#[ignore = "spec: 04_function_no_layout_geom_in_render_slide"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_no_layout_geom_in_render_slide -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_no_layout_geom_in_render_slide \
  --src src/cn/wps/show/render/mvp_pipeline.rs \
  --src src/cn/wps/show/render/rendercommand.rs \
  --report render/features/pr-08-render-from-public-shape/functions/04_function_no_layout_geom_in_render_slide/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

§3.8 **N/A**（产品编排）。I/O 测试绿 + 禁止第二套类型名。

```bash
# 产品编排 / Flutter：无一对一 Java 叶。§3.8 标 N/A；不得 --force 邻近健康模板。
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  # n/a product I/O \
  --report render/features/pr-08-render-from-public-shape/functions/04_function_no_layout_geom_in_render_slide/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | NG-GREP, NG-REG06, NG-REG18 | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
