# 03_function_convert_bullet — convertBullet 不 panic

> feature: pl-12-textbox-live-paths ｜ 状态：**spec** ｜ Java 管线：S7 DataTransform

## 0. 管线能力解锁

`convertBullet:1509` parked；`exToInternal:359` 调用。绘制可 park 到 pr-10。

## 1. 功能说明

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | buNone / buChar / buAutoNum |
| **输出** | buNone type=-1 跳过；其余 BulletsInfo.visible 且不 panic |
| **错误** | 不 panic；未知输入走 Java skip/缺省。 |
| **约束** | 不改 `.java`；不 `--force` PptxrSlide；不发明第二套类型名。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| getParaDirection | 01 | rtl 进 BulletsInfo | spec | — | 本 function 依赖 01 |
| getBulletType | DataTransform | 1/2/3/-1 | parked | buBlip 图 | 绘制交给 pr-10 |

## 2. 核心代码与函数改写列表

**本 function：** 解开 getBulletType + convertBullet 热路径。buBlip 可壳。phType<0||==10 → -1。

Loop 走 **function-java2rust**（有 `// java` 的叶）。产品编排只修 I/O。列表外方法：`todo!("DEFERRED_NON_FUNCTION")` + `/* … */`（禁止 `/**`）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读 Java convertBullet:893；getBulletType:916 buNone→-1。Rust :1509 todo。

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| BU-NONE | options.hasBuNone | phType=1 | type=-1 且不 panic | 公开入口 |
| BU-CHAR | hasBuChar | • | type=1 visible | 关键分支 |
| BU-NUM | hasBuAutonNum | autonum | type=2 不 panic | 关键分支 |
| BU-PH10 | getBulletType 头 | phType=10 | -1 | 边界 |
| BU-EX | 含 bullet 的 TextBody | fixture | 不 panic | 状态转换 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| buNone | options.hasBuNone | 公开入口 | 是 | BU-NONE |
| buChar | hasBuChar | 关键分支 | 是 | BU-CHAR |
| buAutoNum | hasBuAutonNum | 关键分支 | 是 | BU-NUM |
| phType 10 | getBulletType 头 | 边界 | 是 | BU-PH10 |
| exToInternal | 含 bullet 的 TextBody | 状态转换 | 是 | BU-EX |

### 3.3 可测试性

测 N 只用声明的上游输出或最小 zip/fixture，不跑完整 23 页 Sample 作为唯一用例。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| BU-NONE | buNone | — | phType=1 | type=-1 且不 panic | panic |
| BU-CHAR | buChar | — | • | type=1 visible | panic |
| BU-NUM | buAutoNum | — | autonum | type=2 不 panic | panic |
| BU-PH10 | phType 10 | — | phType=10 | -1 | 仍当 char |
| BU-EX | exToInternal | — | fixture | 不 panic | DEFERRED panic |

### 3.5 测试代码位置

```text
crate: libs/ppt-core
src:   src/cn/wps/show/layout/dataaction/DataTransform.rs
test:  tests/cn/wps/show/layout/dataaction/03_function_convert_bullet.rs
[[test]] name = "03_function_convert_bullet"
```

`#[ignore = "spec: 03_function_convert_bullet"]` 直至 Loop。不得标 pass。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_convert_bullet -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_convert_bullet \
  --src src/cn/wps/show/layout/dataaction/DataTransform.rs \
  --report layout/features/pl-12-textbox-live-paths/functions/03_function_convert_bullet/FUNCTION.md
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
  --report layout/features/pl-12-textbox-live-paths/functions/03_function_convert_bullet/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: n/a (split placeholder)

| Java class | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing | result |
|---|---|---|---|---|---|
| (split) | — | BU-NONE, BU-CHAR, BU-NUM, BU-PH10, BU-EX | — | 接线 | **FAIL**（spec） |

**overall: FAIL** — spec；不得标 pass。
<!-- java-rust-align:end -->
