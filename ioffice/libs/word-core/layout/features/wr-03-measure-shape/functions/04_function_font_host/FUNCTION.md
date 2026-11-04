# 04_function_font_host — 族名映射与注入表（Java FontHost）

| 项 | 值 |
|----|----|
| feature | wr-03-measure-shape |
| order | 4 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `FontHost::from_faces` / `add_font_map_scheme` / `resolve_actual_name` / `into_font_stack` / `set_font_table` / `get_font_table` |
| 独立性 | 不测 RunMetrics、不断行；零系统字体依赖（只用 TestFont fixture） |
| 测试状态 | 8/8 pass（2026-10-05）；完整 Java 门面/布局验收 FAIL，见 §3.9–§3.11 |
| 计划 | [FONT_HOST_DEV_PLAN.md](../../../FONT_HOST_DEV_PLAN.md) |

## 1. 功能说明

### 1.1 一句话

把 Java `FontHost` + `FontTool.addFontMapScheme` + `TypefaceFactory.createTypeface` 的 **I/O** 落到 Rust：宿主注入的 `(id, TTF bytes)` 建成表；docx 显示名经 `SchemaItem` 映到实际族；再交给已有 `FontStack`（ttf-parser）。**不含** 扫盘缓存、DelayFontTable、Paint sticky。

### 1.2 输入

| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `from_faces(faces)` | 非空 `InjectedFace[]`（id + bytes + collection_index） | `FontHost.loadCache` 之后表里已有的族文件；宿主 ≈ `Platform.getAllFontsDirectory` 的结果 |
| `add_font_map_scheme(schemes)` | `SchemaItem{display, actual, selectable}` | `FontTool.addFontMapScheme(FontHostSettings.SchemaItem[])` |
| `resolve_actual_name(docx_name)` | run/rFonts 名字或 None | 显示名 → 实际族名 → 表命中；否则默认族 |
| `into_font_stack()` | 本表 | `TypefaceFactory.createTypeface` → `ITypeface`（此处为 `FontStack`） |
| `set_font_table/get_font_table` | 可替换的进程 `Arc<FontStack>` | Java `cn.wps.font.FontHost.setFontTable/getFontTable` 的静态表赋值/读取；用于缺字重试 |

### 1.3 输出

| 字段/副作用 | 含义 |
|------|------|
| `default_font_id` | 第一份注入脸的 id（产品 `DrawText.font_id`） |
| `FontStack` | 默认 Face + name_map（实际名与 scheme 显示名均可 `glyph_advance_twip`） |
| 失败 | 空表 / 坏 TTF → `FontError`（Io/Parse） |

### 1.4 上游

- wr-03 `01_function_font_stack`（`FontStack::from_font_bytes` / `register_name_bytes`）
- 宿主注入（Flutter / 测试）；**内核不探盘**

### 1.5 下游

`layout_document` 建栈；日后 rFonts 多族走 `resolve_actual_name`（本 function 只保证映射 I/O）。

### 1.6 关键不变量

1. `from_faces([])` 必须 `Err`，不得 silently 探系统 Arial。
2. 未知显示名 → 默认 Face（与 `FontStack` FS5 一致，不 panic）。
3. scheme 命中且实际族在表中 → 用实际族字节度量。
4. `collection_index` 传给 `Face::parse`（MVP 测试用 0）。
5. 热路径不调用 `FontStack::default()`。
6. 后续字体注入必须替换进程表；已取得的 `Arc` 读者仍可完成当前布局。Java `sFontTable` 可重新赋值，Rust 不得用 `OnceLock::set` 丢弃第二次注入。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------|------|------|------|
| `FontStack` | wr-03-01 | from_font_bytes、register_name_bytes、glyph_advance_twip | **本轮补 register_name_bytes** | 粗斜多 Face | 不改 hmtx 公式 |
| `ttf-parser` | crates.io | Face::parse(index) | 已有 | — | — |
| `FontHost.java` 全量 | thirdparty/font | 扫盘 / delay / 水印 | — | **暂缓** | 不 stub `cn.wps.font` |
| 精华文案 TTF | 产品资源 | JINGHUA scheme | — | 无文件不注册 | 不硬编码无文件映射当度量源 |

### 1.8 平台 so / 系统字体

度量仍 ttf-parser。系统 Arial 只允许**宿主读字节后注入**。CI 只用 fixture。

## 3. 测试方案

### 3.2 覆盖矩阵

| 覆盖项 | 说明 | 必测 | 用例 |
|--------|------|------|------|
| FH-EMPTY | 空 faces → Err | 是 | FH1 |
| FH-LOAD | 单 fixture → default_id + 可度量 | 是 | FH2 |
| FH-SCHEME | 显示名映到表内实际名 | 是 | FH3 |
| FH-FALLBACK | 未知名 / 空名 → 默认 | 是 | FH4 |
| FH-MULTI | 第二份 id 命中 name_map | 是 | FH5 |
| FH-BAD | 非法 bytes → Parse | 是 | FH6 |
| FH-REINJECT | 进程表替换、旧读者生命周期 | 是 | FH8 |

### 3.3 可测试性

