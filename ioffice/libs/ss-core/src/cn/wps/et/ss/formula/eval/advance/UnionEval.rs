// java package cn.wps.et.ss.formula.eval.advance;
// java
// java import cn.wps.et.ss.formula.eval.IValueEval;
// java
// java public final class UnionEval implements IValueEval {
// java public final IValueEval first;
// java public final IValueEval second;
// java
// java public UnionEval(IValueEval v1, IValueEval v2) {
// java this.first = v1;
// java this.second = v2;
// java }
// java }
use super::super::{ValueRef,IValueEval};pub struct UnionEval{pub first:ValueRef,pub second:ValueRef}impl UnionEval{pub fn new(first:ValueRef,second:ValueRef)->Self{Self{first,second}}}impl IValueEval for UnionEval{fn as_any(&self)->&dyn std::any::Any{self}}
