# 03_function_graph_reader — GraphReader.doPrepareGraphs 读绳

| 项 | 值 |
|----|----|
| feature | wr-29-java-pipeline-cutover |
| order | 3 |
| crate | `libs/word-core/Cargo.toml` |
| 入口 | `GraphReader.doPrepareGraphs`（`prepareGraphs` 展平入口） |
| status | **pass**（复杂叶 Java 1559 行 → Rust 交错模板） |
| 阶段 | **layout** |

## 1. 功能说明

### 1.1 一句话
把 `GraphReader` 从无模块换成与 Java 同名交错模板：`doPrepareGraphs` 遍历 `srcList` 调 `read` → `readCoreNode` / MATH / 其它，写入 `charsList`/`graphList`（hello CORECHARS）。

### 1.2 输入
hello-world.docx 真实 parse 后的 rope + 至少一个 `SrcNode` `TYPE_SRC_CORECHARS`。

### 1.3 输出
`doPrepareGraphs` 后 `charsList`/`graphList` 非空（假 KPList 必须换成可 `obtainAndAddLast` 的真列表，见 GR-D1）。

### 1.4 上游
`02_function_graph_buff`（`prepareGraphs` 已调空钩 `doPrepareGraphs`）。

### 1.5 下游
`04_function_graph_factory` 量宽。

### 1.6 不变量与函数改写列表

**GraphReader.rs（本 function 改写列表，优先）**

- 字段全部
- `GraphReader` `dispose` `setMaxGraphSize` `doCleanGraphReader`
- `doPrepareGraphs` `read` `readCoreNode` `isUnsupportMathChar`
- `addCharsNode` `initFilter` `setIsInMath`
- 嵌套 `IgnoreHpsAndVAlignRunFilter` / `IgnoreCapsAndSmallCapsFilter`
- 其余方法：能接到 hello CORECHARS 路径的必须真实实现；脚注/域/公式/绘图可 `todo!("DEFERRED_NON_FUNCTION")` + 块注释停车，登记 §1.7.1

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `GraphBuff` | 02 | `prepareGraphs` 调 `doPrepareGraphs` | 是 | 抽象空钩由本 function 覆盖 | 不改编排 map |
| `KPList` | util | `firstElement`/`getNext`/`obtainAndAddLast`/`clean` | LayoutState **假** PhantomData | **GR-D1 换真列表** | 不改 util KPList 叶除非接线 |
| `Document.getChpxTable` | parse | `seek(cp)` | 需审计 | — | 不重写 CHPX 叶 |
| `GraphFactory.createNodes` | 04 | 抽象 | 空钩 | 整叶 04 | 禁止假装量宽 |

#### 1.7.1

- **GR-D1**：Java `srcList.firstElement()` / `getNext` / `charsList.obtainAndAddLast`。02 假 `KPList` 只有 `empty`/`clean`。03 必须接到真实节点存储，否则 `doPrepareGraphs` 无法填表。owner：03。
- **GR-D2**：`createNodes`/`commitGraphNode`/`commitAll`/`setWordAlignFont` 是 GraphFactory 抽象。hello 量宽前可空钩，但 CORECHARS 必须留下 CharsNode。owner：04。
- **GR-D3**：脚注/域/公式/drawing 不在 hello 主路径，可 DEFERRED。

## 3. 测试方案（loop 冻结矩阵后再改 src）

覆盖至少：`doPrepareGraphs` 空 srcList 不读；hello CORECHARS 填 chars；`doCleanGraphReader` 复位；`initFilter` 不 panic。管线同一 hello-world.docx sha256 `69063664483d139c76647e6a5b29754c79158e14bc5c480c5fef85bad5dda98e`。

### 3.6 / 3.7 / 3.8 / 3.9 / 3.10 / 3.11

**03 已 PASS（2026-10）**：

| 验收 | 结果 |
|------|------|
| `doPrepareGraphs` 空 srcList 不读 | GR1：`hasContents` false 早退，charsList 空 |
| hello CORECHARS 填 chars | GR3：`prepareGraphs` 后 charsList≥1、每节点带 finalRunProp、`mCurCP` 达正文尾（12） |
| `doCleanGraphReader` 复位 | GR4：`mCurCP=-1` 且复用 srcList 再读仍填 |
| `initFilter` 不 panic | GR2：hello 无 域/脚注/数学 → 过滤器集空 |
| KPList 真实节点存储（GR-D1） | GR5：firstElement/getNext/obtainAndAddLast/lastElement/clean |
| 回归 | `02_function_graph_buff` 20 ✓ / `p0_function_page_break` 3 ✓ / `02_function_flow_to_page` 3 ✓ / `03_function_page_layouter` / `03_function_flow_loop` / `01_function_cp_text` / `03_function_papx_paras` / `63_function_xwpf_assembly_loop` 全绿 |

```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 03_function_graph_reader
```

### 3.12 本 function 落地记录（wr-29）

- `graph/GraphReader.rs`：Java 1559 行 → Rust 交错模板（`// java` 逐行保留）；hello 主路径全真实；
  脚注/域/公式/绘图/修订/符号字体按 GR-D3 `DEFERRED_NON_FUNCTION` 停车（块注释保留 Java 原文）。
- **GR-D1**：`LayoutState::KPList` 假 `empty` 幻影 → 真实 `Vec<Box<T>>` 池（Default 构造、Java 同方法面
  firstElement/lastElement/getNext/getPrev/obtainAndAddLast/clean/isEmpty/size/remove）。
- `TypoPara`：补 `allocGraphs`/`putChars`(char[]/rope)/`putChar`/`hasAlloc`/`getCharBlock`（compile-surface 连续块）。
- env `PropertyTool`/`CachedPropertySet`/`PropertyToolFactory` 接线：纯文本合并级联真实；
  编号（Lfo/Lvf）/修订（237/45/46/48）DEFERRED；LRU 简化 HashMap（L4-5 纪律）；KStyle.getFinalProperty 复用。
- `IPropertyTool` 门面收尾：`Entry` 别名（IChpxTable::Entry→ChpxEntry、IPapxTable::Entry→PapxEntry）+ `&mut self`。
- `LayoutState` 去影子：`StyleResult`(真实)/`BidiTool.clean`/`hasDrawing`→bool/`hasBidiGraph`/`hasCoreCharsContent`/
  `needFlowOMathInDrawing`/`isMathContentState`。
- `DocumentEnv.propToolFactory` Holder → 真实 `PropertyToolFactory`；`ViewEnv.getForceUseShareViewRunProperty` 真实化。
- 已知既有破损（非本 function 引入）：`PLC.findUpper`→util 空树 lowerBound `Tree.rs:688` null mHead；
  `initFilter` 的 PLC 分支以空短路 + DEFERRED 停车，不触碰 PLC（hello 空表 Java 语义等价）。
