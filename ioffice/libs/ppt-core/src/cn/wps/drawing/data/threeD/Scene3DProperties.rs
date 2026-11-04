// Rust stub for Scene3DProperties (auto-generated)
// Original: cn/wps/drawing/data/threeD/Scene3DProperties.java
use crate::*;

pub struct Scene3DProperties {
    // TODO: fields
}

impl Scene3DProperties {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> Scene3DProperties { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> Scene3DProperties { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone::defaultZone()) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: Scene3DProperties) { todo!() }
    pub fn saveBin(&self, zone: KoZone::defaultZone()) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: Scene3DProperties) -> bool { todo!() }
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
    pub fn follow(&self, parent: Scene3DProperties) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> Scene3DProperties { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn camera(&self) -> Camera { todo!() }
    pub fn hasCamera(&self) -> bool { todo!() }
    pub fn deleteCamera(&self) { todo!() }
    pub fn writeCamera(&self, value: Camera) { todo!() }
    pub fn forceWriteCamera(&self, value: Camera) { todo!() }
    pub fn lightRig(&self) -> LightRig { todo!() }
    pub fn hasLightRig(&self) -> bool { todo!() }
    pub fn deleteLightRig(&self) { todo!() }
    pub fn writeLightRig(&self, value: LightRig) { todo!() }
    pub fn forceWriteLightRig(&self, value: LightRig) { todo!() }
    pub fn backdrop(&self) -> BackdropPlane { todo!() }
    pub fn hasBackdrop(&self) -> bool { todo!() }
    pub fn deleteBackdrop(&self) { todo!() }
    pub fn writeBackdrop(&self, value: BackdropPlane) { todo!() }
    pub fn forceWriteBackdrop(&self, value: BackdropPlane) { todo!() }
}
