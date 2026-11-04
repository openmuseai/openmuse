# pp-07-part-open — slide / layout / master part 读入

> depends_on: `pp-01` handlers；`pp-02` spTree；`pp-04` theme cache；`pp-03`/`pp-05` 子 handler  
> 状态：**pass**

## §0 管线位置

```text
PptxrSlide.open / PptxrLayout.open / PptxrMaster.open
  themeOverride rel（可选）
  UnifiedXmlReader.read(part stream, SlideHandler|LayoutHandler|MasterHandler)
  callback.load()
  slide.updateFieldText()
       ↓ MVP 在此停
notes / comment / chart / vml / diagram  → post-MVP
```

本层把 **已有 XML handler** 接到 **part stream**。不创建 slide 列表（pp-08），不打开 package（pp-09）。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_unified_xml_reader` | `InputStream`/`path` + `ElementHandler` | `HLXmlReader.parser` 一次 | MapManager；pp-12:03（part 流） | 03–05；pp-04 02；pp-06；pp-09 |
| 2 | `02_function_handler_callback_impl` | rels + mediaLib + mode | `load()` 解析 blip/media/chartMap；`onInk` 等 | pp-01 HandlerCallback | 03–05 |
| 3 | `03_function_pptx_slide_open` | `KmoSlide` + part | XML 填 slide；load；缺 part 早退 | 01；02；pp-01 05 | pp-08 |
| 4 | `04_function_pptx_layout_open` | `KmoSlideLayout` + part | bOpen 幂等；themeOverride；XML | 01；02；pp-01 06 | pp-08 |
| 5 | `05_function_pptx_master_open` | `KmoSlideMaster` + part | 缺 part → buildDefaultSlideMaster；onSlideLayoutIds 建 layout reader | 01；02；pp-01 07；pp-04 | pp-08 |

## §2 数据流

```text
null part → Java：Slide 仍尝试 rels（mPart==null 则整个 open 空转）
Throwable from XML → IOLog + return（slide）/ catch（master）
callback.load() 必须在 XML 之后、notes 之前
```

## §3 测试面

- 内存 `InputStream` + Recording handler 测 01
- 03–05 用最小 XML fixture（仅 `p:sld` 根）+ fake part

## §4 验收

1. UnifiedXmlReader 三种 overload 都接到 HLXmlReader（path 切片 overload 可 ignore 直到需要）。
2. Slide open：XML 失败不抛给调用方。
3. Master `bOpen`：第二次 open 不重复 parse。
4. 不调用 chart/vml/notes（断言副作用计数为 0）。

## §5 loop

`01 → 02 → 03 → 04 → 05`

## 文档

- [functions.csv](./functions.csv)
