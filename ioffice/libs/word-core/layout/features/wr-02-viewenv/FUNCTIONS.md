# wr-02-viewenv — 视图参数（页尺寸/缩放/模式）

> 分析日期：2026-08-22 · Java：`writer/core/layout/base/env/ViewEnv.java` + `writer/core/data/value/PageSetup.java`
> 输入端点：ViewSettings/设备密度 + 分节 PageSetup
> 输出：ViewEnv（页宽高/边距/模式/缩放）——换行与分页的唯一视图输入
> 切分依据：I/O 边界（页面几何常量 → 视图状态）

## 1. Java 管线分析

### 1.1 ViewEnv（layout/base/env/ViewEnv.java）
字段/方法：`getViewWidth/getViewHeight/setViewWidth/Height`、`getViewMode/setViewMode`（`PAGEVIEW=0/WEBVIEW=1/PHONEVIEW=2/PHONEVIEWSCROLL=3/SHARE=4`，见 MainLayouter 常量）、`isPageMode/isPhoneReadMode/isWebLayoutMode`、`onScroll(pos)`/`getViewTop`/`isScrollUp/isScrollDown`、`getWebViewMarginTop/Left/Right`、`getPhoneViewIndent`、`getPageBreakWidth`、`getContentLengthToBallonsLengthRatio`、`setViewSettings(IViewSettings)`。**MVP 只取：页宽/高（内容区）、模式=页模式、缩放=1.0、单列。**

### 1.2 PageSetup（writer/core/data/value/PageSetup.java）
节属性（wc-05 06 默认节已写 `PageSetup(11906,16838,1800,1800,1440,1440)` 到 prop 676）：页宽 11906 缇、页高 16838、边距 1800/1800/1440/1440。内容区宽 = 页宽 − 左右边距；高 = 页高 − 上下边距。

### 1.3 流程（钉死）
```text
ViewEnv ← setViewSettings(IViewSettings) ＋ 文档节 PageSetup
  → contentWidth = pageWidth − marginLeft − marginRight   (缇→像素按 zoom)
  → contentHeight = pageHeight − marginTop − marginBottom
FlowLayout.changeSize(needClearFontStyleCache) ──视图变化──→ 重排版触发
```

### 1.4 Java↔Rust 现状对照
Rust 无 ViewEnv；`wc-05 06` 默认节（678=312/677=0/682=720/683=720/676=PageSetup）已实现 → 页几何来源已具备。

## 2. Function 清单
| order | function_id | 一句话 | 输入 | 输出 | 上游 | 下游 |
|------|-------------|--------|------|------|------|------|
| 1 | [01_function_page_setup](./functions/01_function_page_setup/FUNCTION.md) | 默认节页几何（缇→内容区） | PageSetup 常量/PropertySet 676 | contentWidth/Height（缇） | wc-05 06 默认节 | 02; wr-04/05 |
| 2 | [02_function_view_env](./functions/02_function_view_env/FUNCTION.md) | 视图状态（宽高/模式/缩放换算） | viewWidth/Height + mode + zoom | ViewEnv 换算后的内容区 | 01 | wr-04/05/07 |

## 3. 依赖与编译面
src（loop 新建）：`libs/word-core/src/cn/wps/moffice/writer/layout/base/env/ViewEnv.rs` + `libs/word-core/src/cn/wps/moffice/writer/data/value/PageSetup.rs` 扩展（PageSetup 是否已有 rs：需 audit data/value/——若已有则只加换算函数）。crate `libs/word-core`。

## 4. 验收顺序
```text
01 page_setup（常量钉死：11906/16838/1800/1440；缇换算；缺省节）
02 view_env  （设置/读取；content 计算；changeSize 触发标志）
```