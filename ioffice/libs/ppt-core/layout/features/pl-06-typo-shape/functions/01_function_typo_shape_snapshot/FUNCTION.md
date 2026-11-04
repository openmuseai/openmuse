# 01_function_typo_shape_snapshot — 快照 flags + rect

> feature: pl-06-typo-shape ｜ 状态：**pass** ｜ Java 管线环节：S2 布局

## 0. 管线能力解锁

实现后：单个 `KmoShowShape` 变成一个可查询的快照 `ShowTypoShape`——`isHidden/isPlaceholder/placeholderType/isTextBox` 四枚 flags 冻结于构造，`getShapeRect()` 随时给出 **render 单位**的绝对矩形（EMU 框经 `UnitsConverter.render().fromEmuX/Y`，带缓存 + 置脏）。render 层 pr-01 的过滤（`ShapeRender._begin`：hidden / `drawFlags&1` 跳过 placeholder / `&32` 跳过 miner ph）与 pr-02/03 的 rect 输入全部消费它。

没有它管线断在哪：pr-01 无对象可过滤（**渲染管线在 S3 断**，`PIPELINE_LAYOUT_RENDER_PLAN.md` §3.2）；pr-02/03 拿不到 rect。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `(KmoShowShape, InterpretCtx\|None, geometry: RectF)`——`geometry` 为 pl-02 产出的**绝对 EMU 框**（`calcRelativeRect` 结果），本层只换算不重算 |
| **输出** | `ShowTypoShape { isHidden, isPlaceholder, placeholderType, isTextBox, getShapeRect() -> RectF(render 单位) }`；`getShowShape()` 回读原 shape |
| **状态与副作用** | 构造时快照 flags（Java ctor 语义）；`getShapeRect` 惰性换算并缓存于 `mGeometry`，`clearShapeRectCache()` 置脏后重算；只读、无外部写 |
| **错误语义** | `KmoShowShape` 缺 `ph`/缺 `transform2D` → Java 语义上 `ph()` 恒可用（空 placeholder），本层容忍返回空 flags；`geometry` 传 None → 空 RectF（与 Java 第二 ctor `new RectF()` 一致，`ShowTypoShape.java:57`）；不 panic |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `typo_shape_snapshot` | `KmoShowShape` + `InterpretCtx` + `geometry: RectF`（EMU） | `ShowTypoShape(KmoShowShape, InterpretCtx, RectF)`（`ShowTypoShape.java:43`） |

### 1.3 输出
| 输出 | 说明 | Java 对标 |
|------|------|-----------|
| `isHidden` | 快照于构造 | `mIsHidden = shapeBase.isHidden()`（`ShowTypoShape.java:50`；`isHidden()` 返回 `mShapeBase==null \|\| mIsHidden`，`:107`） |
| `isPlaceholder` | `shapeBase.isPlaceHolder()` | `ShowTypoShape.java:51/61` |
| `placeholderType` | `shapeBase.ph().type()` | `ShowTypoShape.java:52/65`（`ShowShape.Placeholder.type()`，default 无 type 时 0） |
| `isTextBox` | `shapeBase.isTextBox()` | `ShowTypoShape.java:53/69` |
| `getShapeRect()` | `render().fromEmuX/Y(emul, emut, emur, emub)`，缓存 + 置脏 | `ShowTypoShape.java:119-139`；`UnitsConverter.render()` |

### 1.4 上游
pl-02（geometry 绝对 EMU 框）；pl-03（ph 继承后的 shape 属性，可选）；pl-01（render 档单位语义）。

### 1.5 下游
02（子级遍历的基座）；pr-01（过滤输入）；pr-02/pr-03（rect 输入）。

### 1.6 不变量
1. flags 在构造时冻结，之后 `KmoShowShape` 变化不回流（Java 同名语义）。
2. `getShapeRect` 幂等：未置脏时重复调用返回同一缓存值。
3. 单位：入 EMU、出 render 单位（`fromEmuX/Y`），禁止中间档混用。
4. 本层不重算 geometry（不调 `calcRelativeRect`——那是 pl-02）；`geometry` 传入即冻结。
5. `getShape()`（ConvertTool → 公开 Shape）返回 None 占位，不参与本 function 的判定。

### 1.7 依赖与实现状态（必写，着重）
| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `KmoShowShape` flags | pp-02 / 本 loop | `isHidden/isPlaceHolder/ph().type()/isTextBox` | Rust `KmoShowShape.rs` 42 行 stub 仅 `isPlaceHolder/getPlaceHolderType/getPlaceHolderIdx` 类方法，`isHidden/isTextBox/ph` 缺失（待 pp-02 或 loop 补齐） | ph 的 idx/orient/size | 禁止在本包伪造 flags 数据 |
| `geometry`（绝对 EMU 框） | pl-02 | `RectF` | pl-02 spec | — | 禁止本层重算（calcRelativeRect） |
| `UnitsConverter.render()` | pl-01 / drawing/app/util `UnitsConverter.rs` | `fromEmuX/fromEmuY` | 已有（486 行，审计 keep） | — | 禁止改换算系数 |
| `getShape()`/`RenderModelOverride` | S5 convert（**pl-09**） | 公开 Shape | 否（`DEFERRED_NON_FUNCTION`） | pl-09 getshape_cache | 禁止在本 function 内做 ConvertTool |

## 2. 核心代码与函数改写列表

Java→Rust 表（方法名均已查证，行号 = Java 源）：

