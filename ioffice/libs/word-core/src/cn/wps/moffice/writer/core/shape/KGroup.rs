// Rust stub for KGroup (auto-generated)
// Original: cn/wps/moffice/writer/core/shape/KGroup.java
use crate::*;

pub struct KGroup {
    // TODO: fields
}

impl KGroup {
    pub fn new() -> Self { todo!() }
    pub fn getBoundRect(&self) -> RectF { todo!() }
    pub fn newGroupShape(&self, rectF: RectF) -> GroupShape { todo!() }
    pub fn getRectInGroup(&self, rectBound: RectF, rectCoord: RectF, rectShape: RectF) -> RectF { todo!() }
    pub fn group(&self) -> KShape { todo!() }
    pub fn getRectOutGroup(&self, rectBound: RectF, rectCoord: RectF, rectShape: RectF) -> RectF { todo!() }
    pub fn ungroup(&self) { todo!() }
    pub fn ungroup_1(&self, coreShape: KShape, rectBound: RectF, shapeVec: Vector) { todo!() }
    pub fn delChildShape(&self) { todo!() }
    pub fn notifyDelEvent(&self, shape: Shape) { todo!() }
    pub fn getNewRectCoord(&self, shape: GroupShape) -> RectF { todo!() }
    pub fn refreshShapeSize(&self, parent: GroupShape) { todo!() }
    pub fn ungroup_2(&self, coreShape: KShape) { todo!() }
    pub fn resetShapeTree(&self, shape: Shape) { todo!() }
    pub fn getRotateRect(&self, rectBound: RectF, rectCoord: RectF, rectShape: RectF, rotation: f32) -> RectF { todo!() }
    pub fn handleTextNode(&self, shape: Shape) { todo!() }
    pub fn resetShapePos(&self, oldPos: ShapePos, groupShape: GroupShape, needRelative: bool) -> ShapePos { todo!() }
}
