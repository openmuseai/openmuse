# pl-09-public-shape-convert — Kmo → 公开 Shape（S5 convert）

> 层：layout（渲染期惰性挂视图）｜ 状态：**pass** ｜ depends_on: pp-10; pp-11; pp-16; pl-01; pl-02; pl-03; pl-06  
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S5**  
> 产品 I/O：把 **Kmo 树（文档事实源）** 惰性转换成只读 **公开 `cn.wps.moffice.drawing.Shape`**，供绘制、命中、排版框、位图缓存共用。

## 为什么必须有这一层（不是可选指令捷径）

Rust 现状把几何/填充直接摊进 `RenderCommand`，公开 Shape 被当成「设计内不做」。Java 不是这样切的。本 feature 把中间体接回来，原因是：

1. **解耦（核心）**：Kmo 树服务解析、编辑、协议同步、内存优化——重、内聚、可变。渲染器若直接啃它，就要懂 spid 编码、双 bin 提交、`ShapePr.follow` 惰性链、`PlaceHolderMerger` 继承。公开 Shape 把 **「文档怎么存」和「图形怎么画」切开**。
2. **惰性 + 一次转换多次绘制**：`ShowTypoShape.getShape`（`ShowTypoShape.java:180`）先 `SlideCache.getShape`（`:183` 命中即返）；未命中才 `ConvertTool.convertShapeToPublicShape`（`:185`）并 `addShape`（`:188`）。同一页会被反复画（每帧、缩放、缩略图、打印、编辑拖拽预览）。转换一次、缓存复用。
3. **归一化摊平**：转换时刻算定 EMU→px、`rotation/60000→degree`、相对坐标→绝对框、占位继承冻结、`prstGeom int→path`。渲染器拿到的是最终答案，画时零推导。
4. **多消费者共享**：几何命中、文本排版框、位图缓存、软件画布预览都要同一份几何/填充/线条。Rust 指令化后这类非绘制查询没有中间体可挂——本 feature 补上。

## 目标

实现 Java S5 的 **cache → type 分派 → 属性装配** 热路径，产出 `drawing::Shape`（ShapePos / GRF / FillBase / LineProperty / Geometry(GraphicsPath) / ShapeText）。**不下 Canvas、不发 RenderCommand**。pr-02 在本 feature pass 后改为消费公开 Shape，而不是再从 `layout_shape` 发明第二套几何。

## I/O

| | 契约 |
|--|------|
| **I** | `KmoShowShape`（`type` 0–4 + `shapePr` xfrm/geom/fill/ln + 可选 txBody）+ `InterpretCtx`（filter / color / theme）+ `SlideCache` |
| **O** | 只读公开 `Shape`：`setShapeID`；`ShapePos.rotation`（degree）+ flip；`property(733)` 绝对 px 框；`Geometry`/`GraphicsPath[]`；`FillBase`；`LineProperty`；`ShapeText` |
| **惰性** | `ShowTypoShape.getShape` 首次访问才 convert；`noFilter` 才回填 `SlideCache` |
| **约束** | 不改 `.java`；不写工程 stub 冒充 Shape；`ConvertTool.rs` 现状是脱离交错模板的空 stub，loop 必须走 function-java2rust 重生，禁止手写 `let AUTO=()` |

装配顺序（`ConvertTool.java:140-311` 实测）：xfrm → rotation/flip/`property(733)`/`setShapeRect` → 几何+填充 → useBgFill → 线条 → effectLst → 3D。本 MVP **must** 做到线条为止；effect/3D/ink/graphicFrame 内容 **park**。

## Java 管线对照

| Java | 行号 | Rust 归属 |
|---|---|---|
| `ShowTypoShape.getShape` | `:180-194` | `01_function_getshape_cache` |
| `SlideCache.getShape` / `addShape` | `:78-86` | 01（MRU by `showShape.id()`） |
| `ConvertTool.convertShapeToPublicShape` | `:100-125` | `02_function_type_dispatch` |
| `shapePropertiesToPublicShape` xfrm 段 | `:151-177` | `03_function_xfrm_pos` |
| `GeomtryConvert` + `PresetShapeTypeMap` | geom 文件 | `04_function_geom_path` |
| `FillConvert.convertToFillBase` | `:47-80` | `05_function_fill_base` |
| `LineStyleConvert.convertToLineProperty` | `:17-144` | `06_function_line_property` |
| `shapeToPublicShape` ShapeText | `:1247-1258` | `07_function_shape_text_bind` |

