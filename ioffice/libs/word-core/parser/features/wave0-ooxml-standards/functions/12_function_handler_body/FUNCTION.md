# 12_function_handler_body — document.xml → 段 / run 回调

| 项 | 值 |
|----|----|
| feature | wave0-ooxml-standards |
| order | 12 |
| crate | `libs/core-libs/io/wps-io/Cargo.toml` |
| 入口 | `DocumentHandler` → `DocumentBodyHandler` → `PHandler` → `RHandler` → `THandler` |
| 独立性 | 叶子（一段 XML + RecordingImporter；不跑 OPC / 11 / 13） |
| 测试状态 | pass |

## 1. 功能说明

### 1.1 一句话

把 `w:document/w:body/w:p/w:r/w:t` 降维成 `IDocumentImporter` 的 `onImportParagraph*` / `onImportRun*`，不落库。

### 1.2 输入

主文档 XML 事件（跳过 OPC：`XmlReader.parse` + `DocumentHandler` 根）。假 importer 只记录回调。

### 1.3 输出

有序回调：`onImportParagraphStart` → `onImportRun(text)` → `onImportParagraphEnd(ParagraphSimple, SubDocType, layer)`。

`SubDocType.MAIN_DOCUMENT = 0`。

### 1.4 上游

`08_function_xml_dispatch`；`10_function_handler_props`。

### 1.5 下游

`13_function_xwpf_parse`；`14_function_importer_boundary`。

## 2. 核心代码与流程

| 名字 | 角色 | Java hash / id |
|------|------|----------------|
| `DocumentHandler` | 根；`getElementHandler` | body=`3029410`；background=`-1332194002` |
| `DocumentBodyHandler` | `onStart`→`onMainDocumentStart`；`onEnd`→`onMainDocumentEnd` | `filterIndex=0` |
| `ContentBlockContentHandler` | body 默认子树 | `w:p`=`112` |
| `PHandler` | `onStart`/`onEnd` 段回调 | `w:pPr`=`110226` |
| `ContentRunContentHandler` | 段内 run | `w:r`=`114` |
| `RHandler` / `RunInnerContentHandler` | run 子树 | `w:t`=`116`；`w:rPr`=`112148` |
| `THandler` | `onCharacters` → `onImportRun` | |

```text
DocumentHandler (w:document)
  → DocumentBodyHandler (w:body=3029410)
       → PHandler (w:p=112)
            ├─ PprHandler (110226，本 function 不强制有 pPr)
            └─ RHandler (w:r=114)
                 ├─ RprHandler (112148)
                 └─ THandler (w:t=116) → onImportRun_chars
            段结束 → onImportParagraphEnd
```

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | Java/Rust 路径 | 输入/状态 | 输出/副作用 | 关键分支/错误 |
|--------|----------------|-----------|-------------|---------------|
| `DocumentHandler.getElementHandler` | DocumentHandler | 3029410 | `DocumentBodyHandler` | 其它 id → null（除 background） |
| `DocumentBodyHandler.onStart/onEnd` | DocumentBodyHandler | dummy part | `onMainDocumentStart(false)` / `onMainDocumentEnd` | 无 header/footer rel |
| `PHandler.onStart` | PHandler | p=112 | `onImportParagraphStart(0, attrs)` | |
| `PHandler.onEnd` | PHandler | p=112 | `onImportParagraphEnd(ParagraphSimple, 0, 0)` | mPprHandler 可空 |
| `RHandler.getElementHandler` | RHandler | t=116 | `THandler` | `super.getElementHandler(id, this)` |
| `THandler.onCharacters` | THandler | buffer/start/len | `onImportRun_chars(0, rPr, buf, start, len)` | ruby 路径本层不测 |
| `XmlReader.parse` | 08 分发 | 最小 document XML | 上述回调序列 | ignore=`-1` mode=`0` |

### 3.2 覆盖矩阵

| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| C01 | 单段单 run 文本 | 最小正常 | 是 | B1 |
| C02 | 两段顺序 | 1/N | 是 | B2 |
| C03 | 空 body | 0 次 | 是 | B3 |
| C04 | body 进出 | onMainDocument* | 是 | B1（同 XML 断言） |

### 3.3 可测试性

内存 XML + `RecordingImporter`。不打开 docx。

### 3.4 用例表

| id | 覆盖项 | 前置 | 输入 | 期望 | 失败语义 |
|----|--------|------|------|------|----------|
| B1 | C01+C04 | XmlReader + DocumentHandler | 单段 `Hi` | Start、Run(`Hi`)、End 各一次；MainStart/MainEnd | 分发没落到 THandler |
| B2 | C02 | 同 B1 | 两段 `A`/`B` | End 两次，Run 顺序 A 然后 B | 段栈错乱 |
| B3 | C03 | 同 B1 | `<w:body/>` | 无 Run、无 ParagraphStart | 空 body 仍造段 |

### 3.5 测试代码位置

crate: `libs/core-libs/io/wps-io`  
src:   `libs/core-libs/io/wps-io/src/cn/wps/io/xwpf/element_handler/DocumentHandler.rs`  
test:  `libs/core-libs/io/wps-io/tests/cn/wps/io/xwpf/element_handler/DocumentHandler.rs`

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/core-libs/io/wps-io/Cargo.toml --test 12_function_handler_body

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/core-libs/io/wps-io \
  --test 12_function_handler_body \
  --src src/cn/wps/io/xwpf/element_handler/DocumentHandler.rs \
  --src src/cn/wps/io/xwpf/element_handler/DocumentBodyHandler.rs \
  --src src/cn/wps/io/xwpf/element_handler/PHandler.rs \
  --src src/cn/wps/io/xwpf/element_handler/RHandler.rs \
  --src src/cn/wps/io/xwpf/element_handler/THandler.rs \
  --report libs/word-core/parser/features/wave0-ooxml-standards/functions/12_function_handler_body/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 12_function_handler_body --json`)
date: 2026-08-18 18:22

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/io/xwpf/element_handler/DocumentBodyHandler.rs` | 69/121 | 57.0% | 69/106 | 65.1% |
| `src/cn/wps/io/xwpf/element_handler/DocumentHandler.rs` | 46/97 | 47.4% | 55/90 | 61.1% |
| `src/cn/wps/io/xwpf/element_handler/PHandler.rs` | 87/145 | 60.0% | 89/134 | 66.4% |
| `src/cn/wps/io/xwpf/element_handler/RHandler.rs` | 74/127 | 58.3% | 71/113 | 62.8% |
| `src/cn/wps/io/xwpf/element_handler/THandler.rs` | 75/130 | 57.7% | 63/117 | 53.8% |
| **function total** | **351/620** | **56.6%** |  |  |
<!-- coverage-report:end -->