零平台依赖：`tests/fixtures/01_function_font_stack/TestFont.ttf`。不读 `/System/Library/Fonts`。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|----|--------|----------|------|----------|
| FH1 | FH-EMPTY | `from_faces([])` | Err | 误 Ok 并探盘 |
| FH2 | FH-LOAD | 一份 TestFont，id=`TestFont` | default_id=TestFont；'A' advance=hmtx | 建表失败 |
| FH3 | FH-SCHEME | scheme 宋体→TestFont | resolve(宋体)=TestFont；advance 同默认 | 映射未进 name_map |
| FH4 | FH-FALLBACK | resolve(None / "没有") | default_id | panic 或空 |
| FH5 | FH-MULTI | 两份同文件不同 id | 第二 id `name_hits_file` | 只留第一份 |
| FH6 | FH-BAD | bytes=`[0,1,2]` | Parse Err | 误当默认 |
| FH8 | FH-REINJECT | first Arc → set second Arc | get 返回 second；old_reader 仍可读 first | `OnceLock` 首次表污染重试 |

### 3.5 测试代码位置

crate: `libs/word-core/Cargo.toml`  
src:   `libs/word-core/src/cn/wps/moffice/writer/layout/text/FontHost.rs`  
test:  `libs/word-core/tests/cn/wps/moffice/writer/layout/text/FontHost.rs`  
fixture: `libs/word-core/tests/fixtures/01_function_font_stack/TestFont.ttf`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 04_function_font_host

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 04_function_font_host \
  --src src/cn/wps/moffice/writer/layout/text/FontHost.rs \
  --report libs/word-core/layout/features/wr-03-measure-shape/functions/04_function_font_host/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_font_host --json`)
date: 2026-10-05 11:09

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/text/FontHost.rs` | 80/95 | 84.2% | 122/136 | 89.7% |
| **function total** | **80/95** | **84.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
# FontHost.rs 是管线胶水；本轮对真实 Java FontHost.java 执行完整检查，
# 保留失败结论，不能把早期无 --java 的 PASS 外推为门面对齐。
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/text/FontHost.rs \
  --java koffice/core-libs/thirdparty/font/src/cn/wps/font/FontHost.java
```

2026-10-05：**FAIL**。`setFontTable` 的赋值语义已经对齐并由 FH8/产品重试验证，但 Rust 文件仍是 Writer 适配胶水，Java `FontHost` 的构造、缓存/系统字体/延迟加载等方法及逐句注释未移植。原 2026-08-23 仅 `--rs` 的 PASS 不满足当前完整性门禁。

### 3.9 Java↔Rust 门面对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/core-libs/thirdparty/font/src/cn/wps/font/FontHost.java \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/text/FontHost.rs \
  --report libs/word-core/layout/features/wr-03-measure-shape/functions/04_function_font_host/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-10-05 03:10

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FontHost` | `koffice/core-libs/thirdparty/font/src/cn/wps/font/FontHost.java` | `libs/word-core/src/cn/wps/moffice/writer/layout/text/FontHost.rs` | **FAIL** | `getFontTable` | — | `FontHost`, `initChannelSettings`, `initCache`, `forceUpdateCache`, `loadCache`, `getFonts`, `getSystemFonts`, `getSystemFontList`, `getCustomFonts`, `getCustomFontList`, `getSDCardFontList`, `getCloudFontList`, `initWaterFonts`, `loadFontCache`, `asyncLoadFontCache`, `waitLoadFontCache`, `delayLoadFontCache`, `finishLoadFontCache`, `closeHoldNativePtr` | **FAIL** |

**overall: FAIL** — basename must match; layout must PASS; every Java public method must exist in Rust (implemented or `todo!("DEFERRED_NON_FUNCTION")`).
<!-- java-rust-align:end -->

### 3.10 管线逻辑对照

| Java 步骤 | Rust 步骤 | 本轮结果 |
|---|---|---|
| `FontHost.setFontTable` 覆盖 `sFontTable` | `set_font_table` 写入 `RwLock<Option<Arc<FontStack>>>` | FH8 与冷启动缺字重试通过 |
| `FontHost.getFontTable` 读当前表 | `get_font_table` 克隆当前 Arc | WriterTypeface 真正消费；旧读者安全 |
| `FontFamilyImpl → TypefaceFactory → AndroidTypeface` 真实源面/advance | `WriterTypeface` 适配路径 | **FAIL**：同文档 184/226 行宽不同，仍属 S08 |

### 3.11 管线阶段对齐报告

| 项 | 内容 |
|----|------|
| 阶段 | layout |
| 原始文档 | `sample2.docx` SHA-256 `eb4e1466d75ea7856b62f332da9864ada599f440da9a38ef0e55feec754bafd3` |
| 产品接线 | DOCX → `WordSession` → `LayoutDocument.java_pipeline_pages` → `FontHost.set_font_table` → `WriterTypeface` → FFI/Flutter |
| 测试 | FH1–FH8 8/8；冷进程应用字体重试 8 页/187 命令；Java/Rust 226/226 CP，width 42/226 |
| 关键路径真实实现 | 替换表真实；字体源面与 `AlignBothHandler` 尚未完整实现 |
| 判定 | **FAIL**，不能将本 function 或整个 M3-09 标为完整 Java 逻辑/门面通过 |
