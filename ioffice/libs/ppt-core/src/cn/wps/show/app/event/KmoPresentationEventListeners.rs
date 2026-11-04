#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import java.util.ArrayList
// rust: no-stmt —— `ArrayList<T>` 由 `Vec<T>` 承载（列表本体在基类 `KmoPresentationSelectionListeners`）。
// Java: import java.util.Arrays
// rust: no-stmt —— `Arrays.asList(shapes)` 由实参切片 `shapes` 本身承载。
// Java: import java.util.List
// rust: no-stmt —— 同上（`LocalVar.mKmoShowShapeList`）。
// Java: import java.util.concurrent.atomic.AtomicInteger
// rust: no-stmt —— 单线程等价物 `std::cell::Cell<i32>`（见 `LocalVar.mNestedCount`）。
// Java: （同包）cn.wps.show.app.event.KmoPresentationEventListener
use super::KmoPresentationEventListener::KmoPresentationEventListener;
// Java: （同包）cn.wps.show.app.event.KmoPresentationSelectionListener
use super::KmoPresentationSelectionListener::KmoPresentationSelectionListener;
// Java: （同包）cn.wps.show.app.event.KmoPresentationSelectionListeners
use super::KmoPresentationSelectionListeners::KmoPresentationSelectionListeners;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// rust: 偏差登记（本文件通用；owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b）
//   1. Java `extends KmoPresentationSelectionListeners<KmoPresentationEventListener>` → Rust 组合字段
//      `base`（无继承）。Java 子类直接读写父类 `protected ArrayList<T> eventListeners`，Rust 侧经
//      `self.base.eventListeners` 访问（基类字段为 `pub(crate)`）。
//   2. Java 继承到的 `subscribe`/`unsubscribe`/`contains`/`clear`/`onSelectionChanged` 在 Rust 需逐个
//      显式委托（见各方法注释）。
//   3. `ThreadLocal<LocalVar>` → 普通字段 `RefCell<LocalVar>`（同 `KmoSlideEventListeners` 的登记：
//      单线程对象图，不用 `thread_local!`）。
//   4. 回调循环里 `synchronized(this) { l = eventListeners.get(i); }` 的「取元素/回调分离」在 Rust 由
//      「`Rc::clone` 取出、借用先结束、再回调」等价表达；`if (l == null) break;` 不可达 ⇒ 省略。
//   5. `AtomicInteger` → `Cell<i32>`；`lv` 借用在回调前结束，逐处 `rust:` 注释。
//   6. `KmoPresentationEventListeners implements KmoPresentationEventListener`：Rust 无继承，`impl` 里的
//      同名方法转发到固有 `&self` 实现（固有方法优先，不会自递归）。

// Java: public class KmoPresentationEventListeners extends KmoPresentationSelectionListeners<KmoPresentationEventListener> implements KmoPresentationEventListener
pub struct KmoPresentationEventListeners {
    // Java: extends KmoPresentationSelectionListeners<KmoPresentationEventListener>
    //
    // rust: 偏差登记 —— 泛型实参 `KmoPresentationEventListener` 在 Rust 需写成 `dyn
    //   KmoPresentationEventListener`（基类的 `?Sized` 上界正是为此放开）。子类需要同时持有
    //   「`KmoPresentationEventListener` 集合」与「继承来的 `onSelectionChanged` 面」，故 `dyn` 子 trait
    //   化后其 vtable 亦含父 trait 方法（Rust 特质对象内建）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    pub(crate) base: KmoPresentationSelectionListeners<dyn KmoPresentationEventListener>,
    // Java field: private ThreadLocal<LocalVar> mThreadLocalVar = new ThreadLocal<LocalVar>() {
    //    protected LocalVar initialValue() { return new LocalVar(); } };
    //
    // rust: 偏差登记 —— 同 `KmoSlideEventListeners.mThreadLocalVar`（普通字段 `RefCell<LocalVar>`，
    //   单线程对象图；`get()` 由 `initialValue()` 兜底恒非 null）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    mThreadLocalVar: RefCell<LocalVar>,
}

