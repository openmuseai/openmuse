# 01_function_emu_convert — EMU↔layout/render 单位换算

> feature: pl-01-units-view ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后管线新增「单位换算服务」：S2/S4/S5 里所有 EMU 矩形可以换算成 render（POINT 档）/pixel/doc 单位，系数单一来源。
没有它，pl-02 的 RectF 没有单位语义，pr-01 的白底 `fromEmu` 与 pr-02 的 rect→指令无换算依据——**断在坐标基准层**。

## 1. 功能说明

四档单例（`render()/doc()/layout()/pixel()`）＋ 全源单位 `convertX/Y` 换算＋ 静态档间系数；
Java 引擎已在 Rust `keep`，本 function 负责把换算契约钉死在 layout 层（稳定 API + 数值对拍测试），不重写引擎。

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `convertX(dstUnitType, srcVal, srcUnitType)`；`convertY`；便捷入口 `fromEmuX/Y`、`fromPixelX/Y`、`fromPointX/Y`、`fromDipX/Y`（int 版 `fromDipIntX/Y`）、`fromTwipX/Y`（UnitConverter 基类）；反方向 `toEmuX/Y/toPixelX/Y/toPointX/Y`；静态 `pointToEmu/emuToPoint`、`layoutRenderScaleX/Y`、`docRenderScaleX/Y`、`layoutDocScaleX/Y`、`calcFromDocScaleX/Y`。单位常量（Java 查证）：`UNIT_EMU=0`、`UNIT_PIXEL=1`、`UNIT_POINT=2`、`UNIT_TWIP=3`、`UNIT_COUNT=4`；`POINT_EMU=12700.0`、`TWIP_EMU=635.0`、`INCH_EMU=914400.0`、`POINT_TWIP=20.0`、`INCH_POINTS=72.0`。 |
| **输出** | 目标单位下换算值（f32；`fromDipIntX/Y` 为 `Math.round` 的 i32）。关键路径例（Java 查证）：`render().fromEmuX(12192000)=12192000/12700=960.0`（render/layout 档均为 **POINT**）；`pixel().fromEmuX(emu)=emu*xdpi/914400`；`doc().fromEmuX=identity`。 |
| **状态与副作用** | 单例：`sRender=new UnitsConverter(2)`、`sLayout=new UnitsConverter(2)`、`sDoc=new UnitsConverter(0)`、`sPixel=new UnitsConverter(1)`（Java ctor 实参查证）；静态系数为惰性 memo（`mRenderLayoutScaleX==0` 才算一次）；`setZoom(factor)` 仅写 `mScaleFactor = metrics.scaledDensity*factor`，`getZoom()` 读它；`convertX/Y` **不乘** zoom。 |
| **错误语义** | `convertX/convertY` 的 `switch(srcUnitType)` default → `new IllegalArgumentException()`（Java 查证）；未知源单位报错，无回退路径。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `UnitConverter` 基类 | base/converter | `fromEmuX/Y/fromPointX/Y/fromTwipX/Y/fromDipX/Y/dipToPixel/pixelToDip` | ✅ Rust `UnitConverter.rs`（keep） | — | 不新增单位、不改密度起点 |
| `UnitsConverter` 单例 | base/converter | 四档 × convert/from/to 全家桶 | ✅ Rust `UnitsConverter.rs`（keep，`render()/doc()/layout()/pixel()` 返回 `MutexGuard`） | — | 不删方法、不动 Java 注释锚点 |
| 设备密度 `KDisplayMetrics` | 平台侧 | `xdpi/ydpi/density/scaledDensity` | 固定常量版（96dpi） | 运行时探测延后 | 不引 Android 依赖 |
| layout 消费契约 | 本功能 | layout 层单位 API（`render().fromEmu*` 等） | 待 loop：包一层薄封装 | 不做 | 不复制一份引擎 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java`（180 行）、`UnitConverter.java`（56 行）、`EmuConverter.java`/`PixelConverter.java`/`PointConverter.java`/`TwipConverter.java` |
| Rust 现状 | `libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs`（keep，486 行）、`UnitConverter.rs`（keep，136 行）——**方法齐全** |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/值 | Rust 目标 | 归属 |
|---|---|---|---|
| `UNIT_EMU/PIXEL/POINT/TWIP/COUNT` | `0/1/2/3/4`（byte） | 常量 `i8` | 本 function（核对现有常量） |
| `render()/doc()/layout()/pixel()` | 单例（unitType 2/0/2/1） | `UnitsConverter::render()/doc()/layout()/pixel()`（`MutexGuard`） | 本 function（核对） |
| `convertX/convertY` | `float convertX(byte dst, float srcVal, byte src)` | `convertX/convertY`（已实现） | 本 function（核对 + 测试） |
| `fromEmuX/Y / fromPixelX/Y / fromPointX/Y / fromDipIntX/Y / toEmuX/Y / toPointX/Y` | 见 §1.1 公式 | 同名（已实现） | 本 function（核对 + 测试） |
| `layoutRenderScaleX/Y·docRenderScaleX/Y·layoutDocScaleX/Y` | 惰性 memo | 同名 static（已实现） | 本 function（核对） |
| `pointToEmu/emuToPoint` | `pt*12700 / emu/12700`（static） | 同名（已实现） | 本 function（核对） |
| `setZoom/getZoom` | `mScaleFactor=scaledDensity*factor` | 同名（已实现） | 本 function（核对） |
| `pixelsToCharWidth/charWidthToColumnUnits/pixelsToColumnUnits` | 表格列宽换算 | 同名（已实现） | 壳（无消费方不单测） |
| `dipToPixel/pixelToDip`（基类） | `dip*density / dip/density` | `UnitConverter` 内（已实现） | 壳 |

### 2.2 函数改写列表

- **本 function 改写：** 无引擎改写 —— 引擎已在 `keep` 状态全量实现。loop 内新增：layout 层薄封装（消费契约，如 `render_from_emu(w,h) -> (f32,f32)`）+ 数值对拍测试。若 loop 检查发现 `cargo check` 未过/锚点缺失，只修接线、不改换算系数。
- **停车（`todo!("DEFERRED_NON_FUNCTION")`）：** 无——本 function 没有跨 function 的停车点，所有调用方接口已在 Rust 存在。
- **禁改：** `UnitsConverter.rs`/`UnitConverter.rs` 已有语义（audit keep）、Java 注释锚点、`KDisplayMetrics` 固定密度常量（若已存在）。

## 3. 测试方案

### 3.1 代码审计与测试面

用例如下字段必须读 Java 方法体后写：`convertX/convertY` 四个 case + default；`fromEmuX` 在 POINT 档的 `÷12700`、PIXEL 档的 `×xdpi/914400`、EMU 档 identity；`fromDipX=fromPixel(dip*density)`；memo 系数首算/复用。

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| `convertX/Y` | UnitsConverter.java/.rs | 源单位 0-3 × 目标档 | 换算值 | default → `IllegalArgumentException` |
| `fromEmuX/Y` | 同上 | render/layout/doc/pixel 四档 | 四档换算值 | 档与单位类型绑定正确 |
| `fromDipIntX/Y` | 同上 | dip 值 | `round` 后 i32 | 舍入语义 |
| memo 系数 | `layoutRenderScaleX` 等 | 首调/二调 | 二次同值 | 惰性只算一次 |
| `pointToEmu/emuToPoint` | 同上 | 任意 pt/emu | `×12700/÷12700` | 0/负数 |
| `setZoom/getZoom` | 同上 | factor=0.5/2 | 写读一致 | 不影响 convertX |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 4×4 单位换算矩阵（X/Y） | 正常 | 是 | T01–T04 |
| C02 | 非法 srcUnitType（4/-1） | 失败 | 是 | T05 |
| C03 | 四档 `fromEmuX/Y` | 正常 | 是 | T06 |
| C04 | `fromDipX/Y` 与 `fromDipIntX/Y` | 正常 | 是 | T07 |
| C05 | `toEmu/toPixel/toPoint` 反方向 | 正常 | 是 | T08 |
| C06 | static memo 系数惰性 | 状态 | 是 | T09 |
| C07 | `pointToEmu/emuToPoint` | 边界 | 是 | T10 |
| C08 | zoom 存取不影响换算 | 状态 | 是 | T11 |
| C09 | `pixelsToCharWidth` 族 | 表格消费方 | 否（壳） | 无消费方，不单测 |

### 3.3 可测试性

纯函数 + static 单例；X/Y 各一处核心公式，Y 用相同输入同断言；memo 用「首调后改 intent 再调」验证二调不重算。不启动 XML/解析；不引入设备探测——密度用固定常量 `[64,96).clamp → 96`（Java `MIN_DPI=64/DEFAULT_DPI=96` 语义）。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 默认 | EMU→POINT 12192000 | 960.0 | 系数错即红 |
| T02 | C01 | 默认 | POINT→EMU 72 | 914400.0 | 系数错即红 |
| T03 | C01 | 默认 | TWIP→EMU 20 | 12700.0 | 系数错即红 |
| T04 | C01 | 默认 | PIXEL→EMU（xdpi=96）1 | 9525.0 | 密度语义错即红 |
| T05 | C02 | 默认 | srcUnitType=4 / -1 | `IllegalArgumentException` | 静默换算即红 |
| T06 | C03 | 默认 | four 档 `fromEmuX(12700)` | render/layout=1.0、doc=12700.0、pixel=96.0/914400… | 档错绑 |
| T07 | C04 | 默认 | `fromDipX(1.0)`/`fromDipIntX(1.6)` | `fromPixel(1.0*density)` / `round`=2 | 舍入错 |
| T08 | C05 | 默认 | `toEmuX(960.0)`（render 档） | 12192000.0 | 反方向错 |
| T09 | C06 | 首次 | `layoutRenderScaleX()` 两次 | 两次同值；内部只算一次 | 重算/漂移 |
| T10 | C07 | 默认 | `pointToEmu(0)/emuToPoint(0)` | 0.0 | 边界错 |
| T11 | C08 | setZoom(2) | `convertX(...)` 同输入 | 与 zoom=1 全同 | zoom 进入换算 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs（keep）＋ layout 层薄封装（loop 定）
test:  libs/ppt-core/tests/cn/wps/base/converter/UnitsConverter.rs
[[test]] name = "01_function_emu_convert"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_emu_convert

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_emu_convert \
  --src src/cn/wps/base/converter/UnitsConverter.rs \
  --src src/cn/wps/base/converter/UnitConverter.rs \
  --src src/cn/wps/base/converter/EmuConverter.rs \
  --src src/cn/wps/base/converter/PixelConverter.rs \
  --src src/cn/wps/base/converter/PointConverter.rs \
  --src src/cn/wps/base/converter/TwipConverter.rs \
  --report libs/ppt-core/layout/features/pl-01-units-view/functions/01_function_emu_convert/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_emu_convert --json`)
