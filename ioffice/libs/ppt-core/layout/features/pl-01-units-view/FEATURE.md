# pl-01-units-view — EMU 与 slide 视口

> 层：**layout** ｜ 状态：**pass** ｜ Java：`cn.wps.base.converter.UnitsConverter`（`layout()/render()/doc()/pixel()` 四档单例）＋ `cn.wps.show.data.Presentation.SlideSize`（`KmoPresentation.slideWidth()/slideHeight()`）

## 目标

把「EMU ↔ layout/render 单位」这一贯穿 S1–S5 的换算服务接到 layout 层，并产出 **slide 内容区**（白底尺寸）。
Java 侧换算引擎（`UnitsConverter`/`UnitConverter`）在 Rust 已审计为 `keep`（`base/converter/*.rs`，486 行）——
本 feature 不是从零搬引擎，而是：① 把已 keep 的换算服务以 layout 层的契约形式钉死（供 pl-02…pl-06 与 pr-01 的 `fromEmu` 消费点统一引用）；② 把 `Presentation.slideSize` 换成 render/layout 单位的视口矩形。

## I/O（锁死）

| 输入 | 输出 |
|---|---|
| `srcUnitType(0=EMU/1=PIXEL/2=POINT/3=TWIP) + srcVal`（`UnitsConverter.convertX/convertY`） | `dstUnitType` 下的换算值（f32；非法源单位 → `IllegalArgumentException`） |
| `fromEmuX/Y`、`fromPointX/Y`、`fromDipX/Y`、`fromPixelX/Y`、`fromTwipX/Y` 便捷入口 | 按实例目标单位换算的 f32；`fromDipIntX/Y` 返回 `round` 后的 i32 |
| `toEmuX/Y`、`toPixelX/Y`、`toPointX/Y` | 反方向换算 |
| 静态系数：`layoutRenderScaleX/Y`、`docRenderScaleX/Y`、`layoutDocScaleX/Y`、`calcFromDocScaleX/Y`、`pointToEmu`、`emuToPoint` | 缓存的档间系数 / `pt*12700`、`emu/12700` |
| `setZoom(factor)` / `getZoom()` | 仅状态存取（`mScaleFactor = scaledDensity × factor`），**不参与** `convertX` 计算（Java 源码查证） |
| `Presentation.SlideSize`（`cx()/cy()`，EMU）`+ zoom` | slide 内容区：`(0,0, render().fromEmuX(cx), render().fromEmuY(cy))`；缺省时默认 `12192000×6858000` EMU |

四档单例的单位类型（Java 查证 `UnitsConverter.java` ctor 实参）：`render()=2(POINT)`、`layout()=2(POINT)`、`doc()=0(EMU)`、`pixel()=1(PIXEL)`。即 `render().fromEmuX` 的实际换算路径是 EMU→**point**（÷12700）。

## Java 管线对照

S2 布局的第 0 项：**单位基准与视口**。S3 的白底/背景分支（`ShapeUtil.textShapeSnapshot` 用 `UnitsConverter.render().fromEmuX(presentation.slideWidth())`）、S4/S5 的画布都消费本 feature 的输出。

| Java 类 | 方法/字段 | 对应 Rust 目标 | 环节 |
|---|---|---|---|
| `cn.wps.base.converter.UnitsConverter` | `render()/doc()/layout()/pixel()`、`convertX/Y`、`fromEmuX/Y`、`setZoom/getZoom`、`layoutRenderScaleX/Y`、`pointToEmu/emuToPoint` | `libs/ppt-core/src/cn/wps/base/converter/UnitsConverter.rs`（**已 keep**，方法齐全） | pl-01 01 |
| `cn.wps.base.converter.UnitConverter` | `fromEmuX/Y/…/fromTwipX/Y`（单位基类）、`dipToPixel/pixelToDip` | `libs/ppt-core/src/cn/wps/base/converter/UnitConverter.rs`（**已 keep**） | pl-01 01 |
| `cn.wps.base.converter.EmuConverter/PixelConverter/PointConverter/TwipConverter` | `fromEmuX=emu`、`fromPointX=pt*12700`、`fromTwipX=twip*635`、`fromPixelX=px*914400/xdpi`… | Rust `UnitConverter.rs` 内置（keep） | pl-01 01 |
| `cn.wps.show.data.Presentation$SlideSize` | `cx()/cy()/type()`（key 1/2/3；缺省空） | `libs/ppt-core/src/cn/wps/show/data/Presentation.rs`（已存在：`SlideSize`、`cx/cy`；`show/data/mod.rs` 当前注释掉 `pub mod Presentation;`，loop 接线时启用） | pl-01 02 |
| `cn.wps.show.app.KmoPresentation` | `slideWidth()=slideSize().cx()`、`slideHeight()=slideSize().cy()`、`slideSizeType()` | `libs/ppt-core/src/cn/wps/show/app/KmoPresentation.rs`（已存在，1247 行起） | pl-01 02 |
| `cn.wps.show.pptxr.ppt.PptxrPresentation` | `DEFAULT_SLDSIZE_CX=12192000`、`CY=6858000`；`addDefaultProperty()`（sldSz 缺省时写 12192000×6858000 EMU） | Rust 对应在 **pp-06**（presentation.xml 处理）；layout 侧只消费结果 | pl-01 02 |

