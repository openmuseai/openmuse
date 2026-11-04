use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
pub struct CriteriaIf;impl CriteriaIf{// java public static IAreaEval convertRangeArg(IValueEval eval) throws EvaluationException {
// java if (eval instanceof IAreaEval) {
// java return (IAreaEval)eval;
// java } else if (eval instanceof IRefEval) {
// java return ((IRefEval)eval).offset(0, 0, 0, 0);
// java } else if (eval instanceof NumberEval) {
// java throw EvaluationException.NUM_ERROR;
// java } else {
// java throw EvaluationException.VALUE_INVALID;
// java }
// java }
pub fn convertRangeArg(v:&ValueRef)->FunctionResult<Arc<dyn IAreaEval>> {
if let Some(a)=v.as_area_eval(){Ok(a.sharedArea())}else if let Some(r)=v.as_ref_eval(){Ok(r.offset(0,0,0,0))}else if v.as_any().is::<NumberEval>(){Err(ErrorEval::NUM_ERROR.into())}else{Err(ErrorEval::VALUE_INVALID.into())}
}
}