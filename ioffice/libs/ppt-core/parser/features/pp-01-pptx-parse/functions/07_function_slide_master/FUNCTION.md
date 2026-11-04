# 07_function_slide_master — p:sldMaster XML

> feature: pp-01-pptx-parse ｜ 状态：**pass**（8 tests；line 82.9%；layout/alignment PASS）

## 0. 管线能力解锁

已实现并验收：p:sldMaster part 协议可解析——preserve 写入 SlideMaster，sldLayoutIdLst 的 rId→pid 经 `onSlideLayoutIds` 回调、hf/txStyles/子 handler 全链分发。缺失时断在哪：master part 打不开、三层胶片最底层无数据（pp-07 part-open 断）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:sldMaster` 的 preserve 写入 `SlideMaster.open(protocol)` 得到的数据；子元素分发 cSld /
transition / timing / clrMap / hf / sldLayoutIdLst / txStyles。`sldLayoutIdLst` 收集
`rId → pid` 后在 list `endElement` 回调 `handlerCallback.onSlideLayoutIds`。`txStyles` 把
title/body/other 转给 `TextListStyleHandler`。`HeaderFooterHandler` 在 ctor 把四开关默认置 true，
再按属性覆盖。

### 1.1 I/O 契约

```text
(KmoSlideMaster, DgStore, MasterHandlerCallbackImpl, sldMaster events)
  -> preserve + id map callback + hf defaults/overrides + child handlers + protocol
```

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `CommonSlideDataHandler` | 04 | film+store+callback | **pass** | | |
| `MasterHandlerCallbackImpl` | pptxr master，E | 作为 `HandlerCallback` + `onSlideLayoutIds` | 测试 Recording 实现该额外方法 | 不实现 master 媒体解析 | 不把 Impl 全文件拉进本 function |
| `ColorMapHandler` | DML，C | clrMap 元素 | fixture | | |
| `TextListStyleHandler` | DML，C | title/body/other style | fixture kind | | |
| `SlideMaster`/`HeaderFooter`/`KmoSlideMaster` | model，E | open/putPreserve/save/hf/titleStyle… | fixture | | |
| `HandlerCallback` | 02 | 基类 hook | **pass** | | |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.java` |
| Java | `.../slidemasters/HeaderFooterHandler.java` |
| Java | `.../slidemasters/TextStylesHandler.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs` |
| Rust | `.../HeaderFooterHandler.rs` |
| Rust | `.../TextStylesHandler.rs` |

改写列表：`SlideMasterHandler` ctor/`startElement`/`enterSubElement`/`endElement`；nested
`IdListHandler` ctor/`enterSubElement`/`endElement`/`startElement`；`HeaderFooterHandler`
ctor/`startElement`；`TextStylesHandler` ctor/`enterSubElement`。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 关键分支 |
|---|---|---|---|
| `startElement` | 3211613 有/无 | putPreserve | present/absent |
| `enterSubElement` | 2162692/3211295/3211402/3211598/3211608/3211609/3211611/3211619/3211620/unknown | 各 handler | 每 case |
| `endElement` outer | 任意 | updateProtocol(save) | |
| `IdListHandler.startElement` | 3211618 + 4259841 ± 3211342；其它 element | map put | rid 缺则不 put；pid 缺则 0 |
| `IdListHandler.enterSubElement` | 任意 | 返回 this | |
| `IdListHandler.endElement` | 3211619 vs 其它 | 仅 list 结束回调 | 0/1/N ids |
| `HeaderFooterHandler` ctor | hf Some | 四开关 true | Java 对 null 跳过默认 |
| `HeaderFooterHandler.startElement` | 3211458/3211566/3211567/3211568 | 覆盖对应开关 | 循环 0/1/N |
| `TextStylesHandler` | 3211615/3211616/3211617/unknown | TextListStyleHandler / None | |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | preserve attr | 分支 | 是 | T01 |
| C02 | 九个 master switch id | 分支 | 是 | T02 |
| C03 | outer end save | 状态 | 是 | T03 |
| C04 | id map 0/1/N + pid 缺省 0 | 循环/边界 | 是 | T04–T05 |
| C05 | id list end 才回调 | 状态 | 是 | T06 |
| C06 | hf ctor 默认 true + 属性覆盖 | 状态 | 是 | T07 |
| C07 | txStyles 三 id | 分支 | 是 | T08 |

### 3.3 可测试性

Recording callback 记录 `onSlideLayoutIds` 的 map 快照。hf/master 用 fixture。不打开 master part。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | preserve true/缺 | true / 默认不变 | 漏写 |
| T02 | C02 | 九个 id | 对应 kind；3211295 None | 漏 sldLayoutIdLst |
| T03 | C03 | endElement | protocol_save_count=1 | 未 save |
| T04 | C04 | 一个 sldLayoutId rid=rId1 pid=9 | map {rId1:9} 在 list end 回调 | 未收集 |
| T05 | C04 | 无 pid；两个 rid；非 3211618 元素 | pid=0；N=2；忽略其它 | 缺 pid 不插入 |
| T06 | C05 | start ids 但 end 非 3211619 | 不回调 | 提前回调 |
| T07 | C06 | ctor 后改 sldNum false | 默认全 true 后 sldNum false | ctor 未置 true |
| T08 | C07 | title/body/other/unknown | 三 kind + None | 错 style 槽 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs
       libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs
       libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs
[[test]] name = "07_function_slide_master"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 07_function_slide_master

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 07_function_slide_master \
  --src src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs \
  --src src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs \
  --src src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/07_function_slide_master/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 07_function_slide_master --json`)
date: 2026-08-29 19:51

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs` | 31/45 | 68.9% | 38/66 | 57.6% |
| `src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs` | 113/136 | 83.1% | 128/161 | 79.5% |
| `src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs` | 35/35 | 100.0% | 29/29 | 100.0% |
| **function total** | **179/216** | **82.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.java
```

Loop 结果：**PASS**（SlideMasterHandler 66、HeaderFooterHandler 30、TextStylesHandler 19 条 `// Java:` 锚点对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/07_function_slide_master/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-29 11:51

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SlideMasterHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/SlideMasterHandler.rs` | **PASS** | `SlideMasterHandler`, `startElement`, `enterSubElement`, `endElement` | — | — | **PASS** |
| `HeaderFooterHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/HeaderFooterHandler.rs` | **PASS** | `HeaderFooterHandler`, `startElement` | — | — | **PASS** |
| `TextStylesHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/slidemasters/TextStylesHandler.rs` | **PASS** | `TextStylesHandler`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