// Java: public class KmoPresentationEventListeners extends KmoPresentationSelectionListeners<KmoPresentationEventListener> implements KmoPresentationEventListener
impl KmoPresentationEventListeners {
    // Java: （无显式构造器，隐式默认构造器）
    //
    // rust: 扩展 —— Rust 无隐式默认构造器，补 `new()`（并见文件末 `Default`）。
    pub fn new() -> Self {
        // Java: （父类字段初始化）protected ArrayList<T> eventListeners = new ArrayList();
        let base = KmoPresentationSelectionListeners::new();
        // Java: private ThreadLocal<LocalVar> mThreadLocalVar = ... initialValue() { return new LocalVar(); }
        let mThreadLocalVar = RefCell::new(LocalVar::new());
        // Java: （构造器体结束）
        KmoPresentationEventListeners {
            base,
            mThreadLocalVar,
        }
    }

    // Java: public synchronized void subscribe(T listener)（继承自 KmoPresentationSelectionListeners）
    //
    // rust: 偏差登记 —— Rust 无继承 ⇒ 显式委托到基类。owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    pub fn subscribe(&self, listener: Rc<RefCell<dyn KmoPresentationEventListener>>) {
        // Java: super.subscribe(listener);
        self.base.subscribe(listener);
    }

    // Java: public synchronized void unsubscribe(T listener)（继承自 KmoPresentationSelectionListeners）
    pub fn unsubscribe(&self, listener: &Rc<RefCell<dyn KmoPresentationEventListener>>) {
        // Java: super.unsubscribe(listener);
        self.base.unsubscribe(listener);
    }

    // Java: public synchronized boolean contains(T listener)（继承自 KmoPresentationSelectionListeners）
    pub fn contains(&self, listener: &Rc<RefCell<dyn KmoPresentationEventListener>>) -> bool {
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

    // Java: public void onSlideListChanged()
    pub fn onSlideListChanged(&self) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // rust: `get()` 恒非 null ⇒ `lv` 即 `self.mThreadLocalVar`。
        // Java: if (lv != null && lv.mNestedCount.get() > 0)
        let nested = self.mThreadLocalVar.borrow().mNestedCount.get() > 0;
        if nested {
            // Java: lv.mSlideListChanging = true;
            self.mThreadLocalVar.borrow_mut().mSlideListChanging = true;
        } else {
            // Java: this.onSlideListChangedNonBlock();
            self.onSlideListChangedNonBlock();
        }
    }

    // Java: public void onPresContentsChanged()
    pub fn onPresContentsChanged(&self) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // Java: if (lv != null && lv.mNestedCount.get() > 0)
        let nested = self.mThreadLocalVar.borrow().mNestedCount.get() > 0;
        if nested {
            // Java: lv.mPresContentsChanging = true;
            self.mThreadLocalVar.borrow_mut().mPresContentsChanging = true;
        } else {
            // Java: this.onPresContentsChangedNonBlock();
            self.onPresContentsChangedNonBlock();
        }
    }

    // Java: public void onShapeFontScaleChanged()
    pub fn onShapeFontScaleChanged(&self) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // Java: if (lv != null && lv.mNestedCount.get() > 0)
        let nested = self.mThreadLocalVar.borrow().mNestedCount.get() > 0;
        if nested {
            // Java: lv.mShapeFontScaleChanging = true;
            self.mThreadLocalVar.borrow_mut().mShapeFontScaleChanging = true;
        } else {
            // Java: this.onShapeFontScaleChangedNonBlock();
            self.onShapeFontScaleChangedNonBlock();
        }
    }

    // Java: public void onShapeContentsChanged(int flag, KmoShowShape... shapes)
    pub fn onShapeContentsChanged(&self, flag: i32, shapes: &[KmoShowShape]) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // Java: if (lv != null && lv.mNestedCount.get() > 0)
        let nested = self.mThreadLocalVar.borrow().mNestedCount.get() > 0;
        if nested {
            let mut lv = self.mThreadLocalVar.borrow_mut();
            // Java: lv.mShapeContentsChanging = true;
            lv.mShapeContentsChanging = true;
            // Java: lv.mFlag = flag;
            lv.mFlag = flag;
            // Java: if (shapes != null && shapes.length != 0)
            if !shapes.is_empty() {
                // Java: lv.mKmoShowShapeList.addAll(Arrays.asList(shapes));
                // rust: 值语义 ⇒ 逐元素克隆（`KmoShowShape: Clone`）；Java 只追加引用。
                lv.mKmoShowShapeList.extend_from_slice(shapes);
            }
        } else {
            // Java: this.onShapeContentsChangedNonBlock(flag, shapes);
            self.onShapeContentsChangedNonBlock(flag, shapes);
        }
    }

    // Java: public void onTransactionChanged(int flag)
    pub fn onTransactionChanged(&self, flag: i32) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onTransactionChanged(flag);
            l.borrow_mut().onTransactionChanged(flag);
            // Java: ++i;
            i += 1;
        }
    }

    // Java: public void onSlimmed()
    pub fn onSlimmed(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onSlimmed();
            l.borrow_mut().onSlimmed();
            // Java: ++i;
            i += 1;
        }
    }

    // Java: public void onSlideSizeChanged()
    pub fn onSlideSizeChanged(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onSlideSizeChanged();
            l.borrow_mut().onSlideSizeChanged();
            // Java: ++i;
            i += 1;
        }
    }

    // Java: private void onSlideListChangedNonBlock()
    fn onSlideListChangedNonBlock(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onSlideListChanged();
            l.borrow_mut().onSlideListChanged();
            // Java: ++i;
            i += 1;
        }
    }

    // Java: private void onPresContentsChangedNonBlock()
    fn onPresContentsChangedNonBlock(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onPresContentsChanged();
            l.borrow_mut().onPresContentsChanged();
            // Java: ++i;
            i += 1;
        }
    }

    // Java: private void onShapeFontScaleChangedNonBlock()
    fn onShapeFontScaleChangedNonBlock(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onShapeFontScaleChanged();
            l.borrow_mut().onShapeFontScaleChanged();
            // Java: ++i;
            i += 1;
        }
    }

    // Java: private void onShapeContentsChangedNonBlock(int flag, KmoShowShape... shapes)
    fn onShapeContentsChangedNonBlock(&self, flag: i32, shapes: &[KmoShowShape]) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoPresentationEventListener l;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = (KmoPresentationEventListener)this.eventListeners.get(i); }
            let l = {
                let list = self.base.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 4）
            // Java: l.onShapeContentsChanged(flag, shapes);
            l.borrow_mut().onShapeContentsChanged(flag, shapes);
            // Java: ++i;
            i += 1;
        }
    }

    // Java: public void startMergeEvents()
    pub fn startMergeEvents(&self) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        let lv = self.mThreadLocalVar.borrow();
        // Java: if (lv.mNestedCount.get() < 0)
        if lv.mNestedCount.get() < 0 {
            // Java: lv.mNestedCount.set(0);
            lv.mNestedCount.set(0);
        } else {
            // Java: lv.mNestedCount.incrementAndGet();
            // rust: `Cell<i32>` 无 `incrementAndGet` ⇒ `get()` + 1 再 `set`（单线程下等价）。
            lv.mNestedCount.set(lv.mNestedCount.get() + 1);
        }
    }

    // Java: public void finishMergeEvents()
    pub fn finishMergeEvents(&self) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // Java: if (lv.mNestedCount.get() < 0) { lv.mNestedCount.set(0); } else { lv.mNestedCount.decrementAndGet(); }
        {
            let lv = self.mThreadLocalVar.borrow();
            if lv.mNestedCount.get() < 0 {
                lv.mNestedCount.set(0);
            } else {
                lv.mNestedCount.set(lv.mNestedCount.get() - 1);
            }
        }
        // Java: if (0 == lv.mNestedCount.get())
        let nestedCount = self.mThreadLocalVar.borrow().mNestedCount.get();
        if nestedCount == 0 {
            // Java: if (lv.mShapeContentsChanging && lv.mFlag != -1)
            let shapeContentsChanging = self.mThreadLocalVar.borrow().mShapeContentsChanging;
            let flag = self.mThreadLocalVar.borrow().mFlag;
            if shapeContentsChanging && flag != -1 {
                // Java: KmoShowShape[] shapes = new KmoShowShape[lv.mKmoShowShapeList.size()];
                // Java: lv.mKmoShowShapeList.toArray(shapes);
                // rust: `toArray` 是引用拷贝 ⇒ 值语义下逐元素克隆；用克隆而非 `mem::take`，以保持 Java
                //   「先回调、后清空列表」的顺序。
                let shapes: Vec<KmoShowShape> = self.mThreadLocalVar.borrow().mKmoShowShapeList.clone();
                // Java: this.onShapeContentsChangedNonBlock(lv.mFlag, shapes);
                self.onShapeContentsChangedNonBlock(flag, &shapes);
                // Java: lv.mShapeContentsChanging = false;
                self.mThreadLocalVar.borrow_mut().mShapeContentsChanging = false;
                // Java: lv.mFlag = -1;
                self.mThreadLocalVar.borrow_mut().mFlag = -1;
                // Java: lv.mKmoShowShapeList.clear();
                self.mThreadLocalVar.borrow_mut().mKmoShowShapeList.clear();
            }
            // Java: if (lv.mPresContentsChanging)
            let presContentsChanging = self.mThreadLocalVar.borrow().mPresContentsChanging;
            if presContentsChanging {
                // Java: this.onPresContentsChangedNonBlock();
                self.onPresContentsChangedNonBlock();
                // Java: lv.mPresContentsChanging = false;
                self.mThreadLocalVar.borrow_mut().mPresContentsChanging = false;
            }
            // Java: if (lv.mSlideListChanging)
            let slideListChanging = self.mThreadLocalVar.borrow().mSlideListChanging;
            if slideListChanging {
                // Java: this.onSlideListChangedNonBlock();
                self.onSlideListChangedNonBlock();
                // Java: lv.mSlideListChanging = false;
                self.mThreadLocalVar.borrow_mut().mSlideListChanging = false;
            }
            // Java: if (lv.mShapeFontScaleChanging)
            let shapeFontScaleChanging = self.mThreadLocalVar.borrow().mShapeFontScaleChanging;
            if shapeFontScaleChanging {
                // Java: this.onShapeFontScaleChangedNonBlock();
                self.onShapeFontScaleChangedNonBlock();
                // Java: lv.mShapeFontScaleChanging = false;
                self.mThreadLocalVar.borrow_mut().mShapeFontScaleChanging = false;
            }
        }
    }

    // Java: public int getEventNestCount()
    pub fn getEventNestCount(&self) -> i32 {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // Java: return lv != null ? lv.mNestedCount.get() : 0;
        // rust: `get()` 恒非 null（`initialValue()`）⇒ 恒取计数值，三元退化。
        self.mThreadLocalVar.borrow().mNestedCount.get()
    }
}

