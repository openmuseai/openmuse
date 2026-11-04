#![allow(non_snake_case)]
#![allow(dead_code)]

// Java field: short ANALYZE_TYPE_DATE
pub const ANALYZE_TYPE_DATE: i16 = 0;
// Java field: short ANALYZE_TYPE_NUMERIC
pub const ANALYZE_TYPE_NUMERIC: i16 = 1;
// Java field: short ANALYZE_TYPE_GENERAL
pub const ANALYZE_TYPE_GENERAL: i16 = 2;
// Java field: int ERROR_NONE
pub const ERROR_NONE: i32 = 0;
// Java field: int ERROR_UNKNOWN
pub const ERROR_UNKNOWN: i32 = 1;
// Java field: int ERROR_AT_N
pub const ERROR_AT_N: i32 = 2;
// Java field: int ERROR_QUOTE
pub const ERROR_QUOTE: i32 = 3;
// Java field: int ERROR_SWITCH_ARG
pub const ERROR_SWITCH_ARG: i32 = 4;

// Java: public interface IWordFmtAnalyzer
pub trait IWordFmtAnalyzer {
    // Java: abstract int analyze(String var1, short var2)
    fn analyze(&self, var1: &str, var2: i16) -> i32;

}
