// java public class MarkTypeAll extends MarkType
use super::{super::CalcChain::CalcChain, MarkType::MarkType};
pub struct MarkTypeAll;
impl MarkType for MarkTypeAll {
    // java calcChain.markAllRecalculationNoLock();
    fn reMark(&self, c: &CalcChain) {
        c.markAllRecalculationNoLock();
    }
    // java return o instanceof MarkTypeAll;
    fn equals(&self, o: Option<&dyn MarkType>) -> bool {
        o.is_some_and(|o| o.as_any().is::<Self>())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