date: 2026-08-30 10:17

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/base/converter/UnitsConverter.rs` | 151/322 | 46.9% | 241/546 | 44.1% |
| **function total** | **151/322** | **46.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitConverter.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/EmuConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/EmuConverter.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/PixelConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/PixelConverter.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/PointConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/PointConverter.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/base/converter/TwipConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/TwipConverter.java
```

Loop 结果：PASS（UnitsConverter / UnitConverter / Emu / Pixel / Point / Twip）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/UnitConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/UnitConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/EmuConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/EmuConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/PixelConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/PixelConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/PointConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/PointConverter.rs \
  --java koffice/presentation/core/cn/wps/base/converter/TwipConverter.java \
  --rs libs/ppt-core/src/cn/wps/base/converter/TwipConverter.rs \
  --report libs/ppt-core/layout/features/pl-01-units-view/functions/01_function_emu_convert/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 02:18

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `UnitsConverter` | `koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java` | `libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs` | **PASS** | `UnitsConverter`, `init`, `render`, `doc`, `layout`, `pixel`, `getUnitType`, `convertX`, `convertY`, `fromEmuX`, `fromEmuY`, `fromPixelX`, `fromPixelY`, `fromPointX`, `fromPointY`, `fromDipX`, `fromDipY`, `fromDipIntX`, `fromDipIntY`, `toEmuX`, `toEmuY`, `toPixelX`, `toPixelY`, `toPointX`, `toPointY`, `pointToEmu`, `emuToPoint`, `layoutRenderScaleX`, `layoutRenderScaleY`, `docRenderScaleX`, `docRenderScaleY`, `layoutDocScaleX`, `layoutDocScaleY`, `calcFromDocScaleX`, `calcFromDocScaleY`, `setZoom`, `getZoom`, `pixelsToCharWidth`, `charWidthToColumnUnits`, `pixelsToColumnUnits` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->