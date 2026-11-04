# 06_function_slide_layout — p:sldLayout XML

> feature: pp-01-pptx-parse ｜ 状态：**pass**（6 tests；line 89.8%；layout/alignment PASS）

## 0. 管线能力解锁

已实现并验收：p:sldLayout part 协议可解析——type 经 `SWReaderMap.slideLayoutTypeMap` 查表（未命中写 23），cSld/transition/timing/clrMapOvr/hf 分发并保存 layout protocol。缺失时断在哪：layout part 打不开、ph 继承链下层无数据（pp-07 part-open 断）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:sldLayout` 根属性写入 `SlideLayout`（type 经 `SWReaderMap.slideLayoutTypeMap`，未命中写 `23`），
并把子元素分发给 cSld / transition / timing / clrMapOvr / hf。`endElement` 始终
`mLayout.updateProtocol(layoutData.save())`。

### 1.1 I/O 契约

```text
(KmoSlideLayout, DgStore, HandlerCallback, sldLayout events)
  -> layout type/userDrawn/showMaster*/matchingName/preserve + child handlers + protocol
```

属性遍历：`count` + `getAttrAt`。type 查表失败时 Java 写 `23`（不是保持默认）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `SWReaderMap.slideLayoutTypeMap` | 01，D | type 字符串→i32 | **pass** | 无 | 不改表 |
| `CommonSlideDataHandler` | 04 | film+store+callback | **pass** | | 不重写 cSld |
| `HeaderFooterHandler` | 07，D/constructor | `new(hf)` | **pass**（07 真实类型；本测试仍用 kind fixture） | 无 | 不用 `None` 代替 3211611 |
| `HandlerCallback` | 02 | 传给子 handler | **pass** | | |
| `SlideLayout`/`KmoSlideLayout` | model，E | create/put*/save/film/updateProtocol | fixture | | |
| transition/timing/alt/clrMap | C | 已知 id 构造 | fixture | 各 feature | |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs` |

改写列表：ctor、`enterSubElement`、`startElement`、`endElement`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|
| `startElement` | 3211302 known/unknown；3211573/3211600/3211601/3211612/3211613；null attr | putType 23 on miss；其它精确写 | 循环 0/1/N |
| `enterSubElement` | 2162692/3211295/3211402/3211599/3211608/3211609/3211611/unknown | handler 或 None | 每 case |
| `endElement` | 任意 | updateProtocol(save) | 无条件 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | known layout type | 正常 | 是 | T01 |
| C02 | unknown type → 23 | 失败/边界 | 是 | T02 |
| C03 | 其余五属性 | 分支 | 是 | T03 |
| C04 | 空/null attr | 边界 | 是 | T04 |
| C05 | 七个 switch id | 分支 | 是 | T05 |
| C06 | end save | 状态 | 是 | T06 |

### 3.3 可测试性

真实 `SWReaderMap.get_slideLayoutTypeMap()`。其余同 05。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | type=`title`（Java `slideLayoutTypeMap` 值 0） | type==0 | 查表未接到 01 |
| T02 | C02 | type=`not-a-layout` | type==23 | miss 未写 23 |
| T03 | C03 | userDrawn/showMaster*/matchingName/preserve | 精确写入 | 漏属性 |
| T04 | C04 | 空 attrs | 默认不变（type 未写则保持 create 默认） | 空循环写 23 |
| T05 | C05 | 七个 id | 对应 kind；3211295 None | 漏 hf |
| T06 | C06 | endElement | protocol_save_count=1 | 未 save |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs
[[test]] name = "06_function_slide_layout"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 06_function_slide_layout

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 06_function_slide_layout \
  --src src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/06_function_slide_layout/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 06_function_slide_layout --json`)
date: 2026-08-29 19:51

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs` | 106/118 | 89.8% | 88/107 | 82.2% |
| **function total** | **106/118** | **89.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java
```

Loop 结果：**PASS**（SlideLayoutHandler 61 条 `// Java:` 锚点对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/06_function_slide_layout/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-29 11:51

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideLayoutHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidelayouts/SlideLayoutHandler.rs` | **PASS** | `SlideLayoutHandler`, `enterSubElement`, `startElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
