# wave0-ooxml-standards — OOXML / DOCX 标准与术语

**角色**: 所有 `wc-*` feature 的**前置词汇** + **实现桥**（parse-wave0 /
parse-wave0-xwpf 已全部 done）。原目录：`parser/00-ooxml-standards/`。

> 本 feature 与 `wc-*` 不同：**无待改写种子**（实现层已 done）。本目录的
> `seeds.csv` / `dep_topo.csv` / `rewriting_queue.csv` 记录**已完成**的 16 个枢纽类
> 与 26 个依赖闭包，供下游 feature 引用拓扑与接口面。

## 本目录

| 文件 | 说明 |
|------|------|
| [seeds.csv](./seeds.csv) | 16 个枢纽种子（属性桶/包/XML/elementId/回调边界/正文 handler），全部 done |
| [SEEDS_ANALYSIS.md](./SEEDS_ANALYSIS.md) | 种子逐项解析（按 5 个枢纽分层） |
| [dep_topo.csv](./dep_topo.csv) | 依赖拓扑：16 种子 + 26 依赖闭包 = 42 行 |
| [rewriting_queue.csv](./rewriting_queue.csv) | 改写队列：42/42 done，无 pending |
| [STRUCTURE_ANALYSIS.md](./STRUCTURE_ANALYSIS.md) | 结构分析蓝皮书：类图/流程图/依赖/最小范围/验收 |
| [FUNCTIONS.md](./FUNCTIONS.md) | 按 I/O 拆成 14 个可独立测的 function；组合图 |
| [functions.csv](./functions.csv) | function 机器索引 |
| crate `tests/` | 可执行测试（与 `src` 同相对路径）；见 FUNCTIONS.md §3 |

## 文档

见 [docs/README.md](./docs/README.md)：

| 文档 | 内容 |
|------|------|
| opc-package | OPC 包、Content Types、Relationships |
| wordprocessingml | w:p / w:r / pPr / rPr |
| styles-numbering-settings | styles / numbering / settings |
| section-breaks | 节与 sectPr |
| mapping-to-kernel | OOXML ↔ CP/CHPX/PAPX/PLC |
| wave0-xwpf-java-rust | OPC/XWPF 已改写层对照 |
| ooxml-standard-deep-dive | 标准深读 |

## 与 features 的关系

```text
wave0-ooxml-standards（词汇 + OPC/XWPF 回调桥）
  → wc-01-storage-write
  → wc-02 … → wc-05-orchestration（接 XWPF 回调）
```

## 维护

- 重新生成 dep_topo：`node libs/word-core/parser/scripts/build-feature-dep-topo.mjs --feature wave0-ooxml-standards`
- 重新生成队列：`node .cursor/skills/parse-feature-rewrite/scripts/prepare-queue.mjs prepare wave0-ooxml-standards`
- 完整 135 行 wave0-xwpf 清单见主 CSV：`rg 'parse-wave0-xwpf' libs/word-core/parser/rewriting_files_parse_mvp.csv`

