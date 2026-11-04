// Rust stub for PagePositionMap (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/smallerscale/PagePositionMap.java
use crate::*;

pub struct PagePositionMap {
    // TODO: fields
}

impl PagePositionMap {
    pub fn new() -> Self { todo!() }
    pub fn getGap(&self) -> i32 { todo!() }
    pub fn isFrozen(&self) -> bool { todo!() }
    pub fn getVersion(&self) -> i64 { todo!() }
    pub fn getDocumentWidth(&self) -> i32 { todo!() }
    pub fn getDocumentHeight(&self) -> i32 { todo!() }
    pub fn getMinPageWidth(&self) -> i32 { todo!() }
    pub fn getMaxPageHeight(&self) -> i32 { todo!() }
    pub fn size(&self) -> i32 { todo!() }
    pub fn pageCount(&self) -> i32 { todo!() }
    pub fn getItem(&self, index: i32) -> Box<dyn IPosItem> { todo!() }
    pub fn commit(&self, snapshot: TypoSnapshot) { todo!() }
    pub fn frozeAll(&self) { todo!() }
    pub fn calDocumentHeight(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn checkItemConsecutive(&self) { todo!() }
    pub fn checkMapComplete(&self, snapshot: TypoSnapshot) { todo!() }
    pub fn copyTo(&self) -> PagePositionMap { todo!() }
    pub fn getItemForPageIndex(&self, pageIndex: i32) -> Box<dyn IPosItem> { todo!() }
    pub fn getPageIndexFromPos(&self, layoutY: i32) -> i32 { todo!() }
    pub fn getTopForPageIndex(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn getBottomForPageIndex(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn getTopAndBottomForPageIndex(&self, pageIndex: i32) -> Vec<i32> { todo!() }
    pub fn indexForPageIndex(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn indexForPageYPos(&self, y: i32) -> i32 { todo!() }
    pub fn addPage(&self, pageIndex: i32, pageWidth: i32, pageHeight: i32, documentWidth: i32) -> i32 { todo!() }
    pub fn removePage(&self, pageIndex: i32, documentWidth: i32) { todo!() }
    pub fn toString_function toString() { [native code] }(&self) -> String { todo!() }
    pub fn compare(&self, item: Box<dyn IPosItem>) -> bool { todo!() }
    pub fn getStartPageIndex(&self) -> i32 { todo!() }
    pub fn getEndPageIndex(&self) -> i32 { todo!() }
    pub fn getPageWidth(&self) -> i32 { todo!() }
    pub fn getPageHeight(&self) -> i32 { todo!() }
    pub fn getStartTop(&self) -> i32 { todo!() }
    pub fn getPageIndexForPos(&self, y: i32) -> i32 { todo!() }
    pub fn getTop(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn getBottom(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn getGap_1(&self) -> i32 { todo!() }
    pub fn getEndBottom(&self) -> i32 { todo!() }
    pub fn toString_function toString() { [native code] }1(&self) -> String { todo!() }
}
