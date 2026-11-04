// Rust stub for PredictSpacesInfo (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/charSpace/PredictSpacesInfo.java
use crate::*;

pub struct PredictSpacesInfo {
    // TODO: fields
}

impl PredictSpacesInfo {
    pub fn new() -> Self { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn getTabStops(&self) -> WIntArrayList { todo!() }
    pub fn getStartCp(&self) -> i32 { todo!() }
    pub fn getSpaces(&self) -> ArrayList { todo!() }
    pub fn getOptionNumberAfter(run: Graphs, layoutState: LayoutState) -> OptionNumber { todo!() }
    pub fn getOptionNumberAfter_1(beginIndex: i32, layoutState: LayoutState) -> OptionNumber { todo!() }
    pub fn checkNoCommentEnd(&self, rangeInfo: PhoneViewFitInfo::RangeInfo) -> bool { todo!() }
    pub fn predictSpacePattern(&self, layoutWidth: i32, filterFoSpaces: FilterForSpaces) { todo!() }
    pub fn needExpandAndTabStop(&self) -> bool { todo!() }
    pub fn getTag(&self, bitSet: BoolBitSet) -> bool { todo!() }
    pub fn indentPos(&self) -> i32 { todo!() }
    pub fn merge(&self, rangeInfo: PhoneViewFitInfo::RangeInfo) -> bool { todo!() }
    pub fn isSignable(&self) -> bool { todo!() }
    pub fn isSignable_1(&self) -> bool { todo!() }
    pub fn isSignable_2(&self) -> bool { todo!() }
}

pub struct SpaceItem { /* TODO */ }
pub struct MatchResult { /* TODO */ }
pub trait IContextHelper {}
