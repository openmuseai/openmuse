# 01_function_color_map — a:clrMap 配色槽

> feature: pp-03-dml-style ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：master/layout/theme 的 `a:clrMap` 配色重映射生效，12 个 scheme 色槽（accent1-6/hlink/folHlink/bg1-2/tx1-2）写入 `ColorMapping`。缺失时断在哪：scheme 色槽为空则形状没法用主题色，S1 convert 与 S4 填充拿不到「主题色」来源（画色必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

`a:clrMap` 是 master/layout/theme 上的配色重映射表。本 function 实现 `ColorMapHandler`：读取
clrMap 元素的全部 attrs，把 attr 名（scheme 槽名 id）对应的字符串值经 `DGMap.schemeColorMap`
转成 scheme 索引后写入 `ColorMapping` 的对应槽。该 `ColorMapping` 由构造方
（pp-01:07 `SlideMasterHandler` 3211598 / pp-07 经 `ColorMapOverrideHandler` 1114458 /
pp-04 `ThemeHandler` 1114459）传入，本 function 只负责「写入」：不做 part 打开、不做缺省回退。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `clrMap` 元素 attrs（`XmlRoAttr attrsNotCopy`，仅用 `count/getAttrAt/toString`；`element` 参数被忽略）+ 构造入参 `ColorMapping clrMap`（引用）。 |
| **输出** | 无返回值；写入 `clrMap` 的 12 个 scheme 槽：`accent1`–`accent6`、`hlink`、`folHlink`、`bg1`、`tx1`、`bg2`、`tx2`。 |
| **状态与副作用** | 逐 attr：`getAttrAt(i)==null` 则跳过；命中 switch 的 attr name id 则 `put(x, DGMap.schemeColorMap.get(attrValue))`。name id 取 kfc `dml/A.java`：accent1=1114128、accent2=1114129、accent3=1114130、accent4=1114131、accent5=1114132、accent6=1114133、hlink=1114134、folHlink=1114135、bg1=1114453、tx1=1114454、bg2=1114455、tx2=1114456。同一槽后写的 attr 生效；槽之间独立。 |
| **错误语义** | 未知 attr name id → switch default，无副作用（槽保持默认）。已知 attr 名但值的字符串不在 `schemeColorMap`（17 key：accent1-6 / bg1-2 / dk1-2 / folHlink / hlink / lt1-2 / phClr / tx1-2）→ Java 在 `(Integer)` unbox 处抛 NPE；Rust 是否复刻 panic 或静默跳过 → 待 loop 确认。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `DGMap.schemeColorMap` | drawing map，E | token→i32（17 项字面量） | 测试用 Java 字面量/既有表 | 不恢复 DGMap 全文件 | 不改常量 |
| `ColorMapping` | model，E | putAccent*/putBg*/putTx*/putHlink*/putFolHlink | Rust 已存在（`data/style/ColorMapping.rs` `pub fn put*`，行 546-942） | KoBin 全图 | 不改 mapping 存储 |
| `XmlRoAttr` | kfc，E | count/getAttrAt/getName/toString | 测试内存 attrs | 不实现 reader | |

Rust 目标 `src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs` **尚无**（`style/mod.rs` 仅注释占位）。

## 2. 核心代码与函数改写列表

| Java 类 | Rust 目标 | 关键方法/字段 |
|---|---|---|
| `ColorMapHandler` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs`（新建模板，尚无 Rust） | 字段 `clrMap`（`ColorMapping`）；方法 `ctor(ColorMapping)`、`startElement(int, XmlRoAttr)` |

函数改写列表（模板 `// TODO_RUST_CODE` 全部在本 function 内实现）：

- ctor：保存 `ColorMapping` 引用（Rust 侧 `&mut ColorMapping` / 所有权写法待施工定，模板为准）。
- `startElement`：`count()` 循环 + `getAttrAt` null skip + 12 个 attr name case（数值与 Java switch 完全一致）；不在此实现 DGMap / ColorMapping 本身。
- `todo!("DEFERRED_NON_FUNCTION")` 停车项：无——本类仅 ctor / `startElement`，无内嵌方法需要停车；`schemeColorMap` 以测试字面量代替。
- 属其它 function / 已存在：`ColorMapping.put*`（model，已存在）；`XmlRoAttr` 读取（kfc_kfc_alpha，已有使用示例 `CustDataLstHandler.rs`）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读（禁止只按标题冒烟）：

- `ColorMapHandler.java` `startElement` 方法体：逐 case 与 12 个 attr name id 数值核对。
- `DGMap.java` `schemeColorMap` 字面量：17 项 key → i32，作为期望值来源。
- `ColorMapping.java` 及 Rust `data/style/ColorMapping.rs` 的 `put*` 签名。
- Rust 现状：`src/cn/wps/drawing/dmlr/handler/style/` 下**尚无** `ColorMapHandler.rs`（仅 `mod.rs`）；`data/style/ColorMapping.rs` 已存在。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | startElement 12 scheme 槽（accent1-6/hlink/folHlink/bg1/tx1/bg2/tx2） | 分支 | 是 | T01 |
| C02 | null getAttrAt skip | 边界 | 是 | T02 |
| C03 | 未知 token / 空 attrs | 失败/边界 | 是 | T03 |
| C04 | 多槽混合 + 同槽重复 attr | 循环/状态 | 是 | T04 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 12 个 attr name id 齐备的内存 attrs（值取 17 key 之一） | 每槽 = `schemeColorMap.get(value)`，与 Java 一致 | 红测停在本 function |
| T02 | C02 | 首/中/尾出现 getAttrAt 返回 null | 跳过不写、不 panic | 红测停在本 function |
| T03 | C03 | 空 attrs；或全部为未列入 switch 的 name id | 无任何 put，槽保持默认 | 红测停在本 function |
| T04 | C04 | accent1+bg1+tx2 混合；同一槽 attr 出现两次 | 槽独立写入；重复槽后写生效 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs
[[test]] name = "01_function_color_map"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_color_map

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_color_map \
  --src src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs \
  --report libs/ppt-core/parser/features/pp-03-dml-style/functions/01_function_color_map/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_color_map --json`)
date: 2026-09-01 11:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs` | 51/58 | 87.9% | 77/97 | 79.4% |
| **function total** | **51/58** | **87.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.java
```

Loop 结果：PASS（2026-09-01）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs \
  --report libs/ppt-core/parser/features/pp-03-dml-style/functions/01_function_color_map/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 03:43

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ColorMapHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/style/ColorMapHandler.rs` | **PASS** | `ColorMapHandler`, `startElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->