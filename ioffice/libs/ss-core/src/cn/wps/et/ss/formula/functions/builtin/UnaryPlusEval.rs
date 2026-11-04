use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext}};use std::sync::Arc;
pub struct UnaryPlusEval;impl UnaryPlusEval{pub const instance:Self=Self;
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 1) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java double d;
// java try {
// java IValueEval ve = OperandResolver.getSingleValue(args[0], ec);
// java if (ve instanceof StringEval) {
// java return ve;
// java }
// java
// java if (ve instanceof BoolEval) {
// java return ve;
// java }
// java
// java d = OperandResolver.coerceValueToDouble(ve);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return new NumberEval(d);
// java }
// java }
pub fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef> {
if args.len()!=1{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let v=match OperandResolver::getSingleValue(&args[0],ec)?{Ok(v)=>v,Err(e)=>return Ok(Arc::new(e))};if v.as_any().is::<StringEval>()||v.as_any().is::<BoolEval>(){return Ok(v);}let d=match OperandResolver::coerceValueToDouble(&v)?{Ok(d)=>d,Err(e)=>return Ok(Arc::new(e))};Ok(Arc::new(NumberEval::NumberEval(d)))
}
}

impl super::super::base::MixOpFunction::MixOpFunction for UnaryPlusEval{fn valueArgRequired(&self,_i:usize,_args:&[ValueRef],_ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->bool{true}fn evaluateOnce(&self,rav:i8,args:&[ValueRef],ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->EvalResult<ValueRef>{UnaryPlusEval::evaluateOnce(self,rav,args,ec)}}
