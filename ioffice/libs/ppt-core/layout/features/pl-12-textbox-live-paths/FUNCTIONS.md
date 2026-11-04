# pl-12-textbox-live-paths — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S7 DataTransform**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S7 exToInternal
  → 本 feature 解开三处 todo
  → 才能排完真实 PPT
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_para_direction](./functions/01_function_para_direction/FUNCTION.md) | getParaDirection | paraProp rtl 有/无 + parent 链 | bool RTL；无属性沿 parent 直到 hasRightToLeft | pl-04 | 03 | spec |
| 2 | [02_function_text_spacing](./functions/02_function_text_spacing/FUNCTION.md) | getTextSpacing | spcPts / spcPct / 皆无 | pts→layout units；pct→ fontsize*spcPct*1.2；皆无 0 | pl-04 | — | spec |
| 3 | [03_function_convert_bullet](./functions/03_function_convert_bullet/FUNCTION.md) | convertBullet 不 panic | buNone / buChar / buAutoNum | buNone type=-1 跳过；其余 BulletsInfo.visible 且不 panic | 01 | pr-10 | spec |

## §2 数据流

```text
getParaDirection → convertBullet
getTextSpacing → mSpcBef/Aft
buNone 早退
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_para_direction | libs/ppt-core | `src/cn/wps/show/layout/dataaction/DataTransform.rs` | `01_function_para_direction` | `tests/cn/wps/show/layout/dataaction/01_function_para_direction.rs` |
| 02_function_text_spacing | libs/ppt-core | `src/cn/wps/show/layout/dataaction/DataTransform.rs` | `02_function_text_spacing` | `tests/cn/wps/show/layout/dataaction/02_function_text_spacing.rs` |
| 03_function_convert_bullet | libs/ppt-core | `src/cn/wps/show/layout/dataaction/DataTransform.rs` | `03_function_convert_bullet` | `tests/cn/wps/show/layout/dataaction/03_function_convert_bullet.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| DataTransform | DataTransform.rs | 全部 |
