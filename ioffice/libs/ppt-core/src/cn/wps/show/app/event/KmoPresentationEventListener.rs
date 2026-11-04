#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: （同包）cn.wps.show.app.event.KmoPresentationSelectionListener
use super::KmoPresentationSelectionListener::KmoPresentationSelectionListener;

// Java: int TRANS_UNDO = 1;
pub const TRANS_UNDO: i32 = 1;
// Java: int TRANS_REDO = 2;
pub const TRANS_REDO: i32 = 2;
// Java: int TRANS_DISCARD = 4;
pub const TRANS_DISCARD: i32 = 4;
// Java: int TRANS_COMMIT_FIN = 8;
pub const TRANS_COMMIT_FIN: i32 = 8;

// Java: public interface KmoPresentationEventListener extends KmoPresentationSelectionListener
//
// rust: Java `extends` → Rust 父 trait（supertrait）约束，facade 名与继承关系逐一对齐。
pub trait KmoPresentationEventListener: KmoPresentationSelectionListener {
    // Java: void onSlideListChanged();
    fn onSlideListChanged(&mut self);

    // Java: void onPresContentsChanged();
    fn onPresContentsChanged(&mut self);

    // Java: void onShapeFontScaleChanged();
    fn onShapeFontScaleChanged(&mut self);

    // Java: void onShapeContentsChanged(int var1, KmoShowShape... var2);
    //
    // rust: 偏差登记 —— 与 `KmoSlideEventListener::onShapeContentsChanged` 同一条约定：
    //   varargs → `&[KmoShowShape]`（值语义，接收方拿到副本）。
    //   owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
    fn onShapeContentsChanged(&mut self, var1: i32, var2: &[KmoShowShape]);

    // Java: void onTransactionChanged(int var1);
    fn onTransactionChanged(&mut self, var1: i32);

    // Java: void onSlimmed();
    fn onSlimmed(&mut self);

    // Java: void onSlideSizeChanged();
    fn onSlideSizeChanged(&mut self);
}
