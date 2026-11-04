use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
// java public static final IFunction WEEKDAY = new ValueOpFunction() {
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java try {
// java int arg1 = 1;
// java switch (args.length) {
// java case 2:
// java if (args[1] != MissingArgEval.instance) {
// java IValueEval ve = OperandResolver.getSingleValue(args[1], ec);
// java if (ve == BlankEval.instance || ve instanceof MissingArgEval) {
// java return ErrorEval.NUM_ERROR;
// java } else {
// java arg1 = OperandResolver.coerceValueToInt(ve);
// java }
// java }
// java case 1:
// java if (arg1 >= 1 && arg1 <= 3 || arg1 >= 11 && arg1 <= 17) {
// java IValueEval ve = OperandResolver.getSingleValue(args[0], ec);
// java double arg0 = OperandResolver.coerceValueToDouble(ve);
// java return new NumberEval((double)this.weekday(arg0, arg1, ec.isUsing1904()));
// java }
// java
// java return ErrorEval.NUM_ERROR;
// java default:
// java return ErrorEval.VALUE_INVALID;
// java }
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java }
// java
// java int weekday(double v, int type, boolean using1904) {
// java switch (type) {
// java case 1:
// java case 17:
// java return ((int)v + 6) % 7 + 1;
// java case 2:
// java case 4:
// java case 5:
// java case 6:
// java case 7:
// java case 8:
// java case 9:
// java case 10:
// java case 11:
// java default:
// java return ((int)v + 5) % 7 + 1;
// java case 3:
// java return ((int)v + 5) % 7;
// java case 12:
// java return ((int)v + 4) % 7 + 1;
// java case 13:
// java return ((int)v + 3) % 7 + 1;
// java case 14:
// java return ((int)v + 2) % 7 + 1;
// java case 15:
// java return ((int)v + 1) % 7 + 1;
// java case 16:
// java return ((int)v + 0) % 7 + 1;
// java }
// java }
// java };
pub struct CalendarFunction;impl CalendarFunction{pub const WEEKDAY:Self=Self;
// java public static boolean isValidCalendar(double d) {
// java return (double)0.0F <= d && d < (double)2958466.0F;
// java }
pub fn isValidCalendar(v:f64)->bool {
0.0<=v&&v<2958466.0
}
fn weekday(v:f64,t:i32,_using1904:bool)->i32{let v=v as i32;match t{1|17=>v.wrapping_add(6)%7+1,3=>v.wrapping_add(5)%7,12=>v.wrapping_add(4)%7+1,13=>v.wrapping_add(3)%7+1,14=>v.wrapping_add(2)%7+1,15=>v.wrapping_add(1)%7+1,16=>v%7+1,_=>v.wrapping_add(5)%7+1}}}
impl MixOpFunction for CalendarFunction{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{true}fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.is_empty()||args.len()>2{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}catch((||{let mut t=1;if args.len()==2&&!args[1].as_any().is::<MissingArgEval>(){let v=single(&args[1],ec)?;if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){return Err(ErrorEval::NUM_ERROR.into());}t=int(&v)?;}if !(1..=3).contains(&t)&&!(11..=17).contains(&t){return Err(ErrorEval::NUM_ERROR.into());}let v=double(&single(&args[0],ec)?)?;Ok(Arc::new(NumberEval::NumberEval(Self::weekday(v,t,ec.isUsing1904())as f64))as ValueRef)})())}}
