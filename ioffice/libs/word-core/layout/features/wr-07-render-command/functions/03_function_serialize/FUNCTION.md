# 03_function_serialize — 指令 JSON 序列化（Stage3 格式）

| 项 | 值 |
|----|----|
| feature | wr-07-render-command |
| **入口** | `RenderCommand → serde_json`（`#[derive(Serialize,Deserialize)]` 联合体 + 版本字段） |

## 1. 功能说明
跨端/对比用的稳定格式：`{version, commands: […]}`；命令为 tagged union（`t:"text"|"rect"|"line"`）。**Stage3 对比**（RUST_REWRITE_RULE）：Java LayoutRender 指令列表 vs 本 JSON——序列相等。字段顺序/命名固定（同一 docx 双跑 diff 的可再现性）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 |
|------|------|------|--------|
| serde/serde_json | word-core 依赖 | 序列化 | 已实现 |
| RenderCommand 类型 | 01/02（本 feature） | 指令 | 01/02 先行 |

## 3. 测试方案
### 3.2/3.4
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| S1 | SER-ROUND | 全指令回环 | 反序列化==原值 | 回环错 |
| S2 | SER-SNAPSHOT | 固定输入 | JSON 字符串稳定（快照） | 不稳定 |
| S3 | SER-VERSION | 版本字段 | 写入读取一致 | 版本错 |
| S4 | SER-ORDER | 指令顺序 | 顺序保持 | 乱序 |

### 3.3 可测试性
纯序列化直测 + golden JSON 快照（stage3 对比样本）。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/rendercommand/Serialize.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/rendercommand/Serialize.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_serialize
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 03_function_serialize \
  --src src/cn/wps/moffice/writer/rendercommand/Serialize.rs \
  --report libs/word-core/layout/features/wr-07-render-command/functions/03_function_serialize/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_serialize --json`)
date: 2026-08-22 10:00

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/rendercommand/Serialize.rs` | 6/6 | 100.0% | 8/8 | 100.0% |
| **function total** | **6/6** | **100.0%** |  |  |
<!-- coverage-report:end -->
布局校验：纯 Rust 序列化层（无 Java 直接对标；格式契约见 §1 与 RUST_REWRITE_RULE Stage3）。