## 能力解锁

- **01 `emu_convert`**：EMU↔px（point/pixel/twip 全档）换算可用 → 后续所有 rect 有单位语义；缺失时 S3 `drawSlideWhiteBackground` 的 `fromEmu` 与 S4/S5 的绘制坐标无基准。
- **02 `slide_viewport`**：slide 内容区 = 白底尺寸 → S3 白底/背景有画布（pr-01 的输入之一）。

## 必须 / 壳 / 延后

### 必须
- 四档单例与单位常量（`UNIT_EMU/PIXEL/POINT/TWIP`、`POINT_EMU=12700`、`TWIP_EMU=635`、`INCH_EMU=914400`）在 layout 层以稳定 API 暴露；
- `convertX/convertY` 全源单位分派 + 非法单位 `IllegalArgumentException` 语义；
- `slide_viewport`：`slideWidth/slideHeight`（EMU）→ render 档视口矩形 + 默认尺寸常量。

### 壳
- `setZoom/getZoom` 状态（Rust 已实现，核对即可）；`pixelsToCharWidth/charWidthToColumnUnits/pixelsToColumnUnits`（表格列宽换算，Java 已实现、Rust 已 keep，无消费方前不单测）。

### 延后
- dip/density 设备语义（`KDisplayMetrics`）：MVP 用固定密度（96dpi 基准）或 Java 同式 `xdpi/ydpi` 常量，不做 Android 运行时探测；
- 其它 `doc()` 消费者（页面尺寸换算、notes size 6858000×914400 等）。

## 范围

本 feature **只**处理单位换算与 slide 视口矩形。不含：形状 xfrm（pl-02）、placeholder 继承（pl-03）、文本排版、任何 `android.graphics.Canvas`/Paint 语义。

## 验收

1. `convertX/Y` 四个源单位 × 目标档（render/layout=POINT、doc=EMU、pixel=PIXEL）与 Java 公式逐值一致（用例见 FUNCTIONS.md §4）；
2. `fromEmuX(12192000)` = `12192000/12700 = 960.0`（render 档，POINT）；
3. slide 内容区：sldSz 缺省 → `(0,0,960,540)`（render 档 POINT），或按显式 sldSz 换算；
4. 未知源单位 → `IllegalArgumentException`（Rust `panic!`/`Result::Err` 二选一，用例钉死不漂移）；
5. 不与 Java 发明第二套单位名/系数（复刻 `UnitsConverter` 既有系数，不新造）。

## 依赖

- 前置 feature：无（`—`，manifest 一致）；并行：pl-02。
- 必读 Java：`koffice/presentation/core/cn/wps/base/converter/UnitsConverter.java`（180 行）、`UnitConverter.java`、`EmuConverter.java`、`PixelConverter.java`、`PointConverter.java`、`TwipConverter.java`；`…/show/data/Presentation.java`（`SlideSize`，2938 行起）；`…/show/app/KmoPresentation.java`（722 行起）；`…/show/pptxr/ppt/PptxrPresentation.java`（`addDefaultProperty`，102 行起）。
- Rust 现状：`base/converter/UnitsConverter.rs`（keep，486 行，全方法已实现）、`base/converter/UnitConverter.rs`（keep）；`show/data/Presentation.rs`（keep，`SlideSize`/`cx/cy` 已实现）；`app/KmoPresentation.rs`（`slideWidth/slideHeight` 已实现）。

## 不过扩展

- 不实现 `android.graphics` 度量（字体/位图）；
- 不引入新单位制（双单位并存点）——复刻 Java 系数；
- 不把 zoom 编进换算管线（Java 语义 `convertX` 不含 zoom，锁死）。