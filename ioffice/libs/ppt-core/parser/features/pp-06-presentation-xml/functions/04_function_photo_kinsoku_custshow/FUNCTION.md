# 04_function_photo_kinsoku_custshow — photoAlbum / kinsoku / custShow

> feature: pp-06-presentation-xml ｜ 状态：**pass**

## 0. 管线能力解锁

实现后管线新增：photoAlbum/kinsoku 属性写入 model、custShowLst 经 `onCustomShow` 回调与 `customShowList` write 落地，presentation.xml 根解析收尾。缺失时断在哪：仅相册/禁则/自定义放映等播放侧信息缺失，不阻塞首屏，但属 pp-06 根交付的一部分，整体缺失则 pp-08/09 打开链前提不齐（闭环打开必要）。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

(photoAlbum/kinsoku attrs; custShow rIds) -> model + onCustomShow

### 1.1 I/O 契约

| 方向 | 契约 |
|---|---|
| 输入 | `p:photoAlbum`(3211518) attrs：bw(3211484)/showCaptions(3211485)/layout(3211486)/frame(3211487)；`p:kinsoku`(3211520) attrs：lang(3211490)/invalStChars(3211491)/invalEndChars(3211492)；`p:custShowLst`(3211517) 子树：`p:custShow`(3211443) attrs id(3211342)/name(3211449)、`p:sldLst`(3211483) item `p:sld`(3211440) attrs id(3211342) 或 `r:id`(4259841)。id 均按 `kfc/xsd/pml/P.java`/`kfc/xsd/shared/R.java` 查证 |
| 输出 | `photoAlbum()`：putBlackAndWhite/putShowCaptions(boolean)、putLayout(经 `photoAlbumLayoutMap`)、putFrame(经 `photoAlbumFrameShapeMap`)；`kinsoku()`：putLang/putInvalStChars/putInvalEndChars(String)；custShow：`Presentation.CustomShow`（putId/putName + slideList rId 字符串列表）经 `callback.onCustomShow(slideListEntryList, customShow)`，且 `presData.customShowList().write(customShowList)` |
| 状态与副作用 | `customShowList` 懒初始化（首个 `custShow`(3211443) 才建 `ArrayList`）；`CustomShowListHandler.endElement` 仅 list != null 时逐项 `save()` 后 `write()`；`CustomShowHandler.endElement` 仅 `sldLst` 出现过（`slideListEntryList != null`）才调用 `onCustomShow`；`SlideListEntryHandler` 取 id(3211342) 或 `r:id`(4259841) 任一命中即收集 |
| 错误语义 | layout/frame map miss → `photoAlbumLayoutMap.get`/`photoAlbumFrameShapeMap.get` 返回 null，`putLayout(int)`/`putFrame(int)` 拆箱 NPE（Java 事实，测试用合法枚举，不发明更安全的 Rust）；photoAlbum/kinsoku 的 `getByName` 均判空读写；空 `custShowLst`（无 custShow 子元素）→ 不 write、无 NPE |

### 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| 见 FUNCTIONS.md §6 | | | loop 前未接线 | `todo!("DEFERRED_NON_FUNCTION")` 停车非改写列表 | 不改已 pass 的 pp-01/pp-02 语义 |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs`（尚不存在） |
| `koffice/presentation/core/.../pptxr/handler/custdatas/CustDataLstHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/custdatas/CustDataLstHandler.rs`（已存在） |

### 2.1 Java nested handler → Rust 目标（按 Java 源码查证）

