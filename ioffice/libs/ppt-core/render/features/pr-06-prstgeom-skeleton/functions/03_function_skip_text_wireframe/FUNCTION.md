# 03_function_skip_text_wireframe — 纯文本框不发射空 AABB

> feature: pr-06-prstgeom-skeleton ｜ 状态：**pass**

## 0. 管线能力解锁

文本框默认 `prst=rect` 且无 fill 时，`geometry_commands` 仍发空 `DrawRect`，Flutter 再默认黑描边（pr-07:03）。本 function 在 **指令侧** 不发射该几何；03 不代替 painter 修复。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `LayerItem::Shape`：`is_text_box` 或 `has_text_body`，`!sp_pr.hasFill()`，无 `media_bytes`/`media_r_id`。 |
| **输出** | `geometry_commands` 返回 `[]`。`emit_text` 仍可产生 `DrawText`（本 function 不断言文本内容）。 |
| **错误** | 有实色 fill 的文本框 **仍发** DrawRect（目录页色块+字）。有 media 的 sp 走 pp-18，不走 skip。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要 | 已实现 | 暂缓 | 禁止改 |
|---|---|---|---|---|---|
| `geometry_commands` | rendercommand.rs | 上述谓词 → [] | 一律 DrawRect | — | 不改 greedy_wrap |
| pr-07 painter | Flutter | 双保险 | 仍默认黑描边 | 本 function 不改 Dart | — |
| 02 path | 本 feature | 有 fill 的 pentagon 不 skip | spec | — | — |

## 2. 核心代码

改写：`geometry_commands` Shape 分支开头。停车：placeholder 隐藏字（DrawFlags，pr-03 已有）。禁改：`RenderPainter`（归 pr-07:03）。

## 3. 测试方案

### 3.1 审计

已读：`geometry_commands` 73-95；`mvp_pipeline.rs` 209-228 先 geo 再 text；`render_painter.dart` 85-88。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 必测? | test_id |
|---|---|---|---|
| C01 | 文本+无 fill → 0 条几何 | 是 | T01 |
| C02 | 文本+hasFill → 仍有 DrawRect | 是 | T02 |
| C03 | 无文本+无 fill 形状（装饰线）不误 skip | 是 | T03 |
| C04 | 有 media_bytes 不 skip（pp-18） | 是 | T04 |

### 3.3 可测试性

fixture `KmoShowShape` + `geometry_commands`。不跑 Flutter。

### 3.4 用例表

| id | 覆盖项 | 输入 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | is_text_box，hasFill=false | geo.is_empty() | 空 AABB |
| T02 | C02 | 文本+fill | 含 DrawRect | 字也没了框（错 skip） |
| T03 | C03 | 无文本 prst=line | 02 的 Path，非 [] | 线消失 |
| T04 | C04 | 文本+media_bytes | 不走 skip（DrawImage 在 render_slide） | 城市图被当成文本框 |

### 3.5 位置

```text
src: src/cn/wps/show/render/rendercommand.rs
test: tests/cn/wps/show/render/03_function_skip_text_wireframe.rs
```

`#[ignore = "spec: pr-06-03 skip wireframe"]`

### 3.6 脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_skip_text_wireframe

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_skip_text_wireframe \
  --src src/cn/wps/show/render/rendercommand.rs \
  --report libs/ppt-core/render/features/pr-06-prstgeom-skeleton/functions/03_function_skip_text_wireframe/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_skip_text_wireframe --json`)
date: 2026-09-09 21:05

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 45/183 | 24.6% | 64/241 | 26.6% |
| **function total** | **45/183** | **24.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 / 3.9

产品桥 N/A 布局。

<!-- java-rust-align:begin -->
tool: n/a (product I/O)

| Java class | layout | implemented | deferred | missing | result |
|---|---|---|---|---|---|
| (none owned) | N/A | T01–T04 skip 谓词 | — | — | **PASS** |

**overall: PASS** — product I/O; text wireframe skip green.
<!-- java-rust-align:end -->
