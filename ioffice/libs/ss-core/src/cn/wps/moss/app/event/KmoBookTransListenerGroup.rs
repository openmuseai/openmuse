use super::KmoBookTransListener::KmoBookTransListener;
use crate::java_runtime::{Rc,RefCell};
// java public class KmoBookTransListenerGroup implements KmoBookTransListener {
pub struct KmoBookTransListenerGroup{listeners:RefCell<Vec<Option<Rc<dyn KmoBookTransListener>>>>}
impl KmoBookTransListenerGroup{
    pub fn new()->Self{Self{listeners:RefCell::new(vec![])}}
    // java public void addListener(KmoBookTransListener listener) { if (!this.listeners.contains(listener)) { this.listeners.add(listener); } }
    pub fn addListener(&self,listener:Option<Rc<dyn KmoBookTransListener>>){let exists=self.listeners.borrow().iter().any(|v|match(v,&listener){(None,None)=>true,(Some(a),Some(b))=>Rc::ptr_eq(a,b),_=>false});if !exists{self.listeners.borrow_mut().push(listener);}}
    // java public void removeListener(KmoBookTransListener listener) { this.listeners.remove(listener); }
    pub fn removeListener(&self,listener:Option<&Rc<dyn KmoBookTransListener>>){let i=self.listeners.borrow().iter().position(|v|match(v,listener){(None,None)=>true,(Some(a),Some(b))=>Rc::ptr_eq(a,b),_=>false});if let Some(i)=i{self.listeners.borrow_mut().remove(i);}}
    // java public void onUndo() { int i = 0; for(int size = this.listeners.size(); i < size; ++i) { ((KmoBookTransListener)this.listeners.get(i)).onUndo(); } }
    pub fn onUndo(&self){let size=self.listeners.borrow().len();for i in 0..size{let l=self.listeners.borrow()[i].clone().expect("NullPointerException transaction listener");l.onUndo();}}
    // java public void onRedo() { int i = 0; for(int size = this.listeners.size(); i < size; ++i) { ((KmoBookTransListener)this.listeners.get(i)).onRedo(); } }
    pub fn onRedo(&self){let size=self.listeners.borrow().len();for i in 0..size{let l=self.listeners.borrow()[i].clone().expect("NullPointerException transaction listener");l.onRedo();}}
}
impl KmoBookTransListener for KmoBookTransListenerGroup{fn onUndo(&self){Self::onUndo(self)}fn onRedo(&self){Self::onRedo(self)}}
