pub mod PositiveSize2D;
pub mod Point2D;
#[cfg(feature = "full")]
pub mod IndexRange;
pub mod GroupTransform2D;
// M-PPT-SHAPE-DISPATCH 2B：`RelativeRect` 解除 `full` gate。
//   `fill::{Shade, GradientFill}.rs` 在模块顶层 `use …::basic::RelativeRect::RelativeRect;`，
//   而这两个模块原本也被 gate；一旦按 Java 语义放开（见 `fill/mod.rs` 说明），
//   gate 掉的 `RelativeRect` 立刻表现为 2 处 `E0432 unresolved import`。
//   `RelativeRect` 自身依赖仅为 `KoAr/KoBin/KoClosable/KoZone/KpBinBox/KpMap`（均已默认编译），
//   故可安全放开。owner: ppt.drawing.data.basic    wave: M-PPT-SHAPE-DISPATCH
pub mod RelativeRect;
pub mod Transform2D;
#[cfg(feature = "full")]
pub mod Vector;
