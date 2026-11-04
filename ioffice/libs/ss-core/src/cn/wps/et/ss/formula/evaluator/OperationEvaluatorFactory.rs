use super::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext};use crate::cn::wps::et::ss::formula::{eval::{ValueRef,FunctionEval::FunctionEval},ptg::Ptg::{Ptg,PtgKind},functions::{base::IFunction::IFunction,builtin::{TwoOperandNumericOperation::TwoOperandNumericOperation as N,RelationalOperationEval::RelationalOperationEval as R,ConcatEval::ConcatEval,UnaryPlusEval::UnaryPlusEval,UnaryMinusEval::UnaryMinusEval,PercentEval::PercentEval}}};
pub struct OperationEvaluatorFactory;impl OperationEvaluatorFactory{
// java public static IValueEval evaluate(OperationPtg ptg, IValueEval[] args, EvaluationContext ec) {
// java if (ptg == null) {
// java throw new IllegalArgumentException("ptg must not be null");
// java } else {
// java IFunction result = (IFunction)_instancesByPtgClass.get(ptg);
// java if (result != null) {
// java return result.evaluate(ptg.getPtgClass(), args, ec);
// java } else if (ptg instanceof AbstractFunctionPtg) {
// java AbstractFunctionPtg fptg = (AbstractFunctionPtg)ptg;
// java int functionIndex = fptg.getFunctionIndex();
// java long start = System.currentTimeMillis();
// java
// java try {
// java IValueEval var8 = FunctionEval.getBasicFunction(functionIndex).evaluate(ptg.getPtgClass(), args, ec);
// java return var8;
// java } finally {
// java ;
// java }
// java } else {
// java throw new RuntimeException("Unexpected operation ptg class (" + ptg.getClass().getName() + ")");
// java }
// java }
// java }
pub fn evaluate(ptg:&Ptg,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef> {
use PtgKind::*;let rav=ptg.class as i8;match &ptg.kind{Add=>N::AddEval.evaluate(rav,args,ec),Subtract=>N::SubtractEval.evaluate(rav,args,ec),Multiply=>N::MultiplyEval.evaluate(rav,args,ec),Divide=>N::DivideEval.evaluate(rav,args,ec),Power=>N::PowerEval.evaluate(rav,args,ec),Equal=>R::EqualEval.evaluate(rav,args,ec),NotEqual=>R::NotEqualEval.evaluate(rav,args,ec),GreaterEqual=>R::GreaterEqualEval.evaluate(rav,args,ec),GreaterThan=>R::GreaterThanEval.evaluate(rav,args,ec),LessEqual=>R::LessEqualEval.evaluate(rav,args,ec),LessThan=>R::LessThanEval.evaluate(rav,args,ec),Concat=>ConcatEval::instance.evaluate(rav,args,ec),UnaryPlus=>UnaryPlusEval::instance.evaluate(rav,args,ec),UnaryMinus=>UnaryMinusEval::instance.evaluate(rav,args,ec),Percent=>PercentEval::instance.evaluate(rav,args,ec),Func{meta}|FuncVar{meta,..}=>FunctionEval::getBasicFunction(meta.getIndex())?.evaluate(rav,args,ec),FuncVarExt{index,..}=>FunctionEval::getBasicFunction(*index as i32)?.evaluate(rav,args,ec),Range|Intersection=>std::result::Result::Err(EvalFailure::Unported("RangeEval/IntersectionEval")),_=>std::result::Result::Err(EvalFailure::Runtime("Unexpected operation ptg class"))}
}
}
