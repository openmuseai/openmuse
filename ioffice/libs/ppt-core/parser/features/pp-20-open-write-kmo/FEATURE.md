# pp-20-open-write-kmo — 打开期产物 = Java Kmo 树

> 层：parser ｜ 状态：**pass** ｜ depends_on: pp-19-kmo-tree-write; pp-02-shape-tree-parse; pp-13-document-assembly
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S4 产线写入**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

Java S4：`SwGroupShapeHandler.enterSubElement` 按 int-id 建子 `KmoShowShape`，闭标签 `DgStore.update` + `updateProtocol`。

Rust 现状（2026-09-15）：
- pp-19 **函数面 pass**（spid / addChild / DgStore / SwShape endElement / data ShapePr / TextBody / convert host）。
- **产品打开** `open_pptx_document_bytes` 经 `CommonSlideDataHandler` / `SwGroupShapeHandler` 写入 `PptxDocument.films`（`KmoFilm.spTree()` = app `KmoShapeTree`）。`parse_one_sp` 仍为 layout_shape 喂 render。
- `handler/shape/mod.rs`：`pub mod SwGroupShapeHandler` **已解开**（pp-20:01 pass）。`CommonSlideDataHandler` 已 use 它。
- `ShowTypoShape` 宿主为 `app::KmoShowShape`（pp-20:04 pass）。`parse_one_sp` 仍为 layout_shape 喂 render，经 `ConvertKmo` 进入快照。

## 1. 缺乏的功能与逻辑

1. 产线走 Sw*：`addChild` + 闭标签双 bin，而不是字符串抽标签填 fixture。
2. `open_pptx_document_bytes` 返回 `app` 侧 slide/tree（grep：open 返回路径无 `layout_shape::KmoSlide`）。
3. `a:ln` 经 **open** 后仍可查询（pp-19:05 只保证 handler 测试面）。
4. 快照宿主类型切到 `app::KmoShowShape`。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_sw_group_on_mod` | SwGroup 进默认编译面；3211593/1594/1469 分发 |
| 2 | `02_function_open_tree_is_app_kmo` | 最小 zip 一 sp → shapeCount==1 且 type()==queryType(id) |
| 3 | `03_function_open_ln_survives` | open 后 ln 宽度仍在 data ShapePr |
| 4 | `04_function_typo_host_app_kmo` | ShowTypoShape 不再 import layout_shape::KmoShowShape |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。四 function 均为 `status=pass`。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |
| 2026-09-15 | 01 pass | `01_function_sw_group_on_mod`：mod 接线 + enterSubElement 分发六测全绿；布局/对齐 PASS。 |
| 2026-09-15 | 02 pass | `02_function_open_tree_is_app_kmo`：最小 zip → `film.spTree().shapeCount()`；布局/对齐 PASS。 |
| 2026-09-15 | 03 pass | `03_function_open_ln_survives`：open 后 `dgShape.spPr().ln()` 宽度/noFill/缺 ln；布局/对齐 PASS。 |
| 2026-09-15 | 04 pass / feature pass | `04_function_typo_host_app_kmo`：`ShowTypoShape.mShapeBase` = `app::KmoShowShape`；TH-IMP/GET/REG 绿；布局/对齐 overall PASS。 |

## 3. I/O

| **I** | pptx bytes（pp-13 已能 open） |
| **O** | `app::KmoShapeTree` + DgStore 行 + data ShapePr |
| **错误** | 未知子元素 skip（Java enterSubElement 返回 null） |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| SwGroupShapeHandler.java:43-143 | 01/02 |
| SwShapeHandler.endElement:79-86 | 02/03 |
| KmoShowShape.addChild:451 | 02 |
| ShowTypoShape.java mShapeBase | 04 |

## 5. 必须 / 壳 / 延后

- **must：** 01 mod 接线；02 open 树；03 ln；04 快照宿主。
- **壳：** graphicFrame 空内容；PptxrSlide 全编排。
- **defer：** 加密；ShapePr.follow（pl-11）；effect。

## 6. Feature 验收

四 function 非 ignore 全绿；open 返回路径无 layout_shape::KmoSlide；改过的 Sw*/Kmo*/ShowTypoShape 布局 PASS。

## 7. 依赖

- **upstream：** pp-19-kmo-tree-write; pp-02-shape-tree-parse; pp-13-document-assembly
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** 整棵 wpslayout、Chart/Table、dmlw 写回。
