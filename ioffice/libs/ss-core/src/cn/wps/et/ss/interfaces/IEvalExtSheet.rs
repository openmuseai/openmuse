// java package cn.wps.et.ss.interfaces;

// java public interface IEvalExtSheet
// rust: constants live beside the trait so `dyn IEvalExtSheet` remains object safe.
// java int NULL_BOOK = 1; EMPTY_BOOK = 2; ADDIN_FUNCTION_BOOK = 3; EXTERNAL_BOOK = 4;
pub const NULL_BOOK: i32 = 1;
pub const EMPTY_BOOK: i32 = 2;
pub const ADDIN_FUNCTION_BOOK: i32 = 3;
pub const EXTERNAL_BOOK: i32 = 4;
// java int ANONYMOUS_SHEET_INDEX = -2; NOT_EXIST_SHEET_INDEX = -1;
pub const ANONYMOUS_SHEET_INDEX: i32 = -2;
pub const NOT_EXIST_SHEET_INDEX: i32 = -1;

pub trait IEvalExtSheet {
    // java int getBookType();
    fn getBookType(&self) -> i32;
    // java String getBookName();
    fn getBookName(&self) -> Option<String>;
    // java int getFirstSheetIndex();
    fn getFirstSheetIndex(&self) -> i32;
    // java int getLastSheetIndex();
    fn getLastSheetIndex(&self) -> i32;
    // java String getSheetName(int var1);
    fn getSheetName(&self, sheet_index: i32) -> Option<String>;
}
