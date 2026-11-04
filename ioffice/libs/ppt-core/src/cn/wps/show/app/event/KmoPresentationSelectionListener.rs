#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.event

// Java: public interface KmoPresentationSelectionListener
pub trait KmoPresentationSelectionListener {
    // Java: void onSelectionChanged(int var1);
    fn onSelectionChanged(&mut self, var1: i32);
}
