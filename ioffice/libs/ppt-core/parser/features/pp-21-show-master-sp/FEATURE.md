# pp-21-show-master-sp — 母版形状入胶片

> 层：parser+render ｜ 状态：**pass** ｜ depends_on: pp-14-layout-master-bind; pr-01-slide-layers
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S1 属性 + S9 drawShapesOnLayout**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

胶片分支已在；产品 `show_master_shape` 由 `p:sld` / `p:sldLayout` `showMasterSp` 注入，缺省 **true**（与 Java `Slide.isShowMasterSp` parent==null 一致）。

## 1. 缺乏的功能与逻辑

解析并注入 showMasterSp；true 时 master→layout→slide。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_parse_show_master_sp` | 属性 true/false/缺省（Java 缺省 true） |
| 2 | `02_function_film_master_then_layout` | true 时层序 master 再 layout 再 slide |
| 3 | `03_function_false_skips_master` | false 不 film master 树 |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=pass`：三 function 非 ignore 全绿，§3.8/§3.9 overall PASS。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |
| 2026-09-15 | pass | 01 parse 缺省 true；02 master→layout→slide；03 false 跳过 master/layout |

## 3. I/O

| **I** | sld/sldLayout 属性 + 三树 |
| **O** | 胶片层序 |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| Slide.isShowMasterSp:345 缺省 true | 01 |
| SlideRender.drawShapesOnLayout:104 | 02/03 |

## 5. 必须 / 壳 / 延后

- **must：** 三测；Sample 有母版装饰的页允许 1px 几何差。
- **壳：** showMasterPhAnim。
- **defer：** notes master。

## 6. Feature 验收

三 function 绿。

## 7. 依赖

- **upstream：** pp-14-layout-master-bind; pr-01-slide-layers
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** PptxReader 全编排。
