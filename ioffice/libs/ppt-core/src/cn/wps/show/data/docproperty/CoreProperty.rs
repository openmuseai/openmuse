// Rust stub for CoreProperty (auto-generated)
// Original: cn/wps/show/data/docproperty/CoreProperty.java
use crate::*;

pub struct CoreProperty {
    // TODO: fields
}

impl CoreProperty {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> CoreProperty { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> CoreProperty { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: CoreProperty) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: CoreProperty) -> bool { todo!() }
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
    pub fn follow(&self, parent: CoreProperty) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> CoreProperty { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn category(&self) -> String { todo!() }
    pub fn hasCategory(&self) -> bool { todo!() }
    pub fn deleteCategory(&self) { todo!() }
    pub fn putCategory(&self, value: String) { todo!() }
    pub fn contentStatus(&self) -> String { todo!() }
    pub fn hasContentStatus(&self) -> bool { todo!() }
    pub fn deleteContentStatus(&self) { todo!() }
    pub fn putContentStatus(&self, value: String) { todo!() }
    pub fn created(&self) -> String { todo!() }
    pub fn hasCreated(&self) -> bool { todo!() }
    pub fn deleteCreated(&self) { todo!() }
    pub fn putCreated(&self, value: String) { todo!() }
    pub fn creator(&self) -> String { todo!() }
    pub fn hasCreator(&self) -> bool { todo!() }
    pub fn deleteCreator(&self) { todo!() }
    pub fn putCreator(&self, value: String) { todo!() }
    pub fn description(&self) -> String { todo!() }
    pub fn hasDescription(&self) -> bool { todo!() }
    pub fn deleteDescription(&self) { todo!() }
    pub fn putDescription(&self, value: String) { todo!() }
    pub fn identifier(&self) -> String { todo!() }
    pub fn hasIdentifier(&self) -> bool { todo!() }
    pub fn deleteIdentifier(&self) { todo!() }
    pub fn putIdentifier(&self, value: String) { todo!() }
    pub fn keyWords(&self) -> KeyWords { todo!() }
    pub fn hasKeyWords(&self) -> bool { todo!() }
    pub fn deleteKeyWords(&self) { todo!() }
    pub fn writeKeyWords(&self, value: KeyWords) { todo!() }
    pub fn forceWriteKeyWords(&self, value: KeyWords) { todo!() }
    pub fn language(&self) -> String { todo!() }
    pub fn hasLanguage(&self) -> bool { todo!() }
    pub fn deleteLanguage(&self) { todo!() }
    pub fn putLanguage(&self, value: String) { todo!() }
    pub fn lastModifiedBy(&self) -> String { todo!() }
    pub fn hasLastModifiedBy(&self) -> bool { todo!() }
    pub fn deleteLastModifiedBy(&self) { todo!() }
    pub fn putLastModifiedBy(&self, value: String) { todo!() }
    pub fn lastPrinted(&self) -> String { todo!() }
    pub fn hasLastPrinted(&self) -> bool { todo!() }
    pub fn deleteLastPrinted(&self) { todo!() }
    pub fn putLastPrinted(&self, value: String) { todo!() }
    pub fn modified(&self) -> String { todo!() }
    pub fn hasModified(&self) -> bool { todo!() }
    pub fn deleteModified(&self) { todo!() }
    pub fn putModified(&self, value: String) { todo!() }
    pub fn revision(&self) -> String { todo!() }
    pub fn hasRevision(&self) -> bool { todo!() }
    pub fn deleteRevision(&self) { todo!() }
    pub fn putRevision(&self, value: String) { todo!() }
    pub fn subject(&self) -> String { todo!() }
    pub fn hasSubject(&self) -> bool { todo!() }
    pub fn deleteSubject(&self) { todo!() }
    pub fn putSubject(&self, value: String) { todo!() }
    pub fn title(&self) -> String { todo!() }
    pub fn hasTitle(&self) -> bool { todo!() }
    pub fn deleteTitle(&self) { todo!() }
    pub fn putTitle(&self, value: String) { todo!() }
    pub fn version(&self) -> String { todo!() }
    pub fn hasVersion(&self) -> bool { todo!() }
    pub fn deleteVersion(&self) { todo!() }
    pub fn putVersion(&self, value: String) { todo!() }
}