// Java: public class KmoPresentationEventListeners extends KmoPresentationSelectionListeners<KmoPresentationEventListener> implements KmoPresentationEventListener
impl Default for KmoPresentationEventListeners {
    // rust: 扩展 —— 对齐 `new()`（无 Java 对应；Rust 侧便于 `..Default::default()` 装配）。
    fn default() -> Self {
        Self::new()
    }
}

// Java: public class KmoPresentationEventListeners ... implements KmoPresentationEventListener
//
// rust: 偏差登记 —— Java 的继承面 `implemented via KmoPresentationSelectionListeners`；Rust 需两个 `impl`
//   （父 trait + 子 trait），同名方法均转发到固有实现。
//   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
impl KmoPresentationSelectionListener for KmoPresentationEventListeners {
    // Java: public void onSelectionChanged(int type)（继承自基类）
    fn onSelectionChanged(&mut self, var1: i32) {
        <Self>::onSelectionChanged(self, var1)
    }
}

// Java: public class KmoPresentationEventListeners ... implements KmoPresentationEventListener
impl KmoPresentationEventListener for KmoPresentationEventListeners {
    // Java: public void onSlideListChanged()
    fn onSlideListChanged(&mut self) {
        <Self>::onSlideListChanged(self)
    }

    // Java: public void onPresContentsChanged()
    fn onPresContentsChanged(&mut self) {
        <Self>::onPresContentsChanged(self)
    }

