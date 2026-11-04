// java public class MarkTypeAreaSub extends MarkTypeArea
use super::{super::CalcChain::CalcChain, MarkType::MarkType, MarkTypeArea::MarkTypeArea};
use crate::cn::wps::et::ss::util::Area::Area;
#[derive(Default)]
pub struct MarkTypeAreaSub {
    base: MarkTypeArea,
}
impl MarkTypeAreaSub {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn setArea(&mut self, area: Option<Area>) {
        self.base.setArea(area);
    }
    pub fn setSheetIndex(&mut self, i: i32) {
        self.base.setSheetIndex(i);
    }
}
impl MarkType for MarkTypeAreaSub {
    fn reMark(&self, c: &CalcChain) {
        c.markSubtotalCalculationNoLock(
            self.base.sheetIndex,
            self.base.area.as_ref().expect("NullPointerException area"),
        );
    }
    fn equals(&self, o: Option<&dyn MarkType>) -> bool {
        o.and_then(|o| o.as_any().downcast_ref::<Self>())
            .is_some_and(|o| {
                self.base.sheetIndex == o.base.sheetIndex && {
                    let area = self.base.area.as_ref().expect("NullPointerException area");
                    o.base.area.as_ref().is_some_and(|a| area.equals(a))
                }
            })
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_area(&self) -> Option<&MarkTypeArea> {
        Some(&self.base)
    }
}
