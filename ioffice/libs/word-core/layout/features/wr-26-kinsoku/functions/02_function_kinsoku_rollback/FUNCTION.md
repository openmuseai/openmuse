# 02_function_kinsoku_rollback — kinsoku 行尾回退断点

| 项 | 值 |
|----|----|
| feature | wr-26-kinsoku |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `SubLineLayouterBase::rollbackIfCurLineEndsWithPartOfWord / doAlternativeRollback` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
kinsoku 行尾回退断点

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `SubLineLayouterBase,dispose,rollbackIfCurLineEndsWithPartOfWord,rollbackIfNextLineStartsWithHeadPuncts,rollbackIfCurLineEndsWithTab,doAlternativeRollback`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `paraEnv.kinsoku` | wr-26 01 | 布尔开关 | 是 | — | 禁止改 ParaLayouter |
| `PunctTool.isHeadPunct` | wr-25 | 行首禁则字 | 02 或本轮生成 | 其余 PunctTool 方法停车 | 禁止 stub PunctTool |
| `LineCompressTool` | wr-25 | 无 | 01/02 | 不改 compress | 禁止回退 wr-25 |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| rollback* | SubLineLayouterBase.java:285+ | kinsoku true/false | 断点回退 | 头标点 / 词中 |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| KR1 | kinsoku=false 不因禁则回退 | 关键分支 | 是 | KR1 |
| KR2 | kinsoku+头标点 | 公开入口 | 是 | KR2 |
| KR3 | 词中截断 | 关键分支 | 是 | KR3 |
| KR4 | tab 行尾 | 边界 | 是 | KR4 |
| KR5 | asiaTypography 交替 | 关键分支 | 是 | KR5 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| KR1 | KR1 | kinsoku=false | 行尾「你。」 | 不因禁则 prev | 误 rollback |
| KR2 | KR2 | kinsoku=true 头标点 | 下一行以「（」起 | rollback | 切开 |
| KR3 | KR3 | 西文词中 | 半词 | rollback 到词界 | 切半词 |
| KR4 | KR4 | tab 行尾 | rollbackIfCurLineEndsWithTab | 与 Java 同 | 丢 tab |
| KR5 | KR5 | doAlternativeRollback | asiaTypography | 走 kinsoku 支 | 走错支 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_kinsoku_rollback

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_kinsoku_rollback \
  --src src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs \
  --report libs/word-core/layout/features/wr-26-kinsoku/functions/02_function_kinsoku_rollback/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_kinsoku_rollback --json`)
date: 2026-09-06 09:51

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs` | 103/175 | 58.9% | 225/313 | 71.9% |
| **function total** | **103/175** | **58.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs \
  --report libs/word-core/layout/features/wr-26-kinsoku/functions/02_function_kinsoku_rollback/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 01:51

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SubLineLayouterBase` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/SubLineLayouterBase.rs` | **PASS** | `SubLineLayouterBase`, `dispose`, `rollbackIfCurLineEndsWithPartOfWord`, `rollbackIfNextLineStartsWithHeadPuncts`, `rollbackIfCurLineEndsWithTab`, `doAlternativeRollback` | `doAddExcessGraphs`, `rollbackWhileInMathPara`, `setLastGraphsPosPrevLimit`, `rollbackAndAlignGraphs`, `rollback`, `assertIsValidGraphsPos`, `checkCanAutoWordBreak`, `rollbackIfNextLineStartsWithTailPuncts`, `isRealSpace`, `isRealAsianGraphsPos`, `canSkipRollback`, `rollbackIfHangingSpaces` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
