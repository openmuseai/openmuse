# 02_function_pptx_theme_open — theme part 读入

> feature: pp-04-theme-part ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：theme part 真实读入并解析进 `KmoTheme`（`open()` 经 `ThemeHandler` 填充模型并取 rid→mediaId 图片映射），解析失败另有 `buildDefaultTheme` 产默认主题兜底。缺失时断在哪：theme part 打不开则 `KmoTheme` 无来源，scheme 色/字体/填充继承链在 01 之后断（画色必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoTheme, OpcrPackagePart) -> rid2MediaIds or null

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | 构造：`KmoTheme mTheme`、`OpcrPackagePart mPart`、`KmoMediaLib mMediaLib`、`PptxrMedias mMedias`、`int mMode`；`open()` 时经 `mPart.getInputStream()` + `mPart.getRelationships()` 读取（03/pp-09 传真 part；本层 fixture 传假 part/内存 stream）。 |
| **输出** | `HashMap<String, Integer>`：`rid→mediaId` 图片映射（来自 `handlerCallback.getRidPicMediaIds()`，见 `HandlerCallbackImpl.mRidPicMediaIds`，`onBlipEmbed`/`onBlipLink` 填充）；null = 无图或解析失败。`buildDefaultTheme()`（static）输出新 `KmoTheme`（已 `setThemePath`）。 |
| **状态与副作用** | `ThemeHandler`（01）解析写入 `mTheme`（`updateProtocol(themeData.save())`）；`handlerCallback.load()` 在 open 的 try 内执行（媒体元数据落地，pp-07 语义）；`buildDefaultTheme` 从 `PptResourceFactory.getResPrivider().getTheme()` 读资源流，parse 后 `theme.setThemePath(getThemePath())`。 |
| **错误语义**（Java 源码逐行查证） | `open()`：`mPart == null` → 直接 `return null`（不抛、不进入 try）；其余整段包在 `try { … } catch (Throwable e) { IOLog.d(TAG,"Throwable",e); return null; }` 内 —— 含 `getInputStream`/`UnifiedXmlReader.read`/`handlerCallback.load()`/`getRidPicMediaIds()` 任一异常都吞掉返回 null（方法虽声明 `throws IOException`，运行时不抛出）。`buildDefaultTheme()`：资源流 null → `throw new FileNotFoundException("The resources is not exist.")`；parse 在 `try { read } finally { stream.close() }` 中执行，**没有 catch** —— 解析失败 IOException 向上抛，且流必关。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrTheme.java`（71 行） | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs`（模板未生成；loop 起点 = Java 全量） |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/字段 | Rust 目标 | 归属 |
|---|---|---|---|
| 字段 `mTheme` | `KmoTheme` | `KmoTheme`（库内已有） | 本 function |
| 字段 `mPart` | `OpcrPackagePart` | 假 part（内存 stream + 假 relationships） | 本 function |
| 字段 `mMediaLib` | `KmoMediaLib` | 占位/假 | 本 function |
| 字段 `mMedias` | `PptxrMedias` | 假（pp-09 真装） | 本 function |
| 字段 `mMode` | `int` | `i32` | 本 function |
| ctor | `PptxrTheme(KmoTheme, OpcrPackagePart, KmoMediaLib, PptxrMedias, int)` | `new` | 本 function |
| `open` | `HashMap<String,Integer> open() throws IOException` | `open`（null part→None；任意异常→None） | 本 function |
| `buildDefaultTheme` | `static KmoTheme buildDefaultTheme() throws IOException` | `build_default_theme`（重命名避 Rust 关键字；同名语义） | 本 function |
| （引用）`HandlerCallbackImpl` | ctor 7 参 + `load()` + `getRidPicMediaIds()` | pp-07；本层 fixture 实现 | 停车 |
| （引用）`UnifiedXmlReader.read(InputStream, ElementHandler)` | static | pp-07；本层注入式 `read` fixture | 停车 |
| （引用）`ThemeHandler` | ctor + 事件回调 | 01 | 复用 |
| （引用）`PptResourceFactory.getResPrivider()` | `getTheme()` / `getThemePath()` | 资源提供 trait（fixture 或配置流） | 本 function 边界 |
| （引用）`IOLog.d` | 吞异常前打日志 | API 层日志可空实现；不影响返回 null 语义 | 本 function |

