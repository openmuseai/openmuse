// java package cn.wps.et.ss.formula.functions.base;
// java
// java import cn.wps.et.ss.formula.eval.IValueEval;
// java import cn.wps.et.ss.formula.evaluator.EvaluationContext;
// java
// java public abstract class ArrayOpFunction extends MixOpFunction {
// java protected boolean shouldClip() {
// java return false;
// java }
// java
// java public final boolean valueArgRequired(int argPos, IValueEval[] args, EvaluationContext ec) {
// java return false;
// java }
// java }
pub trait ArrayOpFunction:super::MixOpFunction::MixOpFunction{}
