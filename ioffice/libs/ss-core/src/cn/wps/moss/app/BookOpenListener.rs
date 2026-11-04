use super::{KmoBook::KmoBook,event::KmoBookOpenEventListener::KmoBookOpenEventListener};
// java public class BookOpenListener implements KmoBookOpenEventListener {
pub struct BookOpenListener;
impl KmoBookOpenEventListener for BookOpenListener{
    // java public void onProgressChange(byte progress) {}
    fn onProgressChange(&self,_progress:i8){}
    // java public void bookDidFinishInitializing(KmoBook book) {}
    fn bookDidFinishInitializing(&self,_book:&KmoBook){}
    // java public void activeSheetDidFinishInitializing() {}
    fn activeSheetDidFinishInitializing(&self){}
    // java public void sheetWillReadCurrentViewport(int sheetIndex) {}
    fn sheetWillReadCurrentViewport(&self,_sheetIndex:i32){}
    // java public void sheetDidFinishReadingCurrentViewport(int sheetIndex) {}
    fn sheetDidFinishReadingCurrentViewport(&self,_sheetIndex:i32){}
    // java public void sheetDidFinishReadingEverything(int sheetIndex) {}
    fn sheetDidFinishReadingEverything(&self,_sheetIndex:i32){}
    // java public void bookDidFinishReadingEverything() {}
    fn bookDidFinishReadingEverything(&self){}
}