### 2.2 函数改写列表

- **本 function 改写：** `PptxrTheme` 全类 —— ctor、`open`、`buildDefaultTheme`（错误语义与 finally 关流
  行为按 §1.1 锁死，不「顺手」加 catch）。
- **停车：** `HandlerCallbackImpl`、`UnifiedXmlReader`（均 pp-07）—— loop 前用 fixture；
  `DgStore`/`KmoInkLib`/`PptxrSlides` 构造实参 Java 传 null，Rust 侧对应 `None`。
- **复用：** 01 `ThemeHandler`、`KmoTheme`、`Theme`；禁改这些既有实现。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `PptxrTheme.java` 全量（`open`/`buildDefaultTheme` 逐行，重点核对
`try/catch(Throwable)` 覆盖范围与 `finally` 关流）与 Java 侧 `KmoTheme.updateProtocol` 副作用；
当前 Rust `PptxrTheme.rs` 尚未生成（`show/pptxr/ppt/theme/` 目录不存在），loop 前以 Java 为准。
禁止只按标题冒烟。只读参照：`HandlerCallbackImpl.getRidPicMediaIds`（pp-07）、
`PptxrThemeOverride.open`（同目录，无 try/catch 的对照实现）。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | null part → null | 边界 | 是 | T01 |
| C02 | parse 成功返回 rid→mediaId map | 正常 | 是 | T02 |
| C03 | parse Throwable → null（吞掉） | 失败 | 是 | T03 |
| C04 | `handlerCallback.load()` 在 try 内（load 异常同样吞 → null） | 失败 | 是 | T04 |
| C05 | buildDefaultTheme 资源流 → 新 KmoTheme + setThemePath | 正常 | 是 | T05 |
| C06 | buildDefaultTheme 资源流 null → 错误（Java FileNotFoundException → Rust `io::Error` 等价） | 失败 | 是 | T06 |
| C07 | buildDefaultTheme parse 失败 → 异常上抛且 finally 关流 | 失败 | 是 | T07 |

### 3.3 可测试性

假 `OpcrPackagePart`（内存 `InputStream` + 假 relationships）或直接注入 `read` 函数（fixture reader，
02 不依赖 pp-07 `UnifiedXmlReader`）；callback 为 fixture 实现（`load()` 空、`onBlipEmbed` 记假
rid→id 映射），断言 `open()` 返回的 map 键值。不跑完整 `PptxReader`（本 function 不是 pp-09
编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | `mPart = None` 调 open | 返回 None（不抛） | 红测停在本 function |
| T02 | C02 | 假 part 携带含图片 embed 的 theme XML 片段（fixture callback 记录映射） | 返回 `rid→mediaId` map，与 fixture 一致 | 红测停在本 function |
| T03 | C03 | reader/解析抛异常（注入 panic/Err） | 返回 None（Java 吞 Throwable 等价） | 红测停在本 function |
| T04 | C04 | fixture callback 的 `load()` 抛异常 | 返回 None（load 在 try 内） | 红测停在本 function |
| T05 | C05 | 资源流为合法 theme XML | 返回 `KmoTheme`：scheme 数据可达、`mThemePath` 已设置 | 红测停在本 function |
| T06 | C06 | 资源提供返回 None 流 | 返回 Err（FileNotFoundException 等价） | 红测停在本 function |
| T07 | C07 | 合法资源流 + 畸形 XML | Err 上抛且流被关（无泄漏；Rust 侧 RAII 等价断言） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "02_function_pptx_theme_open"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_pptx_theme_open

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_pptx_theme_open \
  --src src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs \
  --report libs/ppt-core/parser/features/pp-04-theme-part/functions/02_function_pptx_theme_open/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_pptx_theme_open --json`)
date: 2026-09-01 12:00

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs` | 157/161 | 97.5% | 192/199 | 96.5% |
| **function total** | **157/161** | **97.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrTheme.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrTheme.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs \
  --report libs/ppt-core/parser/features/pp-04-theme-part/functions/02_function_pptx_theme_open/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 04:00

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrTheme` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrTheme.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrTheme.rs` | **PASS** | `PptxrTheme`, `buildDefaultTheme` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->