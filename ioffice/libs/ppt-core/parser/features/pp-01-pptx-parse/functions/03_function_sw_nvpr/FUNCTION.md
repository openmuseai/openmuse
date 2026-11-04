# 03_function_sw_nvpr — p:nvPr metadata

> feature: pp-01-pptx-parse ｜ 状态：**pass**（8 tests；line 95.2%；layout/alignment PASS；与 pp-02 `01_function_nonvisual_shape_props` 共用 src/tests）

## 0. 管线能力解锁

已实现并验收：p:nvPr 形状元数据（photo/userDrawn、media mode、custData、placeholder）写入既有 ShowShape，后续过滤/继承有输入。缺失时断在哪：placeholder 过滤与继承链无依据，pr-01 过滤与 pl-03 ph 合并断输入。
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
来自 `01_function_sw_reader_map`，未知枚举保持模型默认值。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓（保留 Java 注释+签名） | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `SWReaderMap` | 01，D/call-only | placeholder size/type lookup | **pass**；测试使用真实 crate 输出 | 无 | 不改映射常量 |
| `HandlerCallback` | 02，D/call-only | 共享 callback context | **pass**；object-safe trait | 无 | 不内嵌 callback 子类 |
| `ShowShape`/`Placeholder` | model，E/fixture contract | flags/media/custData/forceWritePh/save | 默认编译面模型裁剪 | 测试提供同名 contract fixture | 不在本 function 恢复 KoBin 模型 |
| `XmlRoAttr`/`ElementHandler` | kfc XML seam，E/fixture contract | attr lookup + child handler return | trait 存在 | 测试提供内存 attrs/handler contract | 不实现 XML reader |
| media/custData child handlers | DML/parser，C/constructor contract | 对 7 个 child id 构造真实命名 handler | 默认面媒体文件裁剪 | 测试同名 constructor fixtures | 禁止用 `None` 假装支持 |
| `DGMap.orientationMap` | drawing map，E/fixture contract | orientation token lookup | 测试 fixture 使用 Java 字面量 | 不恢复 DGMap 其它表 | 不改 orientation 常量 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` |
| caller | `SwShapeHandler`、`SwPictureHandler`、`SwConnectorShapeHandler`、`SwGroupShapeHandler` |

函数改写列表：outer ctor/`startElement`/`enterSubElement`/`endElement`；nested
`PlaceholderShapeHandler` ctor/`startElement`。Java 中其余成员必须保留模板位置并按 skill 规则停车。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|---|
| outer `startElement` | SwNvPrHandler.java/.rs | attrs 3211572/3211573 | photo/userDrawn | present/absent/false |
| outer `enterSubElement` | 同上 | 7 个 media/custData id、placeholder、unknown | mode 写入 + handler | 每个 case/default |
| outer `endElement` | 同上 | placeholder None/Some | save + forceWritePh | 两侧 |
| placeholder `startElement` | nested class | customPrompt/idx/orient/size/type | placeholder fields | sentinel/normal/unknown map |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|---|---|---|---|---|
| C01 | photo/userDrawn present/absent | 分支 | 是 | T01–T02 |
| C02 | media child ids 1114119–1114123 | 分支 | 是 | T03 |
| C03 | ext/custData child ids | 分支 | 是 | T04 |
| C04 | placeholder create→save→attach | 状态转换 | 是 | T05 |
| C05 | idx sentinel/normal | 边界 | 是 | T06 |
| C06 | orient/size/type known/unknown | 后端契约 | 是 | T07 |
| C07 | unknown element/empty attrs | 失败/边界 | 是 | T08 |

### 3.3 可测试性

integration test 通过 `#[path]` 编译同一份 `SwNvPrHandler.rs`，并提供 `RecordingXmlRoAttr`、
`ShowShape`、media/custData constructor fixture；`SWReaderMap` 与 `HandlerCallback` re-export
`ppt_core` 已通过的真实上游。不启动 XML reader。无 ignored/spec 测试。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|---|---|---|---|---|---|
| T01 | C01 | 默认 shape | 两属性 true/false | 对应字段精确写入 | 标志写错 |
| T02 | C01/C07 | 已有 true 标志 | 空 attrs | 原状态不变 | 空 attrs 清空字段 |
| T03 | C02 | 默认 media mode=-1 | 五种 media child | mode 0–4 且返回对应 handler kind | 错 handler 或错 mode |
| T04 | C03 | 默认 | ext/custData | 返回对应 handler，不改 media mode | 误改 mode |
| T05 | C04 | 无 placeholder | placeholder child 后 end | save 一次并 attach | 未 save/未 attach |
| T06 | C05 | 新建 placeholder | idx sentinel、0、42 | `-1/0/42` | 未归一 4294967295 |
| T07 | C06 | 新建 placeholder | known/unknown tokens | known 映射；unknown 保持默认 | 未知 token 被乱写 |
| T08 | C07 | 默认 | unknown child | `None` 且模型不变 | 未知子元素误写入 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs
[[test]] name = "03_function_sw_nvpr"
```

（与 `01_function_nonvisual_shape_props` 共用同一 path，仅 `[[test]]` name 不同。）

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_sw_nvpr

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_sw_nvpr \
  --src src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/03_function_sw_nvpr/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_sw_nvpr --json`)
date: 2026-08-29 18:34

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

Loop 结果：**PASS**（91 条 `// Java:` 模板锚点保留且顺序对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/03_function_sw_nvpr/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-29 10:32

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwNvPrHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwNvPrHandler.rs` | **PASS** | `SwNvPrHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
