# pl-10-g6-semantic-parity — 钉死 G6 可测四条

> 层：layout ｜ 状态：**spec** ｜ depends_on: pl-02-xfrm-geometry; pl-03-ph-merge; pl-05-text-typo; pl-01-units-view
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S6/S7 语义注记（G6）**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

G6 已钉在源码：anchor 1/2 对调；canRotate 非递归；ph 缺省 0 vs 8；负 EMU `f32::round` vs `Math.round`。getShapeRect 冻结 vs 重算只写 FUNCTION 口径，不改 Java。

## 1. 缺乏的功能与逻辑

与 Java 逐条同公式。G6-④ 旋转矩阵本波不做。

**getShapeRect 冻结 vs Java 重算（G6，只写口径）：** 本 feature **不改 Java**。Rust `ShowTypoShape.getShapeRect` 若继续冻结快照，FUNCTION 断言以冻结值为准；若改重算，须与 Java 同公式并单开测。禁止 silently 混用。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_anchor_1_2` | anchor=1 居中；=2 底 |
| 2 | `02_function_ph_type_default` | 无 type 属性 → 缺省 8 |
| 3 | `03_function_neg_emu_round` | 负 xfrm 舍入与 Java UnitsConverter 一致 |
| 4 | `04_function_canrotate_group` | 组 canRotate 递归子级 |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | 单测输入（anchor/ph/emu/组树） |
| **O** | 与 Java 字面量一致的数值/bool |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| RichTextLayout.modifyTextBoxAlign:1050 | 01 |
| ShowShape.Placeholder.type:865 | 02 |
| Math.round / UnitsConverter | 03 |
| KmoShowShape.canRotate:2261 | 04 |

## 5. 必须 / 壳 / 延后

- **must：** 四测绿；改过的 GreedyWrap/Placeholder/UnitsConverter/KmoShowShape 布局或产品 I/O 口径满足。
- **壳：** 象限 AABB（G6-④）。
- **defer：** 完整 calcShapeMatrix；getShapeRect 政策仅文档。

## 6. Feature 验收

四 function 非 ignore 全绿。

## 7. 依赖

- **upstream：** pl-02-xfrm-geometry; pl-03-ph-merge; pl-05-text-typo; pl-01-units-view
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** follow（pl-11）；wpslayout 断行。
