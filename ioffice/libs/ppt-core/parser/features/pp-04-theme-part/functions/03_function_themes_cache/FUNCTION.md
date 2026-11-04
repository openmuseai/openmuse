# 03_function_themes_cache — theme rel 缓存

> feature: pp-04-theme-part ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：theme rId 按 partName 缓存（`mGlobalThemes`）复用同一 `KmoTheme`，并将解析结果挂进 `KmoPresentation` theme 表，同 part 不再重复解析。缺失时断在哪：无 partName 缓存则主题解析结果落不进模型层、每处引用重开 part，scheme 色/字体定义不可达，实色之外的形状颜色全断（画色必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(KmoPresentation, themeRels, medias) -> cached KmoTheme

### 1.1 I/O 契约

| | 契约 |
|--|------|
| **输入** | 构造 `int mMode`；`open(KmoPresentation pres, OpcrRelationship themeRels, PptxrMedias medias[, boolean addTheme])` —— 缓存键 `partName = themeRels.getPartName()`；`open` 3 参版本委托 4 参版本且 `addTheme = true`（Java 源码查证）。 |
| **输出** | `KmoTheme`：cache hit（`mGlobalThemes.get(partName)` 非空）返回**同一实例**；miss 时 `part != null` → 新 `KmoTheme` + 02 `PptxrTheme.open`（theme 类型）或 `PptxrThemeOverride.open`（themeOverride 类型）+ `setRid2MediaId` + `setThemePath(dump)`；`part == null` → `PptxrTheme.buildDefaultTheme()`。 |
| **状态与副作用** | miss 路径 `mGlobalThemes.put(partName, theme)`；`addTheme == true` → `pres._addTheme(theme)`；`dump` 内 `mGlobalThemesPath.put(partName, themePath)`（`PptFileCache.dumpFile(part, pptRootTmpDir)` 落盘返回路径，`pres.getTmpPath()` 为根目录）。 |
| **错误语义**（Java 源码查证） | parse 失败已被 02 吞成 null → 此处只 `theme.setRid2MediaId(null)`（Java 照常执行）；本类声明 `throws IOException`：`dump` 的 `PptFileCache.dumpFile`、`buildDefaultTheme` 的 `FileNotFoundException`、`getPackagePart/getInputStream` 的 IO 异常**全部向上抛（不吞）**；`dump` 起始 `IOAssert.assertNotNull(themeRels)`（null 断言失败）。 |

relsType 常量（`OxfPptRelationsImpl` 查证）：theme = `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme`；
themeOverride = `http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride`（均经
`OxfPptRelations.theme()/themeOverride().getRelsType()` 取，分派用 `equals`，大小写敏感）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrThemes.java`（77 行） | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs`（模板未生成；loop 起点 = Java 全量） |

### 2.1 关键方法/字段表（Java 源码查证 → Rust 目标）

| Java 成员 | 签名/字段 | Rust 目标 | 归属 |
|---|---|---|---|
| 字段 `mGlobalThemes` | `Map<String, KmoTheme>` | `HashMap<String, KmoTheme>` | 本 function |
| 字段 `mGlobalThemesPath` | `Map<String, String>` | `HashMap<String, String>` | 本 function |
| 字段 `mMode` | `int` | `i32` | 本 function |
| ctor | `PptxrThemes(int mode)` | `new` | 本 function |
| `open` 3 参 | `KmoTheme open(pres, themeRels, medias) throws IOException` | 委托 4 参（addTheme=true） | 本 function |
| `open` 4 参 | `open(pres, themeRels, medias, boolean addTheme)` | 主实现（hit 早退；miss 分派两 relsType 分支；put；addTheme） | 本 function |
| `dump` | `String dump(OpcrRelationship, String pptRootTmpDir) throws IOException` | `dump`（assertNotNull + 路径缓存 + PptFileCache） | 本 function |
| （引用）`PptxrTheme`（open/buildDefaultTheme） | 02 | 复用 | 02 |
| （引用）`PptxrThemeOverride`（open） | 同目录，无 try/catch，直接 `ThemeElementsHandler` | 未规划；本分支待 loop 裁决 | 停车 |
| （引用）`KmoPresentation` | `_addTheme(KmoTheme)`、`getTmpPath()`、`mediaLib()` | `KmoPresentation.rs` 已有（`_addTheme` / `getTmpPath` / `mediaLib`，行号随读取确认） | 复用 |
| （引用）`PptFileCache.dumpFile(part, rootDir)` | 落盘返回路径 | `PptFileCache.rs` 已有 dumpTempFile_* 系列；对应 dumpFile 的等价映射**待 loop 确认** | 复用/待 loop 确认 |
| （引用）`OxfPptRelations` | theme/themeOverride relsType 常量 | 字符串常量（或 kfc 镜像） | 复用 |
| （引用）`IOAssert.assertNotNull` | 断言 themeRels 非空 | debug_assert 等价（行为锁定 null → 断错） | 本 function |

