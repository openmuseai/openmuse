use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
// java package cn.wps.et.ss.formula.functions.util.numbercollector;
// java
// java import cn.wps.et.ss.formula.eval.ErrorEval;
// java import cn.wps.et.ss.formula.evaluator.EvaluationException;
// java import cn.wps.et.ss.formula.util.DoubleList;
// java
// java public interface IErrorCollectingPolicy {
// java IErrorCollectingPolicy IGNORE = new IErrorCollectingPolicy() {
// java public void collect(ErrorEval ee, DoubleList out) throws EvaluationException {
// java }
// java };
// java IErrorCollectingPolicy THROW = new IErrorCollectingPolicy() {
// java public void collect(ErrorEval ee, DoubleList out) throws EvaluationException {
// java throw EvaluationException.valueOf(ee);
// java }
// java };
// java
// java void collect(ErrorEval var1, DoubleList var2) throws EvaluationException;
// java }
#[derive(Clone,Copy,PartialEq,Eq,Hash)]pub enum IErrorCollectingPolicy{IGNORE,THROW}
impl IErrorCollectingPolicy{pub fn collect(&self,v:&ErrorEval,out:&mut DoubleList)->FunctionResult<()>{if *self==Self::THROW{return Err(ErrorEval::valueOf(v.getErrorCode()).into());}Ok(())}}
