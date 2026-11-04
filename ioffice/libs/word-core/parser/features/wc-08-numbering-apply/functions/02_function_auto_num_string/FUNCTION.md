# 02_function_auto_num_string — PAPX 234/235 → autoNumString

| 项 | 值 |
|----|----|
| feature | wc-08-numbering-apply |
| order | 2 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `AutoNumLayouter::calAutoNumString` |
| status | **pass** |

## 1. 功能说明

### 1.1 一句话
PAPX 234/235 → autoNumString

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| 上表入口 | 见入口 | `cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.java` |

### 1.3 输出
见 §1.1 与用例表。

### 1.4 上游
声明的上游 fixture / 假 PropertySet；不跑无关管线。

### 1.5 下游
见 FUNCTIONS.md。

### 1.6 不变量与函数改写列表
- 切在 I/O。
- **改写列表：** `calAutoNumString,getCurAutoNum,getResult,getFollowStr,initEmptyAutoNum,verifyData,dispose`
- 列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */` 停车原模版体。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| AutoNumTable | wc-08 01 | LstLfoTool.isExisting | loop 01 | — | 禁止改表结构 |
| ParaPropEnv | wr-21 | autoNumString 字段 | 是 | 悬挂进 indent 若未接线则本测断言串 | 禁止改 ParaLayouter 段距 |
| Numformat | layout | txFormatNum | 本轮生成或停车 | 复杂 nfc | 禁止 stub AutoNumLayouter |

## 2. 核心代码
对照 Java 方法体移植；禁止 dummy `let X = ()`。

## 3. 测试方案
### 3.1 代码审计与测试面
| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| calAutoNumString | AutoNumLayouter.java | 235+234 | propEnv.autoNumString | 无 235 |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| AN1 | 十进制 1. | 公开入口 | 是 | AN1 |
| AN2 | 无 235 | 失败 | 是 | AN2 |
| AN3 | follow tab/space | 关键分支 | 是 | AN3 |
| AN4 | nfc=23 bullet | 关键分支 | 是 | AN4 |
| AN5 | verifyData null | 失败 | 是 | AN5 |

### 3.3 可测试性
fixture / 内存字节 / 假上游输出；不启动全管线。

### 3.4 用例表
| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| AN1 | AN1 | lfo/lvl 存在 | 非空串 | 空串假绿 |
| AN2 | AN2 | 无 235 | 不加串 | 发明编号 |
| AN3 | AN3 | follow=0 | 含 \t | 丢 follow |
| AN4 | AN4 | nfc=23 | bullet 路径 | 走数字 |
| AN5 | AN5 | state=null | panic/IllegalState | 静默 |

### 3.5 测试代码位置
crate: `libs/word-core/Cargo.toml`
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs`
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs`

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_auto_num_string

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_auto_num_string \
  --src src/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs \
  --report libs/word-core/parser/features/wc-08-numbering-apply/functions/02_function_auto_num_string/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_auto_num_string --json`)
date: 2026-09-06 14:54

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs` | 98/145 | 67.6% | 169/229 | 73.8% |
| **function total** | **98/145** | **67.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.java
```

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.java --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs \
  --report libs/word-core/parser/features/wc-08-numbering-apply/functions/02_function_auto_num_string/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-06 06:54

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `AutoNumLayouter` | `koffice/writer/core/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/base/line/AutoNumLayouter.rs` | **PASS** | `AutoNumLayouter`, `verifyData`, `dispose`, `getCurAutoNum`, `calAutoNumString`, `initEmptyAutoNum`, `getFollowStr`, `getResult`, `getBulletChar` | `getAutoNumStrRunProperty`, `getConvertFont`, `getBulletPicIndex`, `symbol2char` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
