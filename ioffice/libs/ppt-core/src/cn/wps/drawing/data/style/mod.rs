// M-PPT-KMOSLIDE SP0：`show.data.slides.Slide.clrMap` 需要真实 `drawing.data.style.ColorMapping`。
// 原先该模块被 `#[cfg(feature = "full")]` 关在默认构建之外，使 `Slide.rs`（已登记、真实实现）
// 在默认构建下不可达 —— 属「假的编译面」，此处放开。
pub mod ColorMapping;
#[cfg(feature = "full")]
pub mod ColorScheme;
#[cfg(feature = "full")]
pub mod EffectStyle;
#[cfg(feature = "full")]
pub mod FontCollection;
#[cfg(feature = "full")]
pub mod FontRef;
pub mod StyleMatrixReference;
pub mod ShapeStyle;
#[cfg(feature = "full")]
pub mod StyleMatrix;
#[cfg(feature = "full")]
pub mod FontScheme;
#[cfg(feature = "full")]
pub mod Theme;
