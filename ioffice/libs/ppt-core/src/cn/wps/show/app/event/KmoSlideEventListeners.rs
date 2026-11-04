#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import java.util.ArrayList
// rust: no-stmt —— `ArrayList<T>` 由 `Vec<T>` 承载；`contains`/`remove`/`clear` 语义逐方法注释。
// Java: import java.util.Arrays
// rust: no-stmt —— `Arrays.asList(shapes)` 由实参切片 `shapes` 本身承载。
// Java: import java.util.List
// rust: no-stmt —— 同上（`LocalVar.mKmoShowShapeList`）。
// Java: import java.util.concurrent.atomic.AtomicInteger
// rust: no-stmt —— 单线程等价物 `std::cell::Cell<i32>`（见 `LocalVar.mNestedCount`）。
// Java: （同包）cn.wps.show.app.event.KmoSlideEventListener
use super::KmoSlideEventListener::{KmoSlideEventListener, FLAG_CONTENT};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// rust: 偏差登记（本文件通用；owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b）
//   1. Java `synchronized` 方法（`subscribe`/`unsubscribe`/`clear`）→ Rust 普通方法。本仓单线程对象图
//      （计划 §1 原则 1 / §13.3 裁定 R8）下不存在数据竞争；互斥语义退化为 `RefCell` 的运行时借用检查。
//   2. Java 每个回调循环用 `synchronized(this) { l = eventListeners.get(i); }` 把「取元素」与
//      「调用监听器」分开（回调期间不持锁）。Rust 侧逐条 `Rc::clone` 取出、借用立刻结束，再回调 ⇒ 语义等价
//      （也保证回调内重入 `subscribe`/`unsubscribe` 不会 panic）。
//   3. Java 的 `if (l == null) break;` 不可达：`ArrayList` 元素恒非 null（`subscribe` 只接受非 null），
//      Rust `Rc` 更是不可为空 ⇒ 省略该分支。
//   4. Java 在回调期间仍持有 `lv`（ThreadLocal 值）引用；Rust `RefCell` 借用不能跨越回调存活 ⇒
//      一律「先借用取值 / 置位，借用结束，再回调」，逐处 `rust:` 注释说明。
//   5. Java 允许同一监听器在自身回调中再次被回调；Rust 侧 `Rc<RefCell<dyn ..>>` 回调时持有该监听器的
//      `borrow_mut`，自反重入会 panic —— 全仓 `Rc<RefCell<..>>` 约定的固有代价，登记。
//   6. `KmoSlideEventListeners implements KmoSlideEventListener`：Rust 无继承，`impl` 里的同名方法
//      转发到固有 `&self` 实现（固有方法优先，不会递归）。

// Java: public class KmoSlideEventListeners implements KmoSlideEventListener
pub struct KmoSlideEventListeners {
    // Java field: private ArrayList<KmoSlideEventListener> eventListeners = new ArrayList();
    //
    // rust: 偏差登记 —— Java 存接口引用；Rust 侧「共享、可能可变的监听器注册表」按计划 §13.3 裁定 R8 /
    //   §1 原则 1 以 `Rc<RefCell<dyn KmoSlideEventListener>>` 承载（与 `PptxReader.mOpenListener:
    //   Option<Rc<RefCell<dyn KmoPresOpenEventListener>>>` 同一表达）。
    //   外层 `RefCell<Vec<..>>` 提供内部可变性，使 `subscribe`/`unsubscribe`/`clear` 取 `&self`，
    //   从而 `ISlide::eventListeners(&self) -> &KmoSlideEventListeners` 这类共享引用即可订阅。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    eventListeners: RefCell<Vec<Rc<RefCell<dyn KmoSlideEventListener>>>>,
    // Java field: private ThreadLocal<LocalVar> mThreadLocalVar = new ThreadLocal<LocalVar>() {
    //    protected LocalVar initialValue() { return new LocalVar(); } };
    //
    // rust: 偏差登记 —— Java `ThreadLocal<LocalVar>` 的唯一目的是让「嵌套事件批处理」在同线程内可重入
    //   （`startMergeEvents`/`finishMergeEvents` 成对嵌套）。Rust 侧对象图单线程，直接落成普通字段
    //   `RefCell<LocalVar>`；不使用 `thread_local!`（会与 `Rc` 对象图冲突，且 `LocalVar` 必须按实例隔离，
    //   而非按线程共享）。`ThreadLocal.get()` 的 `initialValue()` 兜底 ⇒ Java 侧 `lv` 恒非 null。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    mThreadLocalVar: RefCell<LocalVar>,
}

