# wr-08-ffi-flutter — FFI 与 Flutter 绘制

内核仍在 word-core；本 feature 的 **Flutter 包不强制进 word-core crate**。  
推荐：`render_docx(bytes) -> RenderCommand[]` UniFFI/cxx 导出；Flutter `CustomPainter` 消费。

must：page0 文本可见；twip→logical px 一次换算。  
stub：滚动/缩放手势。  
defer：选中、编辑、与 Java Canvas 截图自动化。

**禁止** Flutter 依赖 Typo 内部结构。
