use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::{CriteriaIf::CriteriaIf,util::matcher::ConditionMatchPredicate::ConditionMatchPredicate};use super::super::evalcache::CountifCache::CountifCache;pub struct Countif;impl MixOpFunction for Countif{fn shouldClip(&self)->bool{false}fn valueArgRequired(&self,i:usize,_:&[ValueRef],_:&EvaluationContext)->bool{i==1}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java try {
// java IAreaEval criteria_range = CriteriaIf.convertRangeArg(args[0]);
// java IMatchPredicate mp = ConditionMatchPredicate.createCriteriaPredicate(args[1], ec);
// java int rs = CountifCache.countif(criteria_range, mp);
// java return new NumberEval((double)rs);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=2{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}catch((||{let a=CriteriaIf::convertRangeArg(&args[0])?;let p=ConditionMatchPredicate::createCriteriaPredicate(&args[1],ec)?;let n=CountifCache::countif(a.as_ref(),p.as_ref())?;Ok(Arc::new(NumberEval::NumberEval(n as f64))as ValueRef)})())}}
