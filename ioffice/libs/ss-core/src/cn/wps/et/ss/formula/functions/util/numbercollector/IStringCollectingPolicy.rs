use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
// java package cn.wps.et.ss.formula.functions.util.numbercollector;
// java
// java import cn.wps.et.ss.formula.eval.OperandResolver;
// java import cn.wps.et.ss.formula.eval.StringEval;
// java import cn.wps.et.ss.formula.evaluator.EvaluationException;
// java import cn.wps.et.ss.formula.util.DoubleList;
// java
// java public interface IStringCollectingPolicy {
// java IStringCollectingPolicy IGNORE = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java }
// java };
// java IStringCollectingPolicy ZERO = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java out.add((double)0.0F);
// java }
// java };
// java IStringCollectingPolicy NAN = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java out.add(Double.NaN);
// java }
// java };
// java IStringCollectingPolicy CONVERT_IGNORE = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java String s = se.getStringValue();
// java Double rs = OperandResolver.parseDouble(s);
// java if (rs != null) {
// java out.add(rs);
// java }
// java
// java }
// java };
// java IStringCollectingPolicy CONVERT_THROW_VALUE_INVALID = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java String s = se.getStringValue();
// java Double rs = OperandResolver.parseDouble(s);
// java if (rs != null) {
// java out.add(rs);
// java } else {
// java throw EvaluationException.VALUE_INVALID;
// java }
// java }
// java };
// java IStringCollectingPolicy CONVERT_ZERO = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java String s = se.getStringValue();
// java Double rs = OperandResolver.parseDouble(s);
// java if (rs != null) {
// java out.add(rs);
// java } else {
// java out.add((double)0.0F);
// java }
// java
// java }
// java };
// java IStringCollectingPolicy THROW_VALUE_INVALID = new IStringCollectingPolicy() {
// java public void collect(StringEval se, DoubleList out) throws EvaluationException {
// java throw EvaluationException.VALUE_INVALID;
// java }
// java };
// java
// java void collect(StringEval var1, DoubleList var2) throws EvaluationException;
// java }
#[derive(Clone,Copy,PartialEq,Eq,Hash)]pub enum IStringCollectingPolicy{IGNORE,ZERO,NAN,CONVERT_IGNORE,CONVERT_THROW_VALUE_INVALID,CONVERT_ZERO,THROW_VALUE_INVALID}
impl IStringCollectingPolicy{pub fn collect(&self,v:&StringEval,out:&mut DoubleList)->FunctionResult<()>{match self{Self::IGNORE=>{},Self::ZERO=>out.add(0.0),Self::NAN=>out.add(f64::NAN),Self::THROW_VALUE_INVALID=>return Err(ErrorEval::VALUE_INVALID.into()),_=>{if let Some(n)=OperandResolver::parseDouble(&v.getStringValue())?{out.add(n);}else if *self==Self::CONVERT_THROW_VALUE_INVALID{return Err(ErrorEval::VALUE_INVALID.into());}else if *self==Self::CONVERT_ZERO{out.add(0.0);}}}Ok(())}}
