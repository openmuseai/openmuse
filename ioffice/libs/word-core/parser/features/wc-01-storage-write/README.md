# wc-01-storage-write — 存储写入路径

**feature_id**: `wc-01-storage-write`  
**batch**: `parse-01-storage`  
**目标**: 打通 `CoreHelper.appendText/appendParagraph` → rope + CHPX/PAPX + PLCSection

## 本目录

| 文件 | 说明 |
|------|------|
| [seeds.csv](./seeds.csv) | 本切片核心种子（改写入口） |
| [dep_topo.csv](./dep_topo.csv) | 依赖拓扑 |
| [rewriting_queue.csv](./rewriting_queue.csv) | java2rust 队列 |
| [SEEDS_ANALYSIS.md](./SEEDS_ANALYSIS.md) | **seeds 核心文件逐项解析** |
| [docs/STORAGE.md](./docs/STORAGE.md) | **唯一**技术文档（职责 / 热路径 / 存储模型） |
| [docs/plc-code-walkthrough.md](./docs/plc-code-walkthrough.md) | PLC/PLCSection 代码走读（Rust 实现 + 上下游数据流 + 流程角色 + 图） |
| [STRUCTURE_ANALYSIS.md](./STRUCTURE_ANALYSIS.md) | 类级蓝皮书 |
| [FUNCTIONS.md](./FUNCTIONS.md) | 按 I/O 拆成 7 个可独立测的 function；组合图 |
| [functions.csv](./functions.csv) | function 机器索引 |
| crate `tests/` | 可执行测试（与 `src` 同相对路径）；见 FUNCTIONS.md §3 |

## 前置

- PropertySet（wave0）  
- textrope / ChpxTable 多数已 done（作为依赖被拉入）  
- 标准词汇：[wave0-ooxml-standards](../wave0-ooxml-standards/docs/README.md)

## 下游

→ [wc-02-document-model](../wc-02-document-model/README.md)
