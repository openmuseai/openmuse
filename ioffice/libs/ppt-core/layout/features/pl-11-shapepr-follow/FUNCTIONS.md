# pl-11-shapepr-follow — function 拆分

> Feature：[FEATURE.md](./FEATURE.md) · 清单：[functions.csv](./functions.csv)
> 切在 **I/O**。Java **S6 ShapePr.follow**。状态：**spec**（跟踪用，未 loop）。

## §0 管线位置

```text
S4 写入 data ShapePr
  → follow(parent)（本 feature）
  → S5 convert 读到继承槽
```

## §1 function 清单

| order | function_id | 一句话 | I | O | upstream | downstream | status |
|---|---|---|---|---|---|---|---|
| 1 | [01_function_follow_fill](./functions/01_function_follow_fill/FUNCTION.md) | follow fill 惰性 | slide ShapePr 无 fill + parent 有 solid | fill() 色 = parent；改 parent 后未覆盖槽可见 | pp-20;pl-03 | 03 | spec |
| 2 | [02_function_follow_xfrm](./functions/02_function_follow_xfrm/FUNCTION.md) | follow xfrm | slide 无 xfrm + parent 有框 | xfrm() 用 parent 框；hosts Transform2D.follow 真实现 | 01 | 03 | spec |
| 3 | [03_function_local_overrides](./functions/03_function_local_overrides/FUNCTION.md) | 本地写了不 follow | slide 已 writeFill / writeXfrm | 读本地；改 parent 不影响覆盖槽 | 01;02 | — | spec |

## §2 数据流

```text
child.follow(parent)
  fill() → parent.fill 若本地无
  xfrm() → parent.xfrm
  本地 write 后隔离
```

## §3 编译面

| function_id | CRATE_DIR | src | `cargo test --test` | tests/ |
|---|---|---|---|---|
| 01_function_follow_fill | libs/ppt-core | `src/cn/wps/drawing/data/shape/ShapePr.rs + shapepr_hosts.rs` | `01_function_follow_fill` | `tests/cn/wps/drawing/data/shape/01_function_follow_fill.rs` |
| 02_function_follow_xfrm | libs/ppt-core | `src/cn/wps/drawing/data/shape/ShapePr.rs + shapepr_hosts.rs` | `02_function_follow_xfrm` | `tests/cn/wps/drawing/data/shape/02_function_follow_xfrm.rs` |
| 03_function_local_overrides | libs/ppt-core | `src/cn/wps/drawing/data/shape/ShapePr.rs` | `03_function_local_overrides` | `tests/cn/wps/drawing/data/shape/03_function_local_overrides.rs` |

## §4 串联

按 csv `order`。不要跳过上游 I/O 假装绿。测 N 只用声明的上游输出或最小 fixture。

## §5 验收顺序

```text
01 → 02 → 03
```

## §6 类级对照

| Java 类 | Rust 目标 | 本 feature |
|---|---|---|
| ShapePr | drawing/data/shape/ShapePr.rs | 全部 |