// Java: public class KmoSlideEventListeners implements KmoSlideEventListener
impl KmoSlideEventListeners {
    // Java: （无显式构造器，隐式默认构造器）
    //
    // rust: 扩展 —— Rust 无隐式默认构造器，补 `new()`（并见文件末 `Default`）。
    pub fn new() -> Self {
        // Java: private ArrayList<KmoSlideEventListener> eventListeners = new ArrayList();
        let eventListeners = RefCell::new(Vec::new());
        // Java: private ThreadLocal<LocalVar> mThreadLocalVar = ... initialValue() { return new LocalVar(); }
        let mThreadLocalVar = RefCell::new(LocalVar::new());
        // Java: （构造器体结束）
        KmoSlideEventListeners {
            eventListeners,
            mThreadLocalVar,
        }
    }

    // Java: public synchronized void subscribe(KmoSlideEventListener listener)
    pub fn subscribe(&self, listener: Rc<RefCell<dyn KmoSlideEventListener>>) {
        // Java: if (!this.eventListeners.contains(listener))
        // rust: `ArrayList.contains` 走 `equals()`；本包监听器均未覆写 `equals()` ⇒ 恒等比较，
        //   等价于 `Rc::ptr_eq`（登记）。
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

    // Java: public synchronized void unsubscribe(KmoSlideEventListener listener)
    pub fn unsubscribe(&self, listener: &Rc<RefCell<dyn KmoSlideEventListener>>) {
        // Java: this.eventListeners.remove(listener);
        // rust: `ArrayList.remove(Object)` 只移除**首个**相等元素 ⇒ 用 `position` + `remove` 精确对齐
        //   （不用 `retain`，`retain` 会移除全部重复项）。
        let mut list = self.eventListeners.borrow_mut();
        if let Some(pos) = list.iter().position(|l| Rc::ptr_eq(l, listener)) {
            list.remove(pos);
        }
    }

    // Java: public synchronized void clear()
    pub fn clear(&self) {
        // Java: this.eventListeners.clear();
        self.eventListeners.borrow_mut().clear();
    }

    // Java: public void onSlideContentsChanged()
    pub fn onSlideContentsChanged(&self) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // rust: `get()` 由 `initialValue()` 兜底恒非 null，`lv` 即 `self.mThreadLocalVar`。
        // Java: if (lv != null && lv.mNestedCount.get() > 0)
        let nested = self.mThreadLocalVar.borrow().mNestedCount.get() > 0;
        if nested {
            // Java: lv.mSlideContentsChanging = true;
            self.mThreadLocalVar.borrow_mut().mSlideContentsChanging = true;
        } else {
            // Java: this.onSlideContentsChangedNonBlock();
            self.onSlideContentsChangedNonBlock();
        }
    }

