# pp-21-show-master-sp — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S1 属性 + S9 drawShapesOnLayout**。状态：**pass**。

## §0 管线位置

```text
S1 属性 → S4 树 → S9 film（本 feature 接通）
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_parse_show_master_sp](./functions/01_function_parse_show_master_sp/FUNCTION.md) | 解析 showMasterSp | p:sld / sldLayout 属性 true/false/缺省 | isShowMasterSp：缺省 **true**（Slide.java:345-347 parent==null ? true） | pp-14 | 02 | pass |
| 2 | [02_function_film_master_then_layout](./functions/02_function_film_master_then_layout/FUNCTION.md) | true 时 master→layout→slide | slide+layout+master 皆 showMasterSp true | 层序含 master 形状再 layout 再 slide（SlideRender.java:104-110） | 01;pr-01 | 03 | pass |
| 3 | [03_function_false_skips_master](./functions/03_function_false_skips_master/FUNCTION.md) | false 不 film master | slide showMasterSp=false | 无 master 树形状 | 02 | — | pass |

## §2 数据流

```text
parse showMasterSp (default true)
  film if layout∧slide → master
  film if slide → layout
  always slide
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_parse_show_master_sp | libs/ppt-core | `src/cn/wps/show/pptxr/pptx_mvp.rs` + SlideHandler / SlideLayoutHandler 模板 | `01_function_parse_show_master_sp` | `tests/cn/wps/show/pptxr/01_function_parse_show_master_sp.rs` |
| 02_function_film_master_then_layout | libs/ppt-core | `src/cn/wps/show/render/SlideRender.rs` | `02_function_film_master_then_layout` | `tests/cn/wps/show/render/02_function_film_master_then_layout.rs` |
| 03_function_false_skips_master | libs/ppt-core | `src/cn/wps/show/render/SlideRender.rs` | `03_function_false_skips_master` | `tests/cn/wps/show/render/03_function_false_skips_master.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| SlideHandler / SlideLayoutHandler | handler 交错模板 + pptx_mvp 产品 I/O | 01 |
| Slide.isShowMasterSp / SlideLayout.isShowMasterSp | 缺省 true → layout_shape::KmoSlide | 01 |
| SlideRender | SlideRender.rs | 02/03 |
