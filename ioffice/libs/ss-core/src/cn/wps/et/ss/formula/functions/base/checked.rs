// rust: Representation of Java's checked EvaluationException versus runtime/not-calculated failures.
use crate::cn::wps::et::ss::formula::{eval::{ValueRef,ErrorEval::ErrorEval,OperandResolver::OperandResolver},evaluator::{EvalResult,EvalFailure,EvaluationContext::EvaluationContext}};use std::sync::Arc;
pub enum FunctionFailure{Checked(&'static ErrorEval),Runtime(EvalFailure)}pub type FunctionResult<T>=Result<T,FunctionFailure>;
impl From<EvalFailure> for FunctionFailure{fn from(e:EvalFailure)->Self{Self::Runtime(e)}}impl From<&'static ErrorEval> for FunctionFailure{fn from(e:&'static ErrorEval)->Self{Self::Checked(e)}}
pub fn catch(r:FunctionResult<ValueRef>)->EvalResult<ValueRef>{match r{Ok(v)=>Ok(v),Err(FunctionFailure::Checked(e))=>Ok(Arc::new(e)),Err(FunctionFailure::Runtime(e))=>Err(e)}}
pub fn single(v:&ValueRef,ec:&EvaluationContext)->FunctionResult<ValueRef>{Ok(OperandResolver::getSingleValue(v,ec)??)}
pub fn double(v:&ValueRef)->FunctionResult<f64>{Ok(OperandResolver::coerceValueToDouble(v)??)}
pub fn int(v:&ValueRef)->FunctionResult<i32>{Ok(OperandResolver::coerceValueToInt(v)??)}
pub fn string(v:&ValueRef)->FunctionResult<Vec<u16>>{Ok(OperandResolver::coerceValueToString(v)??)}
