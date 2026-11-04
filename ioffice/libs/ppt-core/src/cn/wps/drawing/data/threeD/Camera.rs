// Rust stub for Camera (auto-generated)
// Original: cn/wps/drawing/data/threeD/Camera.java
use crate::*;

pub struct Camera {
    // TODO: fields
}

impl Camera {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> Camera { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> Camera { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: Camera) { todo!() }
    pub fn saveBin(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: Camera) -> bool { todo!() }
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
    pub fn follow(&self, parent: Camera) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> Camera { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn fieldOfView(&self) -> i32 { todo!() }
    pub fn hasFieldOfView(&self) -> bool { todo!() }
    pub fn deleteFieldOfView(&self) { todo!() }
    pub fn putFieldOfView(&self, value: i32) { todo!() }
    pub fn prst(&self) -> i32 { todo!() }
    pub fn hasPrst(&self) -> bool { todo!() }
    pub fn deletePrst(&self) { todo!() }
    pub fn putPrst(&self, value: i32) { todo!() }
    pub fn zoom(&self) -> f64 { todo!() }
    pub fn hasZoom(&self) -> bool { todo!() }
    pub fn deleteZoom(&self) { todo!() }
    pub fn putZoom(&self, value: f64) { todo!() }
    pub fn rotation(&self) -> Rotation { todo!() }
    pub fn hasRotation(&self) -> bool { todo!() }
    pub fn deleteRotation(&self) { todo!() }
    pub fn writeRotation(&self, value: Rotation) { todo!() }
    pub fn forceWriteRotation(&self, value: Rotation) { todo!() }
}
