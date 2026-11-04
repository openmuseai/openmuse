use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use kfc::cn::wps::kfc::numfmt::util::DateUtil::DateUtil;pub struct CalendarFieldFunction{dateFieldId:i32}impl CalendarFieldFunction{pub const YEAR:Self=Self{dateFieldId:1};pub const MONTH:Self=Self{dateFieldId:2};pub const DAY:Self=Self{dateFieldId:5};pub const HOUR:Self=Self{dateFieldId:11};pub const MINUTE:Self=Self{dateFieldId:12};pub const SECOND:Self=Self{dateFieldId:13};// java private int getCalField(double serialDate, boolean using1904) {
// java Date d = DateUtil.getJavaDate(serialDate, using1904);
// java Calendar calendar = (Calendar)this.localCalendar.get();
// java calendar.setTime(d);
// java int result = calendar.get(this._dateFieldId);
// java if (this._dateFieldId == 2) {
// java ++result;
// java }
// java
// java return result;
// java }
pub fn getCalField(&self,date:f64,using1904:bool)->i32 {
let d=DateUtil::getJavaDate(date,using1904);let mut c=crate::java_runtime::Calendar::getInstance();c.setTimeInMillis(d);let mut n=c.get(self.dateFieldId);if self.dateFieldId==2{n+=1;}n
}
}
impl MixOpFunction for CalendarFieldFunction{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{true}
// java public final IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 1) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java double val;
// java try {
// java IValueEval ve = OperandResolver.getSingleValue(args[0], ec);
// java val = OperandResolver.coerceValueToDouble(ve);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java if (!CalendarFunction.isValidCalendar(val)) {
// java return ErrorEval.NUM_ERROR;
// java } else if (this._dateFieldId == 11) {
// java int seconds = (int)Math.round((val - Math.floor(val)) * (double)24.0F * (double)60.0F * (double)60.0F);
// java int hour = seconds / 3600;
// java return new NumberEval((double)hour);
// java } else if (this._dateFieldId == 12) {
// java int seconds = (int)Math.round((val - Math.floor(val)) * (double)24.0F * (double)60.0F * (double)60.0F);
// java int min = seconds / 60 % 60;
// java return new NumberEval((double)min);
// java } else if (this._dateFieldId == 13) {
// java int seconds = (int)Math.round((val - Math.floor(val)) * (double)24.0F * (double)60.0F * (double)60.0F);
// java int sec = seconds % 60;
// java return new NumberEval((double)sec);
// java } else {
// java return new NumberEval((double)this.getCalField(val, ec.isUsing1904()));
// java }
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=1{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}catch((||{let v=double(&single(&args[0],ec)?)?;if !super::CalendarFunction::CalendarFunction::isValidCalendar(v){return Err(ErrorEval::NUM_ERROR.into());}let n=if matches!(self.dateFieldId,11|12|13){let seconds=crate::java_runtime::javaMathRound((v-v.floor())*24.0*60.0*60.0)as i32;match self.dateFieldId{11=>seconds/3600,12=>seconds/60%60,_=>seconds%60}}else{self.getCalField(v,ec.isUsing1904())};Ok(Arc::new(NumberEval::NumberEval(n as f64))as ValueRef)})())}}
