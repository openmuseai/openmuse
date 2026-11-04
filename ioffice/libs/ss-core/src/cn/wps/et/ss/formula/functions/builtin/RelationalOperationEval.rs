use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext}};use std::sync::Arc;
use crate::org::apache::poi::ss::util::NumberComparer::NumberComparer;
pub struct RelationalOperationEval{op:u8}
impl RelationalOperationEval{
// java public static final IFunction EqualEval = new RelationalOperationEval() ...
pub const EqualEval:Self=Self{op:11};
// java public static final IFunction GreaterEqualEval = new RelationalOperationEval() ...
pub const GreaterEqualEval:Self=Self{op:12};
// java public static final IFunction GreaterThanEval = new RelationalOperationEval() ...
pub const GreaterThanEval:Self=Self{op:13};
// java public static final IFunction LessEqualEval = new RelationalOperationEval() ...
pub const LessEqualEval:Self=Self{op:10};
// java public static final IFunction LessThanEval = new RelationalOperationEval() ...
pub const LessThanEval:Self=Self{op:9};
// java public static final IFunction NotEqualEval = new RelationalOperationEval() ...
pub const NotEqualEval:Self=Self{op:14};
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 2) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java IValueEval vA;
// java IValueEval vB;
// java try {
// java vA = OperandResolver.getSingleValue(args[0], ec);
// java vB = OperandResolver.getSingleValue(args[1], ec);
// java } catch (EvaluationException e) {
// java return e.getErrorEval();
// java }
// java
// java int cmpResult = doCompare(vA, vB);
// java boolean result = this.convertComparisonResult(cmpResult);
// java return BoolEval.valueOf(result);
// java }
// java }
pub fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef> {
if args.len()!=2{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}let a=match OperandResolver::getSingleValue(&args[0],ec)?{Ok(v)=>v,Err(e)=>return Ok(Arc::new(e))};let b=match OperandResolver::getSingleValue(&args[1],ec)?{Ok(v)=>v,Err(e)=>return Ok(Arc::new(e))};let cmp=Self::doCompare(&a,&b)?;let yes=match self.op{11=>cmp==0,12=>cmp>=0,13=>cmp>0,10=>cmp<=0,9=>cmp<0,_=>cmp!=0};Ok(Arc::new(BoolEval::valueOf(yes)))
}
// java private static int compareBlank(IValueEval v) {
// java if (v != BlankEval.instance && !(v instanceof MissingArgEval)) {
// java if (v instanceof BoolEval) {
// java BoolEval boolEval = (BoolEval)v;
// java return boolEval.getBooleanValue() ? -1 : 0;
// java } else if (v instanceof NumberEval) {
// java NumberEval ne = (NumberEval)v;
// java return NumberComparer.compare((double)0.0F, ne.getNumberValue());
// java } else if (v instanceof StringEval) {
// java StringEval se = (StringEval)v;
// java return se.getStringValue().length() < 1 ? 0 : -1;
// java } else {
// java throw new IllegalArgumentException("bad value class (" + v.getClass().getName() + ")");
// java }
// java } else {
// java return 0;
// java }
// java }
pub fn compareBlank(v:&ValueRef)->EvalResult<i32> {
if v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(){Ok(0)}else if let Some(b)=v.as_any().downcast_ref::<BoolEval>(){Ok(if b.getBooleanValue(){-1}else{0})}else if let Some(n)=v.as_any().downcast_ref::<NumberEval>(){Ok(NumberComparer::compare(0.0,n.getNumberValue()))}else if let Some(s)=v.as_any().downcast_ref::<StringEval>(){Ok(if s.getStringValue().is_empty(){0}else{-1})}else{Err(EvalFailure::Runtime("bad value class"))}
}
// java private static int doCompare(IValueEval va, IValueEval vb) {
// java if (va != BlankEval.instance && !(va instanceof MissingArgEval)) {
// java if (vb != BlankEval.instance && !(vb instanceof MissingArgEval)) {
// java if (va instanceof BoolEval) {
// java if (vb instanceof BoolEval) {
// java BoolEval bA = (BoolEval)va;
// java BoolEval bB = (BoolEval)vb;
// java if (bA.getBooleanValue() == bB.getBooleanValue()) {
// java return 0;
// java } else {
// java return bA.getBooleanValue() ? 1 : -1;
// java }
// java } else {
// java return 1;
// java }
// java } else if (vb instanceof BoolEval) {
// java return -1;
// java } else if (va instanceof StringEval) {
// java if (vb instanceof StringEval) {
// java StringEval sA = (StringEval)va;
// java StringEval sB = (StringEval)vb;
// java return sA.getStringValue().compareToIgnoreCase(sB.getStringValue());
// java } else {
// java return 1;
// java }
// java } else if (vb instanceof StringEval) {
// java return -1;
// java } else if (va instanceof NumberEval && vb instanceof NumberEval) {
// java NumberEval nA = (NumberEval)va;
// java NumberEval nB = (NumberEval)vb;
// java return NumberComparer.compare(nA.getNumberValue(), nB.getNumberValue());
// java } else {
// java throw new IllegalArgumentException("Bad operand types (" + va.getClass().getName() + "), (" + vb.getClass().getName() + ")");
// java }
// java } else {
// java return -compareBlank(va);
// java }
// java } else {
// java return compareBlank(vb);
// java }
// java }
pub fn doCompare(a:&ValueRef,b:&ValueRef)->EvalResult<i32> {
if a.as_any().is::<BlankEval>()||a.as_any().is::<MissingArgEval>(){return Self::compareBlank(b);}if b.as_any().is::<BlankEval>()||b.as_any().is::<MissingArgEval>(){return Self::compareBlank(a).map(|v|-v);}if let Some(a)=a.as_any().downcast_ref::<BoolEval>(){return Ok(if let Some(b)=b.as_any().downcast_ref::<BoolEval>(){(a.getBooleanValue() as i32)-(b.getBooleanValue() as i32)}else{1});}if b.as_any().is::<BoolEval>(){return Ok(-1);}if let Some(a)=a.as_any().downcast_ref::<StringEval>(){return Ok(if let Some(b)=b.as_any().downcast_ref::<StringEval>(){Self::compareIgnoreCase(a.getStringValue(),b.getStringValue())}else{1});}if b.as_any().is::<StringEval>(){return Ok(-1);}if let(Some(a),Some(b))=(a.as_any().downcast_ref::<NumberEval>(),b.as_any().downcast_ref::<NumberEval>()){return Ok(NumberComparer::compare(a.getNumberValue(),b.getNumberValue()));}Err(EvalFailure::Runtime("Bad operand types"))
}
// rust: Java String.CaseInsensitiveComparator compares UTF-16 units with simple Character case conversion.
pub fn compareIgnoreCase(a:&[u16],b:&[u16])->i32{fn map(u:u16,upper:bool)->u16{let Some(c)=char::from_u32(u as u32)else{return u;};let chars=if upper{c.to_uppercase().collect::<Vec<_>>()}else{c.to_lowercase().collect::<Vec<_>>()};if chars.len()==1&&chars[0]as u32<=65535{chars[0]as u16}else if !upper&&u==0x130{0x69}else{u}}for(&a,&b)in a.iter().zip(b){if a!=b{let x=map(a,true);let y=map(b,true);if x!=y{let x=map(x,false);let y=map(y,false);if x!=y{return x as i32-y as i32;}}}}a.len()as i32-b.len()as i32}
}

impl super::super::base::MixOpFunction::MixOpFunction for RelationalOperationEval{fn valueArgRequired(&self,_i:usize,_args:&[ValueRef],_ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->bool{true}fn evaluateOnce(&self,rav:i8,args:&[ValueRef],ec:&crate::cn::wps::et::ss::formula::evaluator::EvaluationContext::EvaluationContext)->EvalResult<ValueRef>{RelationalOperationEval::evaluateOnce(self,rav,args,ec)}}
