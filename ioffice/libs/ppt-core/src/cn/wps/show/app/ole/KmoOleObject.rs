// Rust stub for KmoOleObject (auto-generated)
// Original: cn/wps/show/app/ole/KmoOleObject.java
use crate::*;

pub struct KmoOleObject {
    // TODO: fields
}

impl KmoOleObject {
    pub fn new() -> Self { todo!() }
    pub fn protocol(&self) -> Box<dyn KoBin> { todo!() }
    pub fn updateProtocol(&self, data: Box<dyn KoBin>) { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn copy(&self) -> Box<dyn GraphicObject> { todo!() }
    pub fn setMediaId(&self, mediaId: i32) { todo!() }
    pub fn mediaId(&self) -> i32 { todo!() }
    pub fn picture(&self) -> KmoShowShape { todo!() }
    pub fn setPicture(&self, pic: KmoShowShape) { todo!() }
    pub fn getOleMediaPath(&self) -> String { todo!() }
    pub fn oleProgId(&self) -> String { todo!() }
    pub fn isEmbed(&self) -> bool { todo!() }
}
