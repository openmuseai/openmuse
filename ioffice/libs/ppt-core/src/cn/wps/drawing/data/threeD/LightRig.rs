// Rust stub for LightRig (auto-generated)
// Original: cn/wps/drawing/data/threeD/LightRig.java
use crate::*;

pub struct LightRig {
    // TODO: fields
}

impl LightRig {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> LightRig { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> LightRig { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: LightRig) { todo!() }
    pub fn saveBin(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: LightRig) -> bool { todo!() }
    pub fn equals(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_1(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { todo!() }
    pub fn dirty(&self) -> bool { todo!() }
    pub fn save(&self) -> Box<dyn KoBin> { todo!() }
    pub fn lastBin(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn discard(&self, zone: KoZone::defaultZone()) { todo!() }
    pub fn close(&self) { todo!() }
    pub fn cleanFollow(&self) { todo!() }
    pub fn cleanCache(&self) { todo!() }
    pub fn follow(&self, parent: LightRig) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> LightRig { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn rotation(&self) -> Rotation { todo!() }
    pub fn hasRotation(&self) -> bool { todo!() }
    pub fn deleteRotation(&self) { todo!() }
    pub fn writeRotation(&self, value: Rotation) { todo!() }
    pub fn forceWriteRotation(&self, value: Rotation) { todo!() }
    pub fn rigType(&self) -> i32 { todo!() }
    pub fn hasRigType(&self) -> bool { todo!() }
    pub fn deleteRigType(&self) { todo!() }
    pub fn putRigType(&self, value: i32) { todo!() }
    pub fn dir(&self) -> i32 { todo!() }
    pub fn hasDir(&self) -> bool { todo!() }
    pub fn deleteDir(&self) { todo!() }
    pub fn putDir(&self, value: i32) { todo!() }
}
