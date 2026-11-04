// M-PPT-KMOSLIDE SP4：`show/app/tag/` 目录原先不存在（Java 包 `cn.wps.show.app.tag` 有 12 个类），
// 本波按 Java 门面（路径 + 类名 + 方法名）新建并登记本计划 SP4 的两个目标类：
//   · `KmoSlideBeautifyTags.java`（220 行） → `KmoSlideBeautifyTags.rs`
//   · `KmoStructureTags.java`（58 行）      → `KmoStructureTags.rs`
// Java 同包其余类（`KmoBeautifyTags` 495 行 / `KmoShapeBeautifyTags` 741 行 / `KmoShapeBeautifyTagsTree`
// 517 行 / `KmoFilmBeautifyTags` 81 行 / `KmoDiagramBeautifyTags` / `KmoApplyTagsHelper` /
// `KmoBeautifyOperator` / `KmoBeautifyTagMap` / `KmoBeautifyTextHelper` / `KmoBeautifyPrompt` /
// `BeautifyCallback`）依赖链未闭环，本波**不登记**（避免假编译面），逐项停车注记见两个文件内的
// `DEFERRED_NON_FUNCTION` 注释（SP4 子计划 §4.4 / §7 R1：>200 行则拆 SP4b）。
// owner: show.app.tag    wave: M-PPT-KMOSLIDE-SP4
pub mod KmoSlideBeautifyTags;
pub mod KmoStructureTags;
