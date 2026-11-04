// Rust stub for BuildEmbeddedChart (auto-generated)
// Original: cn/wps/show/data/timing/BuildEmbeddedChart.java
use crate::*;

pub struct BuildEmbeddedChart {
    // TODO: fields
}

impl BuildEmbeddedChart {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> BuildEmbeddedChart { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> BuildEmbeddedChart { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: BuildEmbeddedChart) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: BuildEmbeddedChart) -> bool { todo!() }
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
    pub fn follow(&self, parent: BuildEmbeddedChart) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> BuildEmbeddedChart { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn isAnimateBackground(&self) -> bool { todo!() }
    pub fn hasAnimateBackground(&self) -> bool { todo!() }
    pub fn deleteAnimateBackground(&self) { todo!() }
    pub fn putAnimateBackground(&self, value: bool) { todo!() }
    pub fn build(&self) -> i32 { todo!() }
    pub fn hasBuild(&self) -> bool { todo!() }
    pub fn deleteBuild(&self) { todo!() }
    pub fn putBuild(&self, value: i32) { todo!() }
    pub fn tlBuild(&self) -> TLBuild { todo!() }
    pub fn hasTlBuild(&self) -> bool { todo!() }
    pub fn deleteTlBuild(&self) { todo!() }
    pub fn writeTlBuild(&self, value: TLBuild) { todo!() }
    pub fn forceWriteTlBuild(&self, value: TLBuild) { todo!() }
}
