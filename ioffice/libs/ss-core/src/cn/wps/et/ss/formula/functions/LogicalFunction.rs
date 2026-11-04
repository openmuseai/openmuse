use crate::cn::wps::et::ss::formula::{eval::{ValueRef,BlankEval::BlankEval,MissingArgEval::MissingArgEval,BoolEval::BoolEval,NumberEval::NumberEval,StringEval::StringEval,ErrorEval::ErrorEval,OperandResolver::OperandResolver,advance::ArrayEval::ArrayEval},evaluator::{EvalResult,EvaluationContext::EvaluationContext},functions::base::MixOpFunction::MixOpFunction};use std::sync::Arc;
// java public abstract class LogicalFunction extends ValueOpFunction
pub struct LogicalFunction{kind:u8}
impl LogicalFunction{
// java public static final IFunction ISLOGICAL= ...
pub const ISLOGICAL:Self=Self{kind:0};
// java public static final IFunction ISNONTEXT= ...
pub const ISNONTEXT:Self=Self{kind:1};
// java public static final IFunction ISNUMBER= ...
pub const ISNUMBER:Self=Self{kind:2};
// java public static final IFunction ISTEXT= ...
pub const ISTEXT:Self=Self{kind:3};
// java public static final IFunction ISBLANK= ...
pub const ISBLANK:Self=Self{kind:4};
// java public static final IFunction ISERROR= ...
pub const ISERROR:Self=Self{kind:5};
// java public static final IFunction ISERR= ...
pub const ISERR:Self=Self{kind:6};
// java public static final IFunction ISNA= ...
pub const ISNA:Self=Self{kind:7};
// java public static final IFunction ISREF= ...
pub const ISREF:Self=Self{kind:8};
}
impl MixOpFunction for LogicalFunction{fn valueArgRequired(&self,_i:usize,_a:&[ValueRef],_e:&EvaluationContext)->bool{self.kind!=8}fn shouldClip(&self)->bool{self.kind!=8}fn shouldMarkArray(&self)->bool{self.kind!=8}
// java public IValueEval evaluateOnce(byte funcRAV, IValueEval[] args, EvaluationContext ec) {
// java if (args.length != 1) {
// java return ErrorEval.VALUE_INVALID;
// java } else {
// java IValueEval ve;
// java try {
// java ve = OperandResolver.getSingleValue(args[0], ec);
// java } catch (EvaluationException e) {
// java ve = e.getErrorEval();
// java }
// java
// java return BoolEval.valueOf(this.evaluate(ve));
// java }
// java }
fn evaluateOnce(&self,_rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>{if args.len()!=1{return Ok(Arc::new(ErrorEval::VALUE_INVALID));}if self.kind==8{return Ok(Arc::new(BoolEval::valueOf(args[0].as_ref_eval().is_some()||args[0].as_area_eval().is_some())));}let v=match OperandResolver::getSingleValue(&args[0],ec)?{Ok(v)=>v,Err(e)=>Arc::new(e)};let yes=match self.kind{0=>v.as_any().is::<BoolEval>(),1=>!v.as_any().is::<StringEval>(),2=>v.as_any().is::<NumberEval>(),3=>v.as_any().is::<StringEval>(),4=>v.as_any().is::<BlankEval>()||v.as_any().is::<MissingArgEval>(),5=>v.as_any().is::<ErrorEval>(),6=>v.as_any().downcast_ref::<ErrorEval>().is_some_and(|e|e.getErrorCode()!=42),7=>v.as_any().downcast_ref::<ErrorEval>().is_some_and(|e|e.getErrorCode()==42),_=>unreachable!()};Ok(Arc::new(BoolEval::valueOf(yes)))}}
