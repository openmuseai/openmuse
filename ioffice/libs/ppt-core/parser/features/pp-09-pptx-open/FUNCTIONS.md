# pp-09-pptx-open — PptxReader / PptxrPresentation 编排

> depends_on: `pp-04` theme；`pp-06` presentation XML；`pp-08` slides walk  
> 状态：**pass**  
> 这是 MVP **唯一**允许串 `pptx bytes → KmoPresentation` 的 feature（仍可用内存 `OxfrPackage` fixture）。

## §0 管线位置

见 [PIPELINE.md](../PIPELINE.md) 热路径。本层 **不重写** SlideHandler / PptxrSlides 内部；只锁调用顺序与失败语义。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_doc_props` | App/Core part | AppProperty/CoreProperty 字段；null part 跳过；parse 失败吞掉 | OxfrAppPart/OxfrCorePart | 04 |
| 2 | `02_function_pres_open_global` | pres part rels + stream | setTheme；PresentationHandler+load；addDefaultProperty | pp-04；pp-06 | 03 |
| 3 | `03_function_pres_open` | PptxrPresentation.open | initialize slides；empty→blank slide；slides.open；delayLoadMedias | 02；pp-08 | 04 |
| 4 | `04_function_pptx_reader` | path 或 RAF + flags | FileDamaged / SuffixError；OleManager.install；close 清 package | 01；03；pp-12:02（pres part） | feature-output |

`PptxrCustom` / `openOtherGlobalData` 全量：矩阵项可 `#[ignore = "spec: post-MVP"]`，不得标本 function pass 直到忽略项从必测拿掉或实现。

**建议 MVP pass 口径：** 04 的必测是 getPresPart + 打开顺序 + close；custom/viewProps/commentAuthors 从必测移到附录。

## §2 数据流

```text
getPresPart: officeDocument==null → FileDamagedException("Not OOXML file")
             partName 不含 "ppt" → SuffixErrorException("Not pptx file")
open finally → close() 必跑
encrypted：解密路径本 feature 不测（ignore）
```

## §3 测试面

- 最小 pptx fixture：`[Content_Types]` + `ppt/presentation.xml` + 一页 + theme（loop 后期）
- 前期：mock package 只测 getPresPart 与顺序 Recording listener

## §4 验收

1. readApp：各 getter null 不 put。
2. openGlobalData：无 theme rel 不 setTheme。
3. slideCount==0 与 >0 两条 initializing 回调。
4. PptxReader.close：package.close 失败只 printStackTrace（Java）；encrypted 删临时文件。

## §5 loop

`01 → 02 → 03 → 04`（04 是编排 function，可用 01–03 golden）

## 文档

- [functions.csv](./functions.csv)
