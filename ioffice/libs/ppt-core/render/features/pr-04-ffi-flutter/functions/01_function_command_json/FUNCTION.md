# 01_function_command_json — 指令 JSON 序列化/反序列化

> feature: pr-04-ffi-flutter ｜ 状态：**pass** ｜ Java 管线环节：S6（语义源 `android.graphics.Canvas`，**不做移植目标**）

## 0. 管线能力解锁

实现后：`RenderCommand[]` 变成稳定的 **JSON**——跨端可消费、可落盘、可做 Stage3 对比（对齐 writer `wr-07-03_serialize`：S15 输出即 Stage3 对比文件）。没有它，管线断在 S6 出口：指令只存在于内存数组，出不了内核（主文档断点表「pr-04（ffi）→ 指令出不了内核 → **出屏断**」）。

## 1. 功能说明

`Vec<RenderCommand> ↔ String(JSON)`，serde 双向。字段与 wr-07 的 DrawRect/DrawText 语义对齐（字段名待 loop 锁定）。JSON 里没有任何 `android.graphics.Canvas`/Paint 概念——只有平台无关字段（命令类型、坐标、颜色、样式）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `Vec<RenderCommand>`：`DrawRect{x,y,w,h,fill,stroke}`、`DrawPath{path,fill,stroke}`（pr-02）、`DrawText{text,x,y,font,size,color,bold,italic,...}`（pr-03）；或手写 fixture 指令。 |
| **输出** | JSON 字符串（`serde_json`）；`from_json` roundtrip 等值。字段命名与 writer wr-07-03 对齐（`DrawRect`/`DrawText` 同名，字段细节待 loop 确认）。 |
| **状态与副作用** | 纯序列化；完整性（`#[serde(deny_unknown_fields)]` 与否待 loop 定，以兼容 painter 增量字段为先）。 |
| **错误语义** | 无效 JSON → `Err` 非 panic；未知 variant → 反序列化 Err（Java 无对应；对齐 wr-07-03 语义）。 |

### 1.2-1.6 已并入 1.1（序列化纯函数）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| RenderCommand 模型 | pr-02-01 | DrawRect/DrawPath/DrawText 枚举 + 字段 | spec | 字段名 loop 对齐 wr-07 | 不改已定模型 |
| serde/serde_json | ppt-core Cargo.toml | derive/compose | 依赖待加 | — | — |
| wr-07 JSON 格式 | writer | 字段语义参照 | word-core 已有 | 名称对齐 | 不反向改 wr-07 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | 无对应类（语义源 `android.graphics.Canvas` 不移植；S15/S16 在 writer 侧也没有 JSON 类，序列化为新增） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/rendercommand/Serialize.rs`（新建）+ `RenderCommand.rs`（pr-02-01 已定义，加 derive） |

关键方法（对照，均已查证）：

| symbol | 语义 | Rust 目标 |
|---|---|---|
| wr-07 `03_serialize` | serde JSON，Stage3 对比格式 | `RenderCommand` 加 `Serialize/Deserialize` derive |
| wr-08-01 `uniffi_export` | `render_docx(bytes) -> commands` 单入口（壳） | 本 function 不落 uniFFI；`render_slide` 单入口调用点在 pr-04 loop 末 |
| serde_json roundtrip | `to_string`/`from_str` 等值 | `to_json(&[RenderCommand]) -> String`、`from_json(&str) -> Result<Vec<RenderCommand>>` |

改写列表：`RenderCommand.rs` 加 serde derive（若未加）；新写 `Serialize.rs`。

停车列表：UniFFI 导出（`cfg(feature = uniffi)` 待 loop）、自定义字段压缩/位编码（先完整 JSON，性能后议）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 wr-07 `03_function_serialize` 的 JSON 格式（word-core 已 pass）与 wr-08-01 的单入口语义（作参照）；Rust `RenderCommand.rs` 尚未存在（pr-02-01 spec）。以 wr-07 格式为对齐基准。

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| to_json | 3 种命令 | JSON golden | 空序列 |
| from_json | 合法/非法 | Roundtrip | Err |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| CJ01 | DrawRect roundtrip | 主路径 | 是 | T01 |
| CJ02 | DrawPath roundtrip | 主路径 | 是 | T02 |
| CJ03 | DrawText roundtrip | 主路径 | 是 | T03 |
| CJ04 | 空序列 roundtrip | 边界 | 是 | T04 |
| CJ05 | 非法 JSON → Err | 失败 | 是 | T05 |
| CJ06 | 未知 variant/字段 | 兼容 | 是 | T06 |

### 3.3 可测试性

纯数据 roundtrip，无需 fixture 文件（可内联构造命令）；golden JSON 字符串对照 wr-07 格式风格。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | CJ01 | DrawRect{x,y,w,h,fill,stroke} | from_json(to_json)==原值 | 红测停在本 function |
| T02 | CJ02 | DrawPath 带 path 数组 | roundtrip 等值 | 红测停在本 function |
| T03 | CJ03 | DrawText 全字段 | roundtrip 等值 | 红测停在本 function |
| T04 | CJ04 | [] | roundtrip [] | 红测停在本 function |
| T05 | CJ05 | 乱串 | Err（非 panic） | 红测停在本 function |
| T06 | CJ06 | 未知字段 | 解析策略钉死（skip 或 Err 待 loop） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/rendercommand/Serialize.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_command_json"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_command_json

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_command_json \
  --src src/cn/wps/show/render/rendercommand/Serialize.rs \
  --report libs/ppt-core/render/features/pr-04-ffi-flutter/functions/01_function_command_json/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_command_json --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/rendercommand.rs` | 6/113 | 5.3% | 8/106 | 7.5% |
| **function total** | **6/113** | **5.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

本 function 无 Java 文件（语义源 Canvas 不移植），模板布局校验不适用；对齐校验改用 wr-08-01 的 JSON 格式对照（loop 确认是否套用 check-template-layout）。

```bash
# 预留：若无 Java 底稿则跳过；如与 wr-08 联合打包再跑对应校验
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java libs/word-core/layout/features/wr-08-ffi-flutter/STRUCTURE_ANALYSIS.md \
  --rs libs/ppt-core/src/cn/wps/show/render/rendercommand/Serialize.rs \
  --report libs/ppt-core/render/features/pr-04-ffi-flutter/functions/01_function_command_json/FUNCTION.md
```

<!-- java-rust-align:begin -->
| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
| N/A (`rendercommand.rs`; Canvas not ported) | PASS (`--rs` only / no Java twin) | command I/O in this function | none | none | PASS |
| **overall** | | | | | **PASS** |
<!-- java-rust-align:end -->