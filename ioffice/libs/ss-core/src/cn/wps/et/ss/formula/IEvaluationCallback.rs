// java package cn.wps.et.ss.formula;
use super::{eval::ValueRef, evaluator::EvalResult};
// java public interface IEvaluationCallback
pub trait IEvaluationCallback: Send + Sync {
    // java IValueEval evaluation(int uid,int r,int c,boolean needCalc);
    fn evaluation(
        &self,
        uid: i32,
        row: i32,
        col: i32,
        needCalc: bool,
    ) -> EvalResult<Option<ValueRef>>;
    // java boolean addRef(int uid,int r,int c);
    fn addRef(&self, uid: i32, row: i32, col: i32) -> bool;
}
