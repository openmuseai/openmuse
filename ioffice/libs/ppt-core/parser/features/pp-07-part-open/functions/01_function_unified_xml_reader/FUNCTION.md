# 01_function_unified_xml_reader — UnifiedXmlReader.read

> feature: pp-07-part-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：把 part InputStream/路径接入已有 ElementHandler 事件面（每次新建 HLXmlReader + `parser` 恰好一次），slide/layout/master 的 part XML 统经这里进入已 pass 的 handler。打开链各 part 的解析由此有了唯一、确定的调用点。缺失时断在哪：真实 pptx 的 part 打不开，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(InputStream|path, ElementHandler) -> HLXmlReader.parser once

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | `InputStream` + `ElementHandler`；或 `String srcPath` + `ElementHandler`；或 `String srcPath + int beginPos + int endPos + ElementHandler + boolean isBeginAtRoot`（path 切片 overload，FUNCTIONS.md §4 验收 1 允许 ignore）。每次调用**新建** `HLXmlReader(MapManager.getInstance(), handler)`。 |
| **输出** | `HLXmlReader.parser` **恰好一次**：stream overload → `parser(stream)`；path overload → `parser(srcPath)`；path+切片 overload → `parser(srcPath, beginPos, endPos, isBeginAtRoot)`。parse 事件经 `handler` 回调（startElement / endElement / enterSubElement）。 |
| **状态与副作用** | 本类全静态、无实例字段；`MapManager` 单例查表只读；解析副作用全部落在 `handler` 上。 |
| **错误语义** | 三个 overload 均 `throws IOException` 上抛（流 IO、path 不存在、FilePartInputStream 打开失败）；本类不 catch 任何异常。 |

### 1.2 重载转发契约（查证自 `UnifiedXmlReader.java` + `HLXmlReader.java`）

```text
read(InputStream, ElementHandler)
  → HLXmlReader(...).parser(stream)
  → parser(stream, true) → parser(stream, true, -1) → HLHandlerBuilder + HLElementReader.parser(stream)
read(String srcPath, ElementHandler)
  → parser(srcPath) → FileInputStream → parser(stream, true)
read(String srcPath, int beginPos, int endPos, ElementHandler, boolean isBeginAtRoot)
  → parser(srcPath, beginPos, endPos, isBeginAtRoot) → FilePartInputStream → parser(stream, isBeginAtRoot)
```

