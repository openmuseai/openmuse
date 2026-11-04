# 08_function_xml_dispatch — elementId 事件 → handler 栈

| 项 | 值 |
|----|----|
| feature | wave0-ooxml-standards |
| order | 8 |
| crate | `libs/core-libs/io/wps-io/Cargo.toml` |
| 入口 | `DispatchHandler2` / `ContentHandlerImpl` |
| 独立性 | 叶子（录制假 handler） |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

把 elementId 事件压到 handler 栈：栈顶 `getElementHandler(id)` 得到子 handler 则 push；end 时 pop 并 `onEnd`。

### 1.2 输入

`06_function_xml_elementid` 的事件 + 根 `ElementHandlerSimple`。两条路径：`opt/ContentHandlerImpl`（int id）与 `simple/DispatchHandler2`（SAX 名字 → `Name2IdMap.getId3`）。

### 1.3 输出

对栈顶 handler 的 `onStart` / `onCharacters` / `onEnd` 调用序列（可录制）。不分发 importer。

### 1.4 上游

`06_function_xml_elementid`。

### 1.5 下游

`12_function_handler_body`、`10_function_handler_props`、`13_function_xwpf_parse`。

## 2. 核心代码与流程

| 名字 | 角色 | Java | Rust |
|------|------|------|------|
| `DispatchHandler2` | SAX 路径：名字→id→栈 | `simple/DispatchHandler2.java` | 同名 |
| `ContentHandlerImpl` | opt 路径：int id 栈 | `opt/ContentHandlerImpl.java` | 同名 |
| `ElementHandlerSimple` | 双 flavor 接口 | `simple/ElementHandlerSimple.java` | 同名 |

```mermaid
sequenceDiagram
    participant EV as events
    participant D as DispatchHandler2
    participant H as handler stack
    EV->>D: start(p=112)
    D->>H: handlers.get(112) → PHandler push
    EV->>D: start(t=116)
    D->>H: PHandler.getElementHandler(116) → THandler push
    EV->>D: end(116)
    D->>H: THandler.onEnd; pop
```

## 3. 测试方案与代码

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `addHandler` | `DispatchHandler2` | index + handler | 根表 | 首次 start 用表，其后用栈顶 |
| `onStart` String | 同上 | ns + localName | push；非 null 则 `onStart_with_id` | `isFirst`；peek None |
| `onEnd` String | 同上 | ns + localName | pop + `onEnd_with_id` | pop 到 None 不崩 |
| `onCharacters` buf/str | 同上 | 文本 | 只交给栈顶 | 栈顶 None 忽略 |
| `resetHandlers` | 同上 | — | 清空表和栈 | 不重置 `isFirst`（Java 同） |
| `ContentHandlerImpl::start/end/character` | opt | int id | 同上栈语义 | 根 `isFirst` |
| `getElementHandler` 返回 None | 两路径 | 未知子 id | push None | 栈不崩 |
| `filterIndex` | DispatchHandler2 | — | 0 | |

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id |
|-------------|---------------|----------|-------|---------|
| C01 | DispatchHandler2 p→t 嵌套 | 最小正常 | 是 | D1 |
| C02 | 未知首元素 / 未知子 id | 失败忽略 | 是 | D2 |
| C03 | ContentHandlerImpl 嵌套 p/t | opt 路径 | 是 | D3 |
| C04 | onCharacters 到栈顶 t | 文本 | 是 | D4 |
| C05 | resetHandlers | cleanup | 是 | D5 |
| C06 | 未知子 id 后仍能 end 根 | 栈 None | 是 | D6 |
| C07 | onCharacters_str | 另一后端 | 是 | D7 |
| C08 | filterIndex | 常量 | 是 | D8 |
| C09 | addHandler 根不匹配 | 首次 miss | 是 | D9 |
| C10 | ContentHandlerImpl 未知子 | opt skip | 是 | D10 |

### 3.3 可测试性

内存 Recording `ElementHandlerSimple`：P 只对 `T` 返回子 handler。`DispatchHandler2` 用 `WMap.MAIN_NAMESPACING` + localName `p`/`t`。不跑 XmlReader（06）全文。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| D1 | C01 | addHandler(P) | start p, start t, end t, end p | P/T 各 start+end；嵌套顺序 | 子 handler 未 push |
| D2 | C02 | addHandler(P) | start `unknown` | 不 panic；P 无 start | 空栈 peek panic |
| D3 | C03 | ContentHandlerImpl(root=P) | start 112/116 / end | 同 D1 | opt 栈未接线 |
| D4 | C04 | 已 start p+t | characters `Hi` | T 收到 `Hi`，P 无 | 交给错层 |
| D5 | C05 | 已 addHandler | reset 后再 start p | P 无 start（表已清） | 未清空 |
| D6 | C06 | D1 的 P | start p, start `r`, end r, end p | P start+end；不 panic | 未知子打崩 |
| D7 | C07 | 已 start p | `onCharacters_str("x")` | P 收到 path 文本 | 未转发 |
| D8 | C08 | new | filterIndex | 0 | |
| D9 | C09 | addHandler(T) | 先 start p | T 无 start | 根表误命中 |
| D10 | C10 | ContentHandlerImpl | start 112, start 99, end 99, end 112 | P start+end | panic |

### 3.5 测试代码位置

crate: `libs/core-libs/io/wps-io`  
src:   `libs/core-libs/io/wps-io/src/cn/wps/io/simple/DispatchHandler2.rs`  
src:   `libs/core-libs/io/wps-io/src/cn/wps/io/opt/ContentHandlerImpl.rs`  
test:  `libs/core-libs/io/wps-io/tests/cn/wps/io/simple/DispatchHandler2.rs`

### 3.6 测试运行脚本

在仓库根目录：

```bash
cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 08_function_xml_dispatch

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/core-libs/io/wps-io \
  --test 08_function_xml_dispatch \
  --src src/cn/wps/io/simple/DispatchHandler2.rs \
  --src src/cn/wps/io/opt/ContentHandlerImpl.rs \
  --report libs/word-core/parser/features/wave0-ooxml-standards/functions/08_function_xml_dispatch/FUNCTION.md
```

首次需要：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 08_function_xml_dispatch --json`)
date: 2026-08-18 11:03

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/io/opt/ContentHandlerImpl.rs` | 33/89 | 37.1% | 51/133 | 38.3% |
| `src/cn/wps/io/simple/DispatchHandler2.rs` | 64/108 | 59.3% | 103/177 | 58.2% |
| **function total** | **97/197** | **49.2%** |  |  |
<!-- coverage-report:end -->
