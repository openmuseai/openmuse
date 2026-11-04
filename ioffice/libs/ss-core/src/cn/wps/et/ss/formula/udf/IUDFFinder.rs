// java package cn.wps.et.ss.formula.udf;
// java
// java import cn.wps.et.ss.formula.functions.base.IFunction;
// java
// java public interface IUDFFinder {
// java IFunction findFunction(String var1);
// java }
use super::super::{evaluator::EvalResult,functions::base::IFunction::IFunction};use std::sync::Arc;pub trait IUDFFinder:Send+Sync{fn findFunction(&self,name:&str)->EvalResult<Option<Arc<dyn IFunction>>>;}
