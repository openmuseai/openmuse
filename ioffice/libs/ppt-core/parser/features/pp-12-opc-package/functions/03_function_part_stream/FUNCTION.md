# 03_function_part_stream — part → 字节流

> feature: pp-12-opc-package ｜ 状态：**pass** ｜ Java 管线环节：S0 parse（物源桥接）

## 0. 管线能力解锁

实现后：part（或 partName）→ 解压字节流，pp-07 `UnifiedXmlReader.read(stream, handler)` 与
pp-09 doc props（`readApp/readCore` 吃 app/core part）的**输入落地**。缺失时：pp-07:01/pp-09:01
的「InputStream」只有 fixture 没有真流，闭环 reader 无物可读。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | part 引用（`PackagePart`，wps-io）或 partName；`isOriginInput` 标志（Java `OpcrPackagePart.getInputStream(boolean)` 重载） |
| **输出** | 字节流（`Read`/`Cursor<Vec<u8>>`）：`isOriginInput=false` 走解压读；`true` 走 zip 原始（未解压）输入流（Java `OxfrPackage.getInputStream(mZipFile, mZipEntry, isOriginInput)`） |
| **状态与副作用** | 只读；每次调用新建流，不缓存（Java 同）；`getPartName()` 由 part 内省 |
| **错误语义** | Java `throws IOException`（zip entry 缺失/损坏 → 打开流失败）；Rust 以 `io::Result` 表达；缺失 entry → `Err(NotFound)` |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `part.input_stream(is_origin) -> io::Result<Box<dyn Read>>` | part | `OpcrPackagePart.getInputStream(bool)`（`OpcrPackagePart.java:22-27`） |
| `package.part_stream(part_name, is_origin)` | partName（如 app/core rel 定位后） | 等价路径 |
| `getPartName()` | — | `OpcrPackagePart.getPartName()`（`OpcrPackagePart.java:56`） |

### 1.3 输出
| 输出 | 说明 | 载体 |
|------|------|------|
| 解压流 | zip entry 解压字节（XML part 的主流路径） | `Cursor<Vec<u8>>` / `Read` |
| 原始流 | zip entry 未解压（`isOriginInput=true`，Java Oxfr 走原始字节） | 同上 |
| io::Result 错误 | entry 缺失/损坏 | `io::Error` |

### 1.4 上游
01/02（PptxPackage + pres part）；wps-io `ZipEntrySource`（entry 字节源，wave0 02）。

### 1.5 下游
pp-07:01（UnifiedXmlReader：part 流 → XML 事件）；pp-09:01（`readApp(appPart)`/`readCore(corePart)`）。

### 1.6 不变量
1. 流字节 == 对应 zip entry 解压字节（用 `FakeZipEntry` 已知字节断言）。
2. `isOriginInput` 两分支语义与 Java 一致（`OpcrPackagePart.java:22`：`getInputStream(false)` 默认）。
3. 不缓存流（每次新建），不写回。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `PackagePart`（entry 句柄） | wps-io | 名称/内容类型/entry 引用 | ✅ | — | — |
| `ZipEntrySource::getInputStream` | wps-io（wave0 02） | entry → 字节 | ✅（`FakeZipEntry` 已有） | — | 不改解压 |
| `OpcrPackagePart.getInputStream` | Java 语义源 | 布尔重载 | 待 loop | — | 不发明 isOrigin 默认值 |

### 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/opcr/OpcrPackagePart.java`（22-27/56）、`ooxfr/OxfrPackage.java` |
| Rust 目标 | `libs/ppt-core/src/cn/wps/show/pptxr/opc/PptxPackage.rs`（part_stream 方法） |

| Java 成员 | 签名/值 | Rust 目标 |
|---|---|---|
| `getInputStream(boolean isOriginInput)` | throws IOException；`OxfrPackage.getInputStream(mZipFile, mZipEntry, flag)` | `input_stream(&self, is_origin: bool) -> io::Result<…>` |
| `getInputStream()` | 默认 false | `input_stream(false)` 便捷 |
| `getPartName()` | 字符串 | part 内省 |
| `OxfrPackage.getInputStream(ZipFile, ZipEntry, boolean)` | 解压/原始分发 | wps-io `ZipEntrySource::getInputStream` 语义复用 |

改写列表：input_stream（两分支）、part_name 内省。
停车：无。

### 3. 测试方案

### 3.1 代码审计与测试面
读 `OpcrPackagePart.java` 22-27/56、`OxfrPackage.java` 的 `getInputStream`（静态工具，grep 定位）；wps-io `ZipEntrySource`/`FakeZipEntry::getInputStream`。fixture：已知字节 entry（如 `ppt/slides/slide1.xml` = b"<p:sld/>"），断言流字节逐位相等；缺 entry 走 Err。

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | 解压流字节 == entry 字节 | 正常 | 是 | T01 |
| C02 | isOriginInput=true 分支 | 分支 | 是 | T02 |
| C03 | 缺 entry | 失败 | 是 | T03 |
| C04 | 空 entry / 零长 | 边界 | 是 | T04 |
| C05 | getPartName 内省 | 正常 | 是 | T05 |

### 3.3 可测试性
`FakeZipEntry::new(name, bytes)` 内存构造；无文件 IO。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | entry=b"<p:sld/>" | 流读==同字节 | 解压/错位 |
| T02 | C02 | is_origin=true | 原始字节语义一致 | 分支反转 |
| T03 | C03 | 不存在 entry | Err(NotFound) | 静默空流 |
| T04 | C04 | 零长 entry | 空流 Ok | 误错 |
| T05 | C05 | part 名 | 与 entry 名一致 | 内省错 |

### 3.5 测试代码位置
```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/kfc/ooxfr/OxfrPackage.rs
test:  libs/ppt-core/tests/cn/wps/kfc/ooxfr/OxfrPackage_part_stream.rs
[[test]] name = "03_function_part_stream"
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_part_stream

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_part_stream \
  --src src/cn/wps/kfc/ooxfr/OxfrPackage.rs \
  --report libs/ppt-core/parser/features/pp-12-opc-package/functions/03_function_part_stream/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_part_stream --json`)
date: 2026-09-01 17:10

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/kfc/ooxfr/OxfrPackage.rs` | 33/76 | 43.4% | 49/109 | 45.0% |
| **function total** | **33/76** | **43.4%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/kfc/ooxfr/OxfrPackage.rs \
  --java koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java
```
Loop 结果：**PASS** (`OxfrPackage.rs` ↔ `OxfrPackage.java`)

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java \
  --rs libs/ppt-core/src/cn/wps/kfc/ooxfr/OxfrPackage.rs \
  --report libs/ppt-core/parser/features/pp-12-opc-package/functions/03_function_part_stream/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 09:10

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `OxfrPackage` | `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java` | `libs/ppt-core/src/cn/wps/kfc/ooxfr/OxfrPackage.rs` | **PASS** | `OxfrPackage`, `type`, `getOfficeDocument`, `getRelationships`, `getAppPart`, `getCorePart`, `getCustomPart` | `getMediasSize` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->