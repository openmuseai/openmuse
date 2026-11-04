# 01_function_doc_props — app/core 文档属性

> feature: pp-09-pptx-open ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：app/core 文档属性（application/company/title/creator/日期）在 part 存在时读入模型，readApp 先于 getPresPart、readCore 位于 pres open 之后（04 编排）。缺失时断在哪：打开后文档元数据缺失，产品闭环断在属性环节（渲染主链不受阻）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(OxfrAppPart/OxfrCorePart) -> AppProperty/CoreProperty

### 1.1 I/O 契约

| | 契约 |
|---|---|
| **输入** | `OxfrAppPart`（`OxfrPackage.getAppPart()`）与 `OxfrCorePart`（`OxfrPackage.getCorePart()`），无对应 part 时为 null。调用点在 `PptxReader.open`：`readApp` 先于 `getPresPart`，`readCore` 在 `mPresReader.open()` 之后（属 04 编排）。 |
| **输出** | `AppProperty`：application / appVersion / company / manager；`CoreProperty`：title / creator / lastModifiedBy / created / modified。日期经 `OpcUtil.getDateString` 写 `"yyyy-MM-dd'T'HH:mm:ss'Z'"`（UTC，见 `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/opcr/OpcUtil.java:119`）。 |
| **状态与副作用** | `part == null` 整体跳过（`if (part != null)`）。`part.parse()` 后逐个 getter：返回 null 的字段不 put（Java 每个 put 都包 `if (x != null)`）。`readApp` 的 parse Throwable 吞掉后**继续**读字段；`readCore` 的 parse Throwable 吞掉后**直接 return**（后续字段不读）。`readThumbsnail` 空体。 |
| **错误语义** | `part.parse()` 抛 `Throwable` 不向上抛（`IOLog.d` 记录后继续 / return），属吞掉语义；方法签名 `throws IOException` 但方法体无主动 throw，Rust 侧签名跟随模板，是否保留 Result 由 loop 决定。 |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| OxfrAppPart / OxfrCorePart parse 与 getter | OPC 层（pp-01 oxp 之上） | `getAppName/getAppVersion/getCompany/getManager/getTitle/getCreator/getLastModifiedBy/getCreated/getModified` | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |
| 调用点 `PptxReader.open` | 04 | readApp 在 getPresPart 前；readCore 在 pres.open 后 | 未接线 | 04 编排 | 本 function 不展开 package 打开 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/docprops/PptxrDocProps.java` |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs`（尚不存在，loop 用模板生成） |

关键方法/字段（对照 Java）：

| symbol | 语义（Java） | Rust 目标 |
|---|---|---|
| `readApp(AppProperty, OxfrAppPart)` static throws IOException | part!=null：`part.parse()`（Throwable 吞掉继续）；`getAppName()!=null → putApplication`；`getAppVersion()!=null → putAppVersion`；`getCompany()!=null → putCompany`；`getManager()!=null → putManager` | `Option<String>` 字段，null 不 put |
| `readCore(CoreProperty, OxfrCorePart)` static throws IOException | part!=null：`part.parse()`（Throwable 吞掉**并 return**）；title→putTitle；creator→putCreator；lastModifiedBy→putLastModifiedBy；created/modified→`OpcUtil.getDateString(d)`→putCreated/putModified | 同上；parse 失败后不再读任何字段 |
| `readThumbsnail(OxfrCorePart)` static | 空体 | 空体保留模板（不改） |

改写列表：`readApp`、`readCore`；`readThumbsnail` 空体保留。

停车列表：本类无停车项；`IOLog.d` 日志在 Rust 侧随 crate log 惯例处理（空实现或走 log，loop 决定）。
`PptxrCustom` 不属本 function（其调用点在 04）。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。禁止只按标题冒烟。
审计要点：

| symbol | 输入/状态 | 输出/副作用 | 分支/错误 |
|---|---|---|---|
| readApp | 非 null App part | appName / appVersion / company / manager 逐个写入 | part==null 跳过；parse Throwable 吞掉继续；每个 getter null 不 put（4 个独立分支） |
| readCore | 非 null Core part | title / creator / lastModifiedBy / created / modified 逐个写入 | part==null 跳过；parse Throwable 吞掉且 return（与 readApp 不同）；getter null 不 put；created / modified 走 `getDateString` |
| readThumbsnail | OxfrCorePart | 无 | 空体 |
| 调用点 | `PptxReader.open` | readApp 先于 getPresPart；readCore 在 pres.open 后 | 04 编排面（本 function 单测不启动 PptxReader） |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | null part 跳过 | 边界 | 是 | T01 |
| C02 | 各字段有/无（app 4 路 + core 5 路独立分支） | 分支 | 是 | T02 |
| C03 | parse Throwable 吞掉（readApp 继续读） | 失败 | 是 | T03 |
| C04 | readCore parse Throwable 吞掉后 return（与 readApp 差异） | 失败 | 是 | T04 |
| C05 | 日期 getDateString 格式 `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC | 输出 | 是 | T05 |

### 3.3 可测试性

`OxfrAppPart` / `OxfrCorePart` 用同路径 contract fixture（parse 成功 / 抛 Throwable；getter null / 有值两种形态），
驱动真实 `readApp` / `readCore`；不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。日期断言：实现与 Java
同格式字符串后直接断言具体串，格式细节先由 loop 对金文件确认。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | App / Core part 均 null | 不 put 任何字段、不抛错 | 红测停在本 function |
| T02 | C02 | app 4 字段有值；core 5 字段有值；另含 null 字段 | 有值全写入；null 字段不写 | 红测停在本 function |
| T03 | C03 | App part parse 抛 Throwable 但 getter 仍有值 | readApp 继续写入（Java 无 return） | 红测停在本 function |
| T04 | C04 | Core part parse 抛 Throwable | readCore 直接 return，字段全不写 | 红测停在本 function |
| T05 | C05 | created / modified 有值 | 按 `getDateString` 格式写出 | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "01_function_doc_props"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_doc_props

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_doc_props \
  --src src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/01_function_doc_props/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_doc_props --json`)
date: 2026-09-01 19:18

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs` | 116/119 | 97.5% | 253/267 | 94.8% |
| **function total** | **116/119** | **97.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/docprops/PptxrDocProps.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/docprops/PptxrDocProps.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs \
  --report libs/ppt-core/parser/features/pp-09-pptx-open/functions/01_function_doc_props/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:18

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxrDocProps` | `koffice/presentation/core/cn/wps/show/pptxr/docprops/PptxrDocProps.java` | `libs/ppt-core/src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs` | **PASS** | `readApp`, `readCore`, `readThumbsnail` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->