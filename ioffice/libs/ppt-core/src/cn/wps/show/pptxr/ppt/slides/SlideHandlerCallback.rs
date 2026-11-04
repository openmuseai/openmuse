#![allow(dead_code, non_snake_case, unused_variables, unused_imports)]

// M-PPT-KMOSLIDE W6c-1：本文件原先是**自动生成桩（假编译面形态 E）**——
//   `// Rust stub for SlideHandlerCallback (auto-generated)` + `use crate::*;` + `new() { todo!() }`，
//   11 行、零 Java 语义，且**未登记**（死文件）。本波按 Java 真身（9 行）重写并登记。
//
// ★同时消除一处**形态 G**：`ppt/slides/HandlerCallbackImpl.rs:65` 的 `pub mod compile_surface`
//   曾在 `:251` 内联重声明同名 `pub struct SlideHandlerCallback { pub mode: i32 }`，
//   把本文件（真身）遮蔽成 `compile_surface` 的局部类型。本波已删除该内联影子。

// Java: import cn.wps.drawing.dmlr.model.HandlerCallback
use crate::cn::wps::drawing::dmlr::model::HandlerCallback::{HandlerCallback__mode, HandlerCallbackState};

// Java: public abstract class SlideHandlerCallback extends HandlerCallback
pub struct SlideHandlerCallback {
    // Java: HandlerCallback super_ —— Java 的 `extends HandlerCallback`。
    // rust: 本仓 `HandlerCallback` 是 **trait**（`HandlerCallback.rs:65`），Java 的父类
    //       字段态放在 `HandlerCallbackState` 里 ⇒ 与 `HandlerCallbackImpl.rs:266` 的
    //       `super_: HandlerCallbackState` 同构，保持仓内一致。
    pub(crate) super_: HandlerCallbackState,
}

// Java: public abstract class SlideHandlerCallback extends HandlerCallback
impl SlideHandlerCallback {
    // Java: public SlideHandlerCallback(int mode)
    pub fn new(mode: i32) -> Self {
        // Java: super(mode);
        Self { super_: HandlerCallback__mode(mode) }
    }

    // Java: 继承自 HandlerCallback 的 `boolean isGVml()` / `boolean isPPTX()` 等成员
    //       由 `HandlerCallbackState`（`HandlerCallback` trait 的默认实现）提供；
    //       本类在 Java 里是 `abstract`，自身不新增任何成员。
    // owner: show.pptxr.ppt.slides    wave: M-PPT-KMOSLIDE-W6c
}

// rust: 扩展 —— Java 无无参构造器（只有 `SlideHandlerCallback(int mode)`），但
// `SlideHandlerCallbackImpl`（`:44`）在 Rust 侧带了 `#[derive(Default)]`（模板产物），
// 且 Java 的 `HandlerCallback()` 无参构造器确实存在（`HandlerCallback.rs:53`，`mode = 0`）。
// ⇒ 以 `mode = 0` 实现 `Default`，与 Java 无参语义一致。同 SP1b 对事件包的同类处置。
impl Default for SlideHandlerCallback {
    fn default() -> Self {
        Self::new(0)
    }
}
