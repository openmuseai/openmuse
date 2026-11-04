#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]
//! Compile-surface hosts for parked DgGroup signatures (pp-19 function 02).
//! Not under libs/core-libs/stubs.

use crate::cn::wps::kso::kproto::KoBin::{IntBin, KoBin};

#[derive(Default, Clone, Debug)]
pub struct GroupTransform2D;
#[derive(Default, Clone, Debug)]
pub struct Transform2D;
#[derive(Default, Clone, Debug)]
pub struct KoIntList;
pub trait IIntList {}
#[derive(Default, Clone, Debug)]
pub struct Matrix;

/// Java `cn.wps.drawing.data.shape.Shape` children list only (full ShapePr is function 05).
#[derive(Default, Clone, Debug)]
pub struct Shape {
    children: Vec<i32>,
}

impl Shape {
    pub fn create() -> Self {
        Shape { children: Vec::new() }
    }
    pub fn open(_bin: Box<dyn KoBin>) -> Self {
        Shape { children: Vec::new() }
    }
    pub fn children(&self) -> &Vec<i32> {
        &self.children
    }
    pub fn children_mut(&mut self) -> &mut Vec<i32> {
        &mut self.children
    }
    /// Java `cn.wps.drawing.data.shape.Shape.deleteChildren()` (`Shape.java:1107`)：
    /// `map.put(14, null); if (children != null) children = null;`
    ///
    /// rust: 本宿主 `Shape` 只承载 children 列表（完整 `ShapePr` 属 function 05），
    /// 无 `map` 字段 ⇒ 等价表达为清空列表（使 `hasChildren()`/`save()` 的 children 计数归 0，
    /// 与 Java 置 null 后 `children()` 重建空列表的可观测结果一致）。
    pub fn deleteChildren(&mut self) {
        self.children.clear();
    }
    pub fn save(&self) -> Box<dyn KoBin> {
        let mut v = IntBin::default();
        v.from(self.children.len() as i32);
        Box::new(v)
    }
}
