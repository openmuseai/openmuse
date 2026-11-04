use super::event::KmoBookChangeEventsListener::KmoBookChangeEventsListener;
use crate::{java_runtime::{Rc,RefCell},cn::wps::et::ss::calcchain::RecalcLock};
// java class KmoBookChangeListenerGroup implements KmoBookChangeEventsListener {
pub struct KmoBookChangeListenerGroup {
    // java private ArrayList<KmoBookChangeEventsListener> listeners = new ArrayList();
    listeners: RefCell<Vec<Option<Rc<dyn KmoBookChangeEventsListener>>>>,
    monitor: RecalcLock,
}
impl KmoBookChangeListenerGroup {
    pub fn new()->Self{Self{listeners:RefCell::new(vec![]),monitor:RecalcLock::new()}}
    // java public synchronized void add(KmoBookChangeEventsListener rhs) { this.listeners.add(rhs); }
    pub fn add(&self,rhs:Option<Rc<dyn KmoBookChangeEventsListener>>){let _m=self.monitor.lock();self.listeners.borrow_mut().push(rhs);}
    // java public synchronized void remove(KmoBookChangeEventsListener rhs) { this.listeners.remove(rhs); }
    pub fn remove(&self,rhs:Option<&Rc<dyn KmoBookChangeEventsListener>>){let _m=self.monitor.lock();let index=self.listeners.borrow().iter().position(|v|match(v,rhs){(None,None)=>true,(Some(a),Some(b))=>Rc::ptr_eq(a,b),_=>false});if let Some(i)=index{self.listeners.borrow_mut().remove(i);}}
    // java public synchronized void clear() { this.listeners.clear(); }
    pub fn clear(&self){let _m=self.monitor.lock();self.listeners.borrow_mut().clear();}
    // java public void bookDidFinishInitializing() {
    pub fn bookDidFinishInitializing(&self){
        // java int i = 0;
        let mut i=0;
        // java while(true) {
        loop{
            // java KmoBookChangeEventsListener l = null; synchronized(this) {
            let l={let _m=self.monitor.lock();
                // java if (i >= this.listeners.size()) { break; }
                if i>=self.listeners.borrow().len(){break;}
                // java l = (KmoBookChangeEventsListener)this.listeners.get(i);
                self.listeners.borrow()[i].clone()
            };
            // java if (l == null) { break; }
            let Some(l)=l else{break;};
            // java l.bookDidFinishInitializing();
            l.bookDidFinishInitializing();
            // java ++i;
            i+=1;
        }
    }
    // java public void bookSettingsDidFinishChanging() {
    pub fn bookSettingsDidFinishChanging(&self){
        // java int i = 0;
        let mut i=0;
        // java while(true) {
        loop{
            // java KmoBookChangeEventsListener l = null; synchronized(this) {
            let l={let _m=self.monitor.lock();
                // java if (i >= this.listeners.size()) { break; }
                if i>=self.listeners.borrow().len(){break;}
                // java l = (KmoBookChangeEventsListener)this.listeners.get(i);
                self.listeners.borrow()[i].clone()
            };
            // java if (l == null) { break; }
            let Some(l)=l else{break;};
            // java l.bookSettingsDidFinishChanging();
            l.bookSettingsDidFinishChanging();
            // java ++i;
            i+=1;
        }
    }
    // java public void sheetDidFinishActivating() {
    pub fn sheetDidFinishActivating(&self){
        // java int i = 0;
        let mut i=0;
        // java while(true) {
        loop{
            // java KmoBookChangeEventsListener l = null; synchronized(this) {
            let l={let _m=self.monitor.lock();
                // java if (i >= this.listeners.size()) { break; }
                if i>=self.listeners.borrow().len(){break;}
                // java l = (KmoBookChangeEventsListener)this.listeners.get(i);
                self.listeners.borrow()[i].clone()
            };
            // java if (l == null) { break; }
            let Some(l)=l else{break;};
            // java l.sheetDidFinishActivating();
            l.sheetDidFinishActivating();
            // java ++i;
            i+=1;
        }
    }
    // java public void bookContentsDidFinishChanging() {
    pub fn bookContentsDidFinishChanging(&self){
        // java int i = 0;
        let mut i=0;
        // java while(true) {
        loop{
            // java KmoBookChangeEventsListener l = null; synchronized(this) {
            let l={let _m=self.monitor.lock();
                // java if (i >= this.listeners.size()) { break; }
                if i>=self.listeners.borrow().len(){break;}
                // java l = (KmoBookChangeEventsListener)this.listeners.get(i);
                self.listeners.borrow()[i].clone()
            };
            // java if (l == null) { break; }
            let Some(l)=l else{break;};
            // java l.bookContentsDidFinishChanging();
            l.bookContentsDidFinishChanging();
            // java ++i;
            i+=1;
        }
    }
}
impl KmoBookChangeEventsListener for KmoBookChangeListenerGroup{
    fn bookDidFinishInitializing(&self){Self::bookDidFinishInitializing(self)}
    fn bookSettingsDidFinishChanging(&self){Self::bookSettingsDidFinishChanging(self)}
    fn bookContentsDidFinishChanging(&self){Self::bookContentsDidFinishChanging(self)}
    fn sheetDidFinishActivating(&self){Self::sheetDidFinishActivating(self)}
}
