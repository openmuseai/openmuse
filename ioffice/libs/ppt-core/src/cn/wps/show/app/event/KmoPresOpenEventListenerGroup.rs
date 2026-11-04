#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import cn.wps.moffice.io.FirstPageForceQuitException
// rust: no-stmt —— Java 受检异常；既有接口 `KmoPresOpenEventListener::slideDidFinishReadingEverything`
//   未以 `Result` 表达（沿用已登记的口径），本类不额外引入错误通道。
// Java: import cn.wps.show.app.KmoPresentation
// rust: 偏差登记 —— 实际载体取 `show/util/layout_shape::KmoPresentation`（既有接口
//   `KmoPresOpenEventListener` 的形参类型，`PptxrPresentation` 亦以该类型调用），非 Java 的
//   `cn.wps.show.app.KmoPresentation`。owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
use crate::cn::wps::show::util::layout_shape::KmoPresentation;
// Java: import java.util.ArrayList
// rust: no-stmt —— `ArrayList<T>` 由 `Vec<T>` 承载。
// Java: （同包）cn.wps.show.app.event.KmoPresOpenEventListener
use super::KmoPresOpenEventListener::KmoPresOpenEventListener;
use std::cell::RefCell;
use std::rc::Rc;

// rust: 偏差登记（owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b）
//   Java `ArrayList<KmoPresOpenEventListener>` 存接口引用；Rust 按计划 §13.3 裁定 R8 以
//   `Rc<RefCell<dyn KmoPresOpenEventListener>>` 承载。外层 `RefCell<Vec<..>>` 使 `add`/`remove`/`clear`
//   取 `&self` —— 这是 `KmoPresentation::addPresOpenEventListener(&self, ..)`（共享引用调用面）的要求。
//   Java 的 `for(int size = this.listeners.size(); i < size; ++i)` 是**先快照 size** 的顺序遍历
//   （遍历中新增的监听器本轮不回调），Rust 侧逐字保留该语义。

// Java: public class KmoPresOpenEventListenerGroup implements KmoPresOpenEventListener
pub struct KmoPresOpenEventListenerGroup {
    // Java field: private ArrayList<KmoPresOpenEventListener> listeners = new ArrayList();
    listeners: RefCell<Vec<Rc<RefCell<dyn KmoPresOpenEventListener>>>>,
}

// Java: public class KmoPresOpenEventListenerGroup implements KmoPresOpenEventListener
impl KmoPresOpenEventListenerGroup {
    // Java: （无显式构造器，隐式默认构造器）
    //
    // rust: 扩展 —— Rust 无隐式默认构造器，补 `new()`（并见文件末 `Default`）。
    pub fn new() -> Self {
        // Java: private ArrayList<KmoPresOpenEventListener> listeners = new ArrayList();
        let listeners = RefCell::new(Vec::new());
        // Java: （构造器体结束）
        KmoPresOpenEventListenerGroup { listeners }
    }

    // Java: public void add(KmoPresOpenEventListener listener)
    pub fn add(&self, listener: Rc<RefCell<dyn KmoPresOpenEventListener>>) {
        // Java: this.listeners.add(listener);
        // rust: Java `ArrayList.add` 允许重复（无去重），与 `subscribe` 不同 ⇒ 直接 push。
        self.listeners.borrow_mut().push(listener);
    }

    // Java: public void remove(KmoPresOpenEventListener listener)
    pub fn remove(&self, listener: &Rc<RefCell<dyn KmoPresOpenEventListener>>) {
        // Java: this.listeners.remove(listener);
        // rust: `remove(Object)` 只移除首个相等元素；监听器未覆写 `equals()` ⇒ `Rc::ptr_eq`。
        let mut list = self.listeners.borrow_mut();
        if let Some(pos) = list.iter().position(|l| Rc::ptr_eq(l, listener)) {
            list.remove(pos);
        }
    }

    // Java: public void clear()
    pub fn clear(&self) {
        // Java: this.listeners.clear();
        self.listeners.borrow_mut().clear();
    }
}

// Java: public class KmoPresOpenEventListenerGroup implements KmoPresOpenEventListener
impl Default for KmoPresOpenEventListenerGroup {
    // rust: 扩展 —— 对齐 `new()`（无 Java 对应）。
    fn default() -> Self {
        Self::new()
    }
}

// Java: public class KmoPresOpenEventListenerGroup implements KmoPresOpenEventListener
//
// rust: 接口方法取 `&mut self`（对齐既有 `KmoPresOpenEventListener` trait 的接收者，调用点
//   `listener.borrow_mut().docDidFinishInitializing(..)` 已按此接线）。
impl KmoPresOpenEventListener for KmoPresOpenEventListenerGroup {
    // Java: public void docDidFinishInitializing(KmoPresentation pres, boolean emptySlides)
    fn docDidFinishInitializing(&mut self, pres: &mut KmoPresentation, emptySlides: bool) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int size = this.listeners.size(); i < size; ++i) {
        let size = self.listeners.borrow().len();
        while i < size {
            // Java: ((KmoPresOpenEventListener)this.listeners.get(i)).docDidFinishInitializing(pres, emptySlides);
            let l = self.listeners.borrow()[i].clone();
            l.borrow_mut().docDidFinishInitializing(pres, emptySlides);
            // Java: ++i;
            i += 1;
        }
    }

    // Java: public void slideDidFinishReadingEverything(int slideIndex) throws FirstPageForceQuitException
    fn slideDidFinishReadingEverything(&mut self, slideIndex: i32) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int size = this.listeners.size(); i < size; ++i) {
        let size = self.listeners.borrow().len();
        while i < size {
            // Java: ((KmoPresOpenEventListener)this.listeners.get(i)).slideDidFinishReadingEverything(slideIndex);
            let l = self.listeners.borrow()[i].clone();
            l.borrow_mut().slideDidFinishReadingEverything(slideIndex);
            // Java: ++i;
            i += 1;
        }
    }

    // Java: public void docDidFinishReadingEverything()
    fn docDidFinishReadingEverything(&mut self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int size = this.listeners.size(); i < size; ++i) {
        let size = self.listeners.borrow().len();
        while i < size {
            // Java: ((KmoPresOpenEventListener)this.listeners.get(i)).docDidFinishReadingEverything();
            let l = self.listeners.borrow()[i].clone();
            l.borrow_mut().docDidFinishReadingEverything();
            // Java: ++i;
            i += 1;
        }
    }
}
