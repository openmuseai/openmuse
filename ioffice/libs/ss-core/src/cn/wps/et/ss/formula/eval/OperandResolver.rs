// java package cn.wps.et.ss.formula.eval;
use super::{
    BlankEval::BlankEval, BoolEval::BoolEval, ErrorEval::ErrorEval, MissingArgEval::MissingArgEval,
    NumberEval::NumberEval, StringEval::StringEval, ValueRef,
};
use crate::cn::wps::et::ss::constant::ConstRecognisor::ConstRecognisor;
use crate::cn::wps::et::ss::formula::evaluator::{EvalFailure, EvalResult};
use std::sync::{LazyLock, Mutex};
// java public final class OperandResolver
pub struct OperandResolver;
// java public static final ConstRecognisor constRecognisor = new ConstRecognisor();
pub static constRecognisor: LazyLock<Mutex<ConstRecognisor>> = LazyLock::new(|| {
    let mut r = ConstRecognisor::ConstRecognisor();
    // java static { constRecognisor.preferStateMachine(true); constRecognisor.setForceText(false); }
    r.preferStateMachine(true);
    r.setForceText(false);
    Mutex::new(r)
});
impl OperandResolver {
    // java getSingleValue(arg,ec): dereference once, then throw EvaluationException on ErrorEval.
    pub fn getSingleValue(
        arg: &ValueRef,
        _ec: &crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext,
    ) -> EvalResult<Result<ValueRef, &'static ErrorEval>> {
        let result = if let Some(reference) = arg.as_ref_eval() {
            reference
                .getInnerValueEval()?
                .ok_or(EvalFailure::Runtime("NullPointerException value"))?
        } else {
            if let Some(area)=arg.as_area_eval(){let b=area.base();let r=_ec.getRowIndex();let c=_ec.getColumnIndex();if b.isColumn(){if b.isRow(){area.getRelativeValue(0,0)?}else if !b.containsRow(r){return Ok(Err(ErrorEval::VALUE_INVALID));}else{area.getAbsoluteValue(r,b.getFirstColumn())?}}else if !b.isRow(){if b.contains(r,c){area.getAbsoluteValue(r,c)?}else{return Ok(Err(ErrorEval::VALUE_INVALID));}}else if !b.containsColumn(c){return Ok(Err(ErrorEval::VALUE_INVALID));}else{area.getAbsoluteValue(b.getFirstRow(),c)?}}else if let Some(array)=arg.as_any().downcast_ref::<super::advance::ArrayEval::ArrayEval>(){let v=array.getFirstValue();if let Some(r)=v.as_ref_eval(){r.getInnerValueEval()?.ok_or(EvalFailure::Runtime("NullPointerException"))?}else{v}}else{arg.clone()}
        };
        if let Some(e) = result.as_any().downcast_ref::<ErrorEval>() {
            Ok(Err(ErrorEval::valueOf(e.getErrorCode())))
        } else {
            Ok(Ok(result))
        }
    }
    // java coerceValueToDouble's scalar branches; EvaluationException is the inner Result.
    pub fn coerceValueToDouble(v: &ValueRef) -> EvalResult<Result<f64, &'static ErrorEval>> {
        if v.as_any().is::<BlankEval>() || v.as_any().is::<MissingArgEval>() {
            return Ok(Ok(0.0));
        }
        // java ev instanceof INumericValueEval -> ((INumericValueEval)ev).getNumberValue();
        if let Some(n) = v.as_numeric_eval() {
            return Ok(Ok(n.getNumberValue()));
        }
        if let Some(v)=v.as_any().downcast_ref::<StringEval>(){return Ok(Self::parseDouble(v.getStringValue())?.ok_or(ErrorEval::VALUE_INVALID));}
        Err(EvalFailure::Runtime("Unexpected arg eval type"))
    }
