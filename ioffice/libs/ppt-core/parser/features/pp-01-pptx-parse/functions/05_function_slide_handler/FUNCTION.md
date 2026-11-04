# 05_function_slide_handler — p:sld part XML

> feature: pp-01-pptx-parse ｜ 状态：**pass**（5 tests；line 88.3%；layout/alignment PASS）

## 0. 管线能力解锁

已实现并验收：p:sld part 协议可解析——根属性写入 Slide 数据，cSld/transition/timing/clrMapOvr 按元素 id 分发并 save protocol。缺失时断在哪：真实 slide part 打不开、Slide 层属性无从装载（pp-07 part-open 断）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:sld` 根属性写入 `Slide` 数据，并把已知子元素分发给 cSld / transition / timing /
clrMapOvr。`endElement` 在 `slideData != null` 时 `mSlide.updateProtocol(slideData.save())`。
`3211295` 与 unknown 返回 `null`。

### 1.1 I/O 契约

```text
(KmoSlide, DgStore, HandlerCallback, sld start/sub/end events)
  -> Slide show/showMasterSp/showMasterPhAnim + child handlers + protocol save
```

属性遍历使用 `attrs.count()` + `getAttrAt(i)`，不是 `getByName`。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `CommonSlideDataHandler` | 04，D/constructor | `new(film, dgStore, callback)` | **pass** | 无 | 不重写 cSld |
| `HandlerCallback` | 02，D | 传给子 handler | **pass** | 无 | |
| `Slide.create`/`putShow*`/`clrMap`/`timing`/`transition`/`save` | model，E | 属性写入 + save | fixture | KoBin | 不恢复 Slide 全图 |
| `KmoSlide.film`/`updateProtocol` | app，E | film 交给 04；end 写 protocol | fixture | | |
| `TransitionHandler`/`TimingHandler`/`TransitionAlternateContentHandler` | C/constructor | 对应 element id | fixture kind | 各 feature | 不用 `None` 假装支持已知 id |
| `ColorMapOverrideHandler` | 本 function 改写列表 | 1114457→null；1114458→ColorMapHandler | 本 function | ColorMap 业务体 fixture | |
| `ColorMapHandler` | DML，C | ctor(colorMapping) | fixture | | |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java` |
| Java | `.../slides/ColorMapOverrideHandler.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs` |
| Rust | `.../slides/ColorMapOverrideHandler.rs` |

改写列表：`SlideHandler` ctor/`startElement`/`endElement`/`enterSubElement`；
`ColorMapOverrideHandler` ctor/`enterSubElement`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 路径 | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|---|
| `startElement` | SlideHandler | attrs 3211600/3211601/3211610 组合；null getAttrAt | 三 bool | 循环 0/1/N；null skip |
| `enterSubElement` | 同上 | 2162692/3211295/3211402/3211599/3211608/3211609/unknown | 各 handler 或 None | 每个 case |
| `endElement` | 同上 | slideData Some | updateProtocol(save) 一次 | Java 字段在 ctor 后非 null |
| `ColorMapOverrideHandler` | 同上包 | 1114457/1114458/unknown | null / ColorMapHandler / null | |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 三属性 true/false | 分支 | 是 | T01 |
| C02 | 空 attrs / 中间 null attr | 边界 | 是 | T02 |
| C03 | 六个 switch id | 分支 | 是 | T03 |
| C04 | end save | 状态 | 是 | T04 |
| C05 | clrMapOvr 1114457/1114458/unknown | 分支 | 是 | T05 |

### 3.3 可测试性

内存 attrs 实现 `count`/`get_attr_at`/`get_name`。子 handler 用 constructor fixture。不跑 OPC。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 三属性 true/false 混合 | 精确写入 | 写错 flag |
| T02 | C02 | count=0；count=2 且 index0 null | flags 默认不变 | 空循环写脏 |
| T03 | C03 | 六个 id | 对应 kind 或 None（3211295） | 错分发 |
| T04 | C04 | endElement | protocol_save_count=1 | 未 save |
| T05 | C05 | 三个 clrMapOvr id | null / ColorMapHandler / null | 1114457 被当成 map |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs
       libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/slides/SlideHandler.rs
[[test]] name = "05_function_slide_handler"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 05_function_slide_handler

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 05_function_slide_handler \
  --src src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs \
  --src src/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/05_function_slide_handler/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_slide_handler --json`)
date: 2026-08-29 19:51

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs` | 16/16 | 100.0% | 21/21 | 100.0% |
| `src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs` | 75/87 | 86.2% | 91/114 | 79.8% |
| **function total** | **91/103** | **88.3%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.java
```

Loop 结果：**PASS**（SlideHandler 58、ColorMapOverrideHandler 17 条 `// Java:` 锚点对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/05_function_slide_handler/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-29 11:51

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/SlideHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/SlideHandler.rs` | **PASS** | `SlideHandler`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |
| `ColorMapOverrideHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slides/ColorMapOverrideHandler.rs` | **PASS** | `ColorMapOverrideHandler`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
