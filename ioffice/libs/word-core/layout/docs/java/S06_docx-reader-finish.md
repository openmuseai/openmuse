# S06 — 编排与收尾 → 完整 TextDocument（Stage1 交付物）

## 1. 职责

`DocxReader.read()` 顶层编排：把 S01–S05 的事件（经 S03 分发到 styles / 主文档等 handler）汇总，执行 styles 两阶段、正文收尾（`supplyEndPara`）、默认节兜底、`ISnapshot` 上传，产出**完整 TextDocument**——Stage1 验收交付物。

管线位置：上游 S01–S05；下游 S07（排版装配）。

## 2. 核心 Java 代码

| 类 | 职责 |
|----|------|
| `DocxReader.read()` | 12 步入口：包→XML→styles→正文→收尾→`ISnapshot` 上传 |
| `DocumentImporter` | 路由：styles handler 两阶段、主文档/页眉页脚/批注/脚注等 |
| `FinishReadingWork.go()` | 读后处理：默认样式、缺省页、编号/目录收尾 |
| `StylesImporter` | 样式两阶段：先收集样式定义，再建 KStyles（用户样式先于正文） |

`read()` 顺序（钉死，12 步）：

```text
S01 打开 OPC 包 → 找 officeDocument part
S02/S03 根 XML 解析（styles + 主文档两路）→
S04 属性转换 → S05 正文落库 →
styles 两阶段（收集 → 建 KStyles，Normal sti=0）→
FinishReadingWork.go()：supplyEndPara、默认节、缺省信息 →
ISnapshot 上传（模型快照给编辑器/后续 layout）
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `TextDocument` | 最终交付：rope（末段 `\r` 经 supplyEndPara）+ CHPX/PAPX + `KStyles` + `PLCSection`（≥1 默认节）+ `guessedTotalLength` + metadata |
| `KStyles` | rPr/pPr 默认、Normal sti=0、用户样式先于正文 |
| `PLCSection` | 默认节至少 1 个（无 sectPr 也兜底） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | S01–S05 事件（经 S03 分发）+ OPC 元数据 |
| **O** | **TextDocument**：rope+CHPX/PAPX+KStyles+PLCSection+guessedTotalLength；`ISnapshot` 上传 |

## 5. 上下游

- **上游**：S01–S05。
- **下游**：S07（LayoutManager 装配，从 TextDocument 起步）；Stage1 JSON 验收。

## 6. 边界 / 失败语义 / 不变量

- 正文结束必须有末段 `\r`（`supplyEndPara`）；无 sectPr 也要有 ≥1 默认节。
- styles 用户样式必须先于正文解析，保证 `KStyles` 在正文落库时可用。
- parse 热路径（DocxReader / DocumentImporter）layout feature **禁止回头改**，除非 Stage1 对照证明缺字段。

## 7. Rust 承接与现状

wc-05：`02_style_event` **pass**；`01_io_style` src 完成待 loop；`03_di_construct_route` 模板在编面（ctor/路由 DEFERRED）；`04–07` spec。
styles 链已通（wave0 `15_styles_handler_chain` pass）→ 07 的 e2e 样式断言可去 ignore。
**Stage1 JSON 门闩**：DocumentModel 与 Java 严格相等。