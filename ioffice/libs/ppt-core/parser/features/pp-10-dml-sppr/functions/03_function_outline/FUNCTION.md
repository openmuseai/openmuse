# 03_function_outline — a:ln 子树

> feature: pp-10-dml-sppr ｜ 状态：**pass** ｜ Java 管线环节：S0 parse → S1 convert 输入

## 0. 管线能力解锁

形状有坐标与填充但**无线条**时，S1 convert 的 `LineStyleConvert` 无 `LineProperty` → pr-02 无 line
指令 → 描边缺失（MVP 语义中线宽 0 不可画）。实现后形状可带线宽/复合/端帽/对齐/虚线 →
S4 能画出形状体（含描边）；缺失则链断在「形状无线条」。

## 1. 功能说明

`OutlineHandler`（`a:ln`）写 `Outline`：

- `startElement` 四 attr：w=1114304→`putWidth`（EMU，`toInt`）；cmpd=1114442→
  `DGMap.compoundLineTypeMap`→`putCompoundType`；cap=1114441→`lineCapMap`→`putLineCap`；
  algn=1114328→`penAlignmentMap`→`putStrokeAlignment`（均 miss 判空）。
- `enterSubElement`：fill=1114370–1114373 → `FillPropertiesHandler` 复用（02 function 的共享模板）；
  round=1114433/bevel=1114434/miter=1114435 → `lineJoin().putType(2/0/1)`（miter 另建
  `MiterHandler`，limit attr 1114432→`putMiterJoinLimit`，`toPercentage`）；
  prstDash=1114437 → `PrstDashHandler`（val=1114158→`presetLineDashValueMap`→`putPresetDash`）；
  custDash=1114438 → `CustomDashHandler`（dashStop=1114436 循环复用 handler，d=1114160、
  sp=3211593→`toPercentage`）；headEnd=1114439/tailEnd=1114440 → `LineEndHandler`
  （type=1114388→`lineEndTypeMap`、w=1114304→`lineEndWidthMap`、len=1114431→`lineEndLengthMap`）。
- `endElement`：dashStops save 后 `customDashLst().write`；headEnd/tailEnd 非空才 save +
  `forceWriteHeadEnd/ForceWriteTailEnd`。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `a:ln` attrs + 子元素事件 + 构造入参 `Outline outline` + `HandlerCallback`（透传给 fill 子 handler）。 |
| **输出** | `Outline.putWidth/putCompoundType/putLineCap/putStrokeAlignment/putPresetDash`；`lineJoin().putType + putMiterJoinLimit`；`customDashLst()`（List<DashStop>）；`headEnd/tailEnd`（forceWrite*）；`outline.fill()` 复用 fill 写入。 |
| **状态与副作用** | attr map miss → 不 put。`endElement` 的三类 flush 各自判空（dashStops/headEnd/tailEnd）。dashStop 用单一 handler 实例 `init` 循环复用（Java 语义：list 追加新 stop、handler 绑定当前 stop）。 |
| **错误语义** | 未知子元素 → 返回 null。`toPercentage` 对非法串语义以 kfc `XmlRoAttr` 为准（待 loop 确认）。`compoundLineTypeMap`/`lineEndTypeMap` 等 miss → null 不写。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止去改 |
|---|---|---|---|---|---|
| `HandlerCallback` | pp-01:02 | 透传 fill 色链 | pass | — | pass 语义 |
| 分发入口 | pp-10:01（spPr ln=1114148）；pp-11 rPr 的 ln；OutlineHandler 自身 | `OutlineHandler(ln(), callback)` | spec | pp-11 入口由 pp-11 接线 | 不改 01 分发 |
| `FillPropertiesHandler`（ln 的填充） | pp-10:02 | 复用共享模板 | spec | — | fill morph 归 02 |
| `Outline` 模型 | `data/outline/Outline.rs` | put* + `customDashLst().write` + forceWrite* | 已存在 | — | 不改字段语义 |
| sppr/outline `.rs` | 本 feature | Java 结构模板 | **重做**：审计 `OutlineHandler.rs` 428 行（0 todo/140 stub，「local stub Outline types; handler shells」）；模块接线已注释 | — | 不增量 |

