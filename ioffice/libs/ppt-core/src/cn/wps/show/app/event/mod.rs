// Java: package cn.wps.show.app.event
//
// M-PPT-KMOSLIDE SP1b：本包原先只有 `KmoPresOpenEventListener` 已登记；`KmoSlideEventListener` /
// `KmoPresentationEventListener` 是 `use crate::*;` + 空 trait 的自动桩，两个注册表
// （`KmoSlideEventListeners` 240 行 / `KmoPresentationEventListeners` 278 行）与
// `KmoPresentationSelectionListeners` 基类、其余 4 个同包类整体缺失。本波按 Java 路径 `cn.wps.show.app.event`
// 全部落地并登记（Java 侧共 11 个文件）。
// owner: show.app.event    wave: M-PPT-KMOSLIDE-SP1b
//
// Java 文件 → Rust 模块对应表：
//   KmoSlideEventListener.java               → KmoSlideEventListener（8 个 FLAG 常量 + 5 方法）
//   KmoPresentationSelectionListener.java    → KmoPresentationSelectionListener（1 方法）
//   KmoPresentationEventListener.java        → KmoPresentationEventListener（父 trait + 4 个 TRANS 常量 + 7 方法）
//   KmoSlideEventListeners.java              → KmoSlideEventListeners（注册表 + 嵌套 LocalVar）
//   KmoPresentationSelectionListeners.java   → KmoPresentationSelectionListeners（泛型抽象基类）
//   KmoPresentationEventListeners.java       → KmoPresentationEventListeners（注册表 + 嵌套 LocalVar）
//   KmoPresOpenEventListener.java            → KmoPresOpenEventListener（既有，本波未改契约）
//   KmoPresOpenEventListenerGroup.java       → KmoPresOpenEventListenerGroup
//   KmoPresReadModeEventListeners.java       → KmoPresReadModeEventListeners（空类体子类）
//   KmoPresSaveEventListener.java            → KmoPresSaveEventListener
//   KmoPresSaveEventListenerGroup.java       → KmoPresSaveEventListenerGroup

pub mod KmoPresentationEventListener;
pub mod KmoPresentationEventListeners;
pub mod KmoPresentationSelectionListener;
pub mod KmoPresentationSelectionListeners;
pub mod KmoPresOpenEventListener;
pub mod KmoPresOpenEventListenerGroup;
pub mod KmoPresReadModeEventListeners;
pub mod KmoPresSaveEventListener;
pub mod KmoPresSaveEventListenerGroup;
pub mod KmoSlideEventListener;
pub mod KmoSlideEventListeners;
