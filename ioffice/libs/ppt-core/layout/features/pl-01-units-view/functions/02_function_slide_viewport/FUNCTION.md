# 02_function_slide_viewport — slide 内容区（白底视口）

> feature: pl-01-units-view ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后管线新增「slide 内容区」：`(slideSize EMU) → 白底/内容矩形（render 档）`，S3 白底（`drawSlideWhiteBackground`）与背景分支有画布尺寸。
没有它，pr-01 层序的第一步（白底尺寸）无来源，S4/S5 的坐标换算无参考系——**断在画布基准**。

## 1. 功能说明

读 `KmoPresentation.slideWidth()/slideHeight()`（即 `Presentation.SlideSize.cx()/cy()`，EMU），
经 `UnitsConverter.render().fromEmuX/Y` 换 render 档，产出 slide 内容矩形；sldSz 缺失时取 `PptxrPresentation` 默认
`12192000×6858000`（EMU，4:3）。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `Presentation.SlideSize`（嵌套于 `show/data/Presentation.java`，`$cx=1/$cy=2/$type=3`；`KmoPresentation.slideWidth()=slideSize().cx()`、`slideHeight()=slideSize().cy()`）；zoom（`UnitsConverter.setZoom(factor)`）。sldSz 缺省时由 `PptxrPresentation.addDefaultProperty()` 写入 `12192000×6858000`（Java 查证：`DEFAULT_SLDSIZE_CX=12192000`、`DEFAULT_SLDSIZE_CY=6858000`，`sldSize.putCx(12192000)/putCy(6858000)`）。 |
| **输出** | slide 内容矩形（render 档）：`(0,0, render().fromEmuX(cx), render().fromEmuY(cy))`。缺省 → `(0,0,960,540)`（EMU÷12700，POINT 档）。Java 参照（`ShapeUtil.textShapeSnapshot*`）：`(int)UnitsConverter.render().fromEmuX(presentation.slideWidth())`。 |
| **状态与副作用** | 只读模型；zoom 仅为状态（`getZoom`）；不改 `SlideSize`、不写回 presentation 协议。 |
| **错误语义** | Java 无显式防御：slizeSize 缺失走默认（parse 侧 `addDefaultProperty` 兜底）；layout 侧把「无 slideSize」视为异常输入（Rust `Option` → 用默认或报错，用例钉死二选一）；`cx/cy≤0` Java 未防御——Rust 断言拒绝。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `01_function_emu_convert` | 本 feature | `render().fromEmuX/Y` | ✅（引擎 keep + 01 契约） | — | 不重复实现换算 |
| `Presentation.SlideSize` | show/data | `cx()/cy()/type()` | ✅ Rust `show/data/Presentation.rs`（`SlideSize`、`cx/cy` 已实现） | `show/data/mod.rs` 当前 `// pub mod Presentation;`（注释中）→ loop 接线启用 | 不改 SlideSize 语义 |
| `KmoPresentation` | show/app | `slideWidth()/slideHeight()` | ✅ Rust `app/KmoPresentation.rs`（1247-1260 行已实现） | — | 不删 / 不改 |
| 默认 sldSz | pp-06（parse） | `12192000×6858000` 兜底 | 待 loop：直接常量或走 parse 模型 | 若 fixture 路线则不用 pp-06 | 不提前改 pp-06 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/data/Presentation.java`（`SlideSize` 2938 行起）、`…/show/app/KmoPresentation.java`（722-737 行）、`…/show/pptxr/ppt/PptxrPresentation.java`（28-30、102-119 行） |
| Rust 现状 | `libs/ppt-core/src/cn/wps/show/data/Presentation.rs`（keep；`SlideSize`/`cx/cy` 已实现）、`libs/ppt-core/src/cn/wps/show/app/KmoPresentation.rs`（`slideWidth/slideHeight` 已实现） |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/值 | Rust 目标 | 归属 |
|---|---|---|---|
| `Presentation.SlideSize.cx()/cy()/type()` | `int`（EMU；key 1/2/3） | `SlideSize::cx()/cy()/type()`（已实现） | 本 function（核对/接线） |
| `KmoPresentation.slideWidth()/slideHeight()` | `slideSize().cx()/cy()` | `KmoPresentation::slideWidth()/slideHeight()`（已实现） | 本 function（核对） |
| `slideSizeType()` | `slideSize().type()`（0=screen16x9…枚举） | `slideSizeType()`（已实现） | 壳 |
| `PptxrPresentation.addDefaultProperty()` | 无 sldSz → `putCx(12192000)/putCy(6858000)` | pp-06 领域（parse）；layout 侧仅当输入缺失时用同一常量 | 延后（pp-06 接线） |
| `UnitsConverter.render().fromEmuX/Y` | EMU→POINT（÷12700） | `UnitsConverter::render().fromEmuX/Y`（keep） | 01（复用） |

### 2.2 函数改写列表

- **本 function 改写：** viewport 组合函数（`fn slide_viewport(cx: i32, cy: i32, zoom: f32) -> (f32,f32)` 或等价）——读取 slideSize → `render().fromEmuX/Y` → 内容矩形 + zoom 状态存储。若 Rust `show/data/mod.rs` 的 `pub mod Presentation;` 仍被注释，本 loop 接线启用（只接线不改模型）。
- **停车（`todo!("DEFERRED_NON_FUNCTION")`）：** pp-06 的 sldSz 解析写入（slideSize 默认值写入属 parse）；本 function 用 fixture/常量等价驱动。
- **禁改：** `Presentation.rs`/`KmoPresentation.rs`/`UnitsConverter.rs` 语义；pp-06 未开始前不替它写盘。

## 3. 测试方案

### 3.1 代码审计与测试面

用例如下字段必须读 Java 方法体后写：`KmoPresentation.slideWidth/slideHeight`（722 行起）；`PptxrPresentation.addDefaultProperty`（102-119 行，注意 sldSz 写入发生在 `Presentation.open(protocol)` 后）；`SlideSize` 缺省 → `KoBin.Null` 空解码语义（0）+ parse 侧兜底。

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `slideWidth()/slideHeight()` | KmoPresentation.java/.rs | 默认/显式 sldSz | cx/cy EMU | 缺省=12192000/6858000 |
| `addDefaultProperty()` | PptxrPresentation.java | 无 sldSz | 写 12192000×6858000 | 有 sldSz 不动 |
| `render().fromEmuX/Y` | UnitsConverter | cx/cy | render 档尺寸 | POINT 档÷12700 |
| `setZoom/getZoom` | UnitsConverter | factor | 状态一致 | 不影响换算 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 缺省 sldSz → 120000 内容矩形（960×540） | 正常 | 是 | T01 |
| C02 | 显式 sldSz（16:9）→ fromEmu 换算 | 正常 | 是 | T02 |
| C03 | zoom 状态存取 | 状态 | 是 | T03 |
| C04 | 无 slideSize 输入（Option::None） | 失败 | 是 | T04 |
| C05 | cx/cy ≤ 0 | 边界 | 是 | T05 |

### 3.3 可测试性

不依赖真实 parse：slideSize 直接构造/注入；默认常量独立于 pp-06 单测。viewport 输出只与 (cx,cy,zoom,单位档) 相关，纯函数化即可。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 无 sldSz | None → 默认 | `(0,0,960.0,540.0)`（render 档） | 默认值错即红 |
| T02 | C02 | 显式 sldSz | cx/cy=12192000/6858000 | `(0,0,960.0,540.0)`；16:9 输入按 fromEmu 换算 | 换算错即红 |
| T03 | C03 | setZoom(2.0) | getZoom() | 2.0；换算结果与 zoom=1 全同 | zoom 进入换算 |
| T04 | C04 | 输入 None | viewport(None) | 走默认或 Err（钉死一种） | 两义性漂移 |
| T05 | C05 | cx=0/cy=-1 | viewport(Some) | 断言拒绝（panic/Err，钉死一种） | 静默产出 0 框 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/layout/slide_viewport.rs（layout 组合；默认 sldSz 来自 PptxrPresentation 常量）
test:  libs/ppt-core/tests/cn/wps/show/layout/slide_viewport.rs
[[test]] name = "02_function_slide_viewport"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_slide_viewport

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_slide_viewport \
  --src src/cn/wps/show/layout/slide_viewport.rs \
  --report libs/ppt-core/layout/features/pl-01-units-view/functions/02_function_slide_viewport/FUNCTION.md
```

