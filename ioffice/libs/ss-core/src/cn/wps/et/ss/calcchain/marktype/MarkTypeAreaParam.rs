// java public class MarkTypeAreaParam extends MarkTypeArea
use super::{super::CalcChain::CalcChain, MarkType::MarkType, MarkTypeArea::MarkTypeArea};
use crate::cn::wps::et::ss::util::Area::Area;
#[derive(Default)]
pub struct MarkTypeAreaParam {
    base: MarkTypeArea,
    markCurrentRange: bool,
}
impl MarkTypeAreaParam {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn setArea(&mut self, area: Option<Area>) {
        self.base.setArea(area);
    }
    pub fn setSheetIndex(&mut self, i: i32) {
        self.base.setSheetIndex(i);
    }
    // java this.markCurrentRange=markCurrentRange;
    pub fn setMarkCurrentRange(&mut self, b: bool) {
        self.markCurrentRange = b;
    }
}
impl MarkType for MarkTypeAreaParam {
    fn reMark(&self, c: &CalcChain) {
        c.markRecalculationNoLock__area(
            self.base.sheetIndex,
            self.base.area.as_ref(),
            self.markCurrentRange,
        );
    }
    // java null input/null area returns true; any non-null input with null area returns false, including itself.
    fn equals(&self, o: Option<&dyn MarkType>) -> bool {
        if o.is_none() && self.base.area.is_none() {
            return true;
        }
        if o.is_none() || self.base.area.is_none() {
            return false;
        }
        o.and_then(|o| o.as_any().downcast_ref::<Self>())
            .is_some_and(|o| {
                self.base.sheetIndex == o.base.sheetIndex
                    && o.base
                        .area
                        .as_ref()
                        .is_some_and(|a| self.base.area.as_ref().unwrap().equals(a))
                    && self.markCurrentRange == o.markCurrentRange
            })
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_area(&self) -> Option<&MarkTypeArea> {
        Some(&self.base)
    }
}
