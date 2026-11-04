use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::util::MathX::MathX;pub struct NumericFunction{kind:u8}impl NumericFunction{// java public static final IFunction INT = new OneArg() {
// java protected double evaluate(double d) {
// java return (double)Math.round(MathX.round15(d).doubleValue() - (double)0.5F);
// java }
// java };
pub const INT:Self=Self{kind:0};// java public static final IFunction FLOOR = new TwoArg() {
// java protected IValueEval processArg(int index, IValueEval arg) {
// java return (IValueEval)(index == 1 && arg == MissingArgEval.instance ? NumericFunction.ZERO_EVAL : arg);
// java }
// java
// java protected double evaluate(double d0, double d1) throws EvaluationException {
// java if (d1 == (double)0.0F) {
// java if (d0 == (double)0.0F) {
// java return (double)0.0F;
// java } else {
// java throw EvaluationException.DIV_ZERO;
// java }
// java } else {
// java return MathX.floor(d0, d1);
// java }
// java }
// java };
pub const FLOOR:Self=Self{kind:1};// java public static final IFunction ROUNDDOWN = new TwoArg() {
// java protected IValueEval processArg(int index, IValueEval arg) {
// java return (IValueEval)(index == 1 && arg == MissingArgEval.instance ? NumericFunction.ZERO_EVAL : arg);
// java }
// java
// java protected double evaluate(double d0, double d1) {
// java return MathX.roundDown(d0, (int)d1);
// java }
// java };
pub const ROUNDDOWN:Self=Self{kind:2};// java public static final void checkValue(double result) throws EvaluationException {
// java if (Double.isNaN(result) || Double.isInfinite(result)) {
// java throw EvaluationException.NUM_ERROR;
// java }
// java }
pub fn checkValue(v:f64)->FunctionResult<()> {
if !v.is_finite(){Err(ErrorEval::NUM_ERROR.into())}else{Ok(())}
}
// java public static final double singleOperandEvaluate(IValueEval arg, EvaluationContext ec) throws EvaluationException {
// java if (arg == null) {
// java throw new IllegalArgumentException("arg must not be null");
// java } else {
// java IValueEval ve = OperandResolver.getSingleValue(arg, ec);
// java double result = OperandResolver.coerceValueToDouble(ve);
// java checkValue(result);
// java return result;
// java }
// java }
pub fn singleOperandEvaluate(v:&ValueRef,ec:&EvaluationContext)->FunctionResult<f64> {
let n=double(&single(v,ec)?)?;Self::checkValue(n)?;Ok(n)
}
}
impl MixOpFunction for NumericFunction{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{true}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 1) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java double result;
// java try {
// java double d = NumericFunction.singleOperandEvaluate(args[0], ec);
// java result = this.evaluate(d);
// java NumericFunction.checkValue(result);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return new NumberEval(result);
// java }
// java }
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java double result;
// java try {
// java double d0 = NumericFunction.singleOperandEvaluate(this.processArg(0, args[0]), ec);
// java double d1 = NumericFunction.singleOperandEvaluate(this.processArg(1, args[1]), ec);
// java result = this.evaluate(d0, d1);
// java NumericFunction.checkValue(result);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return new NumberEval(result);
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=if self.kind==0{1}else{2}{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}catch((||{let a=Self::singleOperandEvaluate(&args[0],ec)?;let b=if self.kind==0{0.0}else if args[1].as_any().is::<MissingArgEval>(){0.0}else{Self::singleOperandEvaluate(&args[1],ec)?};let n=match self.kind{0=>{let d=MathX::round15(a).doubleValue()-0.5;crate::java_runtime::javaMathRound(d)as f64},1=>{if b==0.0{if a==0.0{0.0}else{return Err(ErrorEval::DIV_ZERO.into());}}else{MathX::floor(a,b)}},2=>MathX::roundDown(a,b as i32),_=>unreachable!()};Self::checkValue(n)?;Ok(Arc::new(NumberEval::NumberEval(n))as ValueRef)})())}}
