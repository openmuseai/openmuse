#[path = "EvaluationException.rs"]
mod evaluation_exception_impl;
pub use evaluation_exception_impl::EvaluationException;
#[path = "NotCalculatedException.rs"]
mod not_calc_exception_impl;
pub use not_calc_exception_impl::NotCalculatedException;
pub mod EvaluationContext;
pub mod WorkbookEvaluator;
pub mod WorkbookBytesEvaluator;

// rust: Java exceptions cross the evaluator/chain boundary as a typed Result.
pub enum EvalFailure {
    Runtime(&'static str),
    NotCalculated(std::sync::Arc<NotCalculatedException>),
    // Explicit implementation boundary, never converted to a spreadsheet value.
    Unported(&'static str),
}
pub type EvalResult<T> = Result<T, EvalFailure>;
pub mod SheetRefEvaluator;
pub mod OperationEvaluatorFactory;
pub mod IValueIterator;
