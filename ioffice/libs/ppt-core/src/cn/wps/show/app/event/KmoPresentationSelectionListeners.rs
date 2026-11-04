#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import java.util.ArrayList
// rust: no-stmt —— `ArrayList<T>` 由 `Vec<T>` 承载；`contains`/`remove`/`clear` 语义逐方法注释。
// Java: （同包）cn.wps.show.app.event.KmoPresentationSelectionListener
use super::KmoPresentationSelectionListener::KmoPresentationSelectionListener;
use std::cell::RefCell;
use std::rc::Rc;

// rust: 偏差登记（本文件通用；owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b）
//   1. Java `synchronized` 方法 → Rust 普通方法（单线程对象图，计划 §1 原则 1）。
//   2. 回调循环与 `KmoSlideEventListeners` 同构：逐条 `Rc::clone` 取出、借用先结束、再回调。
//   3. Java 的 `if (l == null) break;` 不可达（`subscribe` 只接受非空引用）⇒ 省略。
//   4. Java 泛型上界 `T extends KmoPresentationSelectionListener` → Rust `T: KmoPresentationSelectionListener + ?Sized`；
//      加 `?Sized` 是为了让子类 `KmoPresentationEventListeners` 能以 `T = dyn KmoPresentationEventListener`
//      实例化（`KmoPresentationEventListener: KmoPresentationSelectionListener`，`dyn` 子 trait 亦实现父 trait）。

// Java: public abstract class KmoPresentationSelectionListeners<T extends KmoPresentationSelectionListener> implements KmoPresentationSelectionListener
pub struct KmoPresentationSelectionListeners<T: KmoPresentationSelectionListener + ?Sized> {
    // Java field: protected ArrayList<T> eventListeners = new ArrayList();
    //
    // rust: 偏差登记 —— 同 `KmoSlideEventListeners.eventListeners`：`Rc<RefCell<dyn ..>>` + 外层
    //   `RefCell<Vec<..>>`（计划 §13.3 裁定 R8）。Java `protected` → Rust `pub(crate)`，供子类
    //   `KmoPresentationEventListeners` 的回调循环直接访问（Java 亦直接访问父类 protected 字段）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    pub(crate) eventListeners: RefCell<Vec<Rc<RefCell<T>>>>,
}

// Java: public abstract class KmoPresentationSelectionListeners<T extends KmoPresentationSelectionListener> implements KmoPresentationSelectionListener
impl<T: KmoPresentationSelectionListener + ?Sized> KmoPresentationSelectionListeners<T> {
    // Java: （无显式构造器，隐式默认构造器）
    //
    // rust: 扩展 —— Rust 无隐式默认构造器，补 `new()`（并见文件末 `Default`）。
    pub fn new() -> Self {
        // Java: protected ArrayList<T> eventListeners = new ArrayList();
        let eventListeners = RefCell::new(Vec::new());
        // Java: （构造器体结束）
        KmoPresentationSelectionListeners { eventListeners }
    }

    // Java: public synchronized void subscribe(T listener)
    pub fn subscribe(&self, listener: Rc<RefCell<T>>) {
        // Java: if (!this.eventListeners.contains(listener))
        // rust: `ArrayList.contains` 走 `equals()`；本包监听器未覆写 `equals()` ⇒ 恒等比较
        //   （`Rc::ptr_eq`，登记）。
        let contains = self
            .eventListeners
            .borrow()
            .iter()
            .any(|l| Rc::ptr_eq(l, &listener));
        if !contains {
            // Java: this.eventListeners.add(listener);
            self.eventListeners.borrow_mut().push(listener);
        }
    }

    // Java: public synchronized void unsubscribe(T listener)
    pub fn unsubscribe(&self, listener: &Rc<RefCell<T>>) {
        // Java: this.eventListeners.remove(listener);
        // rust: Java `remove(Object)` 只移除首个相等元素 ⇒ `position` + `remove`。
        let mut list = self.eventListeners.borrow_mut();
        if let Some(pos) = list.iter().position(|l| Rc::ptr_eq(l, listener)) {
            list.remove(pos);
        }
    }

    // Java: public synchronized boolean contains(T listener)
    pub fn contains(&self, listener: &Rc<RefCell<T>>) -> bool {
        // Java: return this.eventListeners.contains(listener);
        self.eventListeners
            .borrow()
            .iter()
            .any(|l| Rc::ptr_eq(l, listener))
    }

    // Java: public synchronized void clear()
    pub fn clear(&self) {
        // Java: this.eventListeners.clear();
        self.eventListeners.borrow_mut().clear();
    }

    // Java: public void onSelectionChanged(int type)
    pub fn onSelectionChanged(&self, r#type: i32) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: T l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (T)(this.eventListeners.get(i)); }
            let l = {
                let list = self.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 3）
            // Java: l.onSelectionChanged(type);
            l.borrow_mut().onSelectionChanged(r#type);
            // Java: ++i;
            i += 1;
        }
    }
}

// Java: public abstract class KmoPresentationSelectionListeners<T extends KmoPresentationSelectionListener> implements KmoPresentationSelectionListener
impl<T: KmoPresentationSelectionListener + ?Sized> Default for KmoPresentationSelectionListeners<T> {
    // rust: 扩展 —— 对齐 `new()`（无 Java 对应；Rust 侧便于 `..Default::default()` 装配）。
    fn default() -> Self {
        Self::new()
    }
}

// Java: public abstract class KmoPresentationSelectionListeners<T extends KmoPresentationSelectionListener> implements KmoPresentationSelectionListener
//
// rust: 偏差登记 —— Rust 无继承；Java 的 `implements KmoPresentationSelectionListener` 在此显式转发到
//   固有 `&self` 实现（固有方法优先，不会自递归）。
//   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
impl<T: KmoPresentationSelectionListener + ?Sized> KmoPresentationSelectionListener
    for KmoPresentationSelectionListeners<T>
{
    // Java: public void onSelectionChanged(int type)
    fn onSelectionChanged(&mut self, var1: i32) {
        <Self>::onSelectionChanged(self, var1)
    }
}
