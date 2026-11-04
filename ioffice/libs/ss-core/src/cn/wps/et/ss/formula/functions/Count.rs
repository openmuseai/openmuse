use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::util::{CountUtils::CountUtils,matcher::IMatchPredicate::IMatchPredicate};pub struct Count;struct Predicate;
// java static final IMatchPredicate predicate = new IMatchPredicate() {
// java public boolean matches(IValueEval valueEval, boolean directlyArg) {
// java if (valueEval instanceof NumberEval) {
// java return true;
// java } else if (valueEval == MissingArgEval.instance) {
// java return true;
// java } else if (valueEval instanceof BoolEval) {
// java return directlyArg;
// java } else {
// java return valueEval == BlankEval.instance ? false : false;
// java }
// java }
// java };
impl IMatchPredicate for Predicate{fn as_any(&self)->&dyn std::any::Any{self}fn matches(&self,v:&ValueRef,direct:bool)->EvalResult<bool>{Ok(v.as_any().is::<NumberEval>()||v.as_any().is::<MissingArgEval>()||(v.as_any().is::<BoolEval>()&&direct))}}
impl MixOpFunction for Count{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{false}fn shouldClip(&self)->bool{true}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java int nArgs = args.length;
// java if (nArgs >= 1 && nArgs <= 30) {
// java int temp = 0;
// java
// java for(int i = 0; i < nArgs; ++i) {
// java temp += CountUtils.countArg(args[i], predicate);
// java }
// java
// java return new NumberEval((double)temp);
// java } else {
// java return ErrorEval.VALUE_INVALID;
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],_:&EvaluationContext)->EvalResult<ValueRef>{if args.is_empty()||args.len()>30{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let mut n=0i32;for v in args{n=n.wrapping_add(CountUtils::countArg(v,&Predicate)?);}Ok(Arc::new(NumberEval::NumberEval(n as f64)))}}
