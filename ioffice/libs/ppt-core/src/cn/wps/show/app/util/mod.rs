// ★W9：`show/app/util/` 此前**没有 `mod.rs`** ⇒ 目录内文件从未进入编译单元（假编译面形态 H）。
//   其中 `ColorInterpretCtx.rs`（583 行、非自动桩、真实转写，含 16 处 `todo!`）
//   是本目录唯一**真实**文件；其余 4 个是自动生成桩（暂不登记，见下）。
// owner: show.app.util    wave: M-PPT-KMOSLIDE-W9
pub mod ColorInterpretCtx;