### 2.2 函数改写列表

- **本 function 改写：** `PptxrThemes` 全类 —— ctor、`open`（3 参委托 + 4 参主实现）、`dump`。
- **停车：** `PptxrThemeOverride.open`（themeOverride 类型分支；逻辑与 02 同构，是否并入 02 或另开
  **待 loop 确认**，未裁决前该分支 `todo!("DEFERRED_NON_FUNCTION")`）；`OpcrRelationship`/
  `OpcrPackagePart` 若 Rust 无镜像，用 fixture/字符串常量替身。
- **复用：** 02 `PptxrTheme`、`KmoTheme`（`setRid2MediaId`/`setThemePath`）、`PptFileCache`、
  `KmoPresentation._addTheme`；禁改这些既有实现。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读 `PptxrThemes.java` 全量（`open` 两 overload + `dump`）与以下参照：`PptxrTheme.java`
（02，`open`/`buildDefaultTheme`）、`PptxrThemeOverride.java`（themeOverride 分支对照）、
`OxfPptRelationsImpl`（relsType 字符串）、`KmoPresentation._addTheme`/`getTmpPath`/`mediaLib`。
当前 Rust `PptxrThemes.rs` 尚未生成（`show/pptxr/ppt/theme/` 目录不存在），loop 前以 Java 为准。
禁止只按标题冒烟。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | miss → 新 KmoTheme + put 缓存 | 状态 | 是 | T01 |
| C02 | hit → 同一实例（`ptr::eq`） | 状态 | 是 | T02 |
| C03 | addTheme=false → 不 `_addTheme` | 分支 | 是 | T03 |
| C04 | addTheme=true → `pres._addTheme` 被调（KmoPresentation.themes 增一） | 状态 | 是 | T04 |
| C05 | part==null → `buildDefaultTheme` | 边界 | 是 | T05 |
| C06 | theme relsType 分支 → PptxrTheme.open + setRid2MediaId/setThemePath | 正常 | 是 | T06 |
| C07 | 重复 dump 同一 partName → 走 `mGlobalThemesPath` 缓存 | 状态 | 是 | T07 |

### 3.3 可测试性

假 `OpcrRelationship`（`getPartName`/`getType`/`getPackagePart` 可编程）+ 假 part/内存 stream +
fixture callback；pres 用真实 `KmoPresentation`（`_addTheme`/`getTmpPath` 已实现，媒体假装）。
themeOverride 分支在 `PptxrThemeOverride` 未裁决前不测试（**待 loop 确认**）。不跑完整
`PptxReader`（本 function 不是 pp-09 编排项时）。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 新 partName（theme 类型）+ 可解析 part | 返回新实例；再次访问缓存 map 命中同一主题 | 红测停在本 function |
| T02 | C02 | 同一 partName 连续 open 两次 | 两次返回同一实例（`ptr::eq`） | 红测停在本 function |
| T03 | C03 | `open(..., addTheme=false)` | pres.themes 不增（Java `_addTheme` 未调等价） | 红测停在本 function |
| T04 | C04 | 默认 3 参 open（addTheme=true） | pres.themes 增一，元素即返回的 theme | 红测停在本 function |
| T05 | C05 | `getPackagePart()` 返回 None | 走 buildDefaultTheme 路径（02 语义），仍 put 缓存 | 红测停在本 function |
| T06 | C06 | theme relsType + fixture callback 记录 rid→id | theme 上 `mRid2MediaId` 与 `mThemePath` 已设置 | 红测停在本 function |
| T07 | C07 | 同 partName 两次 dump（第二次 part 可失效） | 第二次直接命中 `mGlobalThemesPath`，part 不被再次读取 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "03_function_themes_cache"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_themes_cache

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_themes_cache \
  --src src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs \
  --report libs/ppt-core/parser/features/pp-04-theme-part/functions/03_function_themes_cache/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_themes_cache --json`)
date: 2026-09-01 12:07

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs` | 99/103 | 96.1% | 151/160 | 94.4% |
| **function total** | **99/103** | **96.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrThemes.java
```

Loop 结果：PASS

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrThemes.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs \
  --report libs/ppt-core/parser/features/pp-04-theme-part/functions/03_function_themes_cache/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 04:07

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrThemes` | `koffice/presentation/core/cn/wps/show/pptxr/ppt/theme/PptxrThemes.java` | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/theme/PptxrThemes.rs` | **PASS** | `PptxrThemes`, `open`, `dump` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->