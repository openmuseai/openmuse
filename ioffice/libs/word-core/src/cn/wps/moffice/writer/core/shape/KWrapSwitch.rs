// Rust stub for KWrapSwitch (auto-generated)
// Original: cn/wps/moffice/writer/core/shape/KWrapSwitch.java
use crate::*;

pub struct KWrapSwitch {
    // TODO: fields
}

impl KWrapSwitch {
    pub fn new() -> Self { todo!() }
    pub fn setWrap(&self, coreShape: KShape, r#type: WrapType, relX: i32, relY: i32, x: f32, y: f32) -> Long { todo!() }
    pub fn switchWithFloat(&self, shape: Shape, r#type: WrapType) { todo!() }
    pub fn switchWrap(&self, coreShape: KShape, r#type: WrapType, bInline: bool, relX: i32, relY: i32, x: f32, y: f32) -> i64 { todo!() }
    pub fn switchOle(&self, coreShape: KShape, r#type: WrapType, bInline: bool, relX: i32, relY: i32, x: f32, y: f32) -> Long { todo!() }
    pub fn switchOleToInline(&self, document: Box<dyn Document>, shape: Shape, cp: i32, fieldInfo: FieldInfo, propertySet: PropertySet) -> i32 { todo!() }
    pub fn switchOleToFloat(&self, document: Box<dyn Document>, shape: Shape, cp: i32, fieldInfo: FieldInfo, propertySet: PropertySet) -> i32 { todo!() }
    pub fn setPictureID(&self, shape: Shape, textboxDocument: Box<dyn Document>, node: PLCTextboxText::TextboxTextNode) { todo!() }
    pub fn getFieldInfo(&self, document: Box<dyn Document>, shape: Shape, cp: i32) -> FieldInfo { todo!() }
    pub fn switchNormal(&self, coreShape: KShape, r#type: WrapType, bInline: bool, relX: i32, relY: i32, x: f32, y: f32) -> Long { todo!() }
    pub fn swicthToInline(&self, document: Box<dyn Document>, coreShape: KShape, cp: i32, propertySet: PropertySet) -> i32 { todo!() }
    pub fn swicthToFloat(&self, document: Box<dyn Document>, shape: Shape, cp: i32, propertySet: PropertySet) -> i32 { todo!() }
    pub fn updateShapeProp(&self, shape: Shape, r#type: WrapType, bInline: bool, relX: i32, relY: i32, x: f32, y: f32) { todo!() }
    pub fn newShape(&self, document: Box<dyn Document>, rectF: RectF) -> Shape { todo!() }
    pub fn insertShape01(&self, document: Box<dyn Document>, cp: i32, rectF: RectF, revisionPropertySet: PropertySet) { todo!() }
    pub fn submitWrap(&self, bInline: bool) { todo!() }
}
