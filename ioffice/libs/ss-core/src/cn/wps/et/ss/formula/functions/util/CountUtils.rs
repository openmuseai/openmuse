use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::matcher::IMatchPredicate::IMatchPredicate;pub struct CountUtils;impl CountUtils{// java public static int countMatchingCellsInArea(IAreaEval areaEval, IMatchPredicate criteriaPredicate) {
// java int result = 0;
// java int touched = 0;
// java
// java for(IValueIterator iter = areaEval.valueIterator(); iter.hasNext(); ++touched) {
// java iter.next();
// java IValueEval ve = iter.getValue();
// java if (criteriaPredicate.matches(ve, false)) {
// java ++result;
// java }
// java }
// java
// java if (criteriaPredicate.matches(BlankEval.instance, false)) {
// java result += areaEval.getHeight() * areaEval.getWidth() - touched;
// java }
// java
// java return result;
// java }
pub fn countMatchingCellsInArea(a:&dyn IAreaEval,p:&dyn IMatchPredicate)->EvalResult<i32> {
let(mut n,mut touched)=(0i32,0i32);let mut i=a.valueIterator();while i.hasNext(){i.next();if p.matches(&i.getValue()?,false)?{n+=1;}touched+=1;}if p.matches(&(Arc::new(BlankEval::instance)as ValueRef),false)?{n=n.wrapping_add(a.getHeight().wrapping_mul(a.getWidth()).wrapping_sub(touched));}Ok(n)
}
// java public static int countMatchingCellsInArray(ArrayEval arrayEval, IMatchPredicate criteriaPredicate) {
// java int result = 0;
// java int touched = 0;
// java
// java for(IValueEval ve : arrayEval) {
// java if (criteriaPredicate.matches(ve, false)) {
// java ++result;
// java }
// java
// java ++touched;
// java }
// java
// java if (criteriaPredicate.matches(BlankEval.instance, false)) {
// java result += arrayEval.getRowCount() * arrayEval.getColCount() - touched;
// java }
// java
// java return result;
// java }
pub fn countMatchingCellsInArray(a:&ArrayEval,p:&dyn IMatchPredicate)->EvalResult<i32> {
let(mut n,mut touched)=(0i32,0i32);for r in 0..a.getRowCount(){for c in 0..a.getColCount(){if p.matches(&a.getValue(r,c),false)?{n+=1;}touched+=1;}}if p.matches(&(Arc::new(BlankEval::instance)as ValueRef),false)?{n=n.wrapping_add(a.getRowCount().wrapping_mul(a.getColCount()).wrapping_sub(touched));}Ok(n)
}
// java public static int countArg(IValueEval eval, IMatchPredicate criteriaPredicate) {
// java if (eval == null) {
// java throw new IllegalArgumentException("eval must not be null");
// java } else if (eval instanceof UnionEval) {
// java UnionEval ue = (UnionEval)eval;
// java int firstResult = countArg(ue.first, criteriaPredicate);
// java int secondResult = countArg(ue.second, criteriaPredicate);
// java return firstResult + secondResult;
// java } else if (eval instanceof IAreaEval) {
// java return countMatchingCellsInArea((IAreaEval)eval, criteriaPredicate);
// java } else if (eval instanceof IRefEval) {
// java return countMatchingCell((IRefEval)eval, criteriaPredicate);
// java } else if (eval instanceof ArrayEval) {
// java return countMatchingCellsInArray((ArrayEval)eval, criteriaPredicate);
// java } else {
// java return criteriaPredicate.matches(eval, true) ? 1 : 0;
// java }
// java }
pub fn countArg(v:&ValueRef,p:&dyn IMatchPredicate)->EvalResult<i32> {
if let Some(u)=v.as_any().downcast_ref::<UnionEval>(){Ok(Self::countArg(&u.first,p)?.wrapping_add(Self::countArg(&u.second,p)?))}else if let Some(a)=v.as_area_eval(){Self::countMatchingCellsInArea(a,p)}else if let Some(r)=v.as_ref_eval(){Ok(p.matches(&r.getInnerValueEval()?.ok_or(EvalFailure::Runtime("null ref"))?,false)?as i32)}else if let Some(a)=v.as_any().downcast_ref::<ArrayEval>(){Self::countMatchingCellsInArray(a,p)}else{Ok(p.matches(v,true)?as i32)}
}
}