#[path = "IValueEval.rs"]
mod value_eval_impl;
pub use value_eval_impl::IValueEval;
pub mod BlankEval;
pub mod BoolEval;
pub mod ErrorEval;
pub mod MissingArgEval;
pub mod NumberEval;
pub mod OperandResolver;
pub mod StringEval;

// rust: retained Java references to singleton values do not allocate a second
// ErrorEval/BlankEval/BoolEval. instanceof sees the original singleton object.
impl<T: IValueEval> IValueEval for &'static T {
    fn as_any(&self) -> &dyn std::any::Any {
        (*self).as_any()
    }
    fn as_numeric_eval(&self) -> Option<&dyn self::INumericValueEval::INumericValueEval> {
        (*self).as_numeric_eval()
    }
}
pub type ValueRef = std::sync::Arc<dyn IValueEval>;
pub mod INumericValueEval;
pub mod IRefEval;
pub mod IStringValueEval;
pub mod LazyRefEval;
pub mod AreaEvalBase;
pub mod IAreaEval;
pub mod LazyAreaEval;
pub mod advance;
pub mod FunctionEval;
pub mod UDFNameEval;
