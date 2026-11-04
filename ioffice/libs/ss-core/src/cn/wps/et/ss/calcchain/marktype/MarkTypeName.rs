// java public class MarkTypeName extends MarkType
use super::{super::CalcChain::CalcChain, MarkType::MarkType};
#[derive(Default)]
pub struct MarkTypeName {
    nameIndex: i32,
}
impl MarkTypeName {
    pub fn new() -> Self {
        Self::default()
    }
    // java this.nameIndex=nameIndex;
    pub fn setIndex(&mut self, value: i32) {
        self.nameIndex = value;
    }
}
impl MarkType for MarkTypeName {
    // java calcChain.markRecalculationByNameNoLock(self.nameIndex);
    fn reMark(&self, c: &CalcChain) {
        c.markRecalculationByNameNoLock(self.nameIndex);
    }
    // java same concrete type and all recorded coordinates/name index equal.
    fn equals(&self, o: Option<&dyn MarkType>) -> bool {
        o.and_then(|o| o.as_any().downcast_ref::<Self>())
            .is_some_and(|o| self.nameIndex == o.nameIndex)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
