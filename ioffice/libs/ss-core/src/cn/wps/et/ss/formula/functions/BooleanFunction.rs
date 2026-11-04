use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver,advance::ArrayEval::ArrayEval},evaluator::{EvalResult,EvaluationContext::EvaluationContext},functions::base::MixOpFunction::MixOpFunction};use std::sync::Arc;
pub struct BooleanFunction{kind:u8}
impl BooleanFunction{
// java public static final IFunction AND= ...
pub const AND:Self=Self{kind:0};
// java public static final IFunction OR= ...
pub const OR:Self=Self{kind:1};
// java public static final IFunction FALSE= ...
pub const FALSE:Self=Self{kind:2};
// java public static final IFunction TRUE= ...
pub const TRUE:Self=Self{kind:3};
// java public static final IFunction NOT= ...
pub const NOT:Self=Self{kind:4};
}
impl MixOpFunction for BooleanFunction{fn valueArgRequired(&self,_i:usize,_a:&[ValueRef],_e:&EvaluationContext)->bool{self.kind>=2}fn shouldClip(&self)->bool{self.kind>=2}
// java private boolean calculate(IValueEval[] args) throws EvaluationException {
// java boolean result = this.getInitialResultValue();
// java boolean atleastOneNonBlank = false;
// java int i = 0;
// java
// java for(int iSize = args.length; i < iSize; ++i) {
// java IValueEval arg = args[i];
// java if (arg instanceof IAreaEval) {
// java IValueIterator iter = ((IAreaEval)arg).valueIterator();
// java
// java while(iter.hasNext()) {
// java iter.next();
// java IValueEval ve = iter.getValue();
// java Boolean tmpVe = OperandResolver.coerceValueToBoolean(ve, true);
// java if (tmpVe != null) {
// java result = this.partialEvaluate(result, tmpVe);
// java atleastOneNonBlank = true;
// java }
// java }
// java } else if (arg instanceof ArrayEval) {
// java for(IValueEval ve : (ArrayEval)arg) {
// java Boolean tmpVe = OperandResolver.coerceValueToBoolean(ve, true);
// java if (tmpVe != null) {
// java result = this.partialEvaluate(result, tmpVe);
// java atleastOneNonBlank = true;
// java }
// java }
// java } else if (arg == MissingArgEval.instance) {
// java Boolean tmpVe = false;
// java if (tmpVe != null) {
// java result = this.partialEvaluate(result, tmpVe);
// java atleastOneNonBlank = true;
// java }
// java } else {
// java Boolean tempVe;
// java if (arg instanceof IRefEval) {
// java IValueEval ve = ((IRefEval)arg).getInnerValueEval();
// java tempVe = OperandResolver.coerceValueToBoolean(ve, true);
// java } else {
// java tempVe = OperandResolver.coerceValueToBoolean(arg, false);
// java }
// java
// java if (tempVe != null) {
// java result = this.partialEvaluate(result, tempVe);
// java atleastOneNonBlank = true;
// java }
// java }
// java }
// java
// java if (!atleastOneNonBlank) {
// java throw EvaluationException.VALUE_INVALID;
// java } else {
// java return result;
// java }
// java }
fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if self.kind==2||self.kind==3{return Ok(if !args.is_empty(){Arc::new(ErrorEval::VALUE_INVALID)}else{Arc::new(BoolEval::valueOf(self.kind==3))});}if self.kind==4{if args.len()!=1{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let v=match OperandResolver::getSingleValue(&args[0],ec)?{Ok(v)=>v,Err(e)=>return Ok(Arc::new(e))};let yes=match OperandResolver::coerceValueToBoolean(&v,false)?{Ok(v)=>v.unwrap_or(false),Err(e)=>return Ok(Arc::new(e))};return Ok(Arc::new(BoolEval::valueOf(!yes)));}if args.is_empty(){return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let(mut result,mut one)=(self.kind==0,false);for a in args{let mut vs=vec![];if let Some(area)=a.as_area_eval(){let mut iter=area.valueIterator();while iter.hasNext(){iter.next();vs.push((iter.getValue()?,true));}}else if let Some(array)=a.as_any().downcast_ref::<ArrayEval>(){for r in 0..array.getRowCount(){for c in 0..array.getColCount(){vs.push((array.getValue(r,c),true));}}}else if a.as_any().is::<MissingArgEval>(){vs.push((Arc::new(BoolEval::FALSE)as ValueRef,false));}else if let Some(reference)=a.as_ref_eval(){vs.push((reference.getInnerValueEval()?.expect("null reference"),true));}else{vs.push((a.clone(),false));}for(v,strings_blank)in vs{match OperandResolver::coerceValueToBoolean(&v,strings_blank)?{Err(e)=>return Ok(Arc::new(e)),Ok(Some(b))=>{result=if self.kind==0{result&&b}else{result||b};one=true;},Ok(None)=>{}}}}Ok(if one{Arc::new(BoolEval::valueOf(result))}else{Arc::new(ErrorEval::VALUE_INVALID)})}}
