# pp-04-theme-part — theme / themeOverride part

> depends_on: `pp-01-pptx-parse`；`pp-03-dml-style`（ThemeHandler 内部会碰到 clrMap 类 DML）  
> 状态：**pass**

## §0 管线位置

```text
PptxrPresentation.openGlobalData
  rels.getRelationshipByType(theme)
       ↓ 本 feature
PptxrThemes.open(partName cache) → PptxrTheme.open
  UnifiedXmlReader + ThemeHandler(KmoTheme)
       ↓
KmoPresentation.setTheme；slide/layout themeOverride 复用同一 cache
```

`openGlobalData` 在解析 `presentation.xml` **之前**打开 theme。测试本层只用假 `OpcrRelationship`/`part` 或内存 stream fixture，不跑 PptxReader。

## §1 function 清单

| order | function_id | I | O | upstream | downstream |
|---|---|---|---|---|---|
| 1 | `01_function_theme_handler` | theme XML 事件 + `KmoTheme` | theme 模型字段 + 子 handler 分发 | HandlerCallback；ColorMap | 02 |
| 2 | `02_function_pptx_theme_open` | part stream + callback | `rid→mediaId` map；throwable 吞掉返回 null | 01；UnifiedXmlReader（07 前用 fixture reader） | 03；pp-07 |
| 3 | `03_function_themes_cache` | `OpcrRelationship` + pres + medias | cache hit 返回同一 `KmoTheme`；miss 则 open 并 optional `_addTheme` | 02 | pp-07；pp-09 |

## §2 数据流

```text
themeRels.partName ── cache lookup
  miss → new KmoTheme → ThemeHandler parse → rid2MediaIds
  hit  → 同一指针
buildDefaultTheme()：资源流 + ThemeHandler(callback=null)（Master 无 theme rel）
```

## §3 编译与测试面

- crate：`libs/ppt-core`
- src：`drawing/dmlr/handler/style/ThemeHandler.rs`；`show/pptxr/ppt/theme/{PptxrTheme,PptxrThemes}.rs`
- loop L2 落 `tests/` 镜像

## §4 串联验收

1. ThemeHandler 对已知 theme 子元素返回非 None kind。
2. PptxrTheme.open：null part → null；parse 失败 → null（Java 吞 Throwable）。
3. PptxrThemes：同一 partName 两次 open 同一 theme；不同 partName 两次实例。

## §5 loop 顺序

`01 → 02 → 03`

## §6 范围

| Java | 裁决 |
|---|---|
| `ThemeHandler` | 01 |
| `PptxrTheme` | 02 |
| `PptxrThemes` | 03 |
| `HandlerCallbackImpl` | pp-07；本层 ctor fixture |
| `UnifiedXmlReader` | pp-07；02 可用注入式 `read` fixture |

## 文档

- [functions.csv](./functions.csv)
