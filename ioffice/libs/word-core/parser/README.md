# Document Parse MVP — 文档索引

本目录存放 **DOCX 解析 → TextDocument 模型** 的代码分析与改写切片。  
**主入口已收编到 [`features/`](./features/README.md)**（按 feature 组织文档 + seeds 解析）。

## 阅读顺序

1. [features/README.md](./features/README.md) — Feature 总表与依赖链  
2. [features/wave0-ooxml-standards](./features/wave0-ooxml-standards/README.md) — OOXML 术语  
3. 按链阅读各 `wc-*/SEEDS_ANALYSIS.md` + `docs/`  
4. [REWRITE_DESIGN_PARSE_MVP.md](./REWRITE_DESIGN_PARSE_MVP.md) — 总览 / Wave / 检查表  
5. [REWRITE_FILE_ORDER.md](./REWRITE_FILE_ORDER.md) — 改写清单  

**改写 CSV**: [`rewriting_files_parse_mvp.csv`](./rewriting_files_parse_mvp.csv)

## Feature 速查

| feature | Seeds 解析 | 类文档 |
|---------|------------|--------|
| [wc-01-storage-write](./features/wc-01-storage-write/README.md) | [SEEDS](./features/wc-01-storage-write/SEEDS_ANALYSIS.md) | [STORAGE](./features/wc-01-storage-write/docs/STORAGE.md) |
| [wc-02-document-model](./features/wc-02-document-model/README.md) | [SEEDS](./features/wc-02-document-model/SEEDS_ANALYSIS.md) | [docs](./features/wc-02-document-model/docs/README.md) |
| [wc-03-property-convert](./features/wc-03-property-convert/README.md) | [SEEDS](./features/wc-03-property-convert/SEEDS_ANALYSIS.md) | [docs](./features/wc-03-property-convert/docs/README.md) |
| [wc-04-paragraph-import](./features/wc-04-paragraph-import/README.md) | [SEEDS](./features/wc-04-paragraph-import/SEEDS_ANALYSIS.md) | [docs](./features/wc-04-paragraph-import/docs/README.md) |
| [wc-05-orchestration](./features/wc-05-orchestration/README.md) | [SEEDS](./features/wc-05-orchestration/SEEDS_ANALYSIS.md) | [docs](./features/wc-05-orchestration/docs/README.md) |
| [wave0-ooxml-standards](./features/wave0-ooxml-standards/README.md) | — | [docs](./features/wave0-ooxml-standards/docs/README.md) |

## 端到端

```text
.docx → XWPFDocument.parse (wave0-xwpf)
      → DocumentImporter (wc-05)
      → SubDocumentImporter (wc-04)
      → Run/Para converters (wc-03)
      → CoreHelper.append* (wc-01)
      → TextDocument/SubDocument (wc-02 拥有)
```

## 目录结构

```text
libs/word-core/parser/
  README.md
  REWRITE_DESIGN_PARSE_MVP.md
  REWRITE_FILE_ORDER.md
  rewriting_files_parse_mvp.csv
  features/
    README.md
    manifest.csv
    wave0-ooxml-standards/   ← 原 00-ooxml-standards
    wc-01-storage-write/     ← 原 01-storage + seeds 解析
    wc-02-document-model/    ← 原 02-document-model
    wc-03-property-convert/
    wc-04-paragraph-import/
    wc-05-orchestration/
```

Java 源码根：`koffice/writer/core/`；OPC/XWPF 在 core-libs。
