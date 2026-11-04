#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: （同包）cn.wps.show.app.event.KmoPresentationSelectionListener
use super::KmoPresentationSelectionListener::KmoPresentationSelectionListener;
// Java: （同包）cn.wps.show.app.event.KmoPresentationSelectionListeners
use super::KmoPresentationSelectionListeners::KmoPresentationSelectionListeners;
use std::cell::RefCell;
use std::rc::Rc;

// Java: public class KmoPresReadModeEventListeners extends KmoPresentationSelectionListeners<KmoPresentationSelectionListener>
//
// rust: 偏差登记（owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b）
//   1. Java 本类**类体为空**，全部能力继承自 `KmoPresentationSelectionListeners`；Rust 无继承 ⇒
//      以组合字段 `base` + 逐个显式委托复现同一公开面（`subscribe`/`unsubscribe`/`contains`/`clear`/
//      `onSelectionChanged` 的 facade 名与签名不变）。
//   2. 泛型实参 `KmoPresentationSelectionListener` → Rust `dyn KmoPresentationSelectionListener`。

// Java: public class KmoPresReadModeEventListeners extends KmoPresentationSelectionListeners<KmoPresentationSelectionListener>
pub struct KmoPresReadModeEventListeners {
    // Java: extends KmoPresentationSelectionListeners<KmoPresentationSelectionListener>
    pub(crate) base: KmoPresentationSelectionListeners<dyn KmoPresentationSelectionListener>,
}

// Java: public class KmoPresReadModeEventListeners extends KmoPresentationSelectionListeners<KmoPresentationSelectionListener>
impl KmoPresReadModeEventListeners {
    // Java: （无显式构造器，隐式默认构造器）
    //
    // rust: 扩展 —— Rust 无隐式默认构造器，补 `new()`（并见文件末 `Default`）。
    pub fn new() -> Self {
        // Java: （父类字段初始化）protected ArrayList<T> eventListeners = new ArrayList();
        let base = KmoPresentationSelectionListeners::new();
        // Java: （构造器体结束）
        KmoPresReadModeEventListeners { base }
    }

    // Java: public synchronized void subscribe(T listener)（继承自 KmoPresentationSelectionListeners）
    pub fn subscribe(&self, listener: Rc<RefCell<dyn KmoPresentationSelectionListener>>) {
        // Java: super.subscribe(listener);
        self.base.subscribe(listener);
    }

    // Java: public synchronized void unsubscribe(T listener)（继承自 KmoPresentationSelectionListeners）
    pub fn unsubscribe(&self, listener: &Rc<RefCell<dyn KmoPresentationSelectionListener>>) {
        // Java: super.unsubscribe(listener);
        self.base.unsubscribe(listener);
    }

    // Java: public synchronized boolean contains(T listener)（继承自 KmoPresentationSelectionListeners）
    pub fn contains(&self, listener: &Rc<RefCell<dyn KmoPresentationSelectionListener>>) -> bool {
        // Java: return super.contains(listener);
        self.base.contains(listener)
    }

    // Java: public synchronized void clear()（继承自 KmoPresentationSelectionListeners）
    pub fn clear(&self) {
        // Java: super.clear();
        self.base.clear();
    }

    // Java: public void onSelectionChanged(int type)（继承自 KmoPresentationSelectionListeners）
    pub fn onSelectionChanged(&self, r#type: i32) {
        // Java: super.onSelectionChanged(type);
        self.base.onSelectionChanged(r#type);
    }
}

// Java: public class KmoPresReadModeEventListeners extends KmoPresentationSelectionListeners<KmoPresentationSelectionListener>
impl Default for KmoPresReadModeEventListeners {
    // rust: 扩展 —— 对齐 `new()`（无 Java 对应）。
    fn default() -> Self {
        Self::new()
    }
}

// Java: public class KmoPresReadModeEventListeners extends KmoPresentationSelectionListeners<KmoPresentationSelectionListener>
//
// rust: Java 经基类间接 `implements KmoPresentationSelectionListener`；Rust 侧显式补该 impl。
impl KmoPresentationSelectionListener for KmoPresReadModeEventListeners {
    // Java: public void onSelectionChanged(int type)（继承自基类）
    fn onSelectionChanged(&mut self, var1: i32) {
        <Self>::onSelectionChanged(self, var1)
    }
}
