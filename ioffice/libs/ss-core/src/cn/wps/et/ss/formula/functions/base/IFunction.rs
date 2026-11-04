use crate::cn::wps::et::ss::formula::{eval::ValueRef,evaluator::{EvalResult,EvaluationContext::EvaluationContext}};
// java public interface IFunction
pub trait IFunction:Send+Sync{fn evaluate(&self,rav:i8,args:&[ValueRef],ec:&EvaluationContext)->EvalResult<ValueRef>;}
