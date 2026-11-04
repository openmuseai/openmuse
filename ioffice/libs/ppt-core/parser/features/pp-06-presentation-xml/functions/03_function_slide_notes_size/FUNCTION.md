# 03_function_slide_notes_size — sldSz / notesSz

> feature: pp-06-presentation-xml ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：`sldSz`（含 16:9 魔数 12192000×6858000 → type 15 覆盖）与 `notesSz` 写入 `Presentation`，slide 画布尺寸有真值。缺失时断在哪：sldSz 是「能先画」的最少值，缺失时白底/视口无尺寸，仅靠 pp-09 `addDefaultProperty` 的 12192000×6858000 兜底才能先画（闭环打开必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(sldSz attrs; notesSz PositiveSize2D) -> sizes + 16:9 type 15

### 1.1 I/O 契约

| 方向 | 契约 |
|---|---|
| 输入 | `p:sldSz`(3211513) attrs：type(3211302)/cx(3211488)/cy(3211489)；`p:notesSz`(3211514) 子元素交 `PositiveSize2DHandler`：cx=`a:cx`(1114163) 或 `p:cx`(3211488)、cy=`a:cy`(1114164) 或 `p:cy`(3211489)。id 均按 `kfc/xsd/pml/P.java` / `kfc/xsd/dml/A.java` 查证 |
| 输出 | `Presentation.SlideSize`：`putType(slideSizeMap.get(toStr))`/`putCx(toInt)`/`putCy(toInt)`；`presData.notesSize()` 的 `PositiveSize2D`：`putCx`/`putCy`。16:9 魔数 12192000×6858000 → `putType(15)`（在 cx/cy 写入后覆盖任何 type） |
| 状态与副作用 | 各 attr 独立判空写入；cx/cy 写完后立即做 16:9 覆盖判断（`SlideSizeHandler` 私有常量 `SCREEN16X9_X=12192000`/`SCREEN16X9_Y=6858000`）；notesSize 目标来自 `presData.notesSize()`（`Presentation.open` 的 create 默认） |
| 错误语义 | 缺 type/cx/cy 均不写入（保留 create 默认），无 NPE——所有 `getByName` 判空；`SWReaderMap.slideSizeMap` miss（type 值不在表）→ `putType(null)` 拆箱 NPE（Java 事实，测试用合法枚举）；pp-09 `PptxrPresentation.addDefaultProperty` 兜底：缺 slideSize → 12192000×6858000、缺 notesSize → 6858000×9144000 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs`（尚不存在） |
| `koffice/presentation/core/.../dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs`（待建；Rust 数据模型 `PositiveSize2D` 已存在） |

### 2.1 Java 成员 → Rust 目标（按 Java 源码查证）

| Java 成员 | 说明 | Rust 目标 |
|---|---|---|
| `SlideSizeHandler` nested（PresentationHandler.java） | 处理 sldSz(3211513)；私有常量 `SCREEN16X9_X=12192000`、`SCREEN16X9_Y=6858000` | `PresentationHandler.rs` 内 nested；常量 → `const` |
| `startElement` | ① `getByName(3211302 type)` → `sldSize.putType(SWReaderMap.slideSizeMap.get(toStr))`；② `getByName(3211488 cx)` → `putCx(toInt)`；③ `getByName(3211489 cy)` → `putCy(toInt)`；④ `12192000 == cx() && 6858000 == cy()` → `putType(15)` | `fn start_element` 同序 |
| `Presentation.SlideSize`（nested，Presentation.java） | `putCx(int)`/`putCy(int)`/`putType(int)`（行 3131/3152/3173 查证） | `Presentation.rs` 的 `SlideSize`（已存在） |
| `PositiveSize2DHandler`（DML transform 包） | `startElement`：`getByName(1114163 a:cx) \|\| getByName(3211488 p:cx)` → `putCx`；`getByName(1114164 a:cy) \|\| getByName(3211489 p:cy)` → `putCy`；ctor `new PositiveSize2DHandler(this.presData.notesSize())` | 待建；若暂缓则以 contract fixture 吸收双命名空间 cx/cy |

改写列表：`SlideSizeHandler` 全部方法 + `enterSubElement` case 3211514 → `PositiveSize2DHandler` 接线
（D/constructor 改动）。`SWReaderMap.slideSizeMap`（SWReaderMap.java）：screen4x3→0、letter→1、A4→2、
35mm→3、overhead→4、banner→5、custom→6、A3→7、B4ISO→8、B4JIS→9、B5ISO→10、B5JIS→11、
hagakiCard→12、ledger→13、screen16x10→14、screen16x9→15。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。审计面：

1. `SlideSizeHandler.startElement` 四步顺序：type(3211302) → cx(3211488) → cy(3211489) →
   16:9 覆盖（若 cx/cy 已写）。只给 type 不给 cx/cy 时不触发覆盖。
2. type 查表：`SWReaderMap.slideSizeMap` 16 项；不在表 → `putType(null)` 拆箱 NPE（Java 事实）。
3. `PositiveSize2DHandler` 双命名空间：a:cx(1114163)/a:cy(1114164) 与 p:cx(3211488)/p:cy(3211489)
   任一命中即写；`||` 短路对 toInt 取值无影响（都来自同一个 tmp）。
4. notesSize 目标：ctor 固定 `this.presData.notesSize()`（非 null，来自 `Presentation.open` create）。
5. pp-09 兜底默认（`PptxrPresentation.addDefaultProperty`）：本 function 只保证解析写入，不负责默认。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | sldSz type/cx/cy 写入（合法枚举，如 screen4x3=0/letter=1/screen16x9=15） | 正常 | 是 | T01 |
| C02 | cx=12192000 cy=6858000 → `putType(15)`（覆盖 map 值） | 边界 | 是 | T02 |
| C03 | 缺 type / 缺 cx / 缺 cy（`getByName` miss） | 边界 | 是 | T03 |
| C04 | notesSz → `PositiveSize2DHandler`：a: 与 p: 双命名空间 cx/cy 写入 notesSize | 分支 | 是 | T04 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。
`SlideSize`/`PositiveSize2D` 用 Rust 数据模型（`Presentation.rs` 内已存在）直接持有并断言 put 结果；
`PositiveSize2DHandler` 若尚无可直接测，以 contract fixture（同签名双命名空间吸收）替身参与 03 的
组合事件序列。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | sldSz 带 type="screen4x3" cx="9144000" cy="6858000" | `slideSize` type=0、cx=9144000、cy=6858000 | 红测停在本 function |
| T02 | C02 | sldSz 带 type="letter"(1) cx="12192000" cy="6858000" | cx/cy 写入后 `putType(15)`（覆盖 map 的 1） | 红测停在本 function |
| T03 | C03 | sldSz 缺 type；或缺 cx；或缺 cy | 缺失项不写入、无 panic；已有项照写 | 红测停在本 function |
| T04 | C04 | notesSz 带 `a:cx="6858000" a:cy="9144000"`；再一组 `p:cx/p:cy` | notesSize.putCx/putCy 两命名空间各自生效 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/PresentationHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "03_function_slide_notes_size"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_slide_notes_size

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_slide_notes_size \
  --src src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/03_function_slide_notes_size/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_slide_notes_size --json`)
date: 2026-09-01 17:48

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs` | 38/50 | 76.0% | 53/74 | 71.6% |
| `src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | 61/427 | 14.3% | 99/556 | 17.8% |
| **function total** | **99/477** | **20.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/03_function_slide_notes_size/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 09:45

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PresentationHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | **PASS** | `PresentationHandler`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |
| `PositiveSize2DHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/PositiveSize2DHandler.rs` | **PASS** | `PositiveSize2DHandler`, `init`, `startElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