### 1.8 不实现（显式）

headEnd/tailEnd 的箭头线与 `LineEnd` 的线端尺寸 maps 全量（MVP 可只存 type）、`custDash` 全量
（可只存 d/sp 对）——超出「能画出」最小集的部分标注延后。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `OutlineHandler` | `…/sppr/outline/OutlineHandler.rs` | `ctor/init(Outline, HandlerCallback)`、`startElement`（四 attr）、`enterSubElement`（fill 复用 + lineJoin + dash + ends）、`endElement`（三类 flush）；nested `MiterHandler`/`LineEndHandler`/`PrstDashHandler`/`CustomDashHandler`(+`DashStopHandler`) |
| （复用）`FillPropertiesHandler` | `…/sppr/fill/FillPropertiesHandler.rs` | ln 的 fill 分支 → 02 function 模板 |

id 已查证（`dml/A.java`）：w=1114304、cmpd=1114442、cap=1114441、algn=1114328、round=1114433、
bevel=1114434、miter=1114435、lim=1114432、prstDash=1114437、custDash=1114438、headEnd=1114439、
tailEnd=1114440、val=1114158、d=1114160、sp=3211593、type=1114388、len=1114431、ds=1114436。

函数改写列表：

- `startElement`：四 attr 的 `getByName` + map miss 判空顺序与 Java 一致。
- `enterSubElement`：lineJoin 三 case 的 putType（2/0/1）数值；miter 的 `putMiterJoinLimit`；
  fill 四 id 走 02 的共享 `FillPropertiesHandler`。
- `endElement`：三个判空 flush（含 dashStop 的 `save` 对应 Rust 的写回语义）。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：LineEnd 尺寸 map 全量、custDash 停靠细节（标注延后）。

## 3. 测试方案

### 3.1 代码审计与测试面

- `OutlineHandler.java` 全部分支；`DGMap` 五个线型 map 的 key 定义（compoundLineTypeMap/
  lineCapMap/penAlignmentMap/presetLineDashValueMap/lineEndTypeMap/lineEndWidthMap/lineEndLengthMap）。
- `data/outline/Outline.java` 与 Rust `data/outline/Outline.rs` 的写入接口。
- Rust 现状：审计 428 行 outline 壳；不增量。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 四 attr 各 + map miss | 分支 | 是 | T01 |
| C02 | lineJoin 三 case + miter limit | 分支 | 是 | T02 |
| C03 | prstDash/custDash（含多 stop 复用） | 数据/循环 | 是 | T03 |
| C04 | headEnd/tailEnd 出现与缺席 → forceWrite 判空 | 状态 | 是 | T04 |
| C05 | ln 内 fill 子树复用 02 | 集成 | 是 | T05 |

### 3.3 可测试性

内存 attrs/events；真实 `data/outline/Outline.rs`；fill 子树 fixture 走 02 的 contract。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | w=100000 + cmpd/cap/algn 合法值；再各给非法值 | put* 数值正确；非法值不写 | 红测停在本 function |
| T02 | C02 | round/bevel/miter 元素 + miter lim | lineJoin type 2/0/1；lim `toPercentage` 写入 | 红测停在本 function |
| T03 | C03 | prstDash(val)；custDash 两 stop（d/sp） | prst id 写入；两个 stop 依序入 `customDashLst` | 红测停在本 function |
| T04 | C04 | 仅有 headEnd；仅 tailEnd；两者皆无 | 出现才 forceWrite*，不出现无副作用 | 红测停在本 function |
| T05 | C05 | ln 下 solidFill | 走 02 的 FillPropertiesHandler 写入 outline.fill() | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/shape/sppr/outline/03_function_outline.rs
[[test]] name = "03_function_outline"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_outline

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_outline \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/03_function_outline/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_outline --json`)
date: 2026-09-01 11:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs` | 132/195 | 67.7% | 206/288 | 71.5% |
| **function total** | **132/195** | **67.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.java
```

Loop 结果：PASS（2026-09-01）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs \
  --report libs/ppt-core/parser/features/pp-10-dml-sppr/functions/03_function_outline/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 03:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `OutlineHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/outline/OutlineHandler.rs` | **PASS** | `OutlineHandler`, `init`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->