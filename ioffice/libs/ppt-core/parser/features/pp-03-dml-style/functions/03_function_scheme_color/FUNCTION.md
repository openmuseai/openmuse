# 03_function_scheme_color — a:schemeClr → Color.scheme

> feature: pp-03-dml-style ｜ 状态：**pass** ｜ Java 管线环节：S0 parse（主题色槽索引）

## 0. 管线能力解锁

实现后：`a:schemeClr val="accent1"` 把 **scheme 槽索引** 写入 `Color`（mode=4），并 `HandlerCallback.onAdjustThemeColor(color, belonger)`。缺失时 Sample 类真实 pptx 的填充/字色全是 scheme 引用 → S1 convert 拿不到主题色 → 只能画默认黑/空心（当前首屏「白底+框+字」的字色/形状色对不上母版）。
**不打开 theme part**（RGB 查表是 pp-04）；本 function 只锁「token → scheme 索引 + 回调」。

## 1. 功能说明

`ColorHandlerFactory.createColorHandler` 在 `subElement==1114219` 时构造 `SchemeColorHandler`（`ElementAttr` 包装）。`addElementAttr`：

1. attr `val`（1114158）→ `DGMap.schemeColorMap.get(token)` → `Color.putScheme`
2. ctor 已 `color.putMode(4)`
3. `ColorTransferReader.readColorTransferList`（tint/shade 等）——MVP **壳**：有调用则可记一次；变换链延后
4. `handlerCallback != null` → `onAdjustThemeColor(color, mBelonger)`（solidFill 的 belonger=1114371，见 pp-10:02 T06）

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `schemeClr` 元素 attrs（至少 `val`）+ 已构造 `Color` + `HandlerCallback` + `belonger: i32`。 |
| **输出** | `Color.mode==4`；`scheme` = schemeColorMap 命中值；callback 调用 0 或 1 次。 |
| **状态与副作用** | 无 part I/O。callback null → 不回调（Java `null != this.handlerCallback`）。 |
| **错误语义** | `val` 缺 → 不 putScheme。token 不在 schemeColorMap → Java `(Integer)` NPE；Rust loop 裁决：skip 不 panic（与 pp-03:01 T03 对齐，待 L1 钉死）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `DGMap.schemeColorMap` | drawing map | 17 token→i32 | 测试字面量/既有表 | 不恢复 DGMap 全文件 | 不改常量 |
| `Color.putMode`/`putScheme` | `data/color/Color` | mode=4；scheme 槽 | 模型已存在 | transformLst 全量 | 不改 Color 存储语义 |
| `HandlerCallback.onAdjustThemeColor` | pp-01:02 | `(Color, belonger)` | pass 签名 | theme RGB 解析 | **不改 callback 语义** |
| `ColorHandlerFactory` | 本 function 分发 | 1114219 → SchemeColorHandler | Rust 尚无独立文件 | 1114215/17/18/20 其它色模型 | 不把 hsl/sys/prst 塞进本 function 必测 |
| `ColorTransferReader` | color 变换 | transformLst | — | tint/lum 全链 | `todo!("DEFERRED_NON_FUNCTION")` |
| `ColorMapHandler` | pp-03:01 | 槽名重映射 | spec 未 loop | — | 本 function 不写 clrMap |

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法 |
|---|---|---|
| `SchemeColorHandler` | `src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs` | ctor（putMode 4）；`addElementAttr` |
| `ColorHandlerFactory` | `src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs` | `createColorHandler` 的 **1114219** 分支（其它 case 可停车） |
| `ColorHandler` | `…/color/ColorHandler.rs` | `enterSubElement` → factory（belonger=-1）——本 function 只测 factory 的 scheme 分支 |

改写列表：

- `SchemeColorHandler.addElementAttr`：val→putScheme；callback 判空。
- `ColorHandlerFactory` case 1114219。
- `todo!("DEFERRED_NON_FUNCTION")`：`ColorTransferReader` 变换链；hsl/sys/prst/scrgb handlers。

## 3. 测试方案

### 3.1 代码审计与测试面

必须通读：`SchemeColorHandler.java` 全文；`ColorHandlerFactory.java` switch；`DGMap.schemeColorMap` 17 key；`HandlerCallback.onAdjustThemeColor` Java 签名。Rust 现状：`handler/color` 下无 SchemeColorHandler。

| symbol | Java/Rust | 输入 | 输出 | 分支 |
|---|---|---|---|---|
| ctor | SchemeColorHandler.java | Color | putMode(4) | — |
| addElementAttr val | 同上 | val=accent1 | putScheme(map) | 缺 val |
| callback | 同上 | non-null / null | 1 次 / 0 次 | belonger 透传 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | val=accent1/bg1/tx1 → putScheme 索引 | 主路径 | 是 | T01 |
| C02 | 缺 val / 未知 token | 失败 | 是 | T02 |
| C03 | callback null 不调用；non-null belonger=1114371 | 状态 | 是 | T03 |
| C04 | factory subElement=1114219 非 None；1114216 不在本 function 必测（归 pp-10:02） | 分发 | 是 | T04 |

### 3.3 可测试性

内存 attrs + RecordingCallback。不跑 PptxReader、不打开 theme.xml。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | val=`accent1`（及 bg1、tx1 各一条或参数化） | scheme 索引 = Java map | 红测停在本 function |
| T02 | C02 | 无 val；val=`not-a-scheme` | 不 putScheme / skip（loop 钉死 NPE vs skip） | 红测停在本 function |
| T03 | C03 | callback None vs Some | 0 / 1 次；belonger 相等 | 红测停在本 function |
| T04 | C04 | createColorHandler(1114219, …) | kind 含 SchemeColor | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs
[[test]] name = "03_function_scheme_color"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_scheme_color

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_scheme_color \
  --src src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs \
  --report libs/ppt-core/parser/features/pp-03-dml-style/functions/03_function_scheme_color/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_scheme_color --json`)
date: 2026-09-01 11:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs` | 15/23 | 65.2% | 8/16 | 50.0% |
| `src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs` | 34/42 | 81.0% | 36/50 | 72.0% |
| **function total** | **49/65** | **75.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.java
```

Loop 结果：PASS（2026-09-01）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs \
  --report libs/ppt-core/parser/features/pp-03-dml-style/functions/03_function_scheme_color/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 03:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SchemeColorHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/SchemeColorHandler.rs` | **PASS** | `SchemeColorHandler`, `addElementAttr` | — | — | **PASS** |
| `ColorHandlerFactory` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/color/ColorHandlerFactory.rs` | **PASS** | `createColorHandler` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