## 能力解锁

| function | 实现后管线新增 |
|---|---|
| 01 | 同页多次 `getShape` 不重算；filter 路径不污染缓存 |
| 02 | type 0–4 有公开 Shape 壳 + 稳定 `shapeID` |
| 03 | 渲染单位绝对框 + 旋转/翻转冻结在 Shape 上（含组内 `calcRelativeRect`） |
| 04 | 预置/自定义/占位默认矩形 → path；渲染器不再自己查 prst |
| 05 | 六类 fill 中 MVP 实色/图片 + useBgFill 冻结 |
| 06 | 线宽下限 12700 EMU；`fill.type==5` → `lineProperty=null` |
| 07 | 公开 Shape 挂 `ShapeText`，pr-03 / 排版测量有中间体 |

缺失断点：没有 pl-09 → 渲染仍直接啃 Kmo；pr-02 无法从 Shape 读 fill/line/path；命中测试/测量无挂点。

## 必须 / 壳 / 延后

- **must：** 01 缓存命中/未命中/`noFilter`；02 type 1/2/3 分派 + `setShapeID`（type 0 组壳、type 4 空壳）；03 `toDegree` + flip + `property(733)` EMU→px + `ShapeUtil.calcRelativeRect` **只调用**；04 prst 查表 + Vertices/默认 rect 回退 + placeholder 无几何→rect；05 solid + blip + useBgFill；06 width min / noFill 线；07 `new ShapeText(publicShape, textBody)`。
- **壳：** type 4 graphicFrame（Chart/Table/OLE/Model3D 内容）；type 0 组的 `setGroup`/`childOff` 框（不递归 convert 子级——子级各自 `getShape`）。
- **defer：** effectLst（outer/inner/preset shadow、reflection、glow、softEdge）；`convertSp3d`；ink；grad/pattern fill 完整停靠；`shapeStyleToPublicShape` 主题 lnRef/fillRef（pp-16 已有 theme 时 loop 可接，非本波门闩）；`PptxrSlide.rs` 禁止 `--force`。

## 范围

- **in：** S5 热路径到线条 + ShapeText 绑定；输出类型 = `core_libs_drawing` 的 `moffice.drawing.Shape`（不是 `drawing/data/shape/Shape.rs` 解析模型）。
- **out：** 效果/3D/ink；把 `RenderCommand` 当公开 Shape 替代物；在 pl-06 快照 function 里做 convert 冒充绿。

## 验收

1. 同一 `KmoShowShape.id` 两次 `getShape`（无 filter）返回同一缓存实例。
2. type 1 + xfrm + prst=rect + solid fill + ln → 公开 Shape 上 rotation/733/path/fill/line 均为冻结值（单位与 Java 同公式）。
3. `ln.fill.type==5` → `getLineProperty()==null`。
4. 缺几何 + placeholder → `GeomtryConvert.convertPlaceHolderToPublicShape` 默认 rect。
5. pr-02 **不**在本 feature 改指令发射；本波只保证中间体 I/O。联调在后续 loop / pr-02 消费方改造。

## 依赖

- **upstream：** pp-10（`layout_shape` 上的 xfrm/geom/fill/ln）；pp-11（txBody）；pp-16（useBgFill 的 `fillAt`）；pl-01（`UnitsConverter.render().fromEmuX/Y`）；pl-02（`calcRelativeRect` / `toDegree`，**调用不重写**）；pl-03/pl-07（follow 链已折入的有效 ShapePr）；pl-06（`ShowTypoShape` 快照是 `getShape` 宿主）。
- **downstream：** pr-02（几何/fill/line 应从 `typo.getShape()` 读，而不是 `layout_shape.graphics_path`）；pr-03（`ShapeText`）；未来 hit-test / measure。
- **不依赖：** Canvas、Flutter painter、wpslayout 断行。

## 不过扩展

禁止用 `rendercommand.rs` 的 path/fill 冒充公开 Shape。禁止把 `drawing/data/shape/Shape.rs`（KoBin 解析模型）当作 convert 产物。禁止为编译通过把 `ConvertTool` 空 stub 挂进默认 `mod.rs`。禁止在本层重写 `ShapeUtil` 算法。
