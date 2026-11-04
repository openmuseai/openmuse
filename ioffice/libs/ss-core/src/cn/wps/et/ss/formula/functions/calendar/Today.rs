use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use kfc::cn::wps::kfc::numfmt::util::DateUtil::DateUtil;pub struct Today;impl MixOpFunction for Today{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{true}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 0) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java Calendar now = new GregorianCalendar();
// java double v = DateUtil.getExcelDate(now, ec.isUsing1904());
// java return new NumberEval(Math.floor(v));
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if !args.is_empty(){return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let now=crate::java_runtime::Calendar::getInstance();let v=DateUtil::getExcelDate_from_calendar(&now,ec.isUsing1904());Ok(Arc::new(NumberEval::NumberEval(v.floor())))}}
