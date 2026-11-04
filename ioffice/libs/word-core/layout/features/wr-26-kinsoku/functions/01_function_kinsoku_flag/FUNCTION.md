# 01_function_kinsoku_flag — PAPX 201 → paraEnv.kinsoku

| 项 | 值 |
|----|----|
| feature | wr-26-kinsoku |
| order | 1 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `ParaLayouter::onStart / calcProp kinsoku=getBoolean(201,true)` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
PAPX 201 → paraEnv.kinsoku

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/layout/base/ParaLayouter.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `onStart` 已实现路径中的 `paraEnv.kinsoku = getBoolean(201,true)`；本 function 不新增列表外方法
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `PropertySet.getBoolean(201,true)` | util | 缺省 true | 是 | — | 禁止改 PropertySet |
| `ParaEnv.kinsoku` | wr-21 | 字段读写 | 是 | — | 禁止改无关 ParaLayouter 方法 |
| `SubLineLayouterBase` | wr-26 02 | 无 | 否 | 本 function 不生成 | 禁止改 rollback |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `kinsoku=` | ParaLayouter.java:81 | 201 缺省/true/false | paraEnv.kinsoku | 缺省 true |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| K1 | 缺省 | 公开入口 | 是 | K1 |
| K2 | 201=false | 关键分支 | 是 | K2 |
| K3 | 201=true 显式 | 公开入口 | 是 | K3 |
| K4 | onStart 后仍可读 | 状态转换 | 是 | K4 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| K1 | K1 | 无 201 | onStart | kinsoku=true | 变 false |
| K2 | K2 | setBoolean(201,false) | onStart | false | 仍 true |
| K3 | K3 | setBoolean(201,true) | onStart | true | 忽略 sprm |
| K4 | K4 | K2 后再 reset+true | onStart | true | 粘滞 false |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/01_function_kinsoku_flag.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 01_function_kinsoku_flag

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 01_function_kinsoku_flag \
  --src src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-26-kinsoku/functions/01_function_kinsoku_flag/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_kinsoku_flag --json`)
date: 2026-09-06 09:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | 151/429 | 35.2% | 259/793 | 32.7% |
| **function total** | **151/429** | **35.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs \
  --report libs/word-core/layout/features/wr-26-kinsoku/functions/01_function_kinsoku_flag/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 01:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ParaLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/ParaLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/ParaLayouter.rs` | **PASS** | `ParaLayouter`, `onStart`, `isAutoAfterSpacing`, `shouldAdjustRightMarginInCharGridModel`, `calcParaBefore`, `hasSameAutoNum`, `calcParaAfter`, `calcPrevParaAfter`, `calcLineSpace`, `calcIndents`, `calFirstLineIndent`, `getCharGridExtraFirstLineIndent`, `calLeftAndRightIndent`, `getFirstCharSize`, `calcShouldCompressLine`, `calAlign`, `isFirstLineOfPara` | `dispose`, `calcFinalPropertySet`, `getFinalParaProp`, `calclastBrcBetween`, `calcParaProp`, `calcBRC`, `calcSHD`, `getVaildShd`, `calcAutoNum`, `onEnd`, `isLineWithParaBreakAndSectionBreak`, `shouldCalcLastParaAfter`, `getNormalParaFinalProperty`, `getCharGridExtraRightIndent` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
