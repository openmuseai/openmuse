// java public interface IRefEval extends IValueEval
use super::super::evaluator::EvalResult;
use super::{IValueEval, ValueRef};
pub trait IRefEval: IValueEval {
fn offset(&self,fr:i32,lr:i32,fc:i32,lc:i32)->std::sync::Arc<dyn super::IAreaEval::IAreaEval>;
    fn getInnerValueEval(&self) -> EvalResult<Option<ValueRef>>;
    fn getSheetIndex(&self) -> i32;
    fn getRow(&self) -> i32;
    fn getColumn(&self) -> i32;
}
