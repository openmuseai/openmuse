# 01_function_getshape_cache — 惰性 SlideCache 入口

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.1

## 0. 管线能力解锁

实现后：`ShowTypoShape.getShape` 是公开 Shape 的**唯一惰性入口**。同页多帧/缩放/缩略图/打印/拖拽预览共用一份转换结果。filter 路径不写缓存，避免滤镜预览污染正式 Shape。

没有它：每次绘制都走完整 convert，或渲染器直接啃 Kmo。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | 已构造的 `ShowTypoShape`（pl-06 快照）+ `InterpretCtx`（至少 `FilterInterpretCtx`）+ `KmoShowShape.getSlideCache()` |
| **输出** | `Option`/`Shape`：公开 `cn.wps.moffice.drawing.Shape`（Rust：`drawing::…::Shape`，**不是** `ShowTypoShape.rs` 本地空 struct） |
| **状态与副作用** | `mShape` 字段缓存；`noFilter` 时 `SlideCache.addShape(id, shape)`；`hasFilter` 时不读不写 SlideCache |
| **错误语义** | Java：`mCtx==null` NPE。Rust：`interpretCtx=None` → 视为无 filter（与 pl-06 `None` ctx 一致）且 **仍必须 convert**，不 panic。`convert` 返回 null（未知 type）→ `mShape` 保持 None，不 addShape |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `ShowTypoShape.getShape` | `this.mShapeBase` + `this.mCtx` | `ShowTypoShape.java:180-194` |

### 1.3 输出
| 输出 | 说明 | Java 对标 |
|------|------|-----------|
| 命中 | `SlideCache.getShape(base)` 非空 → 赋 `mShape` 并返回 | `:183` |
| 未命中 | `ConvertTool.convertShapeToPublicShape` + `setShapeID(base.id())` | `:185-186` |
| 回填 | `noFilter` 才 `addShape` | `:187-188` |
| 二次调用 | `mShape != null` 直接返回，不再查 cache | `:181` |

### 1.4 上游
pl-06 快照（flags/rect 已 pass，**不**在本 function 重做）；02 convert 门面（miss 路径）。

### 1.5 下游
pr-02 `typo.getShape()`；命中测试；07 已挂在 convert 产物上。

### 1.6 不变量
1. 同一 `ShowTypoShape` 实例两次 `getShape` 返回同一对象（`mShape` 字段）。
2. 无 filter：同一 `KmoShowShape.id` 经 **两个** TypoShape 第一次 miss 后，第二次应命中 SlideCache（按 id）。
3. 有 filter：不读 cache、不 `addShape`；每次 miss 都 convert（Java `:182-183` 在 hasFilter 时 `mShape=null` 强制走 convert）。
4. `setShapeID` 在 convert 之后、addShape 之前（Java `:186` 在 convert 返回值上再 set 一次；02 内也会 set——幂等）。

### 1.7 依赖与实现状态（必写，着重）

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `ShowTypoShape` 快照 | pl-06 | ctor / `getShowShape` | ✅ flags+rect | — | 禁止改 `getShapeRect` 冻结语义 |
| `getShape` | 本 function | 惰性 convert+cache | ❌ `todo!("DEFERRED_NON_FUNCTION")` `:361` | — | — |
| `SlideCache.getShape/addShape` | S5 | id→Shape MRU 32 | ❌ 无 `.rs` | destroy/事件监听 | 禁止项目 stub；最小 HashMap/MRU 在 ppt-core |
| `FilterInterpretCtx.hasFilter` | radapter | `mInterpretFlag!=0` | ✅ 文件在，但 `show/radapter` cfg full | 接到默认面 | 禁止改 flag 位定义（4096/4097/4098） |
| `ConvertTool.convertShapeToPublicShape` | 02 | type 分派 | ❌ stub 未进默认 mod | — | 禁止在 01 内联装配 pos/fill |
| 公开 `Shape` 类型 | core_libs_drawing | `setShapeID` | ✅ `Shape.rs` | ShowTypoShape 本地空 Shape 替换 | 禁止用 `layout_shape::Shape` |

## 2. 核心代码与函数改写列表

| Java | Rust 计划 | 改写范围 |
|------|-----------|----------|
| `ShowTypoShape.getShape`（`:180`） | `ShowTypoShape.rs` | 01 门面；解开 DEFERRED |
| `SlideCache.getShape/addShape`（`:78/83`） | 新建 `show/render/cache/SlideCache.rs`（交错模板） | 01 最小 MRU |
| `KmoShowShape.id/getSlideCache` | `layout_shape.rs` 扩 | 01 读 id；cache 挂点（host 字段，不改 Java） |
| `ConvertTool.convertShapeToPublicShape` | `ConvertTool.rs` | **不实现**（02） |