Rust 侧需等效的只有一个解析调用点：handler 构造后对输入流调用一次 parser。
Rust 侧 `HLXmlReader` / `HLElementReader` 引擎与 `ElementHandler` trait 的接线：**待 loop 确认**
（当前 `util/` 下仅有已 pass 的 `SWReaderMap.rs`，无 `MapManager.rs`，`mod.rs` 中 `MapManager` 被注释）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| MapManager 查表 | pp-01:01 同族（Rust 侧未接线） | token 枚举映射 | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| ElementHandler 实现 | pp-01 05/06/07（已 pass） | SlideHandler / SlideLayoutHandler / SlideMasterHandler | 是 | — | — |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/util/UnifiedXmlReader.java` | `libs/ppt-core/src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs`（尚未创建） |

### 关键方法 / 字段

| Java | Rust 目标 | 契约要点 |
|---|---|---|
| `read(InputStream stream, ElementHandler handler)` | `UnifiedXmlReader::read(stream, handler)` | 新建 HLXmlReader + `parser(stream)` 一次；事件经 handler 回调 |
| `read(String srcPath, ElementHandler handler)` | `UnifiedXmlReader::read(path, handler)` | 同上，`parser(srcPath)`；path 不存在 → IOException 上抛 |
| `read(String srcPath, int beginPos, int endPos, ElementHandler handler, boolean isBeginAtRoot)` | 可 ignore | path 切片；`parser(srcPath, beginPos, endPos, isBeginAtRoot)` |
| `MapManager.getInstance()` | map supply 参数（接线**待 loop 确认**） | 前缀 r/p/a/c/dgm/... → token 枚举，只读单例 |

### 改写列表

- `read` 三个 overload：前两个实写（stream / path）；第三个 **ignore** 直到需要（FUNCTIONS.md §4 验收 1）。
- Rust 侧解析引擎（`HLXmlReader.parser` 等价物）是外部契约：本 function 只保证「一次 parser 调用」的接法，不实现 XML 引擎。

### 停车列表

无移出项（本类只有 3 个 overload）。

## 3. 测试方案

### 3.1 代码审计与测试面

- Java 侧：`UnifiedXmlReader.java` 全文（3 overload）；`HLXmlReader.java` 的 `parser` overload 链与其默认参数（`bBeginAtRoot=true`、`labelPrefix=-1`）——后者决定 stream overload 的等价深参。
- 当前 Rust：`src/cn/wps/show/pptxr/util/` 下无 `UnifiedXmlReader.rs`（`mod.rs` 仅挂 `SWReaderMap`）；`ElementHandler` trait 与解析引擎未接线，**待 loop 确认**。
- fixture：`src/cn/wps/show/pptxr/handler/slides/SlideHandler`（pp-01:05 pass）可作真实 handler 复用，或 Recording handler 记录事件序。

禁止只按标题冒烟：三个 overload 的转发目标（`parser(stream)` vs `parser(srcPath)` vs `parser(srcPath, begin, end, isBeginAtRoot)`）必须逐一断言。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | stream overload | 正常 | 是 | T01 |
| C02 | path overload | 后端 | 是 | T02 |
| C03 | path+begin/end+isBeginAtRoot | 后端 | 是 | T03 |

### 3.3 可测试性

内存 `Vec<u8>`/`Cursor` 作为 `InputStream` 等价输入 + Recording handler（记录 startElement/endElement/enterSubElement 事件序与 attr）：验证一次 parser 调用、事件完整转发、`InputStream` 耗尽即返。path overload 用临时文件。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 最小内存输入覆盖 `stream overload`：`<p:sld/>` 流 + Recording handler | parser 恰好一次；事件序与 Java 一致 | 红测停在本 function |
| T02 | C02 | 最小内存输入覆盖 `path overload`：临时文件 `<p:sld/>` | 同 T01；path 不存在 → IOException | 红测停在本 function |
| T03 | C03 | 最小内存输入覆盖 `path+begin/end+isBeginAtRoot`：切片窗口（可 ignore 时标记 skip） | 切片内事件与 Java 一致 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_unified_xml_reader"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_unified_xml_reader

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_unified_xml_reader \
  --src src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs \
  --src src/cn/wps/show/pptxr/util/MapManager.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/01_function_unified_xml_reader/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_unified_xml_reader --json`)
date: 2026-09-01 18:19

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/util/MapManager.rs` | 59/71 | 83.1% | 113/135 | 83.7% |
| `src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs` | 12/12 | 100.0% | 36/36 | 100.0% |
| **function total** | **71/83** | **85.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/util/UnifiedXmlReader.java

node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/util/MapManager.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/util/MapManager.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/util/UnifiedXmlReader.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/util/MapManager.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/util/MapManager.rs \
  --report libs/ppt-core/parser/features/pp-07-part-open/functions/01_function_unified_xml_reader/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 10:19

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `UnifiedXmlReader` | `koffice/presentation/core/cn/wps/show/pptxr/util/UnifiedXmlReader.java` | `libs/ppt-core/src/cn/wps/show/pptxr/util/UnifiedXmlReader.rs` | **PASS** | `read` | — | — | **PASS** |
| `MapManager` | `koffice/presentation/core/cn/wps/show/pptxr/util/MapManager.java` | `libs/ppt-core/src/cn/wps/show/pptxr/util/MapManager.rs` | **PASS** | `MapManager`, `init`, `getId`, `getInstance` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->