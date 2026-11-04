use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver,advance::ArrayEval::ArrayEval},evaluator::{EvalResult,EvaluationContext::EvaluationContext},functions::base::MixOpFunction::MixOpFunction};use std::sync::Arc;
pub struct IfFunc;impl IfFunc{
// java public static boolean evaluateFirstArg(IValueEval arg, EvaluationContext ec) throws EvaluationException {
// java if (arg == MissingArgEval.instance) {
// java return false;
// java } else {
// java IValueEval ve = OperandResolver.getSingleValue(arg, ec);
// java Boolean b = OperandResolver.coerceValueToBoolean(ve, false);
// java return b == null ? false : b;
// java }
// java }
pub fn evaluateFirstArg(arg:&ValueRef,ec:&EvaluationContext)->EvalResult<Result<bool,&'static ErrorEval>> {
if arg.as_any().is::<MissingArgEval>(){return Ok(Ok(false));}let v=match OperandResolver::getSingleValue(arg,ec)?{Ok(v)=>v,Err(e)=>return Ok(Err(e))};OperandResolver::coerceValueToBoolean(&v,false).map(|v|v.map(|b|b.unwrap_or(false)))
}
}
impl MixOpFunction for IfFunc{
// java public boolean valueArgRequired(int argPos, IValueEval[] args, EvaluationContext ec) {
// java if (!(args[0] instanceof ArrayEval) && (!(args[0] instanceof IAreaEval) || !ec.isArrayFormula())) {
// java return argPos == 0;
// java } else {
// java return true;
// java }
// java }
fn valueArgRequired(&self,pos:usize,args:&[ValueRef],ec:&EvaluationContext)->bool{if !args[0].as_any().is::<ArrayEval>()&&(args[0].as_area_eval().is_none()||!ec.isArrayFormula()){pos==0}else{true}}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2 && args.length != 3) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java boolean b;
// java try {
// java b = evaluateFirstArg(args[0], ec);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java if (b) {
// java return (IValueEval)(args[1] == MissingArgEval.instance ? BlankEval.instance : args[1]);
// java } else if (args.length == 2) {
// java return BoolEval.FALSE;
// java } else {
// java return (IValueEval)(args[2] == MissingArgEval.instance ? BlankEval.instance : args[2]);
// java }
// java }
// java }
fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=2&&args.len()!=3{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let yes=match Self::evaluateFirstArg(&args[0],ec)?{Ok(b)=>b,Err(e)=>return Ok(Arc::new(e))};let v=if yes{args[1].clone()}else if args.len()==2{return Ok(Arc::new(BoolEval::FALSE));}else{args[2].clone()};Ok(if v.as_any().is::<MissingArgEval>(){Arc::new(BlankEval::instance)}else{v})}}
