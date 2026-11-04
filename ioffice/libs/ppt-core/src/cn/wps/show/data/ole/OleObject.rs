// Rust stub for OleObject (auto-generated)
// Original: cn/wps/show/data/ole/OleObject.java
use crate::*;

pub struct OleObject {
    // TODO: fields
}

impl OleObject {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> OleObject { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> OleObject { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: OleObject) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: OleObject) -> bool { todo!() }
    pub fn equals(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_1(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { todo!() }
    pub fn dirty(&self) -> bool { todo!() }
    pub fn save(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn discard(&self, zone: KoZone) { todo!() }
    pub fn close(&self) { todo!() }
    pub fn cleanFollow(&self) { todo!() }
    pub fn cleanCache(&self) { todo!() }
    pub fn follow(&self, parent: OleObject) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> OleObject { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn embeddedColor(&self) -> i32 { todo!() }
    pub fn hasEmbeddedColor(&self) -> bool { todo!() }
    pub fn deleteEmbeddedColor(&self) { todo!() }
    pub fn putEmbeddedColor(&self, value: i32) { todo!() }
    pub fn isLinkUpdateAuto(&self) -> bool { todo!() }
    pub fn hasLinkUpdateAuto(&self) -> bool { todo!() }
    pub fn deleteLinkUpdateAuto(&self) { todo!() }
    pub fn putLinkUpdateAuto(&self, value: bool) { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn hasType(&self) -> bool { todo!() }
    pub fn deleteType(&self) { todo!() }
    pub fn putType(&self, value: i32) { todo!() }
    pub fn progID(&self) -> String { todo!() }
    pub fn hasProgID(&self) -> bool { todo!() }
    pub fn deleteProgID(&self) { todo!() }
    pub fn putProgID(&self, value: String) { todo!() }
    pub fn ole(&self) -> Ole { todo!() }
    pub fn hasOle(&self) -> bool { todo!() }
    pub fn deleteOle(&self) { todo!() }
    pub fn writeOle(&self, value: Ole) { todo!() }
    pub fn forceWriteOle(&self, value: Ole) { todo!() }
}
