# pl-04-text-box — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**，不切 `DataTransform` 整类。`01`–`02` 均为 **pass**。

## 0. 管线位置

```text
S0 parse:  pp-02 spTree → KmoShowShape；pp-11 txBody → KmoTextBody
  + pl-03  ph 继承合并（前置可选，fixture 可绕）
  → 01 ShapeToTextBox   shape + exToInternal → TextBoxData（段/run → TextData/ParaInfo/SpanInfo）
  → 02 TextBoxProps     TextBodyProperties → insets/anchor/vertical/wrap/autofit 字段
  → pl-05 断行（本 feature 不产行）
```

Java 对照：`ShowTextLayout.layout`（编排）→ `ShapeDataTransform`（`getTextOperator`/`covertTextBodyProperties`）+ 基类 `DataTransform`（`exToInternal`/`convertToBoxData`/`fetchSpanInfo`/`getHAlign`/`getLineSpacing`）+ `DefineConverter`（`convertPhHolder`/`convertTxtAnchor`）+ `TextBodyProperties`/`TextBoxData`/`TextData`/`ParaInfo`。

**不**移植：`convertBullet` 全量、`setSpanInfo` 的 effect/blip/gradient/outline/shadow 分支、`CellDataTransform`、域重置（MVP `canRenderCache` 固定 true）。

## 1. 清单

| order | function_id | 一句话 | I | O | 上游 | 下游 | status |
|------|-------------|--------|---|---|------|------|--------|
| 1 | [01_function_shape_to_textbox](functions/01_function_shape_to_textbox/FUNCTION.md) | 段与 run → TextBoxData 文本 | shape（fixture or pp-02/11） | `TextBoxData`（TextData[]/SpanInfo[]/phType） | pp-11; pl-03 | 02; pl-05 | pass |
| 2 | [02_function_textbox_props](functions/02_function_textbox_props/FUNCTION.md) | 框属性（insets/anchor/vertical/wrap/autofit） | `TextBoxData` + `TextBodyProperties` | 框属性字段（layout 单位） | 01 | pl-05 | pass |

## 2. 数据流

`01` 产出 `TextBoxData` 后 `02` 在同一对象上落框属性；两者都只读 `KmoShowShape`/`TextBodyProperties`，不断行、不度量。所有几何字段走 **layout 单位**（`emuToLayoutUnits = units.fromEmuY(emu)`），禁止混入 EMU 或 render 单位。`02` 的默认 inset 常量必须与 Java 一致（`TextBodyProperties` 默认 left/right=91440、top/bottom=45720 EMU）。

## 3. 测试面

- crate：`libs/ppt-core/Cargo.toml`（`wpslayout` 已在依赖中，`TextBoxData` 从 `wpslayout::cn::wps::text::layout::data::TextBoxData` 引入）。
- 测试不跑完整 `ShowTextLayout.layout`：`01` 喂 fixture `KmoShowShape`（或 fixture `KmoTextBody`+最小 `KmoShowShape`），`02` 直接喂 `TextBodyProperties`。
- 断言点：`mSourceText` 与 span 区间、`phType`、insets 换算值、`anchor` 映射、空 body → `isEmpty()`；非法输入（无 textOp）→ empty 而非 panic。
- 待生成 `[[test]]`：`01_function_shape_to_textbox` / `02_function_textbox_props`（loop 时才写）。

## 4. 验收

01 → 02。验收口径：夹具字段逐项与 Java 语义对齐（见 FEATURE.md「验收」）；空 body 语义（`isEmpty` → `ShowTextLayout.layout` 返回 null）用 01 的用例覆盖；02 的 insets 换算对 5 组 EMU 输入与手算 layout 单位一致。全部通过后才能标 **pass**（现为 **spec**）。

## 5. loop

顺序 `01 → 02`；每个 function 独立跑「改写 → 测试 → 回归」。`02` 依赖 `01` 的 `TextBoxData` 结构（先稳定 01 再 loop 02）。loop 列表外方法（bullet 细节、域重置、effect 分支）以 `todo!("DEFERRED_NON_FUNCTION")` 停车，禁止删 `// Java:` 注释。

## 6. Java/Rust 范围

| Java | Rust（计划路径） | 本波 |
|------|------------------|------|
| `ShapeDataTransform`（show/layout/dataaction） | `libs/ppt-core/src/cn/wps/show/layout/dataaction/ShapeDataTransform.rs` | 01 门面；其余 todo 停车 |
| `DataTransform.exToInternal/convertToBoxData/fetchSpanInfo`（基类） | 同包 `DataTransform.rs` | 01 主路径；bullet/effect 分支 DEFERRED |
| `TextBodyProperties` | `libs/ppt-core/src/cn/wps/drawing/data/text/TextBodyProperties.rs`（已有 3209 行） | 02 读取端（读方法已可查） |
| `TextBoxData/TextData/ParaInfo`（core-libs/wpslayout） | `libs/core-libs/wpslayout/src/cn/wps/text/layout/data/{TextBoxData,TextData,ParaInfo}.rs` | 产品字段对齐；缺失方法 待 loop 确认 |
| `DefineConverter` | show/layout/dataaction `DefineConverter.rs` | 02 的 anchor/ph 映射表 |
| `ShowTextLayout.layout` | show/layout `ShowTextLayout.rs` | 不在本波（pl-05 编排） |