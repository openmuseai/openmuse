use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::super::eval::UDFNameEval::UDFNameEval;pub struct UserDefinedFunction;
// java public IValueEval evaluate(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java int nIncomingArgs = args.length;
// java if (nIncomingArgs < 1) {
// java throw new RuntimeException("function name argument missing");
// java } else {
// java IValueEval nameArg = args[0];
// java if (nameArg instanceof UDFNameEval) {
// java String functionName = ((UDFNameEval)nameArg).getFunctionName();
// java IFunction targetFunc = ec.findUserDefinedFunction(functionName);
// java if (targetFunc == null) {
// java return ErrorEval.NAME_INVALID;
// java } else {
// java long start = System.currentTimeMillis();
// java
// java try {
// java int nOutGoingArgs = nIncomingArgs - 1;
// java IValueEval[] outGoingArgs = new IValueEval[nOutGoingArgs];
// java System.arraycopy(args, 1, outGoingArgs, 0, nOutGoingArgs);
// java IValueEval var12 = targetFunc.evaluate(funcRAV, outGoingArgs, ec);
// java return var12;
// java } finally {
// java ;
// java }
// java }
// java } else if (nameArg instanceof ErrorEval) {
// java return nameArg;
// java } else {
// java throw new RuntimeException("First argument should be a UDFNameEval, but got (" + nameArg.getClass().getName() + ")");
// java }
// java }
// java }
impl super::base::IFunction::IFunction for UserDefinedFunction{fn evaluate(&self,rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{let Some(first)=args.first()else{return Err(EvalFailure::Runtime("function name argument missing"));};if let Some(name)=first.as_any().downcast_ref::<UDFNameEval>(){let Some(target)=ec.findUserDefinedFunction(name.getFunctionName())?else{return Ok(Arc::new(ErrorEval::NAME_INVALID));};target.evaluate(rav,&args[1..],ec)}else if first.as_any().is::<ErrorEval>(){Ok(first.clone())}else{Err(EvalFailure::Runtime("First argument should be a UDFNameEval"))}}}
