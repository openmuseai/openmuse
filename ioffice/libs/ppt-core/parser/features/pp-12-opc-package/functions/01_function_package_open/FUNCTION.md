# 01_function_package_open — OPC 包打开与 rel 定位

> feature: pp-12-opc-package ｜ 状态：**pass** ｜ Java 管线环节：S0 parse（物源桥接）

## 0. 管线能力解锁

实现后：真实 `.pptx` 的 path/RAF 第一次能以 Rust 句柄打开（`kfc_alpha` 的
`OxfrPackage`/`OpcrPackage`，与 Java `koffice/.../kfc_alpha` 同树），并按 WPS relsType 定位 officeDocument/app/core/custom 四类 part——产品闭环轨的
**物源输入**就绪。缺失时：整条「真实文件 → 模型 → 渲染」链在进入 pp-07/pp-09 之前即断，
`PptxReader.open` 只能用 fixture（主文档 §1-S0 结论：缺的是桥接而非 OPC 本身）。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | 文件路径（`String`）或 RAF（Java `PptxReader` 双 ctor → `new OxfrPackage(mFilePath)` / `new OxfrPackage(mRaf)`）；加密场景输入为已解密路径 |
| **输出** | `OxfrPackage`：保持 Java 类名，持有 wps-io `OPCPackage` 的桥接句柄 + 4 个 rel 定位入口（`getOfficeDocument`、`getAppPart`、`getCorePart`、`getCustomPart`）+ `close()` |
| **状态与副作用** | 打开即加载 zip 句柄、根 `_rels/.rels` 与 `[Content_Types].xml`（wave0 `07_function_opc_package` 语义）；生命周期由 `close()` 收尾（Java finally）；`getMediasSize`（zip 枚举 `/media/` 压缩尺寸和）可 stub |
| **错误语义** | Java ctor `throwable` → `FileDamagedException(t)` 由 **pp-09:04** 上抛；本 function 只返回「打开失败」（`Result`/Err，不 panic）。repair-file 重试分支（`OpcrPackage` ctor）defer |

### 1.2 输入
| 入口 | 输入 | Java 对标 |
|------|------|-----------|
| `OxfrPackage::open_path(path)` | 文件路径 | `new OxfrPackage(mFilePath)`（`OxfrPackage.java:17`） |
| `OxfrPackage::open_raf(raf)` | 随机访问句柄 | `new OxfrPackage(mRaf)`（`OxfrPackage.java:18`） |
| `getOfficeDocument()` | — | `OxfrPackage.getOfficeDocument`（`OxfrPackage.java:26-32`）：循环 `mRelses` 匹配 relsType `…/officeDocument/2006/relationships/officeDocument` → `rels.getPackagePart()` |
| `getAppPart/getCorePart/getCustomPart` | — | `OxfrPackage.java:39/49/59`：`OxfRelations.APP/CORE/CUSTOM.getRelsType()` 匹配 |
| `close()` | — | `PptxReader.close`（110-131）：`package.close()` 的 IOException 仅 printStackTrace |

### 1.3 输出
| 输出 | 说明 | Rust 载体 |
|------|------|-----------|
| `OxfrPackage` | 桥接句柄（含 `&mut OPCPackage` 借用入口） | `src/cn/wps/kfc/ooxfr/OxfrPackage.rs` |
| rel 定位结果 | `Option<Part>`（officeDocument/app/core/custom） | wps-io `getRelationshipsByType` + `getPart_rel` |
| 打开失败 | `Err(PptxOpenError)`（不抛给调用方） | `Result`（Java Throwable→FileDamaged 归属 pp-09:04） |

### 1.4 上游
wave0 `07_function_opc_package`（`OPCPackage::from_path/from_raf`，core-libs/io，pass）。

### 1.5 下游
02（pres part 定位）、03（part 流）、pp-09:04（PptxReader.open 的 package 输入与 close）。

### 1.6 不变量
1. 同一 `OxfrPackage` 下 officeDocument rel 至多一个有效 part（Java 取首个匹配，OxfrPackage.java:26 循环语义）。
2. `close()` 后句柄不可再用；`close` 幂等。
3. 不写回 package（只读打开，`PackageAccess.READ`）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| `OPCPackage::from_path/from_raf` | core-libs/io（wave0 07，pass） | 打开 + rels/CT 加载 | ✅ | — | 不重写 OPC |
| `getRelationshipsByType(String)` | core-libs/io `OPCPackage.java:430` 等价 | 按 officeDocument/app/core/custom 过滤 | ✅ | — | 不改 rels 语义 |
| `getPart_rel(&PackageRelationship)` | core-libs/io `OPCPackage::getPart_rel` | rel → part | ✅ | — | — |
| relsType 常量 | offDoc `OxfrPackage.java:27`；OxfRelations APP/CORE/CUSTOM | 常量表 | Java 有；Rust 待建 | — | 不发明新 type 名 |
| `zip` crate | ppt-core 依赖 | 底层解压（经 wps-io） | ✅ | — | — |

### 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java 权威 | `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java`（55 行）、`opcr/OpcrPackage.java`、`opcr/OpcrRelationships.java`（getRelationshipByType）、`show/pptxr/PptxReader.java`（open/close） |
| Rust 目标 | `libs/ppt-core/src/cn/wps/kfc/ooxfr/OxfrPackage.rs`（新建）＋ `cn/wps/kfc/ooxfr/mod.rs` 挂载；禁止另造 `PptxPackage` 类型 |
| 复用 | `wps-io OPCPackage`（wave0 07） |

