use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext}};use std::sync::Arc;
pub struct UnaryMinusEval;impl UnaryMinusEval{pub const instance:Self=Self;
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 1) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java double d;
// java try {
// java IValueEval ve = OperandResolver.getSingleValue(args[0], ec);
// java d = OperandResolver.coerceValueToDouble(ve);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return d == (double)0.0F ? NumberEval.ZERO : new NumberEval(-d);
// java }
// java }
pub fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef> {
if args.len()!=1{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let v=match OperandResolver::getSingleValue(&args[0],ec)?{Ok(v)=>v,Err(e)=>return Ok(Arc::new(e))};let d=match OperandResolver::coerceValueToDouble(&v)?{Ok(d)=>d,Err(e)=>return Ok(Arc::new(e))};Ok(Arc::new(NumberEval::NumberEval(if d==0.0{0.0}else{-d})))
}
}

impl super::super::base::MixOpFunction::MixOpFunction for UnaryMinusEval{fn valueArgRequired(&self,_i:usize,_args:&[ValueRef],_ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->bool{true}fn evaluateOnce(&self,rav:i8,args:&[ValueRef],ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->EvalResult<ValueRef>{UnaryMinusEval::evaluateOnce(self,rav,args,ec)}}
