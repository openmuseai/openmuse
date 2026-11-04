use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::{MultiOperandNumericFunction::MultiOperandNumericFunction,util::{MathX::MathX,statistics::StatsLib::StatsLib}};use super::super::evalcache::LargeSmallCache::LargeSmallCache;
// java public abstract class AggregateFunction extends MultiOperandNumericFunction
pub struct AggregateFunction{kind:u8}impl AggregateFunction{
// java public static final AggregateFunction AVERAGE = new AggregateFunction() {
// java protected double evaluate(double[] values) throws EvaluationException {
// java if (values.length < 1) {
// java throw EvaluationException.DIV_ZERO;
// java } else {
// java return MathX.average(values);
// java }
// java }
// java };
pub const AVERAGE:Self=Self{kind:0};
// java public static final AggregateFunction MAX = new AggregateFunction() {
// java protected double evaluate(double[] values) {
// java return values.length > 0 ? MathX.max(values) : (double)0.0F;
// java }
// java };
pub const MAX:Self=Self{kind:1};
// java public static final AggregateFunction MIN = new AggregateFunction() {
// java protected double evaluate(double[] values) {
// java return values.length > 0 ? MathX.min(values) : (double)0.0F;
// java }
// java };
pub const MIN:Self=Self{kind:2};
// java public static final AggregateFunction STDEVP = new AggregateFunction() {
// java protected double evaluate(double[] values) throws EvaluationException {
// java if (values.length < 1) {
// java throw EvaluationException.DIV_ZERO;
// java } else {
// java return StatsLib.stdevp(values);
// java }
// java }
// java };
pub const STDEVP:Self=Self{kind:3};
// java public static final AggregateFunction SUM = new AggregateFunction() {
// java protected double evaluate(double[] values) {
// java return MathX.sum(values);
// java }
// java };
pub const SUM:Self=Self{kind:4};
}
impl MixOpFunction for AggregateFunction{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{false}fn shouldClip(&self)->bool{false}
// java public final IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java double d;
// java try {
// java double[] values = this.getNumberArray(args);
// java d = this.evaluate(values);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return (IValueEval)(!Double.isNaN(d) && !Double.isInfinite(d) ? new NumberEval(d) : ErrorEval.NUM_ERROR);
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],_:&EvaluationContext)->EvalResult<ValueRef>{catch((||{let v=MultiOperandNumericFunction::new(false,false,false,true).getNumberArray(args)?;let n=match self.kind{0=>{if v.is_empty(){return Err(ErrorEval::DIV_ZERO.into());}MathX::average(&v)},1=>if v.is_empty(){0.0}else{MathX::max(&v,0,v.len())},2=>if v.is_empty(){0.0}else{MathX::min(&v,0,v.len())},3=>{if v.is_empty(){return Err(ErrorEval::DIV_ZERO.into());}StatsLib::stdevp(&v)},4=>MathX::sum(&v),_=>unreachable!()};Ok(if n.is_finite(){Arc::new(NumberEval::NumberEval(n))as ValueRef}else{Arc::new(ErrorEval::NUM_ERROR)})})())}}
// java private static final class LargeSmall extends MixOpFunction
pub struct LargeSmall{isLarge:bool}impl LargeSmall{pub const LARGE:Self=Self{isLarge:true};pub const SMALL:Self=Self{isLarge:false};}
impl MixOpFunction for LargeSmall{fn valueArgRequired(&self,i:usize,_:&[ValueRef],_:&EvaluationContext)->bool{i==1}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java double dn;
// java try {
// java IValueEval ve1 = OperandResolver.getSingleValue(args[1], ec);
// java dn = OperandResolver.coerceValueToDouble(ve1);
// java } catch (EvaluationException e1) {
// java return e1.getErrorEval();
// java }
// java
// java if (dn < (double)1.0F) {
// java return ErrorEval.NUM_ERROR;
// java } else {
// java int k = this._isLarge ? (int)Math.ceil(dn) : (int)Math.floor(dn);
// java
// java double result;
// java try {
// java if (args[0] instanceof IAreaEval) {
// java IAreaEval ae = (IAreaEval)args[0];
// java if (!ec.isCondFormula()) {
// java return LargeSmallCache.largesmall(ae, k, dn, this._isLarge);
// java }
// java
// java return LargeSmallCache.largeSmallCond(ae, k, dn, this._isLarge);
// java }
// java
// java int orgCount = -1;
// java if (args[0] instanceof ArrayEval) {
// java orgCount = Math.max(((ArrayEval)args[0]).getOrgColCount(), ((ArrayEval)args[0]).getOrgRowCount());
// java }
// java
// java double[] ds = AggregateFunction.ValueCollector.collectValues(args[0]);
// java if (dn > (double)ds.length) {
// java if (args[0] instanceof ArrayEval) {
// java ArrayEval ae = (ArrayEval)args[0];
// java if (ae.getOrgRowCount() <= ae.getRowCount() && ae.getOrgColCount() <= ae.getColCount()) {
// java return ErrorEval.NUM_ERROR;
// java }
// java
// java if (dn > (double)Math.max(ae.getOrgRowCount(), ae.getOrgColCount())) {
// java return ErrorEval.NUM_ERROR;
// java }
// java
// java return BlankEval.instance;
// java }
// java
// java if (args[0] instanceof IAreaEval) {
// java IAreaEval ae = (IAreaEval)args[0];
// java if (ae.getOrgWidth() <= ae.getWidth() && ae.getOrgHeight() <= ae.getHeight()) {
// java return ErrorEval.NUM_ERROR;
// java }
// java
// java if (dn > (double)Math.max(ae.getOrgWidth(), ae.getOrgHeight())) {
// java return ErrorEval.NUM_ERROR;
// java }
// java
// java return BlankEval.instance;
// java }
// java
// java return ErrorEval.NUM_ERROR;
// java }
// java
// java if (this._isLarge) {
// java try {
// java StatsLib.nth_value(ds, 0, ds.length, ds.length - k);
// java result = ds[ds.length - k];
// java } catch (Exception var13) {
// java return ErrorEval.NUM_ERROR;
// java }
// java } else {
// java try {
// java StatsLib.nth_value(ds, 0, ds.length, k - 1);
// java result = ds[k - 1];
// java } catch (Exception var12) {
// java return ErrorEval.NUM_ERROR;
// java }
// java }
// java
// java NumericFunction.checkValue(result);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java return new NumberEval(result);
// java }
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=2{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}catch((||{let dn=double(&single(&args[1],ec)?)?;if dn<1.0{return Ok(Arc::new(ErrorEval::NUM_ERROR)as ValueRef);}let k=if self.isLarge{dn.ceil()as i32}else{dn.floor()as i32};if let Some(a)=args[0].as_area_eval(){return if ec.isCondFormula(){LargeSmallCache::largeSmallCond(a,k,dn,self.isLarge)}else{LargeSmallCache::largesmall(a,k,dn,self.isLarge)};}let mut ds=MultiOperandNumericFunction::new(false,false,false,true).getNumberArray(&args[..1])?;if dn>ds.len()as f64{if let Some(a)=args[0].as_any().downcast_ref::<ArrayEval>(){if a.getOrgRowCount()<=a.getRowCount()&&a.getOrgColCount()<=a.getColCount()||dn>a.getOrgRowCount().max(a.getOrgColCount())as f64{return Ok(Arc::new(ErrorEval::NUM_ERROR)as ValueRef);}return Ok(Arc::new(BlankEval::instance)as ValueRef);}return Ok(Arc::new(ErrorEval::NUM_ERROR)as ValueRef);}let len=ds.len();let n=if self.isLarge{len as i32-k}else{k-1};if n<0||n as usize>=len{return Ok(Arc::new(ErrorEval::NUM_ERROR)as ValueRef);}StatsLib::nth_value(&mut ds,0,len,n as usize);let result=ds[n as usize];if !result.is_finite(){return Err(ErrorEval::NUM_ERROR.into());}Ok(Arc::new(NumberEval::NumberEval(result))as ValueRef)})())}}
