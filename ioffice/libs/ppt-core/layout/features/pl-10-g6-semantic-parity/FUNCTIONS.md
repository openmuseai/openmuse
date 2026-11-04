# pl-10-g6-semantic-parity — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S6/S7 语义注记（G6）**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S6/S7 已有实现
  → 本 feature 钉语义差
  → 框/字像素对齐前提
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_anchor_1_2](./functions/01_function_anchor_1_2/FUNCTION.md) | bodyPr anchor 1/2 对齐 Java | TextBoxData.anchor 0/1/2 + 行高 < 框高 | 1=垂直居中，2=底（Java RichTextLayout.modifyTextBoxAlign:1050） | pl-05 | — | spec |
| 2 | [02_function_ph_type_default](./functions/02_function_ph_type_default/FUNCTION.md) | ph type 缺省 8 | Placeholder 无 type 属性、无 parent | type()==8（ShowShape.Placeholder.java:865-867）不是 0 | pl-03 | — | spec |
| 3 | [03_function_neg_emu_round](./functions/03_function_neg_emu_round/FUNCTION.md) | 负 EMU 舍入同 Java | 负 f32 emu→px | Math.round 语义 floor(x+0.5) 对负值 | pl-01 | — | spec |
| 4 | [04_function_canrotate_group](./functions/04_function_canrotate_group/FUNCTION.md) | 组 canRotate 递归 | type0 组 + 子 canRotate | 任一子 false → 组 false；type4 永远 false；其它 true | pp-20:02 | — | spec |

## §2 数据流

```text
anchor match 对齐 Java
Placeholder 缺省 8
Math.round 负值
canRotate 递归
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_anchor_1_2 | libs/ppt-core | `src/cn/wps/show/layout/typo/GreedyWrap.rs` | `01_function_anchor_1_2` | `tests/cn/wps/show/layout/typo/01_function_anchor_1_2.rs` |
| 02_function_ph_type_default | libs/ppt-core | `src/cn/wps/show/util/layout_shape.rs 或 data ShowShape.Placeholder` | `02_function_ph_type_default` | `tests/cn/wps/show/layout/02_function_ph_type_default.rs` |
| 03_function_neg_emu_round | libs/ppt-core | `src/cn/wps/base/converter/UnitsConverter.rs` | `03_function_neg_emu_round` | `tests/cn/wps/base/converter/03_function_neg_emu_round.rs` |
| 04_function_canrotate_group | libs/ppt-core | `src/cn/wps/show/app/shape/KmoShowShape.rs（产线）` | `04_function_canrotate_group` | `tests/cn/wps/show/app/shape/04_function_canrotate_group.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03 → 04
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| RichTextLayout / GreedyWrap | GreedyWrap.rs | 01 |
| ShowShape.Placeholder | layout_shape / data | 02 |
| UnitsConverter | UnitsConverter.rs | 03 |
| KmoShowShape | app/shape/KmoShowShape.rs | 04 |
