# 01_function_grp_choff_chext — 解析组 chOff/chExt

> feature: pl-08-group-abs-geom ｜ 状态：**pass** ｜ 管线：[SKELETON_PIPELINE.md](../../../../../SKELETON_PIPELINE.md)

## 0. 管线能力解锁

没有 chOff/chExt，`calcChildRelativeRect` 整段不进（Java `hasChildPoint && hasChildExtents`）。组内色块的 `off` 会被当成幻灯片绝对坐标。

## 1. 功能说明

Java `GroupTransform2DHandler.enterSubElement`：`1114204` → `group.groupXfrm().childPoint()`，`1114205` → `childExtents()`；`1114200`/`1114197` 仍是组自身 off/ext。产品 `walk_xml` 的 `element_id` 必须把 `chOff`/`chExt` 映到这两个 id；`parse_one_grp` 把结果写到 `KmoShowShape.child_off`/`child_ext`（以及自身 `transform2D*`）。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `p:grpSp` 片段（含 `p:grpSpPr/a:xfrm`：`a:off`、`a:ext`、`a:chOff`、`a:chExt`）。单位 EMU。 |
| **输出** | 组 `transform2DX/Y/W/H` = off/ext；`child_off = Some((chOff.x, chOff.y))`；`child_ext = Some((chExt.cx, chExt.cy))`。 |
| **状态** | 只写该组字段；不改子 xfrm、不算绝对框。 |
| **错误** | 无 chOff 或无 chExt → 对应 `Option` 为 `None`（与 Java `hasChildPoint()==false` 一致），不 panic。缺 off/ext → 0。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `element_id` / `walk_xml` | `pptx_mvp.rs` | `chOff→1114204`、`chExt→1114205`；attr `x/y/cx/cy` 已有 | ✅ chOff/chExt 已映射 | — | 不改 zip/OPC |
| `GroupTransform2DHandler` | `Transform2DHandler.rs` | enter 1114204/1114205 写到 **同一** group 的 child 槽，禁止 `xfrm.clone()` 后丢掉 | ✅ 共享 Rc + flush | 1114198/1114199 fall-through（Java 同样 default break） | 不改 Point2DHandler 的 off 语义 |
| `KmoShowShape` | `layout_shape.rs` | `child_off`/`child_ext`/`hasChildPoint` | ✅ `parse_one_grp` 从 `GroupShape` 拷贝 | — | 不重构 KmoAtom |
| `ShapePropertiesHandler` | 同 crate | 组走 `GroupShapePropertiesHandler`（3211589 → GroupTransform2DHandler） | ✅ `parse_one_grp` 二次 walk `grpSpPr` | — | 不 `--force` PptxrSlide |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/GroupTransform2DHandler.java`；`drawing/data/basic/GroupTransform2D.java` |
| Rust | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs`；`src/cn/wps/show/pptxr/pptx_mvp.rs`（`element_id`、`parse_one_grp`） |

### 2.2 函数改写列表

- **本 function：** `element_id("chOff"|"chExt")`；`GroupTransform2DHandler.enterSubElement` 1114204/1114205 写回 group（禁止 detached `Rc` clone）；`parse_one_grp` 把 child 槽拷到 `KmoShowShape`。
- **停车：** 旋转矩阵、flip 对 child 的完整 Java 语义（可读 rot 不换挡）。
- **禁改：** `ShapeUtil.rs` 算法；`PptxrSlide.rs`。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`GroupTransform2DHandler.java` 35-80；Rust handler 164-198（clone xfrm、无 1114204）；`pptx_mvp.rs` `element_id` 27-51、`parse_one_grp` 669-676、`parse_one_sp` 708-713；`layout_shape.rs` `hasChildPoint` 107-118。

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| `element_id("chOff")` | pptx_mvp.rs | 标签名 | 1114204 | 现为 -1 → walker 不进 handler |
| `enterSubElement(1114204)` | GroupTransform2DHandler.java 52-53 | chOff x,y | childPoint | clone 丢失 |
| `parse_one_grp` | pptx_mvp.rs | 最小 grpSp XML | `child_off`/`child_ext` | 只拷 off/ext |
| 缺 chExt | Java hasChildExtents | 仅 chOff | `child_ext=None`，02 不映射 | 不 panic |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 完整 off/ext/chOff/chExt | 公开入口 | 是 | T01 |
| C02 | 无 chOff（仅 off/ext） | 失败/缺失 | 是 | T02 |
| C03 | chExt cx=0 或 cy=0 仍写入 Some | 边界 | 是 | T03 |
| C04 | `element_id` 常量 | 输入后端 | 是 | T04 |

### 3.3 可测试性

`walk_xml` + `GroupTransform2DHandler` 或 `open_pptx_bytes` 最小 zip（1 个 grpSp）。不跑 `render_slide`。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | — | off=(100,200) ext=(300,400) chOff=(10,20) chExt=(50,60) | child_off=(10,20) child_ext=(50,60)；自身框 (100,200,300,400) | chOff 被当成 off |
| T02 | C02 | — | 无 `<a:chOff>` | `child_off.is_none()` | 伪造 (0,0) 当已解析 |
| T03 | C03 | — | chExt cx=0 cy=100 | `child_ext=Some((0,100))` | 当 None 丢掉，02 无法走 cx==0→scale 1 |
| T04 | C04 | — | `element_id("chOff")` | `1114204`；`chExt`→`1114205` | walker 永不进入 01 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/pptx_mvp.rs
       src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs
test:  tests/cn/wps/show/pptxr/01_function_grp_choff_chext.rs
[[test]] name = "01_function_grp_choff_chext"
```

L2 落 `[[test]]` name=`01_function_grp_choff_chext`。Loop 后非 ignore。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_grp_choff_chext

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_grp_choff_chext \
  --src src/cn/wps/show/pptxr/pptx_mvp.rs \
  --src src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs \
  --report libs/ppt-core/layout/features/pl-08-group-abs-geom/functions/01_function_grp_choff_chext/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_grp_choff_chext --json`)
date: 2026-09-09 20:36

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs` | 174/214 | 81.3% | 191/252 | 75.8% |
| `src/cn/wps/show/pptxr/pptx_mvp.rs` | 534/812 | 65.8% | 883/1433 | 61.6% |
| **function total** | **708/1026** | **69.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/GroupTransform2DHandler.java
```

`pptx_mvp.rs` 为产品编排（与 pp-13:02 相同）：**禁止 `--force` 重生** `PptxrSlide.rs`。Loop 后 handler 布局：**PASS**。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/GroupTransform2DHandler.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs \
  --report libs/ppt-core/layout/features/pl-08-group-abs-geom/functions/01_function_grp_choff_chext/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-09 12:36

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `GroupTransform2DHandler` | `koffice/presentation/core/cn/wps/drawing/dmlr/handler/shape/sppr/transform/GroupTransform2DHandler.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/handler/shape/sppr/transform/Transform2DHandler.rs` | **PASS** | `GroupTransform2DHandler`, `startElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