（src 参数指向本次 loop 实际接线文件；若薄封装独立成文件则指向该文件）

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_slide_viewport --json`)
date: 2026-08-30 10:34

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/layout/slide_viewport.rs` | 16/16 | 100.0% | 19/19 | 100.0% |
| **function total** | **16/16** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java
```

Loop 结果：PASS（消费 01 的 UnitsConverter；`slide_viewport.rs` 为无独立 Java 的 layout 组合函数。`Presentation.rs` keep 未接线：`show/data/mod.rs` 仍注释 `pub mod Presentation`，且 layout 校验 FAIL 邻接，本 function 不改该叶。）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs \
  --report libs/ppt-core/layout/features/pl-01-units-view/functions/02_function_slide_viewport/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 01:57

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `UnitsConverter` | `koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java` | `libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs` | **PASS** | `UnitsConverter`, `init`, `render`, `doc`, `layout`, `pixel`, `getUnitType`, `convertX`, `convertY`, `fromEmuX`, `fromEmuY`, `fromPixelX`, `fromPixelY`, `fromPointX`, `fromPointY`, `fromDipX`, `fromDipY`, `fromDipIntX`, `fromDipIntY`, `toEmuX`, `toEmuY`, `toPixelX`, `toPixelY`, `toPointX`, `toPointY`, `pointToEmu`, `emuToPoint`, `layoutRenderScaleX`, `layoutRenderScaleY`, `docRenderScaleX`, `docRenderScaleY`, `layoutDocScaleX`, `layoutDocScaleY`, `calcFromDocScaleX`, `calcFromDocScaleY`, `setZoom`, `getZoom`, `pixelsToCharWidth`, `charWidthToColumnUnits`, `pixelsToColumnUnits` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->