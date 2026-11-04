#![allow(non_snake_case)]
pub mod event;
pub mod shape;
// ★ M-PPT-KMOSLIDE SP2 实测结论（2026-09-26）：**本轮无法放开**，已回退为门禁态以保住绿构建。
//   实测：去掉 `#[cfg(feature = "full")]` ⇒ `KmoPresentation.rs` 自身 **147 个错误**（100% 集中于该文件），
//   且**无法靠解 feature 解决**（`cargo check --features full` 在 `libs/core-libs/thirdparty/picparser` 有 10 错）。
//   缺失依赖（默认构建下**根本不可达**，非本波可补）：
//     · `drawing.app.{DgCanvas, KmoProtocol, Ink}`      —— 未登记/不存在
//     · `kso.{exception, filter, framework}`            —— 模块不存在
//     · `drawing.data.text.TextListStyle`               —— `#[cfg(feature="full")]`（形态 D）
//     · `FileManager` / `IDLProvider` / `POIFSFileSystem` / `StatAnalytics` / `OnlineSecurityTool`
//       / `KmoShowTransaction` / `KmoPresentationOperator` / `ISlideCache` / `RandomAccessFile`
//       / `KmoTipStamp` / `IKmoBookProtect` / `IDocProtection` / `IMediaLib` / `GeneratorId`
//       / `ILogFileObserver`                           —— 均未移植
//   ⇒ `app::KmoPresentation`（Java 1274 行）的**依赖闭包 = 一个独立的大波次**，不属 SP2 边界。
//   ⇒ **裁定 R12**：本里程碑内 `KmoPresentation` 维持门禁态；`KmoSlide.presentation` 用**登记类型槽**
//      （见 SP6/W6a），真装配移交后续里程碑 **M-PPT-PRES-ASM2**。
//   owner: show.app    wave: M-PPT-PRES-ASM2
#[cfg(feature = "full")]
pub mod KmoPresentation;
#[cfg(feature = "full")]
pub mod theme;
pub mod slide;
pub mod KsoWMInfo;
// M-PPT-KMOSLIDE SP4：`show/app/tag/` 原先整目录缺失 ⇒ `KmoSlide.slideTags()`/`structureTags()`
// 的返回类型（`KmoSlideBeautifyTags`/`KmoStructureTags`）无法命名。本波按 Java 路径新建该包并登记。
// owner: show.app.tag    wave: M-PPT-KMOSLIDE-SP4
pub mod tag;
// M-PPT-KMOSLIDE SP3：`show/app/text/` 原先整目录未登记（3 个 form-E 自动桩 + 无 `mod.rs`）⇒
// `KmoSlide.textStyleManager()`（成员 D3）的返回类型 `KmoTextStyleManager` 无法命名。本波按 Java 路径
// `cn.wps.show.app.text` 落地 6 个真类并登记。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
pub mod text;
// M-PPT-KMOSLIDE SP2c：`show/app/media/` 原先 3 个 form-E 自动桩 + 无 `mod.rs`（整目录不可达 ⇒ 死代码）
// ⇒ `KmoFilm` 的 4 个停车成员（`updatePicRef4Copy` / `compareMedia(KmoFilm)` /
// `compareMedia(Blip,Blip,KmoMediaLib,KmoMediaLib)`）的阻塞类型 `KmoMediaLib`/`KmoMediaData` 无法命名。
// 本波按 Java 真身（261/115/73 行）落地并登记。owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
pub mod media;
// ★W9 → ★W6c-4：`show/app/util/` 此前**整目录无 `mod.rs`**，且本文件也未声明它（双重不可达）
// ⇒ 目录内 `ColorInterpretCtx.rs`（583 行、真实转写、非自动桩）从未进入编译单元（形态 H）。
//   已补 `util/mod.rs`（见该文件）。原先的阻塞是 `ISlide` 的形态 F 同名空壳：
//   `ISlide.rs` 内联声明 `KmoTheme`/`KsoWMInfo`/`ColorMapping`/`Background` 四个单元空壳，
//   使 `ISlide::colorMapping()` 等返回**假类型**，而 `ColorInterpretCtx::setSlide(&dyn ISlide)`
//   需要真类型。**W6c-4 已把其中 3 个改为再导出真身**（`KsoWMInfo` / `ColorMapping` /
//   `Background`；`KmoTheme` 仍待 `theme` 模块解锁，见 `ISlide.rs` 登记），故本目录现已可登记。
//   owner: show.app.util    wave: M-PPT-KMOSLIDE-W6c-4
pub mod util;
#[cfg(feature = "full")]
pub mod io;
#[cfg(any())]
pub mod table;
