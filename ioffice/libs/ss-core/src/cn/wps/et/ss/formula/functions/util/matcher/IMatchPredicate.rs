// java package cn.wps.et.ss.formula.functions.util.matcher;
// java
// java import cn.wps.et.ss.formula.eval.IValueEval;
// java
// java public interface IMatchPredicate {
// java boolean matches(IValueEval var1, boolean var2);
// java }
use super::super::super::super::{eval::ValueRef,evaluator::EvalResult};pub trait IMatchPredicate:Send+Sync{fn matches(&self,v:&ValueRef,directlyArg:bool)->EvalResult<bool>;fn as_any(&self)->&dyn std::any::Any;}