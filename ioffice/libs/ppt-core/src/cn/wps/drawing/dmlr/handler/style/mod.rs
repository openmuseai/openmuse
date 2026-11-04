pub mod ColorMapHandler;
pub mod ThemeHandler;
// M-PPT-KMOSLIDE W6c-3：Java 真身 `cn.wps.drawing.dmlr.handler.style.StyleMatrixReferenceHandler`
//   （42 行）在 Rust 侧原先**没有文件**（`BackgroundHandler` 的 3211604 分支因此不可实现）。
//   本波按其真身移植并登记；前置 `DGMap.fontCollectionIndexMap` 同波补齐。
//   owner: ppt.drawing.dmlr.handler.style    wave: M-PPT-KMOSLIDE-W6c-3
pub mod StyleMatrixReferenceHandler;
