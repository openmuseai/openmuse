use crate::cn::wps::et::ss::formula::{eval::{ValueRef,IValueEval,BlankEval::BlankEval,BoolEval::BoolEval,NumberEval::NumberEval,ErrorEval::ErrorEval,StringEval::StringEval,MissingArgEval::MissingArgEval,OperandResolver::OperandResolver,IAreaEval::IAreaEval,advance::{ArrayEval::ArrayEval,UnionEval::UnionEval}},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext},functions::base::{MixOpFunction::MixOpFunction,checked::*},util::DoubleList::DoubleList};use std::sync::Arc;
use super::{IMatchPredicate::IMatchPredicate,CmpOp::CmpOp};use super::super::LookupUtils::LookupUtils;use super::super::super::builtin::RelationalOperationEval::RelationalOperationEval;use super::super::super::super::evalcache::CountifCache::MatchObject;
// rust: Java Double.compare canonicalizes NaN, compares signed long bits after numeric ordering.
pub fn doubleCompare(a:f64,b:f64)->i32{if a<b{-1}else if a>b{1}else{let a=if a.is_nan(){0x7ff8000000000000i64}else{a.to_bits()as i64};let b=if b.is_nan(){0x7ff8000000000000i64}else{b.to_bits()as i64};if a==b{0}else if a<b{-1}else{1}}}
pub fn lower(s:&[u16])->Vec<u16>{let mut out=vec![];for c in char::decode_utf16(s.iter().copied()){match c{Ok(c)=>for c in c.to_lowercase(){let mut b=[0;2];out.extend_from_slice(c.encode_utf16(&mut b));},Err(e)=>out.push(e.unpaired_surrogate())}}out}
pub struct ConditionMatchPredicate;impl ConditionMatchPredicate{
// java private static IValueEval evaluateCriteriaArg(IValueEval arg, EvaluationContext ec) {
// java try {
// java return OperandResolver.getSingleValue(arg, ec);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java }
pub fn evaluateCriteriaArg(v:&ValueRef,ec:&EvaluationContext)->EvalResult<ValueRef> {
Ok(match OperandResolver::getSingleValue(v,ec)?{Ok(v)=>v,Err(e)=>Arc::new(e)})
}
// java public static IMatchPredicate createCriteriaPredicate(IValueEval arg, EvaluationContext ec) {
// java IValueEval evaluatedCriteriaArg = evaluateCriteriaArg(arg, ec);
// java if (evaluatedCriteriaArg instanceof NumberEval) {
// java return new NumberMatcher(((NumberEval)evaluatedCriteriaArg).getNumberValue(), CmpOp.OP_NONE);
// java } else if (evaluatedCriteriaArg instanceof BoolEval) {
// java return new BooleanMatcher(((BoolEval)evaluatedCriteriaArg).getBooleanValue(), CmpOp.OP_NONE);
// java } else if (evaluatedCriteriaArg instanceof StringEval) {
// java return createGeneralMatchPredicate((StringEval)evaluatedCriteriaArg);
// java } else if (evaluatedCriteriaArg instanceof ErrorEval) {
// java return new ErrorMatcher(((ErrorEval)evaluatedCriteriaArg).getErrorCode(), CmpOp.OP_NONE);
// java } else if (evaluatedCriteriaArg != BlankEval.instance && !(evaluatedCriteriaArg instanceof MissingArgEval)) {
// java throw new RuntimeException("Unexpected type for criteria (" + evaluatedCriteriaArg.getClass().getName() + ")");
// java } else {
// java return new NumberMatcher((double)0.0F, CmpOp.OP_NONE);
// java }
// java }
pub fn createCriteriaPredicate(v:&ValueRef,ec:&EvaluationContext)->EvalResult<Arc<dyn IMatchPredicate>> {
let v=Self::evaluateCriteriaArg(v,ec)?;Ok(if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){Arc::new(NumberMatcher::new(n.getNumberValue(),CmpOp::OP_NONE))}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){Arc::new(BooleanMatcher::new(b.getBooleanValue(),CmpOp::OP_NONE))}else if let Some(s)=v.as_any().downcast_ref::<StringEval>(){return Self::createGeneralMatchPredicate(s);}else if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){Arc::new(ErrorMatcher::new(e.getErrorCode(),CmpOp::OP_NONE))}else if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){Arc::new(NumberMatcher::new(0.0,CmpOp::OP_NONE))}else{return Err(EvalFailure::Runtime("Unexpected criteria type"));})
}
// java private static IMatchPredicate createGeneralMatchPredicate(StringEval stringEval) {
// java String value = stringEval.getStringValue();
// java CmpOp operator = CmpOp.getOperator(value);
// java value = value.substring(operator.getLength());
// java Boolean booleanVal = BoolEval.parseBoolean(value);
// java if (booleanVal != null) {
// java return new BooleanMatcher(booleanVal, operator);
// java } else {
// java Double doubleVal = OperandResolver.parseDouble(value);
// java if (doubleVal != null) {
// java return new NumberMatcher(doubleVal, operator);
// java } else {
// java ErrorEval ee = ErrorEval.valueOf(value);
// java return (IMatchPredicate)(ee != null ? new ErrorMatcher(ee.getErrorCode(), operator) : new StringMatcher(value, operator));
// java }
// java }
// java }
pub fn createGeneralMatchPredicate(v:&StringEval)->EvalResult<Arc<dyn IMatchPredicate>> {
let value=v.getStringValue();let op=CmpOp::getOperator(value);let value=&value[op.getLength()..];let text=String::from_utf16(value).ok();Ok(if let Some(b)=text.as_ref().and_then(|s|BoolEval::parseBoolean(s)){Arc::new(BooleanMatcher::new(b,op))}else if let Some(n)=OperandResolver::parseDouble(value)?{Arc::new(NumberMatcher::new(n,op))}else if let Some(e)=text.as_ref().and_then(|s|ErrorEval::valueOf__string(s)){Arc::new(ErrorMatcher::new(e.getErrorCode(),op))}else{Arc::new(StringMatcher::new(value.to_vec(),op)?)})
}
// java public static Object toSimpleMatchObject(IMatchPredicate mp) {
// java if (!(mp instanceof MatcherBase)) {
// java return null;
// java } else {
// java MatcherBase mb = (MatcherBase)mp;
// java if (mb.getCode() != 0) {
// java return null;
// java } else if (mb instanceof NumberMatcher) {
// java return ((NumberMatcher)mb)._value;
// java } else if (mb instanceof BooleanMatcher) {
// java return ((BooleanMatcher)mb)._value != 0;
// java } else if (mb instanceof StringMatcher) {
// java StringMatcher sm = (StringMatcher)mb;
// java return sm._pattern != null ? null : sm._value.toLowerCase();
// java } else {
// java return mb instanceof ErrorMatcher ? (byte)((ErrorMatcher)mb)._value : null;
// java }
// java }
// java }
pub fn toSimpleMatchObject(p:&dyn IMatchPredicate)->Option<MatchObject> {
if let Some(n)=p.as_any().downcast_ref::<NumberMatcher>(){(n.operator.getCode()==0).then(||MatchObject::Number(if n.value.is_nan(){0x7ff8000000000000}else{n.value.to_bits()}))}else if let Some(b)=p.as_any().downcast_ref::<BooleanMatcher>(){(b.operator.getCode()==0).then(||MatchObject::Bool(b.value!=0))}else if let Some(s)=p.as_any().downcast_ref::<StringMatcher>(){(s.operator.getCode()==0&&s.pattern.is_none()).then(||MatchObject::Text(lower(&s.value)))}else if let Some(e)=p.as_any().downcast_ref::<ErrorMatcher>(){(e.operator.getCode()==0).then(||MatchObject::Error(e.value as i8))}else{None}
}
}
pub struct NumberMatcher{pub value:f64,pub operator:CmpOp}impl NumberMatcher{pub fn new(value:f64,operator:CmpOp)->Self{Self{value:value,operator}}pub fn getCode(&self)->i32{self.operator.getCode()}pub fn getValue(&self)->f64{self.value}}
impl IMatchPredicate for NumberMatcher{fn as_any(&self)->&dyn std::any::Any{self}// java public boolean matches(IValueEval x, boolean directlyArg) {
// java if (x instanceof StringEval) {
// java switch (this.getCode()) {
// java case 0:
// java case 1:
// java StringEval se = (StringEval)x;
// java Double val = OperandResolver.parseDouble(se.getStringValue());
// java if (val == null) {
// java return false;
// java }
// java
// java return this._value == val;
// java case 2:
// java return true;
// java default:
// java return false;
// java }
// java } else if (x instanceof NumberEval) {
// java NumberEval ne = (NumberEval)x;
// java double testValue = ne.getNumberValue();
// java return this.evaluate(Double.compare(testValue, this._value));
// java } else if (x != BlankEval.instance && !(x instanceof MissingArgEval)) {
// java return this.getCode() == 2;
// java } else {
// java return this.getCode() == 2;
// java }
// java }
fn matches(&self,v:&ValueRef,_direct:bool)->EvalResult<bool>{if let Some(s)=v.as_any().downcast_ref::<StringEval>(){match self.getCode(){0|1=>Ok(OperandResolver::parseDouble(s.getStringValue())?.is_some_and(|n|self.value==n)),2=>Ok(true),_=>Ok(false)}}else if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){Ok(self.operator.evaluate(doubleCompare(n.getNumberValue(),self.value)))}else{Ok(self.getCode()==2)}}}
pub struct BooleanMatcher{pub value:i32,pub operator:CmpOp}impl BooleanMatcher{pub fn new(value:bool,operator:CmpOp)->Self{Self{value:value as i32,operator}}pub fn getCode(&self)->i32{self.operator.getCode()}pub fn getValue(&self)->i32{self.value}}
impl IMatchPredicate for BooleanMatcher{fn as_any(&self)->&dyn std::any::Any{self}// java public boolean matches(IValueEval x, boolean directlyArg) {
// java if (x instanceof BoolEval) {
// java BoolEval be = (BoolEval)x;
// java int testValue = boolToInt(be.getBooleanValue());
// java return this.evaluate(testValue - this._value);
// java } else {
// java return this.getCode() == 2;
// java }
// java }
fn matches(&self,v:&ValueRef,_direct:bool)->EvalResult<bool>{Ok(if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){self.operator.evaluate(b.getBooleanValue()as i32-self.value)}else{self.getCode()==2})}}
pub struct ErrorMatcher{pub value:i32,pub operator:CmpOp}impl ErrorMatcher{pub fn new(value:i32,operator:CmpOp)->Self{Self{value:value,operator}}pub fn getCode(&self)->i32{self.operator.getCode()}pub fn getValue(&self)->i32{self.value}}
impl IMatchPredicate for ErrorMatcher{fn as_any(&self)->&dyn std::any::Any{self}// java public boolean matches(IValueEval x, boolean directlyArg) {
// java if (x instanceof ErrorEval) {
// java int testValue = ((ErrorEval)x).getErrorCode();
// java return this.evaluate(testValue - this._value);
// java } else {
// java return this.getCode() == 2;
// java }
// java }
fn matches(&self,v:&ValueRef,_direct:bool)->EvalResult<bool>{Ok(if let Some(e)=v.as_any().downcast_ref::<ErrorEval>(){self.operator.evaluate(e.getErrorCode()-self.value)}else{self.getCode()==2})}}
pub struct StringMatcher{value:Vec<u16>,operator:CmpOp,pattern:Option<regex::Regex>}impl StringMatcher{// java public StringMatcher(String value, CmpOp operator) {
// java super(operator);
// java this._value = value;
// java switch (operator.getCode()) {
// java case 0:
// java case 1:
// java case 2:
// java this._pattern = LookupUtils.wildCardToPattern(value, true);
// java break;
// java default:
// java this._pattern = null;
// java }
// java
// java if (this._pattern != null) {
// java this._matcher = this._pattern.matcher("");
// java }
// java
// java }
pub fn new(value:Vec<u16>,operator:CmpOp)->EvalResult<Self>{let pattern=if matches!(operator.getCode(),0|1|2){LookupUtils::wildCardToPattern(&value,true)?}else{None};Ok(Self{value,operator,pattern})}}
impl IMatchPredicate for StringMatcher{fn as_any(&self)->&dyn std::any::Any{self}// java public boolean matches(IValueEval x, boolean directlyArg) {
// java if (x != BlankEval.instance && !(x instanceof MissingArgEval)) {
// java if (!(x instanceof StringEval)) {
// java return this.getCode() == 2;
// java } else {
// java String testedValue = ((StringEval)x).getStringValue();
// java if (testedValue.length() < 1 && this._value.length() < 1) {
// java switch (this.getCode()) {
// java case 0:
// java return true;
// java case 1:
// java return false;
// java case 2:
// java return true;
// java default:
// java return false;
// java }
// java } else if (this._pattern != null) {
// java this._matcher.reset(testedValue);
// java return this.evaluate(this._matcher.matches());
// java } else {
// java return this.evaluate(testedValue.compareToIgnoreCase(this._value));
// java }
// java }
// java } else {
// java switch (this.getCode()) {
// java case 0:
// java case 1:
// java return this._value.length() == 0;
// java case 2:
// java return this._value.length() != 0;
// java default:
// java return false;
// java }
// java }
// java }
fn matches(&self,v:&ValueRef,_direct:bool)->EvalResult<bool>{let op=self.operator.getCode();if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){return Ok(match op{0|1=>self.value.is_empty(),2=>!self.value.is_empty(),_=>false});}let Some(s)=v.as_any().downcast_ref::<StringEval>()else{return Ok(op==2);};let test=s.getStringValue();if test.is_empty()&&self.value.is_empty(){return Ok(matches!(op,0|2));}if let Some(p)=&self.pattern{let text=String::from_utf16(test).map_err(|_|EvalFailure::Unported("UTF16 regex unpaired surrogate"))?;let yes=p.find(&text).is_some_and(|m|m.start()==0&&m.end()==text.len());return Ok(self.operator.evaluateBool(yes));}Ok(self.operator.evaluate(RelationalOperationEval::compareIgnoreCase(test,&self.value)))}}
