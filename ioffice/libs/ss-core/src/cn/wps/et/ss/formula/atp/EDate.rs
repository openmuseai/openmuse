use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::Accrint::Accrint;use super::super::functions::{NumericFunction::NumericFunction,calendar::CalendarFunction::CalendarFunction};use kfc::cn::wps::kfc::numfmt::util::DateUtil::DateUtil;pub struct EDate;impl EDate{
// java static double EDATE(double start_date, double months, boolean using1904, boolean bMonthEnd) throws EvaluationException {
// java TimeCalc.TimeStruct tm = DateUtil.getGregDate(start_date, using1904);
// java int nY = tm.mYear;
// java int nMo = tm.mMonth;
// java int nD = tm.mMonthDay;
// java int nTotalMonth = nY * 12 + nMo - 1;
// java nTotalMonth += (int)months;
// java nY = nTotalMonth / 12;
// java nMo = nTotalMonth % 12 + 1;
// java if (bMonthEnd) {
// java nD = Accrint.countMonthDays(nY, nMo);
// java } else {
// java nD = Math.min(Accrint.countMonthDays(nY, nMo), nD);
// java }
// java
// java double etdRes = DateUtil.getExcelDate(nY, nMo, nD, using1904);
// java if (etdRes < (double)0.0F) {
// java throw EvaluationException.NUM_ERROR;
// java } else {
// java return etdRes;
// java }
// java }
pub fn EDATE(start:f64,months:f64,using1904:bool,monthEnd:bool)->FunctionResult<f64> {
let tm=DateUtil::getGregDate(start,using1904);let mut y=tm.mYear;let mut m=tm.mMonth;let mut d=tm.mMonthDay;let total=(y*12+m-1).wrapping_add(months as i32);y=total/12;m=total%12+1;d=if monthEnd{Accrint::countMonthDays(y,m)}else{Accrint::countMonthDays(y,m).min(d)};let rs=DateUtil::getExcelDate_from_ymd(y,m,d,using1904);if rs<0.0{return Err(ErrorEval::NUM_ERROR.into());}Ok(rs)
}
}
impl MixOpFunction for EDate{fn valueArgRequired(&self,_:usize,_:&[ValueRef],_:&EvaluationContext)->bool{true}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java try {
// java double start_date = NumericFunction.singleOperandEvaluate(args[0], ec);
// java double months = NumericFunction.singleOperandEvaluate(args[1], ec);
// java if (!CalendarFunction.isValidCalendar(start_date)) {
// java throw EvaluationException.NUM_ERROR;
// java } else {
// java start_date = Math.floor(start_date);
// java if (Math.abs(months) > (double)119988.0F) {
// java throw EvaluationException.NUM_ERROR;
// java } else {
// java double rs = EDATE(start_date, months, ec.isUsing1904(), false);
// java NumericFunction.checkValue(rs);
// java return new NumberEval(rs);
// java }
// java }
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java }
// java }
fn evaluateOnce(&self,_:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=2{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}catch((||{let mut start=NumericFunction::singleOperandEvaluate(&args[0],ec)?;let months=NumericFunction::singleOperandEvaluate(&args[1],ec)?;if !CalendarFunction::isValidCalendar(start){return Err(ErrorEval::NUM_ERROR.into());}start=start.floor();if months.abs()>119988.0{return Err(ErrorEval::NUM_ERROR.into());}let rs=Self::EDATE(start,months,ec.isUsing1904(),false)?;NumericFunction::checkValue(rs)?;Ok(Arc::new(NumberEval::NumberEval(rs))as ValueRef)})())}}
