# pl-12-textbox-live-paths — 解开 S7 活路径 panic

> 层：layout ｜ 状态：**spec** ｜ depends_on: pl-04-text-box
> Java 管线：[`01-java-pipeline.md`](../../../rewrite-review/01-java-pipeline.md) **S7 DataTransform**
> 跟踪：[06-next-features.md](../../../rewrite-review/06-next-features.md) · [PIXEL_PIPELINE.md](../../../PIXEL_PIPELINE.md)

## 0. 目前管线阶段的情况

主链 pl-04/05 在；`getParaDirection`/`getTextSpacing`/`convertBullet` 为 DEFERRED_NON_FUNCTION。真实 PPT 带项目符号或段距会 panic。

## 1. 缺乏的功能与逻辑

三方法按 Java 解开（function-java2rust）。不做整棵 wpslayout。

## 2. 完成验收的功能子集列表

| order | function_id | 验收口径 |
|---|---|---|
| 1 | `01_function_para_direction` | 有/无 rtl 属性（含 parent 回退） |
| 2 | `02_function_text_spacing` | spcPts/spcPct → 行距数值 |
| 3 | `03_function_convert_bullet` | buNone 跳过；buChar/buAutoNum 不 panic |

清单与 [functions.csv](./functions.csv) / [FUNCTIONS.md](./FUNCTIONS.md) 同步。`status=spec` 直到 loop 非 ignore 全绿。**禁止**把 `#[ignore]` 标成 pass。

### 跟踪

| date | status | note |
|---|---|---|
| 2026-09-15 | spec | 计划落盘 + function 拆分；未 loop |

## 3. I/O

| **I** | paraProp + TextData |
| **O** | rtl / spacing / BulletsInfo；exToInternal 不 panic |

crate：`libs/ppt-core`

## 4. Java 管线对照

| Java | Rust 归属 |
|---|---|
| getParaDirection:200 | 01 |
| getTextSpacing:1200 | 02 |
| convertBullet:893 | 03 |

## 5. 必须 / 壳 / 延后

- **must：** 三测绿；含 bullet fixture 不 panic；layout PASS。
- **壳：** buBlip 位图。
- **defer：** 整页 WpsLayout 断行。

## 6. Feature 验收

三 function 绿。

## 7. 依赖

- **upstream：** pl-04-text-box
- **loop：** 有 `// java` 的叶走 **function-java2rust**；禁止 `--force` `PptxrSlide.rs` / 健康模板。
- **不依赖：** DrawText 多 run（pr-10）。
