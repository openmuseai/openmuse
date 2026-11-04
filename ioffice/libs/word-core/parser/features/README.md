# Parse MVP — word-core 增量功能切片（Feature Seeds）

目标：先改写 **word-core 上 parse 最小可验证子集**，按小功能渐进推进。

**文档已收编到本目录**：原 `00-ooxml-standards` / `01-storage` … `05-orchestration` 现位于各 feature 的 `docs/`。

## Feature 一览

| feature_id | 文档入口 | Seeds 解析 | 原文档目录 |
|------------|----------|------------|------------|
| [wave0-ooxml-standards](./wave0-ooxml-standards/README.md) | [docs/](./wave0-ooxml-standards/docs/README.md) | （标准词汇，无 seeds） | `00-ooxml-standards` |
| [wc-01-storage-write](./wc-01-storage-write/README.md) | [STORAGE](./wc-01-storage-write/docs/STORAGE.md) | [SEEDS_ANALYSIS](./wc-01-storage-write/SEEDS_ANALYSIS.md) | `01-storage` |
| [wc-02-document-model](./wc-02-document-model/README.md) | [docs/](./wc-02-document-model/docs/README.md) | [SEEDS_ANALYSIS](./wc-02-document-model/SEEDS_ANALYSIS.md) | `02-document-model` |
| [wc-03-property-convert](./wc-03-property-convert/README.md) | [docs/](./wc-03-property-convert/docs/README.md) | [SEEDS_ANALYSIS](./wc-03-property-convert/SEEDS_ANALYSIS.md) | `03-property-convert` |
| [wc-04-paragraph-import](./wc-04-paragraph-import/README.md) | [docs/](./wc-04-paragraph-import/docs/README.md) | [SEEDS_ANALYSIS](./wc-04-paragraph-import/SEEDS_ANALYSIS.md) | `04-paragraph-import` |
| [wc-05-orchestration](./wc-05-orchestration/README.md) | [docs/](./wc-05-orchestration/docs/README.md) | [SEEDS_ANALYSIS](./wc-05-orchestration/SEEDS_ANALYSIS.md) | `05-orchestration` |
| [wc-06-stage1-live](./wc-06-stage1-live/FEATURE.md) | [FUNCTIONS](./wc-06-stage1-live/FUNCTIONS.md) | 实文档 Stage1 vs Java | **spec**（M11） |
| [wc-07-papx-geometry](./wc-07-papx-geometry/FEATURE.md) | [FUNCTIONS](./wc-07-papx-geometry/FUNCTIONS.md) | 缩进/段距 sprm + 行宽 | **spec**（M11） |
| [wc-08-numbering-apply](./wc-08-numbering-apply/FEATURE.md) | — | 编号串 + 悬挂 | pass |
| [wc-09-hdrftr-story](./wc-09-hdrftr-story/FEATURE.md) | — | 页眉页脚故事 | pass |
| [wc-10-subdocument-pipeline](./wc-10-subdocument-pipeline/FEATURE.md) | [FUNCTIONS](./wc-10-subdocument-pipeline/FUNCTIONS.md) | SubDocument 影子→真实模块 + Document 热路径 | **pass** |

每个 `wc-*` 目录结构：

```text
features/<feature_id>/
  README.md              ← feature 入口
  SEEDS_ANALYSIS.md      ← seeds.csv 核心文件逐项解析
  seeds.csv
  dep_topo.csv
  rewriting_queue.csv    ← 可选
  docs/                  ← 类文档 / 深读（自旧子功能目录迁入）
```

## 工作流

```text
1. 读 SEEDS_ANALYSIS.md 理解本切片必须实现的类
2. 按 seeds / dep_topo 顺序改写（java2rust 流水线）
3. 对照 docs/ 类文档做逻辑对齐
```

```bash
cd libs/word-core/parser/scripts

node build-mvp-dep-topo.mjs \
  --csv ../rewriting_files_parse_mvp.csv \
  --feature-name wc-01-storage-write
```

## 依赖链

```text
wave0-ooxml-standards（词汇）+ wave0 PropertySet/XWPF（core-libs）
  → wc-01-storage-write     CoreHelper.append* + Papx/PLC
  → wc-02-document-model    TextDocument / SubDocument / Styles
  → wc-03-property-convert  OOXML → PropertySet
  → wc-04-paragraph-import  段状态机
  → wc-05-orchestration     DocxReader 联调
  → wc-06-stage1-live       实文档 Stage1 金文件（M11）
  → wc-07-papx-geometry     几何 PAPX + 内容宽（M11）
```

下一波 layout 热路径（wr-21..24）依赖 wc-06/07：[state-java-l2-parity/NEXT_PLAN.md](../../state-java-l2-parity/NEXT_PLAN.md)。

## 维护 seeds.csv

- 只写本切片**业务入口/枢纽**；依赖闭包由脚本展开。  
- `feature_role=core` 行为种子。  
- 改完将 `state` 标 `done`。

## 批量改写 Skill

| Skill | 用途 |
|-------|------|
| [parse-feature-rewrite](../../.cursor/skills/parse-feature-rewrite/SKILL.md) | 按 `rewriting_queue.csv` 批量 java2rust |
| [feature-function](../../.cursor/skills/feature-function/SKILL.md) | 把 feature 再拆成 I/O function；`tests/` 镜像；改写-测试-回归 loop；每个 function 的 `FUNCTION.md` 含 cargo test 脚本与 cargo-llvm-cov 覆盖率报告 |
| [feature-cargo-gate](../../.cursor/skills/feature-cargo-gate/SKILL.md) | 全量 `cargo check` + 队列外错误分级；非 MVP 依赖可受控 shell，须在 queue `review` 记录 stub 依赖与理由 |
