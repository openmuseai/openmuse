# 04_function_pptx_reader — PptxReader.open

> feature: pp-09-pptx-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：整条打开链的总闸门——`synchronized` 下包构建（path|RAF）→ `getPresPart`（两条异常语义）→ 03 编排 → `readApp/readCore` → finally `close()`。真实 pptx 由此获得进入管线的唯一入口。缺失时断在哪：整链无入口，pptx 文件进不了管线，产品闭环断；捷径可用 fixture 绕过。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(path|RAF) -> package + getPresPart + close

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | 两个 ctor：`(KmoPresentation, String path, listener, passProvider, encrypted, dlProvider)` 或 `(KmoPresentation, RandomAccessFile, listener, passProvider, encrypted, dlProvider)`；`init` 记录 mPres / bEncrypted / mOpenListener / mPassProvider / mDlProvider。 |
| **输出** | `open()`（整体包在 `synchronized(PptxReadWriteLock.sLock)`，`src/cn/wps/show/PptxReadWriteLock.java:4`）：`mPres.setFileFormat(32)` → `OxfPptRelations.setPptRelation()` → `new OxfrPackage(raf|path)` → `Presentation.open(protocol)` → `readApp(appProperty, mPackage.getAppPart())`（01）→ `updateProtocol` → `getPresPart(mPackage)` → `new PptxrPresentation(...).open()`（03）→ `Presentation.open` → `readCore(coreProperty, mPackage.getCorePart())`（01）→ `PptxrCustom(...).open()`（stub）→ `updateProtocol` → `docDidFinishReadingEverything()` → `setReadOnly(false)` → finally `close()`。`setReadOnly(false)` 仅在正常路径执行。 |
| **状态与副作用** | finally `close()` 必跑：`mPresReader=null`；`mPackage!=null` → `mPackage.close()` → `mPackage=null`；`bEncrypted` → 删除 mFilePath 临时文件。`encrypted=true` 时 open 先 `PptxrFileDecrypt.decryptFile(mFilePath, mRaf, mPres, mPassProvider)` 且 `mRaf=null`（解密路径本 feature ignore）。OLE install：`mPres.getOleManager().install(new PptxOleManager(oriPath, mRaf, pwd, mPres.mediaLib()))` 调用点保留为空体（OLE 业务移出，PIPELINE.md §3）。 |
| **错误语义** | `new OxfrPackage(...)` 抛 `Throwable` → 包装 `FileDamagedException(t)`（不只是 getPresPart 抛）；`getPresPart`：`pack.getOfficeDocument()==null` → `FileDamagedException("Not OOXML file")`；`!part.getPartName().contains("ppt")` → `SuffixErrorException("Not pptx file")`。`close()` 内 `mPackage.close()` 的 `IOException` 仅 `printStackTrace`（不向上抛）。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| OxfrPackage 构建 / close | OPC 层 | `new OxfrPackage(raf|path)`、`getOfficeDocument`、`getAppPart/getCorePart/getCustomPart`、`close` | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| 01 `readApp` / `readCore` | 本 feature/01 | doc props | 未接线 | 01 | — |
| 03 `PptxrPresentation.open` | 本 feature/03 | initialize + slides | 未接线 | 03 | — |
| decrypt / OLE / custom | 移出 | 调用点 stub | 不做 | post-MVP | 不 fake 业务体 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs`（尚不存在，loop 用模板生成） |

关键方法/字段（对照 Java）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| 字段 | mPres / mFilePath / mRaf / bEncrypted / mOpenListener / mPassProvider / mDlProvider / mPackage / mPresReader | 同 Java；mFilePath 与 mRaf 二选一 |
| ctor×2 + `init` | path ctor 存 mFilePath；RAF ctor 存 mRaf；init 记其余 | 两条后端出口 |
| `open()` throws IOException | 见 1.1 输出列；synchronized(PptxReadWriteLock.sLock) 包整体 | 顺序必须与 Java 一致（Recording 锁定） |
| `close()` throws IOException | 见 1.1 状态列；package.close 失败仅 printStackTrace | 必跑（finally 语义由测试锁定） |
| `getPresPart(OxfrPackage)` | 两条异常语义 + 返回 part | 正常 / 两条失败分支 |
| `isLargestFile` / `currentSlide` | 恒 false / 空体 | 空实现保留模板 |

改写列表：两个 ctor + `init`、`open`、`close`、`getPresPart`；`isLargestFile` / `currentSlide` 空实现保留。

停车列表：`PptxrFileDecrypt.decryptFile`（解密全量 `#[ignore]`）、`PptxOleManager` / `install`（OLE 移出，调用点空体）、
`PptxrCustom.open`（解析移附录，调用点保序 stub）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。
审计要点：

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| ctor / init | path 或 RAF | 后端选择 | 两条 ctor 分支 |
| open 前缀 | encrypted 标志；setFileFormat(32) | decrypt（ignore）；package 构建 | OxfrPackage Throwable → FileDamagedException |
| open 主体 | package | readApp → getPresPart → pres.open → readCore → PptxrCustom(stub) → docDidFinishReadingEverything → setReadOnly(false) | officeDocument==null → FileDamaged("Not OOXML file")；partName 无 "ppt" → SuffixError |
| finally close | — | mPresReader=null；package.close；mPackage=null；encrypted 删临时文件 | package.close IOException 仅打印 |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | officeDocument null → FileDamaged | 失败 | 是 | T01 |
| C02 | partName 无 ppt → SuffixError | 失败 | 是 | T02 |
| C03 | 成功顺序 readApp→getPresPart→pres.open→readCore→(custom stub)→docDidFinishReadingEverything→setReadOnly(false) | 状态 | 是 | T03 |
| C04 | finally close package（mPackage.close + 置 null） | cleanup | 是 | T04 |
| C05 | path vs RAF 两条后端 | 后端 | 是 | T05 |
| C06 | OxfrPackage 构造 Throwable → FileDamagedException | 失败 | 是 | T06 |
| C07 | custom / viewProps / commentAuthors 移附录；decrypt ignore | ignore | 否 | T07 `#[ignore = "spec: post-MVP"]` |

