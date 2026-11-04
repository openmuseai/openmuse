# 02_function_pres_part_locate — pres part 定位与后缀校验

> feature: pp-12-opc-package ｜ 状态：**pass** ｜ Java 管线环节：S0 parse（物源桥接）

## 0. 管线能力解锁

实现后：`PptxReader.getPresPart` 的逐句语义在 Rust 落地——`FileDamaged("Not OOXML file")` /
`SuffixError("Not pptx file")` / 正常返回 pres part 三分支精确对齐。缺失时：pp-09:04 的
getPresPart 必测项没有实现载体，非法文件会静默进入后续解析或 panic（Java 有显式失败语义）。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `&PptxPackage`（01 的输出：已打开包 + officeDocument rel 定位） |
| **输出** | `Result<PackagePart, PresPartError>`：`FileDamaged("Not OOXML file")` ｜ `SuffixError("Not pptx file")` ｜ Ok(part) |
| **状态与副作用** | 只读；不缓存、不改 package 状态（Java 每 open 调一次 getPresPart，随后交给 PptxrPresentation） |
| **错误语义** | Java `PptxReader.getPresPart`（134-141）：`pack.getOfficeDocument()` 为 null → `new FileDamagedException("Not OOXML file")`；`!part.getPartName().contains("ppt")` → `new SuffixErrorException("Not pptx file")`；否则返回 part。Rust 用 `Result` 表达同三分支，消息逐字对齐 |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `get_pres_part(&self)` | officeDocument rel 定位结果 | `PptxReader.getPresPart(pack)`（`PptxReader.java:134-141`） |

### 1.3 输出
| 输出 | 说明 | 载体 |
|------|------|------|
| Ok(part) | officeDocument 定位且 partName 含 `ppt` | wps-io `PackagePart` |
| `FileDamaged("Not OOXML file")` | officeDocument==null（Java 原句） | `PresPartError::FileDamaged` |
| `SuffixError("Not pptx file")` | partName 不含 `ppt`（Java 原句） | `PresPartError::SuffixError` |

### 1.4 上游
01（PptxPackage + officeDocument 定位）。

### 1.5 下游
pp-09:04（PptxReader.open：setPptRelation 后 getPresPart → 交给 PptxrPresentation.open）。

### 1.6 不变量
1. 三分支互斥且可机械断言（测试直接比 `Result` 变体 + 消息串）。
2. partName 校验用 `contains("ppt")`（Java 语义，不是后缀 equals）。
3. 不改变失败语义的「更安全 Rust」（Java 无防御时也不发明）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| PptxPackage::office_document | 01 | Option<part> | spec | — | 不绕过 01 重开包 |
| PackagePart::getPartName | wps-io | partName 字符串 | ✅ | — | — |
| `FileDamagedException`/`SuffixErrorException` | moffice/crash（Java 语义源） | 双错误变体 | Rust 待建（`PptxOpenError` 同一个枚举可挂） | — | 不命名成别的错误 |

### 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java`（getPresPart 134-141） |
| Rust 目标 | `libs/ppt-core/src/cn/wps/show/pptxr/opc/PptxPackage.rs`（`get_pres_part`） |

| Java 成员 | 签名/值 | Rust 目标 |
|---|---|---|
| `getPresPart(OxfrPackage)` | `throws IOException` | `get_pres_part(&self) -> Result<PackagePart, PresPartError>` |
| `FileDamagedException("Not OOXML file")` | officeDocument==null | `Err(PresPartError::FileDamaged("Not OOXML file"))` |
| `SuffixErrorException("Not pptx file")` | partName 无 "ppt" | `Err(PresPartError::SuffixError("Not pptx file"))` |
| `getOfficeDocument()` | 复用 01 | `self.office_document()` |

改写列表：get_pres_part（三分支）；错误枚举 PresPartError。
停车：无（本 function 体量小、无跨 function 停车点）。

### 3. 测试方案

### 3.1 代码审计与测试面
读 `PptxReader.java` 134-141 行原文；wps-io `PackagePart::getPartName`。fixture：内存 zip 三种变体——①无 officeDocument；②officeDocument 指向 `ppt/slides/slide1.xml`；③officeDocument 指向 `word/document.xml`（违反 "ppt" 校验）。

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | officeDocument==null | 失败 | 是 | T01 |
| C02 | 合法 pres part | 正常 | 是 | T02 |
| C03 | partName 不含 "ppt" | 失败 | 是 | T03 |
| C04 | 消息串逐字对齐 | 契约 | 是 | T04 |

### 3.3 可测试性
内存 zip fixture 三变体 + `PptxPackage`；无 IO 外依赖。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 无 offDoc rel 的 zip | `Err(FileDamaged("Not OOXML file"))` | 变体/消息错 |
| T02 | C02 | `ppt/…` offDoc | `Ok(part)` | 误报异常 |
| T03 | C03 | `word/…` offDoc | `Err(SuffixError("Not pptx file"))` | 变体/消息错 |
| T04 | C04 | 消息串 | 与 Java 字面量逐字相等 | 漂移 |

### 3.5 测试代码位置
```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/opc/pres_part.rs
[[test]] name = "02_function_pres_part_locate"
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 02_function_pres_part_locate

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 02_function_pres_part_locate \
  --src src/cn/wps/show/pptxr/PptxReader.rs \
  --report libs/ppt-core/parser/features/pp-12-opc-package/functions/02_function_pres_part_locate/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_pres_part_locate --json`)
date: 2026-09-01 19:14

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/PptxReader.rs` | 15/29 | 51.7% | 19/33 | 57.6% |
| **function total** | **15/29** | **51.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java
```
Loop 结果：**PASS**（`check-template-layout.mjs --rs PptxReader.rs --java PptxReader.java`）

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs \
  --report libs/ppt-core/parser/features/pp-12-opc-package/functions/02_function_pres_part_locate/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 11:14

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PptxReader` | `koffice/presentation/core/cn/wps/show/pptxr/PptxReader.java` | `libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs` | **PASS** | `open`, `getPresPart` | `PptxReader`, `init`, `isLargestFile`, `currentSlide`, `close` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->