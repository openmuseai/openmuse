// M-PPT-KMOSLIDE SP2c：`show/app/media/`（Java 包 `cn.wps.show.app.media`）原先只有 3 个 form-E 自动桩
// （`use crate::*;` + 每个成员 `todo!()`）且**没有 `mod.rs`** ⇒ 整个目录是不可达死代码，
// 以致 `KmoFilm::updatePicRef4Copy()` / `KmoFilm::compareMedia(Blip, Blip, KmoMediaLib, KmoMediaLib)`
// 的 `KmoMediaLib` / `KmoMediaData` 形参类型无法命名（该波以 `todo!("DEFERRED_NON_FUNCTION")` 停车）。
// 本波按 Java 真身重写三件并登记：`KmoMediaLib`（261 行）/ `KmoMediaData`（115 行）/ `KmoMediaUtil`（73 行）；
// 真阻塞的停车项见各文件内的 `owner + wave` 注释。
// owner: show.app.media    wave: M-PPT-KMOSLIDE-SP2c
pub mod KmoMediaData;
pub mod KmoMediaLib;
pub mod KmoMediaUtil;
