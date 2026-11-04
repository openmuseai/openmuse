// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.event.KmoWriterOpenEventListener
use crate::cn::wps::moffice::writer::event::KmoWriterOpenEventListener::*;

// java public class KmoWriterOpenEventListenerGroup implements KmoWriterOpenEventListener
#[derive(Default)]
pub struct KmoWriterOpenEventListenerGroup {
    // java private ArrayList<KmoWriterOpenEventListener> listeners = new ArrayList();
    // rust: Java `ArrayList<接口>` 元素是**对象引用**（可共享可变），Rust 以
    //   `Rc<RefCell<dyn KmoWriterOpenEventListener>>` 表达同一份实例；
    //   `remove(Object)` 的 Java `equals` 对监听器就是**引用同一性** ⇒ `Rc::ptr_eq` 精确等价。
    listeners: Vec<std::rc::Rc<std::cell::RefCell<dyn KmoWriterOpenEventListener>>>,
}

// java public interface KmoWriterOpenEventListener
impl KmoWriterOpenEventListener for KmoWriterOpenEventListenerGroup {
    // java public void docDidFinishReadingEverything()
    fn docDidFinishReadingEverything(&mut self) {
      // java int i = 0;
      let mut i: i32 = 0;
      let size: i32 = (self.listeners.len() as i32);
      // java for(int size = this.listeners.size(); i < size; ++i)
      while i < size {
        // java ((KmoWriterOpenEventListener)this.listeners.get(i)).docDidFinishReadingEverything();
        self.listeners[(i) as usize].borrow_mut().docDidFinishReadingEverything();
        i += 1;
      }
    }

}

// java public class KmoWriterOpenEventListenerGroup implements KmoWriterOpenEventListener
impl KmoWriterOpenEventListenerGroup {
    // java public void add(KmoWriterOpenEventListener listener)
    pub fn add(&mut self, listener: std::rc::Rc<std::cell::RefCell<dyn KmoWriterOpenEventListener>>) {
      // java this.listeners.add(listener);
      self.listeners.push(listener);
    }

    // java public void remove(KmoWriterOpenEventListener listener)
    pub fn remove(&mut self, listener: &std::rc::Rc<std::cell::RefCell<dyn KmoWriterOpenEventListener>>) {
      // java this.listeners.remove(listener);
      self.listeners.retain(|x| !std::rc::Rc::ptr_eq(x, listener));
    }

    // java public void clear()
    pub fn clear(&mut self) {
      // java this.listeners.clear();
      self.listeners.clear();
    }

    pub fn new() -> Self { Self::default() }

}
