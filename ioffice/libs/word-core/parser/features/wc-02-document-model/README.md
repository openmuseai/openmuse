# wc-02-document-model — 文档模型

**feature_id**: `wc-02-document-model`  
**depends_on**: `wc-01-storage-write`  
**目标**: TextDocument / SubDocument 拥有存储结构；KStyles + StyleId2Sti 可供 importer

## 本目录

| 文件 | 说明 |
|------|------|
| [FUNCTIONS.md](./FUNCTIONS.md) | I/O function 组合图（feature-function 拆分） |
| [STRUCTURE_ANALYSIS.md](./STRUCTURE_ANALYSIS.md) | 类图 / 最小改写范围；**§4.5** 是 09 与存储模型分界 |
| [docs/09-subdocument-own-scope.md](./docs/09-subdocument-own-scope.md) | 09 依赖发散后的约束与收敛（PLC / CPieceTable / KRange） |
| [functions.csv](./functions.csv) | function 状态表 |
| [functions/](./functions/README.md) | 每个 function 的 `FUNCTION.md`（测试方案 + 运行脚本） |
| [seeds.csv](./seeds.csv) | 核心种子 |
| [SEEDS_ANALYSIS.md](./SEEDS_ANALYSIS.md) | seeds 逐项解析 |
| [docs/](./docs/README.md) | 原 `02-document-model/` 类文档 |

## 下游

→ [wc-03-property-convert](../wc-03-property-convert/README.md)
