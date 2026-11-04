#![allow(non_snake_case)]
#[cfg(feature = "full")]
pub mod diagrams;
// rust: ★W6c-10 —— `drawing/app/graphic`（GraphicObject 接口 + GraphicStore 注册表）是
//   `DgStore` 的**默认构建依赖**（`KmoShowShape::graphic()` 的真实读取链），解除 `full` 门禁。
//   模块仅两个文件、自包含（无全门后依赖），解除后 `cargo check --lib` = 0（实测）。
//   owner: cn.wps.drawing.app.graphic    wave: M-PPT-KMOSLIDE-W6c-10
pub mod graphic;
#[cfg(feature = "full")]
pub mod KmoProtocol;
#[cfg(feature = "full")]
pub mod eval;
#[cfg(feature = "full")]
pub mod chart;
#[cfg(feature = "full")]
pub mod util;
pub mod DgStoreData;
#[cfg(feature = "full")]
pub mod format;
pub mod DgStore;
pub mod shape;
#[cfg(any())]
pub mod text;
#[cfg(any())]
pub mod table;
#[cfg(any())]
pub mod color;
