#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event
// Java: import cn.wps.kso.filter.KsoFilterEvent
// rust: no-stmt —— `cn.wps.kso.filter` 在默认 feature 下不可达（与 `KmoPresOpenEventListener` 同口径：
//   父接口 `KsoFilterEvent` 无法命名，Rust 侧仅保留本接口自身契约）。

// Java: public interface KmoPresSaveEventListener extends KsoFilterEvent
//
// rust: 偏差登记 —— Java 父接口 `KsoFilterEvent` 不可达 ⇒ Rust trait 无父 trait 约束（与已登记的
//   `KmoPresOpenEventListener` 同一处理）。owner: show.app.event  wave: M-PPT-KMOSLIDE-SP1b
pub trait KmoPresSaveEventListener {
    // Java: void currentProgress(int var1);
    fn currentProgress(&mut self, var1: i32);
}
