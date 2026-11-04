// Rust stub for Build (auto-generated)
// Original: cn/wps/show/data/timing/Build.java
use crate::*;

pub struct Build {
    // TODO: fields
}

impl Build {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> Build { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> Build { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: Build) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: Build) -> bool { todo!() }
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
    pub fn follow(&self, parent: Build) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> Build { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn buildDiagram(&self) -> BuildDiagram { todo!() }
    pub fn hasBuildDiagram(&self) -> bool { todo!() }
    pub fn deleteBuildDiagram(&self) { todo!() }
    pub fn writeBuildDiagram(&self, value: BuildDiagram) { todo!() }
    pub fn forceWriteBuildDiagram(&self, value: BuildDiagram) { todo!() }
    pub fn buildGraphics(&self) -> BuildGraphics { todo!() }
    pub fn hasBuildGraphics(&self) -> bool { todo!() }
    pub fn deleteBuildGraphics(&self) { todo!() }
    pub fn writeBuildGraphics(&self, value: BuildGraphics) { todo!() }
    pub fn forceWriteBuildGraphics(&self, value: BuildGraphics) { todo!() }
    pub fn buildEmbeddedChart(&self) -> BuildEmbeddedChart { todo!() }
    pub fn hasBuildEmbeddedChart(&self) -> bool { todo!() }
    pub fn deleteBuildEmbeddedChart(&self) { todo!() }
    pub fn writeBuildEmbeddedChart(&self, value: BuildEmbeddedChart) { todo!() }
    pub fn forceWriteBuildEmbeddedChart(&self, value: BuildEmbeddedChart) { todo!() }
    pub fn buildParagraph(&self) -> BuildParagraph { todo!() }
    pub fn hasBuildParagraph(&self) -> bool { todo!() }
    pub fn deleteBuildParagraph(&self) { todo!() }
    pub fn writeBuildParagraph(&self, value: BuildParagraph) { todo!() }
    pub fn forceWriteBuildParagraph(&self, value: BuildParagraph) { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn hasType(&self) -> bool { todo!() }
    pub fn deleteType(&self) { todo!() }
    pub fn putType(&self, value: i32) { todo!() }
}