| Java nested（PresentationHandler.java） | 说明 | Rust 目标 |
|---|---|---|
| `PhotoAlbumHandler` | 4 attr：bw(3211484)→putBlackAndWhite；frame(3211487)→putFrame(`photoAlbumFrameShapeMap`)；layout(3211486)→putLayout(`photoAlbumLayoutMap`)；showCaptions(3211485)→putShowCaptions。无 enter/end 重载 | `PresentationHandler.rs` 内 nested |
| `KinsokuHandler` | 3 attr：invalEndChars(3211492)→putInvalEndChars；invalStChars(3211491)→putInvalStChars；lang(3211490)→putLang | 同上 |
| `CustomShowListHandler` | 字段 `customShowList = null` 懒初始化；`enterSubElement` case 3211443 → `CustomShow.create()` + 新 `CustomShowHandler`；`endElement` list != null 才逐项 save + `customShowList().write(list)` | 同上；懒初始化 → `Option<Vec<CustomShow>>` |
| `CustomShowHandler`（nested） | attrs：id(3211342)→putId(toInt)、name(3211449)→putName(toStr)；`enterSubElement` case 3211483 → 懒建 `slideListEntryList` + `SlideListHandler`；`endElement` 仅 `slideListEntryList != null` 才 `onCustomShow(list, customShow)` | 同上 |
| `SlideListHandler`（nested） | `enterSubElement` case 3211440(sld) → `SlideListEntryHandler`（其它元素返回 null） | 同上 |
| `SlideListEntryHandler`（nested） | `startElement`：`getByName(3211342 id)` 或 `getByName(4259841 r:id)` 任一命中 → 收集到共享 `slideListEntryList` | 同上 |
| `CustDataLstHandler` | **空类**（`extends DefaultElementHandler {}` 无成员） | `custdatas/CustDataLstHandler.rs` 接线（已存在，独立包） |
| `EmbeddedFontListHandler` 树（nested） | embeddedFontLst(3211516)/embeddedFont(3211482)/font(3211477)/regular(3211478)/bold(3211479)/italic(3211480)/boldItalic(3211481)；data id 取 3211342 或 4259841 → `onTextFontRegularData` 等四回调 | 可随 04（FUNCTIONS.md §1 注）或暂缓 constructor fixture |
| nested `ModifyVerifierHandler` | modifyVerifier(3211522) + cryptAlgorithmSid(3211500)/hashData(3211503)/saltData(3211502)/spinCount(3211501) 密码循环 | **移出**（加密，不进改写列表） |

改写列表：`PhotoAlbumHandler`、`KinsokuHandler`、`CustomShowListHandler` 树（`CustomShowHandler`/
`SlideListHandler`/`SlideListEntryHandler`）、`CustDataLst` 空类；`EmbeddedFontListHandler` 树可随 04
或暂缓 fixture；`ModifyVerifierHandler` 暂缓。`SWReaderMap` 查表（SWReaderMap.java）：
`photoAlbumLayoutMap` {fitToSlide→0、1pic→1、2pic→2、4pic→3、1picTitle→4、2picTitle→5、4picTitle→6}、
`photoAlbumFrameShapeMap` {frameStyle1..7 → 0..6}。

## 3. 测试方案

### 3.1 代码审计与测试面

写 tests 前必须通读上表 Java 方法体与当前 Rust（若尚无 Rust 则只读 Java）。审计面：

1. `PhotoAlbumHandler`：4 attr 独立判空；layout(3211486)/frame(3211487) 查表后拆箱（map miss → NPE 留档）。
2. `KinsokuHandler`：3 attr 判空 → `Presentation.Kinsoku.putLang/putInvalStChars/putInvalEndChars`
   （Presentation.java 行 2619/2640/2661 查证）。
3. `CustomShowListHandler` 懒初始化链：list 为 null → `enterSubElement(3211443)` 才建；`endElement`
   list == null 不 save 不 write。
4. `CustomShowHandler`：id(3211342)→`putId(int)`、name(3211449)→`putName(String)`（行 1695/1716 查证）；
   sldLst(3211483) 出现才建 `slideListEntryList`；`endElement` 它 != null 才 `onCustomShow`。
