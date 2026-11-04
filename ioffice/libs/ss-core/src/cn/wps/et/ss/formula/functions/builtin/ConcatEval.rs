use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext}};use std::sync::Arc;
pub struct ConcatEval;impl ConcatEval{pub const instance:Self=Self;
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java IValueEval ve0;
// java IValueEval ve1;
// java try {
// java ve0 = OperandResolver.getSingleValue(args[0], ec);
// java ve1 = OperandResolver.getSingleValue(args[1], ec);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java StringBuilder sb = new StringBuilder();
// java
// java try {
// java sb.append(getText(ve0));
// java sb.append(getText(ve1));
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return new StringEval(sb.toString());
// java }
// java }
pub fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef> {
if args.len()!=2{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let mut text=vec![];for a in args{let a=match OperandResolver::getSingleValue(a,ec)?{Ok(v)=>v,Err(e)=>return Ok(Arc::new(e))};let s=if let Some(s)=a.as_any().downcast_ref::<StringEval>(){s.getStringValue().to_vec()}else if let Some(n)=a.as_any().downcast_ref::<NumberEval>(){n.getStringValue().encode_utf16().collect()}else if let Some(b)=a.as_any().downcast_ref::<BoolEval>(){b.getStringValue().encode_utf16().collect()}else if a.as_any().is::<BlankEval>()||a.as_any().is::<MissingArgEval>(){vec![]}else if a.as_any().is::<super::super::super::eval::advance::UnionEval::UnionEval>(){return Ok(Arc::new(ErrorEval::VALUE_INVALID));}else{return Err(EvalFailure::Runtime("Unexpected value type"));};text.extend(s);}Ok(Arc::new(StringEval::from_utf16(text)))
}
}

impl super::super::base::MixOpFunction::MixOpFunction for ConcatEval{fn valueArgRequired(&self,_i:usize,_args:&[ValueRef],_ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->bool{true}fn evaluateOnce(&self,rav:i8,args:&[ValueRef],ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->EvalResult<ValueRef>{ConcatEval::evaluateOnce(self,rav,args,ec)}}
