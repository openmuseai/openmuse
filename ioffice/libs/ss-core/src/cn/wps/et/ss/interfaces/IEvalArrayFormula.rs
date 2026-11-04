// java public interface IEvalArrayFormula extends IEvalFormulaBase
use super::IEvalFormulaBase::IEvalFormulaBase;
use crate::cn::wps::et::ss::calcchain::CellFmlEvaluator::ValuePos;
use crate::cn::wps::et::ss::formula::{
    eval::ValueRef,
    evaluator::{EvalResult, EvaluationContext::EvaluationContext},
};
pub trait IEvalArrayFormula: IEvalFormulaBase {
    fn getFmlLastRow(&self) -> i32;
    fn getFmlLastCol(&self) -> i32;
    fn setStringResult(&self, row: i32, col: i32, value: &str);
    fn setNumberResult(&self, row: i32, col: i32, value: f64);
    fn setErrorResult(&self, row: i32, col: i32, value: i8);
    fn setBoolResult(&self, row: i32, col: i32, value: bool);
    fn setDirtyValue(
        &self,
        value: Option<ValueRef>,
        positions: &ValuePos,
        uid: i32,
        ec: &EvaluationContext,
    ) -> EvalResult<()>;
}
