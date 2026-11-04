# 02_function_handler_callback — DML 回调默认策略

> feature: pp-01-pptx-parse ｜ 状态：**pass**（4 tests；函数源码 line/region coverage 100%；layout/alignment PASS）

## 0. 管线能力解锁

已实现并验收：parser mode 互斥判定与默认回调策略固化，基类未覆写 hook 得到稳定 identity/`-1`/`None`/no-op 语义。缺失时断在哪：handler 分发无策略、mode 语义漂移，sld/layout/master 解析默认行为无法对齐。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

输入是 parser mode 与 DML/pptxr handler 发出的 typed callback；输出是四种 mode 的互斥判定，
以及基类未覆写时稳定的 identity、`-1`、`None` 或 no-op 语义。该类不是 XML 元素事件
分发器，也不拥有 OPC、media 或 shape store。

### 1.1 I/O 契约

| 输入 | 输出/副作用 |
|---|---|
| `mode = 0/1/2/3/其它` | `isPPTX/isPPT14Slides/isGVml/isArtTable` 精确判定 |
| origin shape id | 默认 `getCoreShapeId` 原样返回 |
| 未解析的 OLE/ink relationship | 默认返回 `-1` |
| 未提供 chart/diagram collector | 默认返回 `None` |
| 其它 typed hook | 默认不改输入模型、不产生副作用 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| drawing/show 模型类型 | ppt-core model | 作为 callback 参数/返回值 | 由 `HandlerCallback` associated types 强类型绑定 | 具体类型随各 parser 子类接线 | 不在此处造 local model stub |
| `FileManager` | util manager | `onInk` 参数 | 由 associated type + 可空可变借用表达 | 真实 manager 随宿主 feature 接线 | 不实现文件 I/O |
| 具体 callback 子类 | 后续 slide/package feature | 覆写关系解析和资源导入 | object-safe trait 已提供动态分发 seam | 子类业务逻辑后续实现 | 不把子类逻辑塞进基类 |

## 2. 核心代码与改写边界

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/drawing/dmlr/model/HandlerCallback.java` |
| Rust 现状 | `libs/ppt-core/src/cn/wps/drawing/dmlr/model/HandlerCallback.rs` |
| 直接调用方 | `.../pptxr/ppt/slides/HandlerCallbackImpl.java`、`SwShapeHandler.java`、`SwConnectorShapeHandler.java` |

函数改写列表：两个构造入口、四个 mode predicate、`getCoreShapeId`、`onOleObject`、
`getChartMap`、`getDiagramPairs`、`onInk` 与所有默认 no-op hook。Java 的抽象可覆写语义必须在
Rust 设计中可替换。Rust 使用 `HandlerCallback` object-safe trait；`HandlerCallbackState` 承接 Java
构造器初始化的 `mode`，具体实现通过 `callback_state` 暴露该状态。Java 引用可空性映射为
`Option`，模型对象使用借用而非按值消费，map getter 返回内部 map 的共享借用。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| constructors + `is*` | HandlerCallback.java/.rs | 默认或显式 mode | 四个 bool | mode 0/1/2/3/其它 |
| `getCoreShapeId` | 同上 | 任意正/零/负 id | identity | 无 |
| `onOleObject`/`onInk` | 同上 | relationship id | `-1` | unresolved 默认 |
| `getChartMap`/`getDiagramPairs` | 同上 | 无 collector | `None` | null 语义 |
| typed no-op hooks | 同上 | 各模型值 | 无副作用 | 重复调用 |
| subclass consumer | `pptxr/ppt/slides/HandlerCallbackImpl.java` | override 调用 | 可替换行为 | 基类不能封死动态分发 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | 默认 ctor | 正常 | 是 | T01 |
| C02 | mode 0/1/2/3 | 分支 | 是 | T02–T05 |
| C03 | 其它 mode | 边界 | 是 | T06 |
| C04 | shape id identity | 边界 | 是 | T07 |
| C05 | OLE/ink sentinel | 失败语义 | 是 | T08 |
| C06 | chart/diagram `None` | 失败语义 | 是 | T09 |
| C07 | 所有默认 no-op hook | 交互契约 | 是 | T10 |
| C08 | callback 可覆写/可记录 | 状态转换 | 是 | T11 |

### 3.3 可测试性

测试用最小 fixture 绑定 associated types，并通过 `dyn HandlerCallback<...>` 调用
`RecordingCallback`，验证真正的动态分发，而非泛型静态分发。默认 hook 同时覆盖非空和 Java-null
输入，检查 callback/model 快照不变；无 ignored/spec 测试，不启动 XML reader 或完整 PPTX 管线。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 无 | 默认构造 | 仅 `isPPTX=true` | 默认 mode 漂移即红 |
| T02–T05 | C02 | 无 | mode 0/1/2/3 | 各自只有一个 predicate 为 true | mode 混叠即红 |
| T06 | C03 | 无 | mode -1、4 | 四者均 false | 未知 mode 被误归类 |
| T07 | C04 | 无 | id -1/0/42 | 原样返回 | id 被改写 |
| T08 | C05 | 无 | 未知 rId | OLE/ink 均 `-1` | 默认误报成功 |
| T09 | C06 | 无 | getter | 两者均 `None` | 空 collector 被伪造 |
| T10 | C07 | 合法最小模型 | 每个 no-op hook 调一次并重复一次 | Recording model 快照不变 | 基类意外写状态 |
| T11 | C08 | Recording seam | override hook | 记录次数、顺序、参数 | Rust 丢失 Java override 契约 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/drawing/dmlr/model/HandlerCallback.rs
test:  libs/ppt-core/tests/cn/wps/drawing/dmlr/model/HandlerCallback.rs
[[test]] name = "02_function_handler_callback"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_handler_callback

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_handler_callback \
  --src src/cn/wps/drawing/dmlr/model/HandlerCallback.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/02_function_handler_callback/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_handler_callback --json`)
date: 2026-08-28 12:31

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/dmlr/model/HandlerCallback.rs` | 82/82 | 100.0% | 74/74 | 100.0% |
| **function total** | **82/82** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/model/HandlerCallback.rs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/model/HandlerCallback.java
```

结果：**PASS**。56 条 `// Java` 锚点保持不变；原同文件 `FileManager` 空壳已删除，接口通过
associated type 保持强类型且不提前引入文件 I/O 实现。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/dmlr/model/HandlerCallback.java \
  --rs libs/ppt-core/src/cn/wps/drawing/dmlr/model/HandlerCallback.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/02_function_handler_callback/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 04:31

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `HandlerCallback` | `koffice/presentation/core/cn/wps/drawing/dmlr/model/HandlerCallback.java` | `libs/ppt-core/src/cn/wps/drawing/dmlr/model/HandlerCallback.rs` | **PASS** | `HandlerCallback`, `isGVml`, `isPPTX`, `isPPT14Slides`, `isArtTable`, `load`, `onTags`, `onBlipEmbed`, `onBlipLink`, `onAudioFileLink`, `onExtMediaFileLink`, `onQuickTimeFileLink`, `onVideoFileLink`, `onEmbeddedWAVAudioFileEmbed`, `onHyperlinkRid`, `onHyperlinkAction`, `onShapeId`, `onConnectorShape`, `getCoreShapeId`, `onOleObject`, `onChartRelationShip`, `addDiagramPair`, `onInk`, `onModel3D`, `onAdjustThemeColor`, `onMergeThemeTypeface`, `onConvertStyle2ShapeProps` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
