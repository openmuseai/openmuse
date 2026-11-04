// java package cn.wps.moss.app.event;
use crate::cn::wps::moss::app::KmoBook::KmoBook;
// java public interface KmoBookOpenEventListener
pub trait KmoBookOpenEventListener: Send + Sync {
    fn onProgressChange(&self, _progress: i8) {}
    fn bookDidFinishInitializing(&self, _book: &KmoBook) {}
    fn activeSheetDidFinishInitializing(&self) {}
    fn sheetWillReadCurrentViewport(&self, _sheet: i32) {}
    fn sheetDidFinishReadingCurrentViewport(&self, _sheet: i32) {}
    fn sheetDidFinishReadingEverything(&self, _sheet: i32) {}
    fn bookDidFinishReadingEverything(&self) {}
}
