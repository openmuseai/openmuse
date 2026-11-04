// Rust stub for KmoSlideMaster (auto-generated)
// Original: cn/wps/show/app/slide/KmoSlideMaster.java
use crate::*;

pub struct KmoSlideMaster {
    // TODO: fields
}

impl KmoSlideMaster {
    pub fn new() -> Self { todo!() }
    pub fn id(&self) -> i64 { todo!() }
    pub fn protocol(&self) -> Box<dyn KoBin> { todo!() }
    pub fn updateProtocol(&self, data: Box<dyn KoBin>) { todo!() }
    pub fn theme(&self) -> KmoTheme { todo!() }
    pub fn film(&self) -> KmoFilm { todo!() }
    pub fn eventListeners(&self) -> KmoSlideEventListeners { todo!() }
    pub fn spTree(&self) -> KmoShapeTree { todo!() }
    pub fn replaceSpid(&self, oldSpId: i32, newSpId: i32) { todo!() }
    pub fn layoutCount(&self) -> i32 { todo!() }
    pub fn indexOf(&self, slideLayout: KmoSlideLayout) -> i32 { todo!() }
    pub fn _addLayout(&self, layout: KmoSlideLayout) { todo!() }
    pub fn addLayout(&self, dstLayout: KmoSlideLayout) -> KmoSlideLayout { todo!() }
    pub fn layoutAt(&self, index: i32) -> KmoSlideLayout { todo!() }
    pub fn bodyStyle(&self) -> TextListStyle { todo!() }
    pub fn otherStyle(&self) -> TextListStyle { todo!() }
    pub fn titleStyle(&self) -> TextListStyle { todo!() }
    pub fn isPreserve(&self) -> bool { todo!() }
    pub fn setPreserve(&self, preserve: bool) { todo!() }
    pub fn headerFooter(&self) -> HeaderFooter { todo!() }
    pub fn hasHeaderFooter(&self) -> bool { todo!() }
    pub fn hasColorMapping(&self) -> bool { todo!() }
    pub fn colorMapping(&self) -> ColorMapping { todo!() }
    pub fn getPresentation(&self) -> KmoPresentation { todo!() }
    pub fn getTiming(&self) -> KmoTiming { todo!() }
    pub fn getTransition(&self) -> KmoTransition { todo!() }
    pub fn r#type(&self) -> i32 { todo!() }
    pub fn background(&self) -> Background { todo!() }
    pub fn ksoWMInfo(&self) -> KsoWMInfo { todo!() }
    pub fn templateTags(&self) -> KsoWMInfo { todo!() }
    pub fn createAnimOperator(&self) -> KmoAnimOperator { todo!() }
    pub fn addAudio(&self, audioPath: String, picPath: String, duration: i32, width: i32, height: i32) { todo!() }
    pub fn addVideo(&self, videoPath: String, picPath: String, rotation: i32, width: i32, height: i32) { todo!() }
    pub fn textStyleManager(&self) -> KmoTextStyleManager { todo!() }
    pub fn copy(&self) -> KmoSlideMaster { todo!() }
    pub fn copyFrom(&self, src: KmoSlideMaster) { todo!() }
    pub fn copyEx(&self, src: KmoSlideMaster) { todo!() }
    pub fn setIOReading(&self, b: bool) { todo!() }
    pub fn isIOReading(&self) -> bool { todo!() }
    pub fn createTransition(&self, transProperty: Vec<i32>) { todo!() }
    pub fn setEffect(&self, transProperty: Vec<i32>) { todo!() }
    pub fn getTransType(&self) -> i32 { todo!() }
    pub fn getSpeed(&self, transType: i32) -> i32 { todo!() }
    pub fn getDuration(&self, transType: i32) -> i32 { todo!() }
    pub fn getEffect(&self, transType: i32) -> List { todo!() }
    pub fn hashCode(&self) -> i32 { todo!() }
    pub fn equalsLax(&self, master: KmoSlideMaster) -> bool { todo!() }
    pub fn equals(&self, o: Box<dyn std::any::Any + Send + Sync>) -> bool { todo!() }
    pub fn equals_1(&self, o: Box<dyn std::any::Any + Send + Sync>, isStrict: bool) -> bool { todo!() }
    pub fn clear(&self) { todo!() }
}
