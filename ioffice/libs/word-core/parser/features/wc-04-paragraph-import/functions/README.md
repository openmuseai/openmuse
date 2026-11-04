# functions/ — 本 feature 的最小功能单元

索引与串联说明在上一级 [FUNCTIONS.md](../FUNCTIONS.md)。

目录名 `{NN}_function_*`：`NN` 为依赖拓扑序号，`ls` 顺序即编译/测试顺序。

每个目录含：

- `FUNCTION.md` — I/O、上下游、依赖实现状态、**测试方案**（§3）
- `CODE_ANALYSIS.md` — **拆分理由 + 当前代码现状 + 实现级分析**（Java 源码 + 已实现 Rust 依赖 + 流程图）

可执行测试**不**在本目录。它们与 `src` 隔离，镜像落在对应 crate：

```text
libs/word-core/src/<relpath>/<Type>.rs      实现
libs/word-core/tests/<relpath>/<Type>.rs    测试
```

运行：`cargo test --manifest-path libs/word-core/Cargo.toml --test <function_id>`。

改写-测试-回归 loop：`.cursor/skills/feature-function/SKILL.md`。

**切在 I/O，不按类切。** `SubDocumentImporter` 拆成 run 文本（02）/ 段生命周期（03）/ 表格（04）/ 域与锁（05）；`MainDocumentImporter` 拆成 guard+节记忆（06）与建节（07）。

---

## 为什么这么拆分（I/O 切分理由）

wc-04 的三个 Java 类里，`SubDocumentImporter`（717 行）和 `MainDocumentImporter`（216 行）不是「一个类一个 I/O 变换」，而是**多个可独立观察的 I/O 事件**的组合。按类切会让一个 function 同时吞进 Start/Run/End/Table/Field/Section 六种输入输出，无法单点测试。因此按事件边界切成 7 个 function：

| function | 独立观察点 | 为什么不合并/不继续拆 |
|----------|-----------|----------------------|
| 01 段会话 | `mPositionInserted`/`mParaId`/`mEntry` 三个字段 | 纯数据载体，是 03/04 共享的最小状态；再拆只剩字段，无意义 |
| 02 run 文本 | rope + CHPX（`document.getLength()` 增量） | Run 是独立 I/O（text+chpx→rope）；与 Start/End 无关，可单独断言 CHPX 挂点 |
| 03 段生命周期 | PAPX 挂点 = `mPositionInserted`（Start 抓、End 用） | Start 的私有会话只能通过 End 的 PAPX cp 观察，所以 Start+End 必须同属一个 function；Run 是独立的中间事件，故拆出去 |
| 04 表格 cell/row | PAPX 223/224/226 + cellMark 字节 | 表格是独立回调族，输入是 layer/cellMark/tableProps，与段落热路径分开 |
| 05 域与锁 | `mFieldsNotEnded` 计数 + 写锁回调 | 域控制字符（19/20/21）和 lock/unlock 是 06 的输入，独立成 function 让 06 只做 guard |
| 06 主文档 guard | ForceQuit/ReadingLimit 异常 + `mCpLastSectionEnd` | 只做「段前后护栏 + 节 CP 记忆」，不碰 rope/PAPX/PLCField |
| 07 建节 | `PLCSection.SectionNode` + sepx | sectPr 属性是独立 I/O；首节/非首节分支必须单独测 |

依赖拓扑：`01 → 02 → 03 → 04 → 05 → 06 → 07`。04 依赖 03 的会话；06 依赖 03/05 的 super 与锁；07 依赖 06 的 `mCpLastSectionEnd`。