// java public static Boolean coerceValueToBoolean(IValueEval ve, boolean stringsAreBlanks) throws EvaluationException {
// java if (ve != null && ve != BlankEval.instance && !(ve instanceof MissingArgEval)) {
// java if (ve == MissingArgEval.instance) {
// java throw EvaluationException.NA;
// java } else if (ve instanceof BoolEval) {
// java return ((BoolEval)ve).getBooleanValue();
// java } else if (ve instanceof StringEval) {
// java if (stringsAreBlanks) {
// java return null;
// java } else {
// java String str = ((StringEval)ve).getStringValue();
// java if (str.equalsIgnoreCase("true")) {
// java return Boolean.TRUE;
// java } else if (str.equalsIgnoreCase("false")) {
// java return Boolean.FALSE;
// java } else {
// java throw EvaluationException.VALUE_INVALID;
// java }
// java }
// java } else if (ve instanceof INumericValueEval) {
// java INumericValueEval ne = (INumericValueEval)ve;
// java double d = ne.getNumberValue();
// java if (Double.isNaN(d)) {
// java throw EvaluationException.VALUE_INVALID;
// java } else {
// java return d != (double)0.0F;
// java }
// java } else if (ve instanceof ErrorEval) {
// java throw EvaluationException.valueOf((ErrorEval)ve);
// java } else {
// java throw new RuntimeException("Unexpected eval (" + ve.getClass().getName() + ")");
// java }
// java } else {
// java return null;
// java }
// java }
pub fn coerceValueToBoolean(v:&ValueRef,stringsAreBlanks:bool)->EvalResult<Result<Option<bool>,&'static ErrorEval>> {
if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){return Ok(Ok(None));}if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){return Ok(Ok(Some(b.getBooleanValue())));}if let Some(s)=v.as_any().downcast_ref::<StringEval>(){if stringsAreBlanks{return Ok(Ok(None));}let s=String::from_utf16_lossy(s.getStringValue());return Ok(if s.eq_ignore_ascii_case("true"){Ok(Some(true))}else if s.eq_ignore_ascii_case("false"){Ok(Some(false))}else{Err(ErrorEval::VALUE_INVALID)});}if let Some(n)=v.as_numeric_eval(){let d=n.getNumberValue();return Ok(if d.is_nan(){Err(ErrorEval::VALUE_INVALID)}else{Ok(Some(d!=0.0))});}if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){return Ok(Err(ErrorEval::valueOf(e.getErrorCode())));}Err(EvalFailure::Runtime("Unexpected eval"))
}
// java public static String coerceValueToString(IValueEval ve) throws EvaluationException {
// java if (ve instanceof IStringValueEval) {
// java IStringValueEval sve = (IStringValueEval)ve;
// java return sve.getStringValue();
// java } else if (ve == BlankEval.instance) {
// java return "";
// java } else if (ve == MissingArgEval.instance) {
// java throw EvaluationException.NA;
// java } else if (ve instanceof UnionEval) {
// java throw EvaluationException.VALUE_INVALID;
// java } else {
// java throw new IllegalArgumentException("Unexpected eval class (" + ve.getClass().getName() + ")");
// java }
// java }
pub fn coerceValueToString(v:&ValueRef)->EvalResult<Result<Vec<u16>,&'static ErrorEval>> {
if let Some(s)=v.as_any().downcast_ref::<StringEval>(){Ok(Ok(s.getStringValue().to_vec()))}else if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){Ok(Ok(n.getStringValue().encode_utf16().collect()))}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){Ok(Ok(b.getStringValue().encode_utf16().collect()))}else if v.as_any().is::<BlankEval>(){Ok(Ok(vec![]))}else if v.as_any().is::<MissingArgEval>(){Ok(Err(ErrorEval::NA))}else if v.as_any().is::<super::advance::UnionEval::UnionEval>(){Ok(Err(ErrorEval::VALUE_INVALID))}else{Err(EvalFailure::Runtime("Unexpected eval class"))}
}
// java public static int coerceValueToInt(IValueEval ev) throws EvaluationException {
// java if (ev == BlankEval.instance) {
// java return 0;
// java } else {
// java double d = coerceValueToDouble(ev);
// java return (int)Math.floor(d);
// java }
// java }
pub fn coerceValueToInt(v:&ValueRef)->EvalResult<Result<i32,&'static ErrorEval>> {
if v.as_any().is::<BlankEval>(){return Ok(Ok(0));}Self::coerceValueToDouble(v).map(|r|r.map(|v|v.floor() as i32))
}
// java public static Double parseDouble(String pText) {
// java RecogniseResult rs = constRecognisor.recognise(pText);
// java return rs != null && !rs.hasErrorFlag() && rs.getValue() instanceof Double ? rs.getDblValue() : null;
// java }
pub fn parseDouble(text:&[u16])->EvalResult<Option<f64>> {
let r=constRecognisor.lock().unwrap().recognise(Some(text))?;Ok(r.and_then(|r|if !r.hasErrorFlag(){match r.getValue(){Some(crate::cn::wps::et::ss::statemachine::RecogniseResultRl::Value::Number(v))=>Some(v),_=>None}}else{None}))
}
}
