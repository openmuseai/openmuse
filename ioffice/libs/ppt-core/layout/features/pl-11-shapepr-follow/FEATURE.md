# pl-11-shapepr-follow — ShapePr.follow 惰性链

> 层：layout ｜ 状态：**spec** ｜ depends_on: pp-20-open-write-kmo; pl-03-ph-merge
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S6 ShapePr.follow**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

Java `follow:360` 惰性：未写槽读 parent。Rust 指针已挂，子对象 follow 空，xfrm() 内 todo。layout_shape::ShapePr 仅 fill/ln 递归回退。

## 1. 缺乏的功能与逻辑

data ShapePr 上 getXfrm/getGeom/getFill/getLn 未设置则 parent；禁止 merge 深拷贝冒充。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_follow_fill` | slide 无 fill、layout 有 solid → 色=layout；改 parent 可见 |
| 2 | `02_function_follow_xfrm` | slide 无 xfrm → 用 parent 框 |
| 3 | `03_function_local_overrides` | slide 写了 fill 则不 follow |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | child ShapePr + parent ShapePr |
| **O** | 未覆盖槽读 parent，覆盖槽隔离 |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| ShapePr.follow:360 | 全部 |
| xfrm/fill/ln getters | 01-03 |

## 5. 必须 / 壳 / 延后

- **must：** 三测含 live parent 断言。
- **壳：** geom/effect/3D follow 可后置但 02 至少 xfrm。
- **defer：** scene3D/sp3d。

## 6. Feature 验收

三 function 绿。

## 7. 依赖

- **upstream：** pp-20-open-write-kmo; pl-03-ph-merge
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** PlaceHolderMerger 查找（已 pass）。
