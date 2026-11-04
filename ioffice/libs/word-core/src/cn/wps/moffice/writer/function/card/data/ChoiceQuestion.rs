// Rust stub for ChoiceQuestion (auto-generated)
// Original: cn/wps/moffice/writer/function/card/data/ChoiceQuestion.java
use crate::*;

pub struct ChoiceQuestion {
    // TODO: fields
}

impl ChoiceQuestion {
    pub fn new() -> Self { todo!() }
    pub fn identify(cardInfo: CardInfo, isChineseNfc: bool) -> ChoiceQuestion { todo!() }
    pub fn collect(&self, collectCardInfo: CollectCardInfo) -> bool { todo!() }
    pub fn identify_1(txt: String) -> bool { todo!() }
    pub fn highlight(&self) { todo!() }
    pub fn getType(&self) -> PaperType { todo!() }
    pub fn size(&self) -> i32 { todo!() }
    pub fn get(&self, index: i32) -> ChoiceItem { todo!() }
    pub fn collectInnerCardInfo(&self, startCps: WIntArrayList) { todo!() }
    pub fn collectUICardInfo(&self, cardInfos: WIntObjectHashMap) { todo!() }
    pub fn dispose(&self) { todo!() }
}
