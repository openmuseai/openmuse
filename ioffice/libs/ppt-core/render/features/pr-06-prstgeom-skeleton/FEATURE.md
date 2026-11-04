# pr-06-prstgeom-skeleton — 预设几何骨架（基础形状）

> 层：parse + render ｜ 状态：**pass** ｜ depends_on: pr-02; pl-08  
> 产品 I/O：`prstGeom@prst` **持久化到 `ShapePr.geom_obj.prst`**，再按绝对框发 `DrawPath`；纯文本无 fill **不发射空 AABB**。

## 目标

第 7 页五边形/椭圆/线不再是套叠 `DrawRect`。切在 I/O（落盘 prst → path 指令 → 跳过文本线框），**不按形状类再拆 crate**（line/ellipse/pentagon 同属「prst+框→Path」）。

## I/O

| 输入 | 输出 |
|---|---|
| `a:prstGeom prst="rect"\|"ellipse"\|"line"\|"pentagon"` | `geom_obj.prst` = Java `DGMap.presetShapeTypeMap`（line=20, ellipse=3, pentagon=56, rect=1） |
| prst + 绝对框 | `DrawPath`（线 2 点；椭圆闭合点列；五边形 ≥5 点）；rect 保持 `DrawRect`+fill |
| `is_text_box && !hasFill && media 空` | **0 条**几何指令 |

未知 prst：仍 AABB；无 fill 时不写 stroke（stroke 政策归 pr-07）。`custGeom` / SVG 本波不做。
