pub mod theme;
pub mod slides;
pub mod slidelayouts;
pub mod slidemasters;
pub mod CommentIdContext;
pub mod PresentationHandlerCallback;
// rust: 以下模块已按 Java 逐行改写完成（真实 part 读流 + 真实 XML 事件 + 真实后置状态），
// 但**尚未接入编译**：kfc 真实 OPC 类型（`OpcrPackagePart`/`OpcrRelationship`/`OpcrRelationships`）
// 是 `&mut self` + 不可 `Clone` 的引用语义（与 Java 一致），
// 端口必须以 `&mut` 借用 + 生命周期参数承载，而不是拥有值。
// 下一步按此口径重构后再打开（见 M_PPT_PRES_BASELINE §6）。
pub mod customXml;
pub mod media;
pub mod PresentationHandlerCallbackImpl;
pub mod PptxrCommentAuthors;
pub mod PptxrPresProps;
pub mod PptxrTableStyles;
pub mod PptxrViewProps;
pub mod PptxrPresentation;
