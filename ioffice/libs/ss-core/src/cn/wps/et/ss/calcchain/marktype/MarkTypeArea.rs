// java public class MarkTypeArea extends MarkType
use super::{super::CalcChain::CalcChain, MarkType::MarkType};
use crate::cn::wps::et::ss::util::Area::Area;
#[derive(Default)]
pub struct MarkTypeArea {
    pub(crate) area: Option<Area>,
    pub(crate) sheetIndex: i32,
}
impl MarkTypeArea {
    pub fn new() -> Self {
        Self::default()
    }
    // java this.area=area;
    pub fn setArea(&mut self, area: Option<Area>) {
        self.area = area;
    }
    // java this.sheetIndex=sheetIndex;
    pub fn setSheetIndex(&mut self, index: i32) {
        self.sheetIndex = index;
    }
}
impl MarkType for MarkTypeArea {
    fn reMark(&self, c: &CalcChain) {
        c.markRecalculationNoLock__area(self.sheetIndex, self.area.as_ref(), true);
    }
    // java instanceof MarkTypeArea includes subclasses; area.equals(null) is false, null receiver throws.
    fn equals(&self, o: Option<&dyn MarkType>) -> bool {
        o.and_then(|o| o.as_area()).is_some_and(|o| {
            self.sheetIndex == o.sheetIndex && {
                let area = self.area.as_ref().expect("NullPointerException area");
                o.area.as_ref().is_some_and(|other| area.equals(other))
            }
        })
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_area(&self) -> Option<&MarkTypeArea> {
        Some(self)
    }
}
