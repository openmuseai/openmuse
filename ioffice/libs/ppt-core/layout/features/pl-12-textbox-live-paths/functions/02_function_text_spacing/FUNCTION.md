# 02_function_text_spacing — getTextSpacing

> feature: pl-12-textbox-live-paths ｜ 状态：**spec** ｜ Java 管线：S7 DataTransform

## 0. 管线能力解锁

`getTextSpacing:2035` parked。exToInternal `:319` 会调它 → panic。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | spcPts / spcPct / 皆无 |
| **输出** | pts→layout units；pct→ fontsize*spcPct*1.2；皆无 0 |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| point100ToLayoutUnits | DataTransform | pts 换算 | 需有 | — | 禁止发明第二套单位 |
| getTextSpacing | DataTransform.rs:2035 | f64 | todo | lnSpc within 已另方法 | — |

## 2. 核心代码与函数改写列表

**本 function：** Java `:1200-1215` SPACE_SCALE=1.2F。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java getTextSpacing:1200；Rust 热路径 :319/:325 已调用。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| SP-PTS | hasSpcPts | pts=100 | point100ToLayoutUnits(100) | 公开入口 |
| SP-PCT | hasSpcPct | pct + font 12 | 12*pct*1.2 | 关键分支 |
| SP-0 | no pts/pct | 空 spacing | 0.0 | 边界 |
| SP-MAX | otherSpans 更大 | max fontsize | 用 max | 边界 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| spcPts | hasSpcPts | 公开入口 | 是 | SP-PTS |
| spcPct | hasSpcPct | 关键分支 | 是 | SP-PCT |
| 皆无 | no pts/pct | 边界 | 是 | SP-0 |
| 多 span 字号 | otherSpans 更大 | 边界 | 是 | SP-MAX |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| SP-PTS | spcPts | — | pts=100 | point100ToLayoutUnits(100) | panic |
| SP-PCT | spcPct | — | pct + font 12 | 12*pct*1.2 | 漏 1.2 |
| SP-0 | 皆无 | — | 空 spacing | 0.0 | NaN |
| SP-MAX | 多 span 字号 | — | max fontsize | 用 max | 只用 firstSpan |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/dataaction/DataTransform.rs
test:  tests/cn/wps/show/layout/dataaction/02_function_text_spacing.rs
[[test]] name = "02_function_text_spacing"
```

`#[ignore = "spec: 02_function_text_spacing"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_text_spacing -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_text_spacing \
  --src src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --report layout/features/pl-12-textbox-live-paths/functions/02_function_text_spacing/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: (split placeholder — Loop L3b 由 function-coverage.sh 写入)
<!-- coverage-report:end -->

### 3.8 模板布局校验

Loop L5b 必须 PASS。不删 `// java`；列表外方法 `todo!("DEFERRED_NON_FUNCTION")` + 普通 `/* */`。

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/layout/dataaction/DataTransform.java --rs libs/ppt-core/src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --report layout/features/pl-12-textbox-live-paths/functions/02_function_text_spacing/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | SP-PTS, SP-PCT, SP-0, SP-MAX | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
