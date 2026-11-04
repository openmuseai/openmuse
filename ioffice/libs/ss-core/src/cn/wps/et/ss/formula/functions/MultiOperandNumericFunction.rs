use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::util::numbercollector::{IBlankCollectingPolicy::IBlankCollectingPolicy as BP,IBooleanCollectingPolicy::IBooleanCollectingPolicy as BLP,IStringCollectingPolicy::IStringCollectingPolicy as SP,IErrorCollectingPolicy::IErrorCollectingPolicy as EP};use super::super::evalcache::AreaNumberCollector::AreaNumberCollector;
// java public abstract class MultiOperandNumericFunction extends ArrayOpFunction
pub struct MultiOperandNumericFunction{isReferenceBoolCounted:bool,isBlankCounted:bool,isMissCounted:bool,isReferenceStringCounted:bool}
impl MultiOperandNumericFunction{
// java protected MultiOperandNumericFunction(boolean isReferenceBoolCounted, boolean isBlankCounted, boolean isReferenceStringCounted, boolean isMissCounted) {
// java this._isReferenceBoolCounted = isReferenceBoolCounted;
// java this._isBlankCounted = isBlankCounted;
// java this._isReferenceStringCounted = isReferenceStringCounted;
// java this._isMissCounted = isMissCounted;
// java }
pub fn new(b:bool,blank:bool,s:bool,miss:bool)->Self {
Self{isReferenceBoolCounted:b,isBlankCounted:blank,isReferenceStringCounted:s,isMissCounted:miss}
}
// java protected final double[] getNumberArray(IValueEval[] operands) throws EvaluationException {
// java if (operands.length > this.getMaxNumOperands()) {
// java throw EvaluationException.VALUE_INVALID;
// java } else {
// java DoubleList retval = new DoubleList();
// java int i = 0;
// java
// java for(int len = operands.length; i < len; ++i) {
// java this.collectValues(operands[i], false, retval);
// java }
// java
// java return retval.toArray();
// java }
// java }
pub fn getNumberArray(&self,operands:&[ValueRef])->FunctionResult<Vec<f64>> {
if operands.len()>255{return Err(ErrorEval::VALUE_INVALID.into());}let mut out=DoubleList::new();for v in operands{self.collectValues(v,false,&mut out)?;}Ok(out.toArray())
}
// java private void collectValues(IValueEval operand, boolean isViaReference, DoubleList temp) throws EvaluationException {
// java if (operand == null) {
// java throw new IllegalArgumentException("ve must not be null");
// java } else if (operand instanceof UnionEval) {
// java UnionEval ue = (UnionEval)operand;
// java this.collectValues(ue.first, true, temp);
// java this.collectValues(ue.second, true, temp);
// java } else if (operand instanceof ArrayEval) {
// java ArrayEval ae = (ArrayEval)operand;
// java Iterator<IValueEval> iter = ae.iterator();
// java
// java int touched;
// java for(touched = 0; iter.hasNext(); ++touched) {
// java IValueEval e = (IValueEval)iter.next();
// java this.collectValues(e, true, temp);
// java }
// java
// java if (this._isBlankCounted) {
// java int i = 0;
// java
// java for(int count = ae.getRowCount() * ae.getColCount() - touched; i < count; ++i) {
// java temp.add((double)0.0F);
// java }
// java }
// java
// java } else if (operand instanceof IAreaEval) {
// java IAreaEval ae = (IAreaEval)operand;
// java AreaNumberCollector.collectNumbers(ae, this._isBlankCounted ? IBlankCollectingPolicy.ZERO : IBlankCollectingPolicy.IGNORE, this._isReferenceBoolCounted ? IBooleanCollectingPolicy.CONVERT : IBooleanCollectingPolicy.IGNORE, this._isReferenceStringCounted ? IStringCollectingPolicy.ZERO : IStringCollectingPolicy.IGNORE, IErrorCollectingPolicy.THROW, temp);
// java } else if (operand instanceof IRefEval) {
// java IRefEval re = (IRefEval)operand;
// java this.collectValues(re.getInnerValueEval(), true, temp);
// java } else if (operand instanceof NumberEval) {
// java NumberEval ne = (NumberEval)operand;
// java temp.add(ne.getNumberValue());
// java } else if (operand instanceof ErrorEval) {
// java throw EvaluationException.valueOf((ErrorEval)operand);
// java } else if (operand instanceof StringEval) {
// java if (isViaReference) {
// java if (this._isReferenceStringCounted) {
// java temp.add((double)0.0F);
// java }
// java
// java } else {
// java String s = ((StringEval)operand).getStringValue();
// java Double d = OperandResolver.parseDouble(s);
// java if (d == null) {
// java throw EvaluationException.VALUE_INVALID;
// java } else {
// java temp.add(d);
// java }
// java }
// java } else if (!(operand instanceof BoolEval)) {
// java if (operand == BlankEval.instance) {
// java if (this._isBlankCounted) {
// java temp.add((double)0.0F);
// java }
// java
// java } else if (operand == MissingArgEval.instance) {
// java if (this._isMissCounted) {
// java temp.add((double)0.0F);
// java }
// java
// java } else {
// java throw new RuntimeException("Invalid ValueEval type passed for conversion: (" + operand.getClass() + ")");
// java }
// java } else {
// java if (!isViaReference || this._isReferenceBoolCounted) {
// java BoolEval boolEval = (BoolEval)operand;
// java temp.add(boolEval.getNumberValue());
// java }
// java
// java }
// java }
pub fn collectValues(&self,v:&ValueRef,via:bool,out:&mut DoubleList)->FunctionResult<()> {
if let Some(u)=v.as_any().downcast_ref::<UnionEval>(){self.collectValues(&u.first,true,out)?;self.collectValues(&u.second,true,out)?;}else if let Some(a)=v.as_any().downcast_ref::<ArrayEval>(){let mut touched=0;for r in 0..a.getRowCount(){for c in 0..a.getColCount(){self.collectValues(&a.getValue(r,c),true,out)?;touched+=1;}}if self.isBlankCounted{for _ in 0..a.getRowCount()*a.getColCount()-touched{out.add(0.0);}}}else if let Some(a)=v.as_area_eval(){AreaNumberCollector::collectNumbers(a,if self.isBlankCounted{BP::ZERO}else{BP::IGNORE},if self.isReferenceBoolCounted{BLP::CONVERT}else{BLP::IGNORE},if self.isReferenceStringCounted{SP::ZERO}else{SP::IGNORE},EP::THROW,out)?;}else if let Some(r)=v.as_ref_eval(){self.collectValues(&r.getInnerValueEval()?.ok_or(EvalFailure::Runtime("null ref"))?,true,out)?;}else if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){out.add(n.getNumberValue());}else if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){return Err(ErrorEval::valueOf(e.getErrorCode()).into());}else if let Some(s)=v.as_any().downcast_ref::<StringEval>(){if via{if self.isReferenceStringCounted{out.add(0.0);}}else{out.add(OperandResolver::parseDouble(&s.getStringValue())?.ok_or(ErrorEval::VALUE_INVALID)?);}}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){if !via||self.isReferenceBoolCounted{out.add(if b.getBooleanValue(){1.0}else{0.0});}}else if v.as_any().is::<BlankEval>(){if self.isBlankCounted{out.add(0.0);}}else if v.as_any().is::<MissingArgEval>(){if self.isMissCounted{out.add(0.0);}}else{return Err(EvalFailure::Runtime("Invalid ValueEval type").into());}Ok(())
}
}
