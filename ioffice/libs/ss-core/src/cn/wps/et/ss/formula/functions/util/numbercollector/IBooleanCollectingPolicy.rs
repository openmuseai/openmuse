use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
// java package cn.wps.et.ss.formula.functions.util.numbercollector;
// java
// java import cn.wps.et.ss.formula.eval.BoolEval;
// java import cn.wps.et.ss.formula.evaluator.EvaluationException;
// java import cn.wps.et.ss.formula.util.DoubleList;
// java
// java public interface IBooleanCollectingPolicy {
// java IBooleanCollectingPolicy IGNORE = new IBooleanCollectingPolicy() {
// java public void collect(BoolEval be, DoubleList out) throws EvaluationException {
// java }
// java };
// java IBooleanCollectingPolicy CONVERT = new IBooleanCollectingPolicy() {
// java public void collect(BoolEval be, DoubleList out) throws EvaluationException {
// java if (be.getBooleanValue()) {
// java out.add((double)1.0F);
// java } else {
// java out.add((double)0.0F);
// java }
// java
// java }
// java };
// java IBooleanCollectingPolicy NAN = new IBooleanCollectingPolicy() {
// java public void collect(BoolEval be, DoubleList out) throws EvaluationException {
// java out.add(Double.NaN);
// java }
// java };
// java IBooleanCollectingPolicy THROW_VALUE_INVALID = new IBooleanCollectingPolicy() {
// java public void collect(BoolEval be, DoubleList out) throws EvaluationException {
// java throw EvaluationException.VALUE_INVALID;
// java }
// java };
// java
// java void collect(BoolEval var1, DoubleList var2) throws EvaluationException;
// java }
#[derive(Clone,Copy,PartialEq,Eq,Hash)]pub enum IBooleanCollectingPolicy{IGNORE,CONVERT,NAN,THROW_VALUE_INVALID}
impl IBooleanCollectingPolicy{pub fn collect(&self,v:&BoolEval,out:&mut DoubleList)->FunctionResult<()>{match self{Self::IGNORE=>{},Self::CONVERT=>out.add(if v.getBooleanValue(){1.0}else{0.0}),Self::NAN=>out.add(f64::NAN),Self::THROW_VALUE_INVALID=>return Err(ErrorEval::VALUE_INVALID.into())}Ok(())}}
