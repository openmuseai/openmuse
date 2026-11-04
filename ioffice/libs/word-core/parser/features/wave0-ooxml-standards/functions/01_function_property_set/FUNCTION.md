# 01_function_property_set — 属性桶读写与 freeze

| 项 | 值 |
|----|----|
| feature | wave0-ooxml-standards |
| order | 1 |
| crate | `libs/core-libs/util/util/Cargo.toml` |
| 入口 | `MutablePropertySet::{setInt,setValue,…,freeze}` / `PropertySet::getInt` |
| 独立性 | 叶子（**无 zip 依赖，应最先测**） |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

以 `propId → Object` 为契约的扁平桶：可变门面写入，`freeze()` 交出不可变 `PropertySet` 供共享。

### 1.2 输入

整数 `propId` + 值（int/float/bool/string/object）。无 XML。

### 1.3 输出

`PropertySet`：`getInt/getFloat/getString/getBoolean/getValue/contains`；`PropertySet.empty` 兜底。

### 1.4 上游

无包依赖。

### 1.5 下游

`10_function_handler_props`；全部 `wc-*` 存储/样式/CHPX。调用方：`SubDocument` papx `setInt(193,1)`、`TextDocument` freeze 发布。

## 2. 核心代码与流程

| 名字 | 角色 |
|------|------|
| `PropertySet` | 不可变桶，`BUCKET_SIZE=8`，`FREEZE_MASK=1073741824` |
| `MutablePropertySet` | 可变门面，写走 `_setValue` |
| `SetPropertyMode` | compose：NOP=0 REPLACE=1 MERGE=2 CLEAR=3 |

```mermaid
flowchart LR
    M[MutablePropertySet] -->|setInt(id,v)| M
    M -->|freeze| P[PropertySet]
    P -->|getInt(id)| V[value]
```

深读：[property-deep-dive.md](../../docs/property-deep-dive.md)。propId 是内核契约不是 OOXML 编号。

## 3. 测试方案

### 3.1 代码审计与测试面

Java：`koffice/core-libs/util/util/src/cn/wps/moffice/property/{PropertySet,MutablePropertySet}.java`  
Rust：`libs/core-libs/util/util/src/cn/wps/moffice/property/{PropertySet,MutablePropertySet}.rs`

| symbol | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|-----------|-------------|---------------|
| `MutablePropertySet::setInt/setFloat/setBoolean/setString` | propId+primitive | `_setValue` 装箱 | 曾丢值 stub |
| `freeze` | 未冻/已冻 | `_freeze` + 返回 data（Rust clone） | 冻后再写抛 `PropertyException` |
| `getInt` / 带默认 | 有值 / 缺 id | 值 / 默认 / panic | 无默认缺 id 抛 `No such property` |
| `_setValue` | 新槽/覆盖/remove(None) | 维护 mCount | frozen 抛 |
| `add` / `addLimit` | 另一桶 | 后写覆盖；`[b,e)` | 空源 no-op |
| `compose` mode 0–3 / default | old+new | NOP/REPLACE/MERGE/CLEAR/fallback old | CLEAR 删 new 的 key |
| `merge` / `intersect` | 两桶 | 并/交（值相等） | 任一侧 mMap null → empty |
| `remove` / `clear` / `dispose` | 已有槽 | size=0 | clear 在 frozen 抛 |
| `empty` / `createEmpty` / `valueOf` | 无 / id+值 | 已冻桶 | |
| `equals` / `isObjectEqual` | 两桶或两值 | 内容相等 | 装箱 Arc 须按 primitive 值比 |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| C-SET-INT | setInt roundtrip | 主路径 | 是 | PS1 |
| C-MISS | 缺 id contains/getValue/default | 边界 | 是 | PS2 |
| C-FREEZE | freeze isFrozen + getAll ids | 状态 | 是 | PS3 |
| C-TYPED | float/bool/string | 主路径 | 是 | PS4 |
| C-OVERWRITE | 同 id 覆盖 size=1 | 分支 | 是 | PS5 |
| C-REMOVE | remove | 状态 | 是 | PS6 |
| C-CLEAR | clear 多槽 | 状态 | 是 | PS7 |
| C-BUCKET | id 0/7/8/193 跨桶 | 循环 N | 是 | PS8 |
| C-DEFAULT | MPS 带默认 getter | 边界 | 是 | PS9 |
| C-EMPTY | createEmpty + static empty | 主路径 | 是 | PS10 |
| C-VALUEOF | valueOf / ctor | 主路径 | 是 | PS11 |
| C-ADD-WIN | add 后写覆盖 | 分支 | 是 | PS12 |
| C-ADD-MERGE | add 不同 key | 主路径 | 是 | PS13 |
| C-ADDLIMIT | addLimit [0,8) | 边界 | 是 | PS14 |
| C-COPY | new_from_ps / mutable | 主路径 | 是 | PS15 |
| C-CLONE | clone_mps 独立 | 状态 | 是 | PS16 |
| C-COMPOSE | compose 0/1/2/3/default | 分支 | 是 | PS17 |
| C-MERGE | static merge | 主路径 | 是 | PS18 |
| C-GETALL | getAll + values | 主路径 | 是 | PS19 |
| C-CONTAIN | contain empty/self/other | 分支 | 是 | PS20 |
| C-DISPOSE | dispose | 资源 | 是 | PS21 |
| C-DIFF | differenceSet | 分支 | 是 | PS22 |
| C-INTERSECT | intersect 共享 key | 分支 | 是 | PS23 |
| C-EQUALS | equals 同/异 int | 主路径 | 是 | PS24 |
| C-HASH | hashCode 稳定 | 状态 | 是 | PS25 |
| C-GET-PANIC | getInt 缺 id | 失败 | 是 | PS26 |
| C-FRZ-WRITE | freeze 后再 setInt | 失败 | 是 | PS27 |
| C-NEW-EMPTY | new() 空 | 边界 | 是 | PS28 |
| C-GETOBJ | getObject | 主路径 | 是 | PS29 |
| C-ISEQ | isPropertyEqual / isObjectEqual | 主路径 | 是 | PS30 |