| Java 成员 | 签名/值 | Rust 目标 |
|---|---|---|
| `OxfrPackage(String)` / `OxfrPackage(RandomAccessFile)` | throws IOException | `OxfrPackage::OxfrPackage/OxfrPackage__raf -> Result<Self, OxfrPackageError>` |
| `getOfficeDocument()` | 扫 rels 匹配 officeDocument relsType → part | `office_document() -> Option<Part>` |
| `getAppPart()/getCorePart()/getCustomPart()` | OxfRelations APP/CORE/CUSTOM → part | `app_part()/core_part()/custom_part()` |
| `type()` | 返回 1 | 壳（可省） |
| `getMediasSize()` | 枚举 zip `/media/` 压缩尺寸和 | stub（首屏不需要） |
| `close()` | `mZipFile.close()` | `close()`（IOException 仅 eprintln，Java printStackTrace 语义） |

改写列表：open_path/open_raf、office_document、app/core/custom part 定位、close。
停车：getMediasSize（`todo!("DEFERRED_NON_FUNCTION")`）、repair-file 分支、`setPptRelation`（OxfPptRelations，post-MVP）。

### 3. 测试方案

### 3.1 代码审计与测试面
读 `OxfrPackage.java`（全 55 行）、`PptxReader.open/close`（40-131 行）、wps-io `OPCPackage::from_path/from_raf/getRelationshipsByType/getPart_rel`。用内存 zip fixture（wave0 `FakeZipEntry` / `ZipEntrySource` 风格）构造：合法 pptx 形状（含 officeDocument rel）、无 officeDocument、空 zip、损坏字节。

| symbol | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|---|---|---|---|
| open_path | 合法/损坏/不存在路径 | Ok/Err | Throwable → Err（不 panic） |
| open_raf | 内存 RAF | Ok | 双入口等价 |
| office_document | 有/无 offDoc rel | Some/None | 首个匹配 |
| close | 正常/已 close | 幂等 | IOException 吞掉 |

### 3.2 覆盖矩阵
| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | open_path 合法 zip | 正常 | 是 | T01 |
| C02 | open_path 损坏/非 zip | 失败 | 是 | T02 |
| C03 | open_raf 等价 | 正常 | 是 | T03 |
| C04 | office_document 命中/未命中 | 分支 | 是 | T04 |
| C05 | app/core/custom 定位 | 正常 | 是 | T05 |
| C06 | close 幂等 + 吞异常 | 状态/清理 | 是 | T06 |
| C07 | getMediasSize | 壳 | 否（stub） | — |

### 3.3 可测试性
内存 zip fixture（`ZipEntrySource` trait 已有 `FakeZipEntry`），不依赖真实 pptx 文件；不做文件 IO 之外的平台调用。

### 3.4 用例表
| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | 合法内存 zip | Ok(Package) | 打开失败即红 |
| T02 | C02 | 损坏字节 | Err | panic 即红 |
| T03 | C03 | RAF 等价 | 与 path 同 | 双入口不一致 |
| T04 | C04 | 带/不带 offDoc rel | Some/None | rel 类型错 |
| T05 | C05 | APP/CORE/CUSTOM rels | 对应 part | 常量错 |
| T06 | C06 | close×2 | 无 panic/无重抛 | 二次异常 |

### 3.5 测试代码位置
```text
crate: libs/ppt-core/Cargo.toml
src:   libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.rs
       （ppt-core `src/cn/wps/kfc/ooxfr/OxfrPackage.rs` 再导出）
test:  libs/ppt-core/tests/cn/wps/kfc/ooxfr/OxfrPackage.rs
[[test]] name = "01_function_package_open"
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_package_open
cargo test --manifest-path libs/core-libs/kfc/kfc_alpha/Cargo.toml --test 01_function_package_open

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/core-libs/kfc/kfc_alpha --test 01_function_package_open \
  --src src/cn/wps/kfc/ooxfr/OxfrPackage.rs \
  --src src/cn/wps/kfc/opcr/OpcrPackage.rs \
  --src src/cn/wps/kfc/opcr/OpcrPackagePart.rs \
  --src src/cn/wps/kfc/opcr/OpcrRelationships.rs \
  --src src/cn/wps/kfc/opcr/OpcrRelationship.rs \
  --report libs/ppt-core/parser/features/pp-12-opc-package/functions/01_function_package_open/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_package_open --json`)
date: 2026-09-09 15:35

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/kfc/ooxfr/OxfrPackage.rs` | 74/103 | 71.8% | 124/162 | 76.5% |
| `src/cn/wps/kfc/opcr/OpcrPackage.rs` | 68/102 | 66.7% | 101/147 | 68.7% |
| `src/cn/wps/kfc/opcr/OpcrPackagePart.rs` | 12/45 | 26.7% | 14/65 | 21.5% |
| `src/cn/wps/kfc/opcr/OpcrRelationship.rs` | 53/119 | 44.5% | 75/182 | 41.2% |
| `src/cn/wps/kfc/opcr/OpcrRelationships.rs` | 58/99 | 58.6% | 100/165 | 60.6% |
| **function total** | **265/468** | **56.6%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.rs \
  --java koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java
```
Loop 结果：**PASS**（68 条 `// Java` 注释；Java method comments covered）。

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java \
  --rs libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.rs \
  --report libs/ppt-core/parser/features/pp-12-opc-package/functions/01_function_package_open/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-09 07:29

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `OxfrPackage` | `koffice/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.java` | `libs/core-libs/kfc/kfc_alpha/src/cn/wps/kfc/ooxfr/OxfrPackage.rs` | **PASS** | `OxfrPackage`, `type`, `getOfficeDocument`, `getRelationships`, `getAppPart`, `getCorePart`, `getCustomPart` | `getMediasSize` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
