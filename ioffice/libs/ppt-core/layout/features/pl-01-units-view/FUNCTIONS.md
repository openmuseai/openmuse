# pl-01-units-view — function 清单

> 层：layout ｜ 状态：**pass** ｜ crate：`libs/ppt-core`

## §0 管线位置（S2 内数据流）

本 feature 是 S2 布局的**第 0 段**：先定坐标系，再算框。数据流：

```text
Presentation.SlideSize(EMU cx/cy)  ┐
                                   ├→ [02 slide_viewport] → 白底/内容矩形（render 档）
UnitsConverter(EMU/PIXEL/POINT/TWIP) ┘
        │
        └→ [01 emu_convert] → 档间系数/换算值 ──→ pl-02（绝对 RectF 单位语义）
                                              ──→ pr-01（白底尺寸）/ pr-02（rect→指令）
```

S2 后续环节（pl-02…pl-06）全部以 EMU 矩形计算，**出 S4/S5 前**才经 `render().fromEmuX/Y` 换 render 档——单位换算贯穿但不改变 shape 数据流。

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream |
|---|---|---|---|---|---|---|
| 1 | [01_function_emu_convert](./functions/01_function_emu_convert/FUNCTION.md) | EMU↔layout/render 单位换算 | srcUnitType+srcVal / `fromEmuX/Y` 便捷入口 | 目标单位换算值（f32/i32） | 无（`base/converter` 已 keep，本 feature 只契约化 + 测试） | 02；pl-02；pr-01；pr-02 |
| 2 | [02_function_slide_viewport](./functions/02_function_slide_viewport/FUNCTION.md) | slide 内容区（白底视口） | `SlideSize.cx/cy`（EMU）+ zoom | render 档内容矩形（默认 12192000×6858000 → 960×540pt） | 01；pp-06（sldSz 实际写入） | pr-01（白底）；pl-06 |

## §2 数据流

```text
(EMU)   slideSize cx/cy ──[02]──> (render) (0,0,960,540) / (0,0,fromEmu(cx),fromEmu(cy))
(src)   unit + val    ──[01]──> convertX/Y ──> 目标单位值
系数:   layoutRenderScaleX/Y（惰性缓存 2 档 POINT 间换算 = 1.0）──> 分派给 pr-01 白底换算
```

单向、无环；02 消费 01 的 `render().fromEmuX/Y`。

## §3 测试面

- **01**：全部 4 源单位 × 4 目标单位换算表；`fromEmuX/Y/fromPointX/Y/fromDipX/Y`；`toEmu/toPixel/toPoint`；`pointToEmu/emuToPoint`；静态系数惰性缓存（连调两次同值）；非法源单位 `IllegalArgumentException`。
- **02**：sldSz 缺省默认（12192000×6858000）；显式 sldSz（如 16:9 12192000×6858000/6858000…）；zoom 状态存取不影响换算；0/负尺寸输入语义。

## §4 验收

```text
01 emu_convert：数值对拍 Java 公式（12700/635/914400/20/72 系数）＋穷举单位矩阵；未知单位抛错
02 slide_viewport：缺省=960×540（render 档）；显式 sldSz 按 fromEmu 换算；zoom 只存不乘
+ 与 pl-02 联调：RectF 以 pl-01 单位语义产出
```

## §5 loop 顺序

```text
01_function_emu_convert（纯引擎核对，先行）
02_function_slide_viewport（依赖 01 的从 EMU 换算 + pp-06 接线或 fixture sldSz）
```

## §6 Java/Rust 范围

| 层 | Java | Rust |
|---|---|---|
| 01 | `base/converter/UnitsConverter.java`（180 行，全量）+ `UnitConverter.java` + Emu/Pixel/Point/TwipConverter | ⚠ 已 keep：`src/cn/wps/base/converter/UnitsConverter.rs`（486 行）+ `UnitConverter.rs` —— **不改 Java 注释锚点、不删方法**；本 function 只新增 layout 层消费契约 + 测试 |
| 02 | `show/data/Presentation.java`（`SlideSize`）；`show/app/KmoPresentation.java`（`slideWidth/Height`）；`pptxr/ppt/PptxrPresentation.java`（默认 12192000×6858000） | `src/cn/wps/show/data/Presentation.rs`（keep；`show/data/mod.rs` 当前 `// pub mod Presentation;` → loop 接线）；`app/KmoPresentation.rs`（`slideWidth/Height` 已实现） |

> 注：Rust `UnitsConverter.rs` 中 `doc()`/`render()`/`layout()`/`pixel()` 返回 `MutexGuard`（static 可变单例），与 Java 单例语义等价；loop 确认锁粒度不引入 `async`。