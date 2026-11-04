#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// Java: import cn.wps.kso.filter.KsoFilterEvent
// Java: import cn.wps.moffice.io.FirstPageForceQuitException
// Java: import cn.wps.show.app.KmoPresentation
// rust: 偏差登记 —— Java 形参类型是 `cn.wps.show.app.KmoPresentation`，Rust 侧实际载体取
//   `show/util/layout_shape::KmoPresentation`（既有接线：`PptxrPresentation::mPres` 与
//   `PptxrPresentation::docDidFinishInitializing` 均以该类型调用本 trait 方法）。
//   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
use crate::cn::wps::show::util::layout_shape::KmoPresentation;

// java public interface KmoPresOpenEventListener extends KsoFilterEvent
//
// `rust:` Java 里这是接口（父接口 `KsoFilterEvent` 属 `cn.wps.kso.filter`，
// 在 default feature 下不可达），Rust 侧用 trait 表达同一契约，方法名与 Java 逐一对齐。
pub trait KmoPresOpenEventListener {
    // java void docDidFinishInitializing(KmoPresentation var1, boolean var2)
    fn docDidFinishInitializing(&mut self, pres: &mut KmoPresentation, hasBlankSlide: bool);
    // java void slideDidFinishReadingEverything(int var1) throws FirstPageForceQuitException
    fn slideDidFinishReadingEverything(&mut self, slideIndex: i32);
    // java void docDidFinishReadingEverything()
    fn docDidFinishReadingEverything(&mut self);
}
