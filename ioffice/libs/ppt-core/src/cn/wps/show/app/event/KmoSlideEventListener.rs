#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;

// Java: int FLAG_MOVE = 0;
pub const FLAG_MOVE: i32 = 0;
// Java: int FLAG_RESIZE = 1;
pub const FLAG_RESIZE: i32 = 1;
// Java: int FLAG_ROTATE = 2;
pub const FLAG_ROTATE: i32 = 2;
// Java: int FLAG_CONTENT = 3;
pub const FLAG_CONTENT: i32 = 3;
// Java: int FLAG_LAYER = 4;
pub const FLAG_LAYER: i32 = 4;
// Java: int FLAG_GROUP = 5;
pub const FLAG_GROUP: i32 = 5;
// Java: int FLAG_UNGROUP = 6;
pub const FLAG_UNGROUP: i32 = 6;
// Java: int FLAG_DELETE = 7;
pub const FLAG_DELETE: i32 = 7;

// Java: public interface KmoSlideEventListener
//
// rust: 偏差登记 —— Java 接口内的 8 个 `int` 常量（接口成员隐式 `public static final`）平移到模块级
//   `const`。Rust trait 关联常量需要 `Self::FLAG_*` 限定，而 Java 调用点写作
//   `KmoSlideEventListener.FLAG_MOVE`（例如 `KmoSlideEventListeners.onShapeContentsChanged` 里的字面量 3）；
//   模块级常量保留同名，调用点改为 `KmoSlideEventListener::FLAG_MOVE`，facade 名不变。
//   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
pub trait KmoSlideEventListener {
    // Java: void onSlideContentsChanged();
    fn onSlideContentsChanged(&mut self);

    // Java: void onRenderFinished();
    fn onRenderFinished(&mut self);

    // Java: void onShapeContentsChanged(int var1, KmoShowShape... var2);
    //
    // rust: 偏差登记 —— Java varargs `KmoShowShape...` 以 `&[KmoShowShape]` 承载（仓库既有约定，
    //   见 `show/app/tag/KmoSlideBeautifyTags.rs` 的 `UpdateWatcher::onShapeContentsChanged`：
    //   `pub fn onShapeContentsChanged(&mut self, flag: i32, shapes: &[KmoShowShape])`）。
    //   Java 传的是对象引用（多个监听器看到同一对象）；Rust 侧为值切片 ⇒ 接收方拿到的是克隆副本
    //   （`KmoShowShape: Clone`）。`show/app/event` 不拥有 `KmoShowShape` 的共享载体，按约定取 `&[..]`。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    fn onShapeContentsChanged(&mut self, var1: i32, var2: &[KmoShowShape]);

    // Java: void onAddInkShape(KmoShowShape var1);
    //
    // rust: 偏差登记 —— 同上，Java 引用语义 → Rust 值语义（`KmoShowShape` 按值传入）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    fn onAddInkShape(&mut self, var1: KmoShowShape);

    // Java: void onPartContentChanged(int var1);
    fn onPartContentChanged(&mut self, var1: i32);
}