5. `SlideListEntryHandler`：id(3211342) 或 r:id(4259841) 任一命中即收集（`||` 短路，第一个命中即用）。
6. EmbeddedFont 回调签名（`PresentationHandlerCallback`）：`onTextFontRegularData`/
   `onTextFontBoldData`/`onTextFontItalicData`/`onTextFontBoldItalicData(String rid, EmbeddedFontListEntry)`。

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | photoAlbum 四属性写入 + layout/frame 查表（合法枚举；map miss → NPE 留档，不发明更安全的 Rust） | 分支 | 是 | T01 |
| C02 | kinsoku 三字符串写入 | 分支 | 是 | T02 |
| C03 | custShow id/name + sldLst 的 sld rId 列表 → `onCustomShow(list, customShow)` + `customShowList().write` | 交互 | 是 | T03 |
| C04 | 空 customShowLst（无 custShow）→ 不 write、不回调 | 边界 | 是 | T04 |
| C05 | sld item 双 id 路径：id(3211342) 或 r:id(4259841) | 分支 | 是 | T05 |
| C06 | custShow 无 sldLst 子元素 → 不回调 `onCustomShow`（slideListEntryList==null） | 边界 | 是 | T06 |

### 3.3 可测试性

内存 attrs/events 或假 part/rels。不跑完整 PptxReader（本 function 不是 pp-09 编排项时）。
Recording `PresentationHandlerCallback` 记录 `onCustomShow` 的 `(List<String>, CustomShow)`；
`Presentation.CustomShow`/`PhotoAlbum`/`Kinsoku` 用 Rust 数据模型（`Presentation.rs` 内）直接断言。
photoAlbum/kinsoku 事件序列与 custShow 序列可分夹具喂入同一个 handler 实例。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 | 失败语义 |
|---|---|---|---|---|
| T01 | C01 | photoAlbum 带 bw="true" showCaptions="false" layout="1pic" frame="frameStyle2" | b/w=true、showCaptions=false、layout=1、frame=1；查表命中 | 红测停在本 function |
| T02 | C02 | kinsoku 带 lang="zh-CN" invalStChars="…" invalEndChars="…" | 三字符串写入 `kinsoku()` 字段（putLang/putInvalStChars/putInvalEndChars） | 红测停在本 function |
| T03 | C03 | custShowLst 含 1 个 custShow（id/name + sldLst 2 个 `p:sld[@r:id]`） | `onCustomShow` 收到 (2 个 rId, customShow)；`customShowList().write` 被调且含该 custShow | 红测停在本 function |
| T04 | C04 | custShowLst 空（无 custShow 子元素） | 无 write、无回调，无 panic | 红测停在本 function |
| T05 | C05 | sldLst 内 `p:sld` 分别用 `id` 与 `r:id` 提供 | 两类 attrs 都被收集到同一列表 | 红测停在本 function |
| T06 | C06 | custShow 只有 id/name、无 sldLst | `onCustomShow` 不被调用；`customShowList().write` 仍执行（list 非空） | 红测停在本 function |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/pptxr/handler/PresentationHandler.rs
test:  libs/ppt-core/tests/<mirrors src relpath>
[[test]] name = "04_function_photo_kinsoku_custshow"
```

Loop L2 才创建 `tests/` 与 `[[test]]`。本 split 不落可执行测试。

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_photo_kinsoku_custshow

bash .codex/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_photo_kinsoku_custshow \
  --src src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/04_function_photo_kinsoku_custshow/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_photo_kinsoku_custshow --json`)
date: 2026-09-01 17:53

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | 227/633 | 35.9% | 336/912 | 36.8% |
| **function total** | **227/633** | **35.9%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java
```

Loop 结果：**PASS**

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs \
  --report libs/ppt-core/parser/features/pp-06-presentation-xml/functions/04_function_photo_kinsoku_custshow/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-01 09:53

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `PresentationHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/PresentationHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/PresentationHandler.rs` | **PASS** | `PresentationHandler`, `startElement`, `endElement`, `enterSubElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
