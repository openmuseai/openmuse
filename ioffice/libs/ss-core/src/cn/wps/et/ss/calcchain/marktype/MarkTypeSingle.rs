// java public class MarkTypeSingle extends MarkType
use super::{super::CalcChain::CalcChain, MarkType::MarkType};
#[derive(Default)]
pub struct MarkTypeSingle {
    sheetIndex: i32,
    row: i32,
    col: i32,
}
impl MarkTypeSingle {
    pub fn new() -> Self {
        Self::default()
    }
    // java this.sheetIndex=sheetIndex;
    pub fn setSheetIndex(&mut self, value: i32) {
        self.sheetIndex = value;
    }
    // java this.row=row;
    pub fn setRow(&mut self, value: i32) {
        self.row = value;
    }
    // java this.col=col;
    pub fn setCol(&mut self, value: i32) {
        self.col = value;
    }
}
impl MarkType for MarkTypeSingle {
    // java calcChain.markRecalculationNoLock(self.sheetIndex,self.row,self.col);
    fn reMark(&self, c: &CalcChain) {
        c.markRecalculationNoLock(self.sheetIndex, self.row, self.col);
    }
    // java same concrete type and all recorded coordinates/name index equal.
    fn equals(&self, o: Option<&dyn MarkType>) -> bool {
        o.and_then(|o| o.as_any().downcast_ref::<Self>())
            .is_some_and(|o| {
                self.sheetIndex == o.sheetIndex && self.row == o.row && self.col == o.col
            })
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
