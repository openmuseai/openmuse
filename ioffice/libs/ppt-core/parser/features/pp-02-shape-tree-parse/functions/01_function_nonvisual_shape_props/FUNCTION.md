# 01_function_nonvisual_shape_props — p:nvPr metadata

> feature: pp-02-shape-tree-parse ｜ 状态：**pass**

## 0. 管线能力解锁

已实现并验收：spTree 中每个形状的 p:nvPr 元数据（ph/media/custData）写入既有 ShowShape，为叶子提交提供占位与媒体信息。缺失时断在哪：形状树节点元数据缺失，ph 过滤/继承无输入（pr-01/pl-03 断）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:nvPr` 的属性和直接子元素写入既有 `ShowShape`：photo/userDrawn 标志、media mode、
custom-data，以及保存后的 placeholder。未知属性和未知子元素不产生写入。

### 1.1 I/O 契约

```text
(ShowShape, HandlerCallback, nvPr start/sub/end events)
  -> updated ShowShape metadata
```

placeholder 的 `idx="4294967295"` 必须归一为 `-1`；orientation 来自 `DGMap`，size/type
来自 pp-01 的 `SWReaderMap`，未知枚举保持模型默认值。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `SWReaderMap` | pp-01/01，D/call-only | placeholder size/type lookup | **pass**；测试使用真实 crate 输出 | 无 | 不改映射常量 |
| `HandlerCallback` | pp-01/02，D/call-only | 共享 callback context | **pass**；object-safe trait 已完成 | 无 | 不内嵌 callback 子类 |
| `ShowShape`/`Placeholder` | model，E/fixture contract | flags/media/custData/forceWritePh/save | Rust 文件在当前工作树已删除 | 测试提供同名 contract fixture；模型 feature 恢复后直接接线 | 不在本 function 恢复 886 行模型及 KoBin/KpMap 图 |
| `XmlRoAttr`/`ElementHandler` | kfc XML seam，E/fixture contract | attr lookup + child handler return | trait 存在但 receiver 与 parser 可变语义尚未接通 | 测试提供内存 attrs/handler contract | 不实现 XML reader/dispatcher |
| media/custData child handlers | DML/parser deps，C/constructor contract | 对 7 个 child id 构造真实命名 handler | media Rust 文件已删除；custData 模板未完成 | 测试提供同名 constructor fixtures并验证实际返回类型；业务体由各自 feature 实现 | 禁止用 `None` 假装支持；不实现媒体解析 |
| `DGMap.orientationMap` | drawing map，E/fixture contract | orientation token lookup | Rust 文件已删除 | 测试 fixture 使用 Java 字面量中的 orientation 映射 | 不恢复 DGMap 其它数百张表 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` |
| caller | `SwShapeHandler`、`SwPictureHandler`、`SwConnectorShapeHandler`、`SwGroupShapeHandler` |

改写列表：outer ctor/startElement/enterSubElement/endElement；nested `PlaceholderShapeHandler`
ctor/startElement。Java 中其余成员必须保留模板位置并按 skill 规则停车。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|
| outer `startElement` | attrs 3211572/3211573 | photo/userDrawn | present/absent/false |
| outer `enterSubElement` | 7 个 media/custData id、placeholder、unknown | mode 写入 + handler | 每个 case/default |
| outer `endElement` | placeholder None/Some | save + forceWritePh | 两侧 |
| placeholder `startElement` | customPrompt/idx/orient/size/type | placeholder fields | sentinel/normal/unknown map |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | photo/userDrawn present/absent | 分支 | 是 | T01–T02 |
| C02 | media child ids 1114119–1114123 | 分支 | 是 | T03 |
| C03 | ext/custData child ids | 分支 | 是 | T04 |
| C04 | placeholder create→save→attach | 状态转换 | 是 | T05 |
| C05 | idx sentinel/normal | 边界 | 是 | T06 |
| C06 | orient/size/type known/unknown | 后端契约 | 是 | T07 |
| C07 | unknown element/empty attrs | 失败/边界 | 是 | T08 |

### 3.3 可测试性

integration test 通过 `#[path]` 编译**同一份** `SwNvPrHandler.rs`，并提供与生产路径同名的
`RecordingXmlRoAttr`、`ShowShape`、media/custData handler contract fixture；`SWReaderMap` 与
`HandlerCallback` 直接 re-export `ppt_core` 已通过的真实上游。这样可以逐个投递 start/sub/end
事件并验证真实 handler 构造，不启动 XML reader，也不恢复被裁剪的模型/媒体业务体。所有用例
必须非 ignore；模型 feature 恢复时该源码无需改变 I/O 语义即可接入正式模块树。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 |
|---|---|---|---|
| T01 | C01 | 两属性 true/false | 对应字段精确写入 |
| T02 | C01/C07 | 空 attrs | 原状态不变 |
| T03 | C02 | 五种 media child | mode 0–4 且返回对应 handler |
| T04 | C03 | ext/custData | 返回对应 handler，不改 media mode |
| T05 | C04 | placeholder child 后 end | placeholder 仅保存/attach 一次 |
| T06 | C05 | idx sentinel、0、42 | `-1/0/42` |
| T07 | C06 | known/unknown tokens | known 映射；unknown 保持默认 |
| T08 | C07 | unknown child | `None` 且模型不变 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs
[[test]] name = "01_function_nonvisual_shape_props"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_nonvisual_shape_props
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_nonvisual_shape_props \
  --src src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/01_function_nonvisual_shape_props/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_nonvisual_shape_props --json`)
date: 2026-08-28 14:43

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` | 180/189 | 95.2% | 177/194 | 91.2% |
| **function total** | **180/189** | **95.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java
```

Loop 结果：**PASS**（91 条 `// Java:` 模板锚点保留且顺序对齐；outer 与 nested handler
均由同一文件校验）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/01_function_nonvisual_shape_props/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 06:38

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwNvPrHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` | **PASS** | `SwNvPrHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
