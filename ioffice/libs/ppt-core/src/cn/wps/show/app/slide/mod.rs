pub mod FilmAtomData;
pub mod ISlide;
pub mod KmoFilm;
pub mod SlideAtomData;
pub mod SlideIdGenerator;
// M-PPT-KMOSLIDE SP5：`SlideAtomData.layout` / `.noteSlide` 的真实装配需要
// `KmoSlideLayout` / `KmoNoteSlide` 两个类真实存在。二者原先都是自动生成死壳
// （63 行 / 53 `todo!` 与 36 行 / 26 `todo!`）且**未登记**（假编译面形态 C），现按 Java 真身
// （293 行 / 243 行）连同其数据成员类 `LayoutAtomData`（Java 16 行）/`NoteSlideAtomData`
// （Java 14 行）一并重写并登记。停车项见两个文件内的 `owner + wave` 注释。
// owner: show.app.slide    wave: M-PPT-KMOSLIDE-SP5
pub mod KmoSlideLayout;
pub mod KmoNoteSlide;
pub mod LayoutAtomData;
pub mod NoteSlideAtomData;
// M-PPT-KMOSLIDE W6a（重试波）：`KmoSlide` 原为自动生成死壳（1934 行 / 547 个自动生成占位标记、
// 无一行 Java 转写）且被编译面门（`cfg(any())` 属性）排除在外（假编译面形态 C）。本波把它重写为
// 真实类并登记：与 pptxr 导入管线相关 + Java 核心访问器共 44 个成员逐语句转写，其余编辑态/UI 成员
// 按 Java 签名落位并以 `todo!("DEFERRED_NON_FUNCTION")` 停车（逐项 `Java: KmoSlide.java:NNN` +
// `owner + wave` 溯源）。裁定 R11（film 两段式构造）/ R12（presentation 类型槽，真身
// `show::app::KmoPresentation`）/ R10（本波不 `impl ISlide`，方法面已对齐，impl 留给 W6c）。
// 回归见 `tests/cn/wps/show/app/slide/01_function_kmoslide_body.rs`（`[[test]] 01_function_kmoslide_body`）。
// owner: show.app.slide    wave: M-PPT-KMOSLIDE-W6a
pub mod KmoSlide;
