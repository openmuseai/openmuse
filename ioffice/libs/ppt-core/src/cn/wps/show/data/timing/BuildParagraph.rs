// Rust stub for BuildParagraph (auto-generated)
// Original: cn/wps/show/data/timing/BuildParagraph.java
use crate::*;

pub struct BuildParagraph {
    // TODO: fields
}

impl BuildParagraph {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> BuildParagraph { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> BuildParagraph { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: BuildParagraph) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: BuildParagraph) -> bool { todo!() }
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
    pub fn follow(&self, parent: BuildParagraph) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> BuildParagraph { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn templateLst(&self) -> TemplateList { todo!() }
    pub fn hasTemplateLst(&self) -> bool { todo!() }
    pub fn deleteTemplateLst(&self) { todo!() }
    pub fn writeTemplateLst(&self, value: TemplateList) { todo!() }
    pub fn forceWriteTemplateLst(&self, value: TemplateList) { todo!() }
    pub fn autoAdvanceTime(&self) -> i32 { todo!() }
    pub fn hasAutoAdvanceTime(&self) -> bool { todo!() }
    pub fn deleteAutoAdvanceTime(&self) { todo!() }
    pub fn putAutoAdvanceTime(&self, value: i32) { todo!() }
    pub fn isAnimateBackground(&self) -> bool { todo!() }
    pub fn hasAnimateBackground(&self) -> bool { todo!() }
    pub fn deleteAnimateBackground(&self) { todo!() }
    pub fn putAnimateBackground(&self, value: bool) { todo!() }
    pub fn isAutoUpdateAnimBg(&self) -> bool { todo!() }
    pub fn hasAutoUpdateAnimBg(&self) -> bool { todo!() }
    pub fn deleteAutoUpdateAnimBg(&self) { todo!() }
    pub fn putAutoUpdateAnimBg(&self, value: bool) { todo!() }
    pub fn buildLevel(&self) -> i32 { todo!() }
    pub fn hasBuildLevel(&self) -> bool { todo!() }
    pub fn deleteBuildLevel(&self) { todo!() }
    pub fn putBuildLevel(&self, value: i32) { todo!() }
    pub fn buildTypes(&self) -> i32 { todo!() }
    pub fn hasBuildTypes(&self) -> bool { todo!() }
    pub fn deleteBuildTypes(&self) { todo!() }
    pub fn putBuildTypes(&self, value: i32) { todo!() }
    pub fn isReverse(&self) -> bool { todo!() }
    pub fn hasReverse(&self) -> bool { todo!() }
    pub fn deleteReverse(&self) { todo!() }
    pub fn putReverse(&self, value: bool) { todo!() }
    pub fn tlBuild(&self) -> TLBuild { todo!() }
    pub fn hasTlBuild(&self) -> bool { todo!() }
    pub fn deleteTlBuild(&self) { todo!() }
    pub fn writeTlBuild(&self, value: TLBuild) { todo!() }
    pub fn forceWriteTlBuild(&self, value: TLBuild) { todo!() }
    pub fn loadBin_1(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load_2(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_3(&self, value: TemplateList) { todo!() }
    pub fn saveBin_1(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax_1(&self, o: TemplateList) -> bool { todo!() }
    pub fn equals_2(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_3(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn hashCode_1(&self) -> i32 { todo!() }
    pub fn dirty_1(&self) -> bool { todo!() }
    pub fn save_1(&self) -> Box<dyn KoBin> { todo!() }
    pub fn discard_2(&self, zone: KoZone) { todo!() }
    pub fn lastBin_1(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn close_1(&self) { todo!() }
    pub fn initialize_1(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create_1() -> TemplateList { todo!() }
    pub fn open_1(bin: Box<dyn KoBin>) -> TemplateList { todo!() }
    pub fn read(&self, valueLst: Collection) { todo!() }
    pub fn write(&self, valueLst: Iterable) { todo!() }
    pub fn forceWrite(&self, valueLst: Iterable) { todo!() }
}