`isArrayEqual` / `primitiveArrayEqual`：本 function I/O 不经数组值；未覆盖，非必测。

### 3.3 可测试性

直接 `MutablePropertySet::new()`，无 zip/XML。propId `193` 与 papx 调用方相同。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入摘要 | 期望 | 失败语义 |
|----|--------|------|----------|------|----------|
| PS1 | C-SET-INT | new | setInt(193,1)+freeze | getInt==1 | setter 丢值 |
| PS2 | C-MISS | PS1 | 读 0/194 | contains false；default 不 panic | 缺 id 当有值 |
| PS3 | C-FREEZE | PS1 | freeze vs get() | 双方 frozen；ids 一致 | freeze 未置位 |
| PS4 | C-TYPED | new | 四类 setter | 读回一致 size=4 | 装箱丢值 |
| PS5 | C-OVERWRITE | set 1 再 2 | 同 id | size=1 值为 2 | 当成两槽 |
| PS6 | C-REMOVE | 有值 | remove | empty | 槽残留 |
| PS7 | C-CLEAR | 两 id | clear | empty | 清不干净 |
| PS8 | C-BUCKET | 跨桶 id | 0,7,8,193 | getAll 顺序与值 | 桶扩失败 |
| PS9 | C-DEFAULT | 部分有值 | 缺 id 默认 | 返回默认 | panic |
| PS10 | C-EMPTY | — | createEmpty/empty | frozen+empty | 未冻 |
| PS11 | C-VALUEOF | — | valueOf/ctors | 已冻可读 | 构造未冻 |
| PS12 | C-ADD-WIN | 两桶同 key | add | 后值 | 先值留下 |
| PS13 | C-ADD-MERGE | 不同 key | add | size=2 | 丢 key |
| PS14 | C-ADDLIMIT | 0,7,8 | addLimit 0..8 | 无 8 | 越界拷贝 |
| PS15 | C-COPY | 已冻 | new_from_* | 值相同 | 未拷 |
| PS16 | C-CLONE | 有值 | clone 后改 | 原值不变 | 共享可变 |
| PS17 | C-COMPOSE | old+new | mode 0–3/default | Java 语义 | 错 mode |
| PS18 | C-MERGE | 两 id | merge | 并集 frozen | 丢一侧 |
| PS19 | C-GETALL | 一槽 | getAll values | n=1 | 漏槽 |
| PS20 | C-CONTAIN | 一槽 | contain | empty/self 真 | 空集失败 |
| PS21 | C-DISPOSE | 已冻 | dispose | size=0 | map 残留 |
| PS22 | C-DIFF | 超集 | differenceSet | 去掉共有 key | 未删 |
| PS23 | C-INTERSECT | 超集 | intersect | 只留共有 | 交为空 |
| PS24 | C-EQUALS | 两独立桶 | 同/异 int | 同真异假 | Arc 指针相等 |
| PS25 | C-HASH | 已冻 | hashCode 两次 | 相等 | 每次变 |
| PS26 | C-GET-PANIC | 一槽 | getInt(0) | panic No such property | 静默 0 |
| PS27 | C-FRZ-WRITE | freeze 后 | setInt | panic frozen | 仍可写 |
| PS28 | C-NEW-EMPTY | new | — | empty | 脏桶 |
| PS29 | C-GETOBJ | 一槽 | getObject | downcast i32=1 | panic |
| PS30 | C-ISEQ | 两桶/两值 | is*Equal | 同值真 | 指针假 |

Java `freeze()` 返回同一 `data`；Rust 返回 clone。PS3 不要求指针同一。primitive 装箱后按值比较（`property_values_equal`），对齐 Java `Integer.equals`。

### 3.5 测试代码位置

crate: `libs/core-libs/util/util`  
src:   `libs/core-libs/util/util/src/cn/wps/moffice/property/PropertySet.rs`  
src:   `libs/core-libs/util/util/src/cn/wps/moffice/property/MutablePropertySet.rs`  
test:  `libs/core-libs/util/util/tests/cn/wps/moffice/property/PropertySet.rs`

### 3.6 测试运行脚本

在仓库根目录：

```bash
# 功能测试
cargo test --manifest-path libs/core-libs/util/util/Cargo.toml --test 01_function_property_set

# 本 function 源文件覆盖率（写入本节下方报告）
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/core-libs/util/util \
  --test 01_function_property_set \
  --src src/cn/wps/moffice/property/PropertySet.rs \
  --src src/cn/wps/moffice/property/MutablePropertySet.rs \
  --report libs/word-core/parser/features/wave0-ooxml-standards/functions/01_function_property_set/FUNCTION.md
```

首次需要：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_property_set --json`)
date: 2026-08-18 08:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/property/MutablePropertySet.rs` | 96/126 | 76.2% | 160/211 | 75.8% |
| `src/cn/wps/moffice/property/PropertySet.rs` | 479/681 | 70.3% | 969/1438 | 67.4% |
| **function total** | **575/807** | **71.3%** |  |  |
<!-- coverage-report:end -->