| Java | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `ShowTypoShape(KmoShowShape, InterpretCtx, RectF)` ctor（`ShowTypoShape.java:43`） | `libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs: new()` | 01 门面（flags 快照） |
| `isPlaceholder/placeholderType/isTextBox/getShowShape`（`ShowTypoShape.java:61/65/69/73`） | 同 | 01 getter 集 |
| `isHidden()`（`ShowTypoShape.java:107`） | 同 | 01（`mShapeBase==null \|\| mIsHidden` 语义） |
| `getShapeRect()/clearShapeRectCache()`（`ShowTypoShape.java:119/141`） | 同 | 01（`fromEmuX/Y` + `mGeometry` 缓存/置脏） |
| `KmoShowShape.isHidden/isPlaceHolder/ph().type()/isTextBox`（`KmoShowShape.java:2284/552/566/938`） | `KmoShowShape.rs`（扩） | 01 读取端最小集 |
| `ShapeUtil.calcRelativeRect`（`ShapeUtil.java:187`） | `show/util/ShapeUtil.rs`（待 loop） | **不实现**（pl-02）；仅注明消费方 |

不在改写列表（`todo!("DEFERRED_NON_FUNCTION")` 停车）：`getShape`（`ShowTypoShape.java:180`）、`setRenderOverride`/`getRenderModel`（pr-02）、`render(ShapeEnv)`、`isModularGroupDefaultPic`、ink 分支（`:128`）。

## 3. 测试方案

### 3.1 审计面
| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `typo_shape_snapshot` | 普通 shape | hidden=false, ph=无, isTextBox=true | flags 齐全 | — |
| `typo_shape_snapshot` | ph shape | ph= 有（type=1 title） | placeholder=true, type=1 | ph 分支 |
| `typo_shape_snapshot` | hidden | isHidden=true | isHidden=true | hidden 分支 |
| `typo_shape_snapshot` | null shape | shape=None（Java `mShapeBase==null` 语义） | isHidden=true、其余默认 | null 分支 |
| `get_shape_rect` | 换算 | EMU 框（已知 EMU 值） | render 单位 | `fromEmuX/Y` |
| `get_shape_rect` | 缓存 | 同框两次调用 | 同值 | 缓存 |
| `clear_shape_rect_cache` | 置脏 | 置脏后重取 | 重算 | 置脏 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| TS-FLAGS | typo_shape_snapshot | 四 flags 组合 | 是 | TS1 |
| TS-PH | typo_shape_snapshot | ph type 0..N 透传 | 是 | TS2 |
| TS-HIDDEN | typo_shape_snapshot | isHidden 组合 | 是 | TS3 |
| TS-NULL | typo_shape_snapshot | shape=None | 是 | TS4 |
| TS-RECT | get_shape_rect | EMU→render 换算 | 是 | TS5 |
| TS-CACHE | get_shape_rect | 重复调用幂等 | 是 | TS6 |
| TS-DIRTY | clear_shape_rect_cache | 置脏重算 | 是 | TS7 |
| TS-NOGEOM | typo_shape_snapshot | geometry 不重算（pl-02 边界） | 是 | TS8 |

### 3.3 可测试性
喂 fixture `KmoShowShape`（flags 手工构造）/ None + 显式 `geometry`，不改 pl-02/不跑 `calcRelativeRect`/不连 `SlideRender`。rect 断言用已知 EMU 值 + `UnitsConverter.render()` 同公式手算。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| TS1 | TS-FLAGS | 4 种组合 | new(shape, None, rect) | flags 逐项等于输入 | 快照错位 |
| TS2 | TS-PH | ph type=1/2/… | placeholderType | 同号透传 | 类型号篡改 |
| TS3 | TS-HIDDEN | hidden true/false | isHidden | 逐项 | 反转 |
| TS4 | TS-NULL | shape=None | isHidden | true；其余默认 | None panic |
| TS5 | TS-RECT | EMU=(0,0,1219200,685800) | get_shape_rect | 与 `fromEmuX/Y` 手算相等 | 单位换错 |
| TS6 | TS-CACHE | 同输入两次 | get_shape_rect | 值相等（同一缓存） | 每次重算 |
| TS7 | TS-DIRTY | 置脏 | clear→get | 重算后仍正确 | 缓存失效遗漏 |
| TS8 | TS-NOGEOM | geometry 传入 | new + get_shape_rect | 就是传入值的换算，无二次重算 | 偷偷重算 |

### 3.5 位置
crate：`libs/ppt-core/Cargo.toml`
src：`libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs`（loop 创建）
test：`libs/ppt-core/tests/cn/wps/show/render/ShowTypoShape.rs`

### 3.6 脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_typo_shape_snapshot

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_typo_shape_snapshot \
  --src src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/layout/features/pl-06-typo-shape/functions/01_function_typo_shape_snapshot/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_typo_shape_snapshot --json`)
date: 2026-08-30 17:15

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/ShowTypoShape.rs` | 88/130 | 67.7% | 80/124 | 64.5% |
| **function total** | **88/130** | **67.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（loop 必跑；PASS 才能标 pass）
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java
```
当前 **PASS**（`check-template-layout.mjs --rs --java`）。

### 3.9 Java↔Rust 结构对齐报告（loop 必跑；overall PASS 才能标 pass）
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/layout/features/pl-06-typo-shape/functions/01_function_typo_shape_snapshot/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 09:17

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShowTypoShape` | `koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java` | `libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs` | **PASS** | `ShowTypoShape`, `isPlaceholder`, `placeholderType`, `isTextBox`, `getShowShape`, `isHidden`, `getRect`, `getShapeRect`, `clearShapeRectCache`, `getChildCount`, `getChild`, `getShapeRenderRect` | `setRenderOverride`, `setTextRenderModel`, `getTextRenderModel`, `render`, `isModularGroupDefaultPic`, `getRenderModel`, `getInterpretCtx`, `getShape`, `calculateBounds`, `update3dTransfromMatrix`, `setIsPlayer`, `hasPlayer` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->