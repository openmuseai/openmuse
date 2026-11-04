// java public interface IEvalCell
use crate::cn::wps::et::ss::formula::evaluator::EvalResult;
pub trait IEvalCell: Send + Sync {
    fn checkRecalculationFlag(&self) -> EvalResult<Option<bool>>;
    fn getFinalCellType(&self) -> i32;
    fn updateCell(&self);
    fn getNumericCellValue(&self) -> f64;
    fn getStringCellValue(&self) -> Option<String>;
    fn getBooleanCellValue(&self) -> bool;
    fn getErrorCellValue(&self) -> i8;
    fn getSheetUid(&self) -> i32;
    fn getRowIndex(&self) -> i32;
    fn getColIndex(&self) -> i32;
}
