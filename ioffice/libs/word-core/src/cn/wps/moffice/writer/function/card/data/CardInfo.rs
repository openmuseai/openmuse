// Rust stub for CardInfo (auto-generated)
// Original: cn/wps/moffice/writer/function/card/data/CardInfo.java
use crate::*;

pub struct CardInfo {
    // TODO: fields
}

impl CardInfo {
    pub fn new() -> Self { todo!() }
    pub fn getStart(&self) -> i32 { todo!() }
    pub fn getTitleEnd(&self) -> i32 { todo!() }
    pub fn highlight(&self) { todo!() }
    pub fn getTitleRange(&self) -> KRange { todo!() }
    pub fn getMaxEnd(&self) -> i32 { todo!() }
    pub fn setMaxEnd(&self, maxEnd: i32) { todo!() }
    pub fn ignoreTitle(&self) { todo!() }
    pub fn getRange(&self) -> KRange { todo!() }
    pub fn getType(&self) -> PaperType { todo!() }
    pub fn contains(&self, cp: i32) -> bool { todo!() }
    pub fn size(&self) -> i32 { todo!() }
    pub fn get(&self, index: i32) -> CardInfo { todo!() }
    pub fn collectInnerCardInfo(&self, startCps: WIntArrayList) { todo!() }
    pub fn collectUICardInfo(&self, cardInfos: WIntObjectHashMap) { todo!() }
    pub fn getLevel(&self) -> i32 { todo!() }
    pub fn dispose(&self) { todo!() }
}
