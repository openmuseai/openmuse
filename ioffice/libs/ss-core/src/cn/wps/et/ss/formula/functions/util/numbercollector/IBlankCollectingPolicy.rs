use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
// java package cn.wps.et.ss.formula.functions.util.numbercollector;
// java
// java import cn.wps.et.ss.formula.eval.BlankEval;
// java import cn.wps.et.ss.formula.evaluator.EvaluationException;
// java import cn.wps.et.ss.formula.util.DoubleList;
// java
// java public interface IBlankCollectingPolicy {
// java IBlankCollectingPolicy IGNORE = new IBlankCollectingPolicy() {
// java public void collect(BlankEval be, DoubleList out) throws EvaluationException {
// java }
// java };
// java IBlankCollectingPolicy ZERO = new IBlankCollectingPolicy() {
// java public void collect(BlankEval be, DoubleList out) throws EvaluationException {
// java out.add((double)0.0F);
// java }
// java };
// java IBlankCollectingPolicy NAN = new IBlankCollectingPolicy() {
// java public void collect(BlankEval be, DoubleList out) throws EvaluationException {
// java out.add(Double.NaN);
// java }
// java };
// java IBlankCollectingPolicy THROW_VALUE_INVALID = new IBlankCollectingPolicy() {
// java public void collect(BlankEval be, DoubleList out) throws EvaluationException {
// java throw EvaluationException.VALUE_INVALID;
// java }
// java };
// java
// java void collect(BlankEval var1, DoubleList var2) throws EvaluationException;
// java }
#[derive(Clone,Copy,PartialEq,Eq,Hash)]pub enum IBlankCollectingPolicy{IGNORE,ZERO,NAN,THROW_VALUE_INVALID}
impl IBlankCollectingPolicy{pub fn collect(&self,v:&BlankEval,out:&mut DoubleList)->FunctionResult<()>{match self{Self::IGNORE=>{},Self::ZERO=>out.add(0.0),Self::NAN=>out.add(f64::NAN),Self::THROW_VALUE_INVALID=>return Err(ErrorEval::VALUE_INVALID.into())}Ok(())}}