不在列表：`render(ShapeEnv)`、`calculateBounds`、effect/3D、`removeShape` 脏标记全量事件。停车：`todo!("DEFERRED_NON_FUNCTION")` + `/* DEFERRED_NON_FUNCTION … */`。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`ShowTypoShape.java:180-194`（cache + convert + setShapeID + addShape）；`SlideCache.java:78-89`（get 按 id，add 按 id，remove=markDirty）；Rust `ShowTypoShape.rs:361-375` DEFERRED 块与 Java 逐行对应；`FilterInterpretCtx.rs` `hasFilter`；`layout_shape.rs` 无 `getSlideCache`。

| symbol | 路径 | 输入/状态 | 输出 | 关键分支 |
|--------|------|-----------|------|----------|
| `getShape` 首次 | miss、noFilter | fixture shape id=7 | convert 一次 + addShape | miss |
| `getShape` 二次 | 同实例 | 再调 | 同指针 / 同 ID，不再 convert | `mShape!=null` |
| cache 跨实例 | 同 id 新 TypoShape | noFilter | SlideCache hit，不二次 convert | `:183` |
| hasFilter | flag≠0 | — | 不 addShape；不读 cache | `:182` |
| convert null | type 非法 | — | None，不 add | `:120` 后 shape==null |
| None ctx | pl-06 默认 | — | 仍 convert，当 noFilter | Rust 约定 |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| GS-MISS | getShape | 首次 miss convert | 是 | GS1 |
| GS-MEMO | getShape | 同实例二次 | 是 | GS2 |
| GS-CACHE | SlideCache | 同 id 跨实例 hit | 是 | GS3 |
| GS-FILTER | hasFilter | 不读写 cache | 是 | GS4 |
| GS-ID | setShapeID | id 透传 | 是 | GS5 |
| GS-NULLCV | convert 返回空 | 不 addShape | 是 | GS6 |
| GS-NOCTX | ctx=None | 当 noFilter convert | 是 | GS7 |

### 3.3 可测试性
fixture `KmoShowShape { id, shape_type:1, … }` + `ShowTypoShape::typo_shape_snapshot`。不跑 `render_slide`。convert 未接线前 GS1–GS7 `#[ignore = "spec: getShape DEFERRED; SlideCache 缺失"]`。可绿：无（本 function 入口现 panic）。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| GS1 | GS-MISS | cache 空 | 首次 getShape | Some(Shape)，id=shape.id | 仍 DEFERRED panic / 返回空 struct |
| GS2 | GS-MEMO | GS1 后 | 再 getShape | 同一公开 Shape | 每次新对象 |
| GS3 | GS-CACHE | GS1 后新 Typo 同 id | getShape | 不二次走 convert 副作用（cache 计数=1） | 当新转换覆盖 |
| GS4 | GS-FILTER | setFilterFlag(4096) | getShape 两次 | cache 无条目 | filter 预览写进 LRU |
| GS5 | GS-ID | id=42 | getShapeID | 42 | 丢掉 id |
| GS6 | GS-NULLCV | type=99 | getShape | None；cache 空 | 空 Shape 进 cache |
| GS7 | GS-NOCTX | ctx=None | getShape | Some（走 convert） | 当 hasFilter 跳过 |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/render/ShowTypoShape.rs
       src/cn/wps/show/render/cache/SlideCache.rs（loop 新建）
test:  tests/cn/wps/show/render/01_function_getshape_cache.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_getshape_cache

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_getshape_cache \
  --src src/cn/wps/show/render/ShowTypoShape.rs \
  --src src/cn/wps/show/render/cache/SlideCache.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/01_function_getshape_cache/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_getshape_cache --json`)
date: 2026-09-11 00:54

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/render/ShowTypoShape.rs` | 70/217 | 32.3% | 92/260 | 35.4% |
| **function total** | **70/217** | **32.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java
```
SlideCache 在 loop 落地后追加一组 `--rs/--java`。`getShape` 解开后，其余方法保持 `DEFERRED_NON_FUNCTION` 停车。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java \
  --rs libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/01_function_getshape_cache/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-10 16:52

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `ShowTypoShape` | `koffice/presentation/core/cn/wps/show/render/ShowTypoShape.java` | `libs/ppt-core/src/cn/wps/show/render/ShowTypoShape.rs` | **PASS** | `ShowTypoShape`, `isPlaceholder`, `placeholderType`, `isTextBox`, `getShowShape`, `isHidden`, `isModularGroupDefaultPic`, `getRect`, `getShapeRect`, `clearShapeRectCache`, `getChildCount`, `getChild`, `getShape`, `getShapeRenderRect` | `setRenderOverride`, `setTextRenderModel`, `getTextRenderModel`, `render`, `getRenderModel`, `getInterpretCtx`, `calculateBounds`, `update3dTransfromMatrix`, `setIsPlayer`, `hasPlayer` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