### 3.3 可测试性

Mock / slot `OxfrPackage`（内存 fixture）驱动 open 主体；Recording `KmoPresOpenEventListener`
记录回调顺序；close 用可注入 package 断言 close 调用与 printStackTrace 语义；encrypted 分支整体 ignore
（不 fake decrypt）。FUNCTIONS.md **建议 MVP pass 口径**：必测是 getPresPart + 打开顺序 + close，
custom / viewProps / commentAuthors 从必测移到附录。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | getOfficeDocument()==null | FileDamagedException("Not OOXML file") | 红测停在本 function |
| T02 | C02 | partName 不含 "ppt"（如 "/word/document.xml"） | SuffixErrorException("Not pptx file") | 红测停在本 function |
| T03 | C03 | 内存 fixture path 后端 | Recording：readApp→getPresPart→pres.open→readCore→custom(stub)→docDidFinishReadingEverything→setReadOnly(false) | 红测停在本 function |
| T04 | C04 | 正常路径 + package.close 抛 IOException | close 必跑且仅 printStackTrace（不向上抛） | 红测停在本 function |
| T05 | C05 | 同一份 bytes 走 path 与 RAF 两条后端 | 两次结果（含回调顺序）一致 | 红测停在本 function |
| T06 | C06 | OxfrPackage 构造抛 Throwable | open 抛 FileDamagedException（包装原生异常） | 红测停在本 function |
| T07 | C07 | custom part / viewProps / commentAuthors rel 存在 | 调用点 stub 不解析；测试标 ignore | — |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/PptxReader.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "04_function_pptx_reader"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_pptx_reader

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_pptx_reader \
  --src src/cn/wps/show/pptxr/PptxReader.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/04_function_pptx_reader/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_pptx_reader --json`)
date: 2026-09-01 19:29

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/PptxReader.rs` | 137/168 | 81.5% | 369/427 | 86.4% |
| **function total** | **137/168** | **81.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java
```

Loop 结果：（未跑）

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/04_function_pptx_reader/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:29

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxReader` | `koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java` | `libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs` | **PASS** | `PptxReader`, `init`, `open`, `isLargestFile`, `currentSlide`, `close`, `getPresPart` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->