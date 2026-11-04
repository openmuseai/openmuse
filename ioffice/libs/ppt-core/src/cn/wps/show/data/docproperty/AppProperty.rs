// Rust stub for AppProperty (auto-generated)
// Original: cn/wps/show/data/docproperty/AppProperty.java
use crate::*;

pub struct AppProperty {
    // TODO: fields
}

impl AppProperty {
    pub fn new() -> Self { todo!() }
    pub fn initialize(clazz: Box<dyn std::any::Any + Send + Sync>, bin: Box<dyn KoBin>) ->  { todo!() }
    pub fn create() -> AppProperty { todo!() }
    pub fn open(bin: Box<dyn KoBin>) -> AppProperty { todo!() }
    pub fn loadBin(&self, bin: Box<dyn KoBin>, zone: KoZone) { todo!() }
    pub fn load(&self, bin: Box<dyn KoBin>) { todo!() }
    pub fn load_1(&self, value: AppProperty) { todo!() }
    pub fn saveBin(&self, zone: KoZone) -> Box<dyn KoBin> { todo!() }
    pub fn equalsLax(&self, o: AppProperty) -> bool { todo!() }
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
    pub fn follow(&self, parent: AppProperty) { todo!() }
    pub fn unFollow(&self) { todo!() }
    pub fn hasParent(&self) -> bool { todo!() }
    pub fn parent(&self) -> AppProperty { todo!() }
    pub fn discard_1(&self) { todo!() }
    pub fn cleanParent(&self) { todo!() }
    pub fn template(&self) -> String { todo!() }
    pub fn hasTemplate(&self) -> bool { todo!() }
    pub fn deleteTemplate(&self) { todo!() }
    pub fn putTemplate(&self, value: String) { todo!() }
    pub fn manager(&self) -> String { todo!() }
    pub fn hasManager(&self) -> bool { todo!() }
    pub fn deleteManager(&self) { todo!() }
    pub fn putManager(&self, value: String) { todo!() }
    pub fn company(&self) -> String { todo!() }
    pub fn hasCompany(&self) -> bool { todo!() }
    pub fn deleteCompany(&self) { todo!() }
    pub fn putCompany(&self, value: String) { todo!() }
    pub fn pages(&self) -> i32 { todo!() }
    pub fn hasPages(&self) -> bool { todo!() }
    pub fn deletePages(&self) { todo!() }
    pub fn putPages(&self, value: i32) { todo!() }
    pub fn words(&self) -> i32 { todo!() }
    pub fn hasWords(&self) -> bool { todo!() }
    pub fn deleteWords(&self) { todo!() }
    pub fn putWords(&self, value: i32) { todo!() }
    pub fn characters(&self) -> i32 { todo!() }
    pub fn hasCharacters(&self) -> bool { todo!() }
    pub fn deleteCharacters(&self) { todo!() }
    pub fn putCharacters(&self, value: i32) { todo!() }
    pub fn presentationFormat(&self) -> String { todo!() }
    pub fn hasPresentationFormat(&self) -> bool { todo!() }
    pub fn deletePresentationFormat(&self) { todo!() }
    pub fn putPresentationFormat(&self, value: String) { todo!() }
    pub fn lines(&self) -> i32 { todo!() }
    pub fn hasLines(&self) -> bool { todo!() }
    pub fn deleteLines(&self) { todo!() }
    pub fn putLines(&self, value: i32) { todo!() }
    pub fn paragraphs(&self) -> i32 { todo!() }
    pub fn hasParagraphs(&self) -> bool { todo!() }
    pub fn deleteParagraphs(&self) { todo!() }
    pub fn putParagraphs(&self, value: i32) { todo!() }
    pub fn slides(&self) -> i32 { todo!() }
    pub fn hasSlides(&self) -> bool { todo!() }
    pub fn deleteSlides(&self) { todo!() }
    pub fn putSlides(&self, value: i32) { todo!() }
    pub fn notes(&self) -> i32 { todo!() }
    pub fn hasNotes(&self) -> bool { todo!() }
    pub fn deleteNotes(&self) { todo!() }
    pub fn putNotes(&self, value: i32) { todo!() }
    pub fn totalTime(&self) -> i32 { todo!() }
    pub fn hasTotalTime(&self) -> bool { todo!() }
    pub fn deleteTotalTime(&self) { todo!() }
    pub fn putTotalTime(&self, value: i32) { todo!() }
    pub fn hiddenSlides(&self) -> i32 { todo!() }
    pub fn hasHiddenSlides(&self) -> bool { todo!() }
    pub fn deleteHiddenSlides(&self) { todo!() }
    pub fn putHiddenSlides(&self, value: i32) { todo!() }
    pub fn multimediaClips(&self) -> i32 { todo!() }
    pub fn hasMultimediaClips(&self) -> bool { todo!() }
    pub fn deleteMultimediaClips(&self) { todo!() }
    pub fn putMultimediaClips(&self, value: i32) { todo!() }
    pub fn isScaleCrop(&self) -> bool { todo!() }
    pub fn hasScaleCrop(&self) -> bool { todo!() }
    pub fn deleteScaleCrop(&self) { todo!() }
    pub fn putScaleCrop(&self, value: bool) { todo!() }
    pub fn headingPairs(&self) -> Vector { todo!() }
    pub fn hasHeadingPairs(&self) -> bool { todo!() }
    pub fn deleteHeadingPairs(&self) { todo!() }
    pub fn writeHeadingPairs(&self, value: Vector) { todo!() }
    pub fn forceWriteHeadingPairs(&self, value: Vector) { todo!() }
    pub fn partTitles(&self) -> Vector { todo!() }
    pub fn hasPartTitles(&self) -> bool { todo!() }
    pub fn deletePartTitles(&self) { todo!() }
    pub fn writePartTitles(&self, value: Vector) { todo!() }
    pub fn forceWritePartTitles(&self, value: Vector) { todo!() }
    pub fn isLinksUpToDate(&self) -> bool { todo!() }
    pub fn hasLinksUpToDate(&self) -> bool { todo!() }
    pub fn deleteLinksUpToDate(&self) { todo!() }
    pub fn putLinksUpToDate(&self, value: bool) { todo!() }
    pub fn charactersWithSpaces(&self) -> i32 { todo!() }
    pub fn hasCharactersWithSpaces(&self) -> bool { todo!() }
    pub fn deleteCharactersWithSpaces(&self) { todo!() }
    pub fn putCharactersWithSpaces(&self, value: i32) { todo!() }
    pub fn isSharedDoc(&self) -> bool { todo!() }
    pub fn hasSharedDoc(&self) -> bool { todo!() }
    pub fn deleteSharedDoc(&self) { todo!() }
    pub fn putSharedDoc(&self, value: bool) { todo!() }
    pub fn hyperlinkBase(&self) -> String { todo!() }
    pub fn hasHyperlinkBase(&self) -> bool { todo!() }
    pub fn deleteHyperlinkBase(&self) { todo!() }
    pub fn putHyperlinkBase(&self, value: String) { todo!() }
    pub fn hyperlinkLst(&self) -> Vector { todo!() }
    pub fn hasHyperlinkLst(&self) -> bool { todo!() }
    pub fn deleteHyperlinkLst(&self) { todo!() }
    pub fn writeHyperlinkLst(&self, value: Vector) { todo!() }
    pub fn forceWriteHyperlinkLst(&self, value: Vector) { todo!() }
    pub fn isHyperlinksChanged(&self) -> bool { todo!() }
    pub fn hasHyperlinksChanged(&self) -> bool { todo!() }
    pub fn deleteHyperlinksChanged(&self) { todo!() }
    pub fn putHyperlinksChanged(&self, value: bool) { todo!() }
    pub fn digitalSignature(&self) -> i32 { todo!() }
    pub fn hasDigitalSignature(&self) -> bool { todo!() }
    pub fn deleteDigitalSignature(&self) { todo!() }
    pub fn putDigitalSignature(&self, value: i32) { todo!() }
    pub fn application(&self) -> String { todo!() }
    pub fn hasApplication(&self) -> bool { todo!() }
    pub fn deleteApplication(&self) { todo!() }
    pub fn putApplication(&self, value: String) { todo!() }
    pub fn appVersion(&self) -> String { todo!() }
    pub fn hasAppVersion(&self) -> bool { todo!() }
    pub fn deleteAppVersion(&self) { todo!() }
    pub fn putAppVersion(&self, value: String) { todo!() }
    pub fn docSecurity(&self) -> i32 { todo!() }
    pub fn hasDocSecurity(&self) -> bool { todo!() }
    pub fn deleteDocSecurity(&self) { todo!() }
    pub fn putDocSecurity(&self, value: i32) { todo!() }
}
