// java package cn.wps.et.ss.formula.eval;

// java public interface IValueEval {}
pub trait IValueEval: std::any::Any + Send + Sync {
    fn as_numeric_eval(&self) -> Option<&dyn super::INumericValueEval::INumericValueEval> {
        None
    }
    fn as_area_eval(&self)->Option<&dyn super::IAreaEval::IAreaEval>{None}
    fn as_ref_eval(&self) -> Option<&dyn super::IRefEval::IRefEval> {
        None
    }
    // rust: Java instanceof dispatch without replacing the Java concrete classes.
    fn as_any(&self) -> &dyn std::any::Any;
}
