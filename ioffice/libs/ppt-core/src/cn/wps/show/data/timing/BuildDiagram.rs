// Rust stub for BuildDiagram (auto-generated)
// Original: cn/wps/show/data/timing/BuildDiagram.java
use crate::*;

pub struct BuildDiagram {
    // TODO: fields
}

impl BuildDiagram {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> BuildDiagram { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> BuildDiagram { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: BuildDiagram) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: BuildDiagram) -> bool { todo!() }
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
    pub fn follow(&self, parent: BuildDiagram) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> BuildDiagram { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn hasType(&self) -> bool { todo!() }
    pub fn deleteType(&self) { todo!() }
    pub fn putType(&self, value: i32) { todo!() }
    pub fn tlBuild(&self) -> TLBuild { todo!() }
    pub fn hasTlBuild(&self) -> bool { todo!() }
    pub fn deleteTlBuild(&self) { todo!() }
    pub fn writeTlBuild(&self, value: TLBuild) { todo!() }
    pub fn forceWriteTlBuild(&self, value: TLBuild) { todo!() }
}
