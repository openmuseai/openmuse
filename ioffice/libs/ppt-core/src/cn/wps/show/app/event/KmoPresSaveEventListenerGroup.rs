#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import java.util.ArrayList
// rust: no-stmt —— `ArrayList<T>` 由 `Vec<T>` 承载。
// Java: （同包）cn.wps.show.app.event.KmoPresSaveEventListener
use super::KmoPresSaveEventListener::KmoPresSaveEventListener;
use std::cell::RefCell;
use std::rc::Rc;

// rust: 偏差登记（owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b）
//   Java `ArrayList<KmoPresSaveEventListener>` 存接口引用；Rust 按计划 §13.3 裁定 R8 以
//   `Rc<RefCell<dyn KmoPresSaveEventListener>>` 承载。外层 `RefCell<Vec<..>>` 使 `add`/`clear` 取 `&self`
//   （对齐 `KmoPresentation::addPresSaveEventListener(&self, ..)` 的共享引用调用面）。
//   Java 的 `for(int size = this.listeners.size(); i < size; ++i)` 是先快照 size 的顺序遍历，逐字保留。

// Java: public class KmoPresSaveEventListenerGroup implements KmoPresSaveEventListener
pub struct KmoPresSaveEventListenerGroup {
    // Java field: private ArrayList<KmoPresSaveEventListener> listeners = new ArrayList();
    listeners: RefCell<Vec<Rc<RefCell<dyn KmoPresSaveEventListener>>>>,
}

// Java: public class KmoPresSaveEventListenerGroup implements KmoPresSaveEventListener
impl KmoPresSaveEventListenerGroup {
    // Java: （无显式构造器，隐式默认构造器）
    //
    // rust: 扩展 —— Rust 无隐式默认构造器，补 `new()`（并见文件末 `Default`）。
    pub fn new() -> Self {
        // Java: private ArrayList<KmoPresSaveEventListener> listeners = new ArrayList();
        let listeners = RefCell::new(Vec::new());
        // Java: （构造器体结束）
        KmoPresSaveEventListenerGroup { listeners }
    }

    // Java: public void add(KmoPresSaveEventListener listener)
    pub fn add(&self, listener: Rc<RefCell<dyn KmoPresSaveEventListener>>) {
        // Java: this.listeners.add(listener);
        self.listeners.borrow_mut().push(listener);
    }

    // Java: public void clear()
    pub fn clear(&self) {
        // Java: this.listeners.clear();
        self.listeners.borrow_mut().clear();
    }
}

// Java: public class KmoPresSaveEventListenerGroup implements KmoPresSaveEventListener
impl Default for KmoPresSaveEventListenerGroup {
    // rust: 扩展 —— 对齐 `new()`（无 Java 对应）。
    fn default() -> Self {
        Self::new()
    }
}

// Java: public class KmoPresSaveEventListenerGroup implements KmoPresSaveEventListener
impl KmoPresSaveEventListener for KmoPresSaveEventListenerGroup {
    // Java: public void currentProgress(int progress)
    fn currentProgress(&mut self, progress: i32) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int size = this.listeners.size(); i < size; ++i) {
        let size = self.listeners.borrow().len();
        while i < size {
            // Java: ((KmoPresSaveEventListener)this.listeners.get(i)).currentProgress(progress);
            let l = self.listeners.borrow()[i].clone();
            l.borrow_mut().currentProgress(progress);
            // Java: ++i;
            i += 1;
        }
    }
}
