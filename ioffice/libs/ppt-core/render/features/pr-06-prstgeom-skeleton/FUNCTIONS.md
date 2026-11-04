# pr-06-prstgeom-skeleton — function 清单

> 状态：**pass** ｜ 管线：[SKELETON_PIPELINE.md](../../../SKELETON_PIPELINE.md)

## §0 管线位置

```text
spPr / a:prstGeom
  → [01 prst_bind] geom_obj.prst 落盘（含 DGMap line/pentagon）
  → [02 prst_path] 绝对框 + prst → DrawPath / DrawRect
  → [03 skip_text_wireframe] 纯文本无 fill → 0 条几何
```

## §1 清单

| order | function_id | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|
| 1 | [01_function_prst_bind](./functions/01_function_prst_bind/FUNCTION.md) | prstGeom XML | `ShapePr.geom_obj.prst: Some(i32)` | pp-10 | 02 | pass |
| 2 | [02_function_prst_path](./functions/02_function_prst_path/FUNCTION.md) | prst + abs rect | DrawPath 或 DrawRect | 01; pl-08; pr-02 | 03; pr-07 | pass |
| 3 | [03_function_skip_text_wireframe](./functions/03_function_skip_text_wireframe/FUNCTION.md) | 无 fill 文本框 | 0 条 DrawRect | pr-03; 02 | pr-07 | pass |

line / ellipse / pentagon **同属 02**（同一 I/O：prst+框→path）。矩阵用 T01/T02/T03 覆盖三种，不按类拆 function。

## §2 数据流

```text
ShapePropertiesHandler enter 1114429:
  现：PresetGeometryHandler(geom_obj.clone())；commit 不写回 geom_obj
  目标：handler 写同一 Rc，commit 拷 geom_obj；DGMap 含 pentagon=56, line=20

geometry_commands(Shape):
  skip 文本线框（03）
  match prst → path in abs rect（02）
  else DrawRect
```

## §3 测试面

| function_id | tests | cargo |
|---|---|---|
| 01 | `tests/cn/wps/show/pptxr/01_function_prst_bind.rs` | `--test 01_function_prst_bind` |
| 02 | `tests/cn/wps/show/render/02_function_prst_path.rs` | `--test 02_function_prst_path` |
| 03 | `tests/cn/wps/show/render/03_function_skip_text_wireframe.rs` | `--test 03_function_skip_text_wireframe` |

## §4 验收

Sample 第 7 页 JSON：`DrawPath` 条数 > 0；「这里输入文字」不以空 fill 黑框 Rect 为主。

## §5 loop

`01 → 02 → 03`

## §6 类级

| Java | function |
|---|---|
| `PresetGeometryHandler` + `DGMap.presetShapeTypeMap` | 01 |
| `GemoRender.drawShape` / `Shape.getGraphicsPaths` | 02 产品桥 `geometry_commands`；禁止第二套几何类型名 |
| 文本层 | 03 只跳过几何，不断行 |

## 文档

- [functions.csv](./functions.csv)