    // Java: public void onShapeFontScaleChanged()
    fn onShapeFontScaleChanged(&mut self) {
        <Self>::onShapeFontScaleChanged(self)
    }

    // Java: public void onShapeContentsChanged(int flag, KmoShowShape... shapes)
    fn onShapeContentsChanged(&mut self, var1: i32, var2: &[KmoShowShape]) {
        <Self>::onShapeContentsChanged(self, var1, var2)
    }

    // Java: public void onTransactionChanged(int flag)
    fn onTransactionChanged(&mut self, var1: i32) {
        <Self>::onTransactionChanged(self, var1)
    }

    // Java: public void onSlimmed()
    fn onSlimmed(&mut self) {
        <Self>::onSlimmed(self)
    }

    // Java: public void onSlideSizeChanged()
    fn onSlideSizeChanged(&mut self) {
        <Self>::onSlideSizeChanged(self)
    }
}

// Java: public static class LocalVar
//
// rust: Java 嵌套静态类 → 模块级同名类型（调用点 `KmoPresentationEventListeners.LocalVar`
//   → `KmoPresentationEventListeners::LocalVar`）。
pub struct LocalVar {
    // Java field: AtomicInteger mNestedCount = new AtomicInteger();
    // rust: 偏差登记 —— `AtomicInteger` → `Cell<i32>`（单线程对象图；见文件头偏差登记 5）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    mNestedCount: Cell<i32>,
    // Java field: boolean mPresContentsChanging = false;
    mPresContentsChanging: bool,
    // Java field: boolean mShapeContentsChanging = false;
    mShapeContentsChanging: bool,
    // Java field: boolean mSlideListChanging = false;
    mSlideListChanging: bool,
    // Java field: boolean mShapeFontScaleChanging = false;
    mShapeFontScaleChanging: bool,
    // Java field: int mFlag = -1;
    mFlag: i32,
    // Java field: List<KmoShowShape> mKmoShowShapeList = new ArrayList();
    mKmoShowShapeList: Vec<KmoShowShape>,
}

// Java: public static class LocalVar
impl LocalVar {
    // Java: （无显式构造器；字段默认值 `mNestedCount = 0`、`mFlag = -1`、列表为空）
    fn new() -> Self {
        LocalVar {
            // Java field initializer: AtomicInteger mNestedCount = new AtomicInteger();
            mNestedCount: Cell::new(0),
            // Java field initializer: boolean mPresContentsChanging = false;
            mPresContentsChanging: false,
            // Java field initializer: boolean mShapeContentsChanging = false;
            mShapeContentsChanging: false,
            // Java field initializer: boolean mSlideListChanging = false;
            mSlideListChanging: false,
            // Java field initializer: boolean mShapeFontScaleChanging = false;
            mShapeFontScaleChanging: false,
            // Java field initializer: int mFlag = -1;
            mFlag: -1,
            // Java field initializer: List<KmoShowShape> mKmoShowShapeList = new ArrayList();
            mKmoShowShapeList: Vec::new(),
        }
    }
}
