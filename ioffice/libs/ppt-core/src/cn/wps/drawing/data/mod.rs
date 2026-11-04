#![allow(non_snake_case)]
// M-PPT-SHAPE-DISPATCH 2B：本文件的 `#[cfg(feature = "full")]` gate 是**假编译面形态 D** ——
//   文件在树中却永不参与默认构建。`data::outline` 与 `data::geometry` 已在本步解除，
//   理由是它们正是**真 `ShapePr` 的落点命名空间**：
//     `ShapePr.rs:37 pub(crate) geom: Option<Box<Geometry2D>>,`   ← `data::geometry`
//     `ShapePr.rs:41 pub(crate) ln:   Option<Box<Outline>>,`      ← `data::outline`
//   在此之前默认构建里 `geom`/`ln` **根本没有目标类型**，`sppr/geometry` 与 `sppr/outline`
//   的 handler 无处写入 —— 这正是 PLAN §27.3「真树结构真、属性不真」的机制。
//   实测：解除二者后 `cargo check --lib` = 0、`cargo test --no-run` = 0（两者自身无新增错误）。
//
//   仍保持 gate 的模块及其**实测理由**（清偿前不得放开）：
//     · `effect`  —— `ShapePr.effect` 的目标命名空间；其自身依赖仍待测（下一步）。
//     · `blip` / `font` / `media` / `threeD` / `table` / `model3d` —— 非本波核心，保持原状。
//   owner: ppt.drawing.data    wave: M-PPT-SHAPE-DISPATCH
pub mod basic;
pub mod color;
pub mod effect;
pub mod fill;
pub mod geometry;
#[cfg(feature = "full")]
pub mod blip;
#[cfg(feature = "full")]
pub mod font;
pub mod outline;
#[cfg(feature = "full")]
pub mod media;
#[cfg(feature = "full")]
pub mod threeD;
pub mod hyperlink;
pub mod style;
pub mod shape;
pub mod text;
#[cfg(feature = "full")]
pub mod table;
#[cfg(feature = "full")]
pub mod model3d;
