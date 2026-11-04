# pp-20-open-write-kmo — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S4 产线写入**。状态：01–04 **pass**。

## §0 管线位置

```text
S2/S3 walk_xml
  → S4 产线（本 feature，替换 pptx_mvp layout_shape 写入）
  → S5/S8 ShowTypoShape.getShape
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_sw_group_on_mod](./functions/01_function_sw_group_on_mod/FUNCTION.md) | SwGroup 进默认 mod 并分发 | grpSp 事件 int-id（3211593/1594/1469/1592/1595/1596） | enterSubElement 返回对应 handler；sp/grp/pic 已 addChild | pp-19:04 | 02 | pass |
| 2 | [02_function_open_tree_is_app_kmo](./functions/02_function_open_tree_is_app_kmo/FUNCTION.md) | open 产物是 app Kmo 树 | 最小 pptx zip（一页一 sp） | app::KmoShapeTree.shapeCount()==1 且 type()==DgStore.queryType(id) | 01;pp-19:01 | 03;04 | pass |
| 3 | [03_function_open_ln_survives](./functions/03_function_open_ln_survives/FUNCTION.md) | open 路径 ln 不丢 | spPr 含 a:ln w=25400 的 zip | dgShape.spPr().ln() 可查宽度 | 02;pp-19:05 | pr-08 | pass |
| 4 | [04_function_typo_host_app_kmo](./functions/04_function_typo_host_app_kmo/FUNCTION.md) | ShowTypoShape 宿主切 app Kmo | app::KmoShowShape | ShowTypoShape.mShapeBase 类型非 layout_shape | 02 | pr-08 | pass |

## §2 数据流

```text
open zip → SwGroup(root)
  enter 3211593 → new KmoShowShape(parent,1)+addChild
  endElement → dgStore.update + updateProtocol
ShowTypoShape(app::KmoShowShape)
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_sw_group_on_mod | libs/ppt-core | `src/cn/wps/show/pptxr/handler/shape/SwGroupShapeHandler.rs + mod.rs` | `01_function_sw_group_on_mod` | `tests/cn/wps/show/pptxr/handler/shape/01_function_sw_group_on_mod.rs` |
| 02_function_open_tree_is_app_kmo | libs/ppt-core | `src/cn/wps/show/pptxr/pptx_mvp.rs + show/app/shape/KmoShapeTree.rs` | `02_function_open_tree_is_app_kmo` | `tests/cn/wps/show/pptxr/02_function_open_tree_is_app_kmo.rs` |
| 03_function_open_ln_survives | libs/ppt-core | `src/cn/wps/show/pptxr/pptx_mvp.rs + dmlr/.../ShapePropertiesHandler.rs` | `03_function_open_ln_survives` | `tests/cn/wps/show/pptxr/03_function_open_ln_survives.rs` |
| 04_function_typo_host_app_kmo | libs/ppt-core | `src/cn/wps/show/render/ShowTypoShape.rs` | `04_function_typo_host_app_kmo` | `tests/cn/wps/show/render/04_function_typo_host_app_kmo.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03 → 04
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| SwGroupShapeHandler | pptxr/handler/shape/SwGroupShapeHandler.rs | 01 |
| KmoShapeTree | show/app/shape/KmoShapeTree.rs | 02 |
| ShapePropertiesHandler | dmlr/.../ShapePropertiesHandler.rs | 03 |
| ShowTypoShape | show/render/ShowTypoShape.rs | 04 |
