// M-PPT-SHAPE-DISPATCH 2B：本节此前把 `Shade` / `PatternFill` / `RecolorInfo` / `GradientFill` /
//   `BlipFill` 五个**真身**模块全部置于 `#[cfg(feature = "full")]` 之下（假编译面形态 D：
//   默认构建下这些文件存在却永不编译）。实测阻塞如下，逐条可复现：
//     · `Fill.rs`（`pub mod Fill;`，**无 gate 却依赖上面五个**）的 `solidFill()` / `gradFill()` /
//       `blipFill()` / `pattFill()` 四个访问器原先体为 `todo!()`，故 gate 未暴露问题；
//       一旦按 Java 语义改成 `Color::open(bin)` / `GradientFill::open(bin)` /
//       `BlipFill::open(bin)` / `PatternFill::open(bin)`，五个模块必须进入默认构建
//       ⇒ 3 处 `E0432 unresolved import … BlipFill/GradientFill/PatternFill`。
//     · `FillMode` 仍是注释态（`// pub mod FillMode;`），其自身有 13 处
//       `todo!("lazy init requires &mut self")`。
//   ⇒ 本波**只放开 Fill 的四个直接依赖**（`GradientFill` / `PatternFill` / `BlipFill` / `Shade`
//      —— 后者是 `GradientFill` 的依赖），`RecolorInfo` 保持原样（改它需要 `RecolorEntry*`，
//      而那两个仍是注释态）。`FillMode` 的放开与 `todo!` 清偿登记为
//      **M-PPT-DATA-STUBS**（owner: ppt.drawing.data.fill    wave: M-PPT-SHAPE-DISPATCH）。
pub mod Shade;
pub mod PatternFill;
// pub mod FillMode;
#[cfg(feature = "full")]
pub mod RecolorInfo;
pub mod GradientFill;
// pub mod RecolorEntryBlush;
// pub mod RecolorEntryColor;
pub mod BlipFill;
pub mod Fill;
