# 02_function_begin_filter — 形状开始过滤

> feature: pr-01-slide-layers ｜ 状态：**pass** ｜ Java 管线环节：S3

## 0. 管线能力解锁

实现后：层列表里的形状在进 pr-02 之前先做 **画/跳过决策**——hidden 形状、`drawFlags&1` 下的 placeholder、`&32` 下的 miner placeholder（ph type 4/5/6/7）不会产生任何指令。没有它，管线断在 S3 的过滤环节：隐藏或不该出现的占位框会被误画（主文档断点表「过滤规则对齐 → 只画该画的形状」）。

## 1. 功能说明

`(LayerItem::Shape, drawFlags) → Pass | Skip`，对 01 层列表的每个形状实施 Java `ShapeRender._begin()` 的四条过滤。

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `LayeredTypoShape`（`isHidden`/`isPlaceholder`/`placeholderType`/`isModularGroupDefaultPic`）+ `drawFlags: i32`。 |
| **输出** | `Pass`（进入下游 pr-02）/ `Skip`。 |
| **状态与副作用** | 纯决策，无副作用。Java 的 `mEnv.isDrawModularDefaultPic()` 由 ShapeRender ctor 依 `drawFlags & 16777216`（NOT_DRAW_MODULAR_GROUP_PIC_PLACEHOLDER）设定——Rust 侧同一位置计算。 |
| **错误语义** | 无异常路径；Java `ShowTypoShape.isHidden()` = `mShapeBase==null \|\| mIsHidden`（shapeBase 为 null 即 hidden）。Rust 侧基准：typo==None → Skip（待 loop 确认）。 |

### 1.2-1.6 已并入 1.1（纯决策函数，无额外接口）

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| LayeredTypoShape | pl-06 | isHidden/isPlaceholder/placeholderType/isModularGroupDefaultPic | spec 未接线 | 先 fixture | 不改字段 |
| drawFlags 常量 | 本 feature/01 | 1 / 32 / 16777216 | 无 | loop 确认 | — |
| DrawFlags 位测（drawPHolder etc.） | — | 位运算 | — | 全部位与真实场景对照待 loop | 不 hand-pick |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/render/ShapeRender.java`（`_begin`，31-53 行附近；ctor 67-76 行） |
| Rust | `libs/ppt-core/src/cn/wps/show/render/ShapeRender.rs`（尚不存在，loop 用模板生成；本 function 只落 `_begin` 语义） |

关键方法（对照 Java，均已查证）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| ctor `ShapeRender(ShapeEnv, ShowTypoShape, int drawFlags)` | `0 != (drawFlags & 16777216)` → `mEnv.setDrawModularDefaultPic(false)`，否则 true | 由 ctor 计算 modular 默认图开关 |
| `_begin()` | ①`mTypoShape.isHidden()`→false；②`(drawFlags&1)!=0 && isPlaceholder()`→false；③`(drawFlags&32)!=0 && isPlaceholder() && isMinerPlaceHolder(placeholderType())`→false；④`return mEnv.isDrawModularDefaultPic() \|\| !mTypoShape.isModularGroupDefaultPic()` | `_begin() -> bool` 四条同序 |
| `isMinerPlaceHolder(int type)` | `switch { case 4/5/6/7: true; default: false }` | `is_miner_place_holder(type: i32) -> bool` |

改写列表：`ShapeRender` ctor 的 modular 开关 + `_begin` + `isMinerPlaceHolder`。`_render` 与装饰方法（renderPlaceHolderDashFrame/renderDashBorder/renderLayoutBorder/renderPicPhHolderIcon）属 pr-02/编辑场景，本 function 不写。

停车列表：`_render` 内 `drawFlags&4` dash frame、`DrawFlags.drawLayout` 布局边框、`&2097152` 图片 ph 图标、modular group 子 ph 装饰（均 edit/back/thumb 装饰，post-MVP）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `ShapeRender.java` 的 `_begin`/`isMinerPlaceHolder` 与 ctor（重点：`isHidden()` 在 ShowTypoShape 中 = `mShapeBase==null || mIsHidden`；`placeholderType()` = `ph().type()`）。当前 Rust 无对应文件，以 Java 为准。

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| _begin | hidden / ph / miner / 普通 | bool | 四条分支 |
| isMinerPlaceHolder | type 4/5/6/7 与 0/1/2/3 | true/false | switch |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| F01 | isHidden → Skip | 过滤 | 是 | T01 |
| F02 | flags&1 + isPlaceholder → Skip | 过滤 | 是 | T02 |
| F03 | flags&32 + placeholderType∈{4,5,6,7} → Skip | 过滤 | 是 | T03 |
| F04 | flags&32 + placeholderType∉{4..7} → Pass | 过滤 | 是 | T04 |
| F05 | 普通形状（全零 flags）→ Pass | 主路径 | 是 | T05 |
| F06 | modular 默认图条件（&16777216） | 过滤 | 否 | `#[ignore = "spec: post-MVP"]` |

### 3.3 可测试性

fixture `LayeredTypoShape`（每条过滤一个变体）+ 表驱动测试枚举 flags 组合；不依赖真实 canvas/env。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | F01 | hidden=true | Skip | 红测停在本 function |
| T02 | F02 | flags=1，ph=true | Skip | 红测停在本 function |
| T03 | F03 | flags=32，ph=true，type=4 | Skip | 红测停在本 function |
| T04 | F04 | flags=32，ph=true，type=2（title） | Pass | 红测停在本 function |
| T05 | F05 | flags=0，普通形状 | Pass | 红测停在本 function |
| T06 | F06 | flags=16777216 | modular 开关两分支 | ignore |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/ShapeRender.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_begin_filter"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_begin_filter

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_begin_filter \
  --src src/cn/wps/show/render/ShapeRender.rs \
  --report libs/ppt-core/render/features/pr-01-slide-layers/functions/02_function_begin_filter/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_begin_filter --json`)
date: 2026-08-30 17:26

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/ShapeRender.rs` | 57/76 | 75.0% | 65/84 | 77.4% |
| **function total** | **57/76** | **75.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShapeRender.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeRender.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShapeRender.java \
  --rs libs/ppt-core/src/cn/wps/show/render/ShapeRender.rs \
  --report libs/ppt-core/render/features/pr-01-slide-layers/functions/02_function_begin_filter/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-30 09:26

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShapeRender` | `koffice/presentation/core/cn/wps/show/render/ShapeRender.java` | `libs/ppt-core/src/cn/wps/show/render/ShapeRender.rs` | **PASS** | `ShapeRender`, `_begin`, `isMinerPlaceHolder` | `getShapeId`, `update3DDrawTag`, `_render`, `renderPlaceHolderDashFrame`, `renderDashBorder`, `renderLayoutBorder`, `renderPicPhHolder`, `renderPicPhHolderIcon` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->