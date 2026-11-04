use crate::java_runtime::{Rc,RefCell,Cell};
use super::{KmoBook::KmoBook,event::KmoBookOpenEventListener::KmoBookOpenEventListener};
// java class KmoBookOpenEventListenerGroup implements KmoBookOpenEventListener {
pub struct KmoBookOpenEventListenerGroup{
    // java private ArrayList<KmoBookOpenEventListener> listeners=new ArrayList();
    listeners:RefCell<Vec<Option<Rc<dyn KmoBookOpenEventListener>>>>,
    // Native ArrayList iterator state; callbacks may mutate the same Java list.
    modCount:Cell<i32>,
}
#[cfg(test)]
#[path="KmoBookOpenEventListenerGroup.review.rs"]
mod review;
impl KmoBookOpenEventListenerGroup{
    pub fn new()->Self{Self{listeners:RefCell::new(vec![]),modCount:Cell::new(0)}}
    // java public void add(KmoBookOpenEventListener listener){this.listeners.add(listener);}
    pub fn add(&self,listener:Option<Rc<dyn KmoBookOpenEventListener>>){self.listeners.borrow_mut().push(listener);self.modCount.set(self.modCount.get().wrapping_add(1));}
    // java public void clear(){this.listeners.clear();}
    pub fn clear(&self){self.listeners.borrow_mut().clear();self.modCount.set(self.modCount.get().wrapping_add(1));}
    // Native transport of Java ArrayList's enhanced-for Iterator, without making
    // a snapshot of the listener list or holding a field lock across callbacks.
    fn forEach(&self,f:impl Fn(&dyn KmoBookOpenEventListener)){
        let expected=self.modCount.get();let mut cursor=0;
        // Android ArrayList.Itr.limit is captured at iterator construction;
        // hasNext compares cursor with limit, even after a callback clears it.
        let limit=self.listeners.borrow().len();
        while cursor<limit{
            assert!(expected==self.modCount.get(),"ConcurrentModificationException");
            let next=self.listeners.borrow().get(cursor).unwrap_or_else(||panic!("NoSuchElementException")).clone();cursor+=1;
            f(next.as_deref().expect("NullPointerException"));
        }
    }
}
impl KmoBookOpenEventListener for KmoBookOpenEventListenerGroup{
    // java public void onProgressChange(byte progress){for(KmoBookOpenEventListener i:this.listeners){i.onProgressChange(progress);}}
    fn onProgressChange(&self,progress:i8){self.forEach(|i|i.onProgressChange(progress));}
    // java public void bookDidFinishInitializing(KmoBook book){for(KmoBookOpenEventListener i:this.listeners){i.bookDidFinishInitializing(book);}}
    fn bookDidFinishInitializing(&self,book:&KmoBook){self.forEach(|i|i.bookDidFinishInitializing(book));}
    // java public void activeSheetDidFinishInitializing()throws FirstPageForceQuitException{for(KmoBookOpenEventListener i:this.listeners){i.activeSheetDidFinishInitializing();}}
    fn activeSheetDidFinishInitializing(&self){self.forEach(|i|i.activeSheetDidFinishInitializing());}
    // java public void sheetWillReadCurrentViewport(int sheetIndex){for(KmoBookOpenEventListener i:this.listeners){i.sheetWillReadCurrentViewport(sheetIndex);}}
    fn sheetWillReadCurrentViewport(&self,sheetIndex:i32){self.forEach(|i|i.sheetWillReadCurrentViewport(sheetIndex));}
    // java public void sheetDidFinishReadingCurrentViewport(int sheetIndex){for(KmoBookOpenEventListener i:this.listeners){i.sheetDidFinishReadingCurrentViewport(sheetIndex);}}
    fn sheetDidFinishReadingCurrentViewport(&self,sheetIndex:i32){self.forEach(|i|i.sheetDidFinishReadingCurrentViewport(sheetIndex));}
    // java public void sheetDidFinishReadingEverything(int sheetIndex){for(KmoBookOpenEventListener i:this.listeners){i.sheetDidFinishReadingEverything(sheetIndex);}}
    fn sheetDidFinishReadingEverything(&self,sheetIndex:i32){self.forEach(|i|i.sheetDidFinishReadingEverything(sheetIndex));}
    // java public void bookDidFinishReadingEverything(){for(KmoBookOpenEventListener i:this.listeners){i.bookDidFinishReadingEverything();}}
    fn bookDidFinishReadingEverything(&self){self.forEach(|i|i.bookDidFinishReadingEverything());}
}