    // Java: public void onRenderFinished()
    pub fn onRenderFinished(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoSlideEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = this.eventListeners.get(i); }
            let l = {
                let list = self.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 3）
            // Java: l.onRenderFinished();
            l.borrow_mut().onRenderFinished();
            // Java: ++i;
            i += 1;
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

    // Java: public void onAddInkShape(KmoShowShape inkshape)
    pub fn onAddInkShape(&self, inkshape: KmoShowShape) {
        // Java: LocalVar lv = (LocalVar)this.mThreadLocalVar.get();
        // Java: if (lv != null && lv.mNestedCount.get() > 0)
        let nested = self.mThreadLocalVar.borrow().mNestedCount.get() > 0;
        if nested {
            let mut lv = self.mThreadLocalVar.borrow_mut();
            // Java: lv.mAddInkShape = true;
            lv.mAddInkShape = true;
            // Java: lv.mKmoShowShape = inkshape;
            lv.mKmoShowShape = Some(inkshape);
        } else {
            // Java: this.onAddInkShapeNonBlock(inkshape);
            self.onAddInkShapeNonBlock(inkshape);
        }
    }

    // Java: public void onPartContentChanged(int shapeIndex)
    pub fn onPartContentChanged(&self, shapeIndex: i32) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoSlideEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = this.eventListeners.get(i); }
            let l = {
                let list = self.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 3）
            // Java: l.onPartContentChanged(shapeIndex);
            l.borrow_mut().onPartContentChanged(shapeIndex);
            // Java: ++i;
            i += 1;
        }
    }

    // Java: private void onSlideContentsChangedNonBlock()
    fn onSlideContentsChangedNonBlock(&self) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoSlideEventListener l = null;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = this.eventListeners.get(i); }
            let l = {
                let list = self.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 3）
            // Java: l.onSlideContentsChanged();
            l.borrow_mut().onSlideContentsChanged();
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
            // Java: KmoSlideEventListener l;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = this.eventListeners.get(i); }
            let l = {
                let list = self.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 3）
            // Java: l.onShapeContentsChanged(flag, shapes);
            l.borrow_mut().onShapeContentsChanged(flag, shapes);
            // Java: ++i;
            i += 1;
        }
    }

    // Java: private void onAddInkShapeNonBlock(KmoShowShape inkshape)
    fn onAddInkShapeNonBlock(&self, inkshape: KmoShowShape) {
        // Java: int i = 0;
        let mut i = 0;
        // Java: while(true) {
        loop {
            // Java: KmoSlideEventListener l;
            // Java: synchronized(this) { if (i >= this.eventListeners.size()) break; l = this.eventListeners.get(i); }
            let l = {
                let list = self.eventListeners.borrow();
                if i >= list.len() {
                    break;
                }
                list[i].clone()
            };
            // Java: if (l == null) break;（不可达，见文件头偏差登记 3）
            // Java: l.onAddInkShape(inkshape);
            // rust: `inkshape` 为值语义 ⇒ 每个监听器收到一份克隆（Java 传同一引用）。
            l.borrow_mut().onAddInkShape(inkshape.clone());
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
                // rust: `toArray` 是引用拷贝 ⇒ 值语义下逐元素克隆。此处用克隆而非 `mem::take`，
                //   以保持 Java「先回调、后清空列表」的顺序（回调内重入写入的项同样会被随后清掉）。
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
            // Java: if (lv.mSlideContentsChanging)
            let slideContentsChanging = self.mThreadLocalVar.borrow().mSlideContentsChanging;
            if slideContentsChanging {
                // Java: this.onSlideContentsChangedNonBlock();
                self.onSlideContentsChangedNonBlock();
                // Java: lv.mSlideContentsChanging = false;
                self.mThreadLocalVar.borrow_mut().mSlideContentsChanging = false;
            }
            // Java: if (lv.mAddInkShape && lv.mKmoShowShape != null)
            let addInkShape = self.mThreadLocalVar.borrow().mAddInkShape;
            let inkShape: Option<KmoShowShape> = self.mThreadLocalVar.borrow().mKmoShowShape.clone();
            if addInkShape && inkShape.is_some() {
                // Java: this.onAddInkShapeNonBlock(lv.mKmoShowShape);
                if let Some(inkShape) = inkShape {
                    self.onAddInkShapeNonBlock(inkShape);
                }
                // Java: lv.mAddInkShape = false;
                self.mThreadLocalVar.borrow_mut().mAddInkShape = false;
                // Java: lv.mKmoShowShape = null;
                self.mThreadLocalVar.borrow_mut().mKmoShowShape = None;
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

    // Java: public void onShapeContentsChanged(KmoShowShape... shapes)
    //
    // rust: 重载消歧 —— 与 `onShapeContentsChanged(int, &[KmoShowShape])` 在 Rust 无法同名共存，
    //   Java 的无 flag 重载以 `_noFlag` 后缀承载（沿用 `KmoSlideBeautifyTags::isDirty_flag` /
    //   `Slide::equals_obj` 的后缀消歧约定）。owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    pub fn onShapeContentsChanged_noFlag(&self, shapes: &[KmoShowShape]) {
        // Java: this.onShapeContentsChanged(3, shapes);
        // rust: 字面量 3 == `KmoSlideEventListener.FLAG_CONTENT`。
        self.onShapeContentsChanged(FLAG_CONTENT, shapes);
    }
}

// Java: public class KmoSlideEventListeners implements KmoSlideEventListener
impl Default for KmoSlideEventListeners {
    // rust: 扩展 —— 对齐 `new()`（无 Java 对应；Rust 侧便于 `..Default::default()` 装配）。
    fn default() -> Self {
        Self::new()
    }
}

// Java: public class KmoSlideEventListeners implements KmoSlideEventListener
//
// rust: 偏差登记 —— Rust 无继承；Java 的 `implements KmoSlideEventListener` 在此显式转发到固有
//   `&self` 实现（固有方法优先，不会自递归）。转发保留 facade 名与接口关系。
//   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
impl KmoSlideEventListener for KmoSlideEventListeners {
    // Java: public void onSlideContentsChanged()
    fn onSlideContentsChanged(&mut self) {
        <Self>::onSlideContentsChanged(self)
    }

    // Java: public void onRenderFinished()
    fn onRenderFinished(&mut self) {
        <Self>::onRenderFinished(self)
    }

    // Java: public void onShapeContentsChanged(int flag, KmoShowShape... shapes)
    fn onShapeContentsChanged(&mut self, var1: i32, var2: &[KmoShowShape]) {
        <Self>::onShapeContentsChanged(self, var1, var2)
    }

    // Java: public void onAddInkShape(KmoShowShape inkshape)
    fn onAddInkShape(&mut self, var1: KmoShowShape) {
        <Self>::onAddInkShape(self, var1)
    }

    // Java: public void onPartContentChanged(int shapeIndex)
    fn onPartContentChanged(&mut self, var1: i32) {
        <Self>::onPartContentChanged(self, var1)
    }
}

// Java: public static class LocalVar
//
// rust: Java 嵌套静态类 → 模块级同名类型（沿用 `ShowShape.Placeholder` / `Shade.LinearShade` 约定）；
//   调用点 `KmoSlideEventListeners.LocalVar` → `KmoSlideEventListeners::LocalVar`。
pub struct LocalVar {
    // Java field: AtomicInteger mNestedCount = new AtomicInteger();
    //
    // rust: 偏差登记 —— `AtomicInteger` 的原子性是为跨线程 `ThreadLocal` 准备的；单线程对象图下以
    //   `Cell<i32>` 承载（`get`/`set` 取 `&self`，保留 Java 的可变语义）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    mNestedCount: Cell<i32>,
    // Java field: boolean mShapeContentsChanging = false;
    mShapeContentsChanging: bool,
    // Java field: boolean mSlideContentsChanging = false;
    mSlideContentsChanging: bool,
    // Java field: boolean mAddInkShape = false;
    mAddInkShape: bool,
    // Java field: int mFlag = -1;
    mFlag: i32,
    // Java field: List<KmoShowShape> mKmoShowShapeList = new ArrayList();
    mKmoShowShapeList: Vec<KmoShowShape>,
    // Java field: KmoShowShape mKmoShowShape;
    // rust: Java 未赋值即 null ⇒ `None` ≙ null。
    mKmoShowShape: Option<KmoShowShape>,
}

// Java: public static class LocalVar
impl LocalVar {
    // Java: （无显式构造器；字段默认值 `mNestedCount = 0`、`mFlag = -1`、列表为空）
    fn new() -> Self {
        LocalVar {
            // Java field initializer: AtomicInteger mNestedCount = new AtomicInteger();
            mNestedCount: Cell::new(0),
            // Java field initializer: boolean mShapeContentsChanging = false;
            mShapeContentsChanging: false,
            // Java field initializer: boolean mSlideContentsChanging = false;
            mSlideContentsChanging: false,
            // Java field initializer: boolean mAddInkShape = false;
            mAddInkShape: false,
            // Java field initializer: int mFlag = -1;
            mFlag: -1,
            // Java field initializer: List<KmoShowShape> mKmoShowShapeList = new ArrayList();
            mKmoShowShapeList: Vec::new(),
            // Java field initializer: KmoShowShape mKmoShowShape（Java 默认 null）
            mKmoShowShape: None,
        }
    }
}
