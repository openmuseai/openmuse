# pp-09-pptx-open — PptxReader / PptxrPresentation 编排

> status: **pass**
> 拆分：[FUNCTIONS.md](./FUNCTIONS.md)（4 function，01–04 均 `pass`）
> 依赖：pp-04-theme-part；pp-06-presentation-xml；pp-08-slides-walk
> 管线：[PIPELINE.md](../PIPELINE.md) / [README.md](../README.md)

## 目标

**MVP 唯一允许串 `pptx bytes → KmoPresentation` 的 feature**（仍可用内存 `OxfrPackage` fixture，不要求 ZIP 落盘）。
锁定 `PptxReader.open` 的调用顺序与失败语义：OxfrPackage 构建 → readApp（app part）→ `getPresPart` 后缀校验 →
`PptxrPresentation.open`（slides initialize、openGlobalData、空白/首屏回调、slides.open、media delay）→ readCore（core part）→
PptxrCustom（stub）→ `docDidFinishReadingEverything` → `setReadOnly(false)` → finally `close()`。
**不重写 SlideHandler / PptxrSlides 内部**（pp-07 / pp-08 已管）；decrypt、OLE install、custom / viewProps / commentAuthors
只在调用点留 stub，业务体移后置。

## I/O（锁死）

| function | I | O | 契约 |
|---|---|---|---|
| 01 doc_props | `OxfrAppPart` / `OxfrCorePart` | `AppProperty`（appName/appVersion/company/manager）、`CoreProperty`（title/creator/lastModifiedBy/created/modified；日期经 `OpcUtil.getDateString` → `yyyy-MM-dd'T'HH:mm:ss'Z'` UTC） | null part 跳过；`part.parse()` Throwable 吞掉（readApp 继续读 getter，readCore 直接 `return`）；各 getter null 不 put |
| 02 pres_open_global | pres part rels + presentation.xml stream | 有 theme rel → `mThemes.open` → `mPres.setTheme`；`PresentationHandler` + load（pp-06/pp-07）；`addDefaultProperty` 缺 sldSz/notesSz 写默认 | 无 theme rel **不** setTheme；tableStyles rel 可 fixture skip；theme 先于 presentation.xml（PIPELINE.md §4） |
| 03 pres_open | `KmoPresentation` + pres part + listener/media/mode | slides initialize；空白/首屏回调；`setInitialized(true)`；slides.open；`delayLoadMedias` | slideCount==0 → 空白 slide + `docDidFinishInitializing(pres, true)`；>0 → `(pres, false)`；`setInitialized(true)` 在 slides.open 前 |
| 04 pptx_reader | path 或 RAF + flags | 打开顺序（§2 数据流）+ 后缀校验 + close | officeDocument==null → `FileDamagedException("Not OOXML file")`；partName 不含 "ppt" → `SuffixErrorException("Not pptx file")`；close 必跑 |

## Java 对照

| Java 类 | Rust 目标 | 裁决 |
|---|---|---|
| `PptxReader.java`（04） | `libs/ppt-core/src/cn/wps/show/pptxr/PptxReader.rs`（尚无） | ctors + `init`、`open`、`close`、`getPresPart` 必写；decrypt、OLE install 停车 |
| `PptxrPresentation.java`（02/03） | `libs/ppt-core/src/cn/wps/show/pptxr/ppt/PptxrPresentation.rs`（尚无） | `open`、`openGlobalData`、`addDefaultProperty` 必写；`openOtherGlobalData` 全量停车（presProps / viewProps / commentAuthors / ITEM / ITEMPROPS） |
| `PptxrDocProps.java`（01） | `libs/ppt-core/src/cn/wps/show/pptxr/docprops/PptxrDocProps.rs`（尚无） | `readApp`、`readCore` 必写；`readThumbsnail` 空体保留 |
| 移出（调用点预留，业务体 post-MVP） | `PptxrCustom`、`PptxrFileDecrypt`、`PptxOleManager.install`、`openOtherGlobalData` 全量、`PptxrCommentAuthors` | 矩阵项 `#[ignore = "spec: post-MVP"]`，不得算进 pass |

Rust 根路径前缀：`libs/ppt-core/src/cn/wps/show/pptxr/`。

## 必须 / 壳 / 延后

- **must：** 04 的打开顺序（readApp → getPresPart → pres.open → readCore → custom stub → 完成回调 → setReadOnly(false)）+ finally close + 两条异常语义；01 的 null part 跳过、parse 吞掉（readApp 继续 / readCore return）、各 getter null 不 put、日期字符串格式；02 的无 theme rel 不 setTheme + 缺 sldSz/notesSz 默认 + theme 先于 presentation.xml；03 的 slideCount==0/>0 两条回调与 `setInitialized(true)` 先于 slides.open。
- **stub：** `PptxTableStyles`（openGlobalData 有 rel 时 fixture skip）；`PptxrCustom.open` 调用点保序空体；`PptxOleManager.install` 调用点空体（PIPELINE.md §3）。
- **defer：** decrypt（`PptxrFileDecrypt` / `ModifyVerifier`）、custom xml、viewProps / commentAuthors（`openOtherGlobalData`）、notes / handout、timing 子树。

## 范围内 / 范围外

- **in：** `getPresPart` 后缀校验；打开顺序与失败语义；close / cleanup（package.close 失败仅 printStackTrace、encrypted 删临时文件）；readApp / readCore 字段写入语义；theme 先于 presentation.xml 的顺序（02）；slideCount==0 空白 slide（03）。
- **out：** decrypt / OLE / custom / viewProps / commentAuthors 业务体；SlideHandler / PptxrSlides 内部（pp-07 / pp-08）；渲染。

## 验收

FUNCTIONS.md §4：

1. readApp：各 getter null 不 put。
2. openGlobalData：无 theme rel 不 setTheme。
3. slideCount==0 与 >0 两条 initializing 回调。
4. PptxReader.close：package.close 失败只 printStackTrace（Java）；encrypted 删临时文件。

**建议 MVP pass 口径**（loop 采用）：04 必测是 getPresPart + 打开顺序 + close；custom / viewProps / commentAuthors
从必测移到附录（`#[ignore = "spec: post-MVP"]`）。

## 依赖

- **depends：** pp-04（`PptxrThemes.open` → `KmoTheme` + `setTheme`）；pp-06（`PresentationHandler` + callback.load 写 `Presentation`）；pp-08（`PptxrSlides.initialize` / `open` 装配顺序）。
- **does not depend：** 渲染；timing 子树（pp-05）；notes / handout；解密（pp-10 起）。

## 不过扩展

禁止实现 decrypt（`PptxrFileDecrypt`）。禁止重写 `PresentationHandler` / `SlideHandler` / `PptxrSlides` 内部（与其内部 parser 行为以 pp-06/07/08 为准）。
`PptxrCustom` / `openOtherGlobalData` 全量矩阵项 `#[ignore = "spec: post-MVP"]`，不得标本 function pass 直到忽略项
从必测拿掉或实现（FUNCTIONS.md §1）。