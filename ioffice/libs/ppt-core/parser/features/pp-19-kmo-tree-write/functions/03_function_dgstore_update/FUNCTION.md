# 03_function_dgstore_update — DgStore.add / update / query

> feature: pp-19-kmo-tree-write ｜ 状态：**pass** ｜ Java 管线：S4 步骤 4.3

## 0. 管线能力解锁

实现后：双 bin 的 dg 半边可测。`SwShapeHandler.endElement` 的 `store.update(id, dgShape.save())` 有仓。

2026-09-12：`drawing/app` 默认导出 `DgStore`/`DgStoreData`。`copyOf` 停车 `DEFERRED_NON_FUNCTION`。`KoBin` 走 `kso.kproto.KoBin`（`IntBin`/`Null`），禁止第二套 KoBin 名。`kso` 默认只开 `kproto::KoBin`；`KoZone`/`KoAr` 用叶旁 `kobin_hosts.rs`，不编 detached stub。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **add(type)** | 对应仓 `push(ShapeBin)`；返回 `helper_storeIndex2Id(type, len-1)` |
| **update(spid, bin)** | `helper_isGood` 才 `setBh(bin.asInt())`；否则空操作 |
| **query(spid)** | good → bin；否则 `KoBin.Null` |
| **非法 type** | `helper_storeByType` default → `IllegalArgumentException` / `panic!("IllegalArgumentException")` |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `DgStore.add/update/query` | 本 function | 六仓 | ✅ | copyOf | 禁止第二套 spid 公式 |
| `helper_isGood` | DgStoreData | index 在仓长内 | ✅ | — | — |
| 真 `KoBin` | kso.kproto | asInt / Null.value1 / IntBin.from | ✅ `kso.kproto.KoBin` | Encode/Factory/ByteArrayBin/RealBin/TextBin | 禁止项目 stub；禁止第二套 KoBin |

## 2. 核心代码与函数改写列表

- **本 function：** `add`/`update`/`query`/`clear`；`helper_isGood`；把本地 `KoBin` 换成协议类型（java2rust，保留 `// java`）。
- **停车：** `copyOf`（ShapeBin 无 Clone）。
- **禁改：** queryType 位域（01）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`DgStore.java:14-31`；`DgStore.rs` add/update/query；`helper_isGood`。

| symbol | 输入 | 输出 | 分支 |
|--------|------|------|------|
| add(1) | 空仓 | queryType==1 index0 | 首行 |
| add 二次 | 已有 1 | index1 | 递增 |
| add(0/2/3/4) | — | 各 type | 分仓 |
| update good | 已 add | 不 panic | setBh |
| update bad | index99 | 空操作 | !isGood |
| add(99) | — | panic IllegalArgument | default |

### 3.2 覆盖矩阵

| coverage_id | 必测? | test_id |
|-------------|-------|---------|
| DG-A1 | 是 | DG1 |
| DG-A2 | 是 | DG2 |
| DG-AT | 是 | DG3 |
| DG-UG | 是 | DG4 |
| DG-UB | 是 | DG5 |
| DG-ILL | 是 | DG6 |

### 3.3 可测试性

`DgStore::DgStore_new()`。默认编译面已含 `drawing::app::{DgStore,DgStoreData}` 与 `kso.kproto.KoBin`。DG1–DG6 全绿。

### 3.4 用例表

| id | 覆盖项 | 期望 | 失败语义 |
|----|--------|------|----------|
| DG1 | add type1 | queryType 1 | 未进仓 |
| DG2 | 二次 add | index 0 然后 1 | 覆盖首行 |
| DG3 | type 0,2,3,4 | 各 queryType | 进错仓 |
| DG4 | update 合法 | 不 panic | isGood 错 |
| DG5 | update 越界 | 不 panic | 误 panic |
| DG6 | add(99) | catch_unwind err | 吞掉非法 type |

### 3.5 测试代码位置

test: `tests/cn/wps/drawing/app/03_function_dgstore_update.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_dgstore_update -- --test-threads=1

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_dgstore_update \
  --src src/cn/wps/drawing/app/DgStore.rs \
  --src src/cn/wps/drawing/app/DgStoreData.rs \
  --src src/cn/wps/kso/kproto/KoBin.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/03_function_dgstore_update/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_dgstore_update --json`)
date: 2026-09-12 15:34

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/drawing/app/DgStore.rs` | 26/55 | 47.3% | 46/94 | 48.9% |
| `src/cn/wps/drawing/app/DgStoreData.rs` | 48/74 | 64.9% | 73/107 | 68.2% |
| `src/cn/wps/kso/kproto/KoBin.rs` | 4/121 | 3.3% | 6/145 | 4.1% |
| **function total** | **78/250** | **31.2%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/app/DgStore.rs \
  --java koffice/presentation/core/cn/wps/drawing/app/DgStore.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/drawing/app/DgStoreData.rs \
  --java koffice/presentation/core/cn/wps/drawing/app/DgStoreData.java
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs \
  --java koffice/presentation/core/cn/wps/kso/kproto/KoBin.java
```

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/drawing/app/DgStore.java \
  --rs libs/ppt-core/src/cn/wps/drawing/app/DgStore.rs \
  --java koffice/presentation/core/cn/wps/drawing/app/DgStoreData.java \
  --rs libs/ppt-core/src/cn/wps/drawing/app/DgStoreData.rs \
  --java koffice/presentation/core/cn/wps/kso/kproto/KoBin.java \
  --rs libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs \
  --report libs/ppt-core/parser/features/pp-19-kmo-tree-write/functions/03_function_dgstore_update/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-12 07:34

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `DgStore` | `koffice/presentation/core/cn/wps/drawing/app/DgStore.java` | `libs/ppt-core/src/cn/wps/drawing/app/DgStore.rs` | **PASS** | `graphicStore`, `query`, `add`, `update`, `clear`, `copyOf` | — | — | **PASS** |
| `DgStoreData` | `koffice/presentation/core/cn/wps/drawing/app/DgStoreData.java` | `libs/ppt-core/src/cn/wps/drawing/app/DgStoreData.rs` | **PASS** | `queryType`, `isGroup`, `isShape`, `isPicture`, `isConnector`, `isGraphicFrame`, `isTextShape`, `helper_storeIndex2Id`, `helper_getIndexById`, `helper_storeByType`, `helper_storeById`, `helper_isGood` | — | — | **PASS** |
| `KoBin` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `size`, `read`, `asInt`, `asKey` | — | — | **PASS** |
| `Encode` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `from` | `fromBH` | — | **PASS** |
| `Decode` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | — | `toBool`, `toInt`, `toDouble`, `toText`, `toBH` | — | **PASS** |
| `Factory` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | — | `wrap`, `toInt`, `toBool`, `toText`, `toDouble`, `fromBool`, `fromInt`, `fromText`, `fromDouble`, `dup` | — | **PASS** |
| `ByteArrayBin` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `size`, `read`, `asInt`, `asKey`, `from` | `wrap` | — | **PASS** |
| `IntBin` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `from`, `size`, `read`, `asInt`, `asKey` | — | — | **PASS** |
| `RealBin` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `from`, `size`, `read`, `asInt`, `asKey` | — | — | **PASS** |
| `TextBin` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `from` | — | — | **PASS** |
| `Null` | `koffice/presentation/core/cn/wps/kso/kproto/KoBin.java` | `libs/ppt-core/src/cn/wps/kso/kproto/KoBin.rs` | **PASS** | `value1`, `empty`, `equalsEmpty`, `notEmpty` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
