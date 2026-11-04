#![allow(non_snake_case, non_upper_case_globals)]

// Java: public class FormulaType
pub struct FormulaType;
impl FormulaType {
    pub const ERROR: i32 = 0;
    pub const MUL_DIV: i32 = 1;
    pub const ADD_SUBRACT: i32 = 2;
    pub const ADD_DIV: i32 = 3;
    pub const IF_ELSE: i32 = 4;
    pub const ABS: i32 = 5;
    pub const AT2: i32 = 6;
    pub const CAT2: i32 = 7;
    pub const COS: i32 = 8;
    pub const MAX: i32 = 9;
    pub const MIN: i32 = 10;
    pub const MOD: i32 = 11;
    pub const PIN: i32 = 12;
    pub const SAT2: i32 = 13;
    pub const SIN: i32 = 14;
    pub const SQRT: i32 = 15;
    pub const TAN: i32 = 16;
    pub const VAL: i32 = 17;
    pub const S_MUL_DIV: &'static str = "*/";
    pub const S_ADD_SUBRACT: &'static str = "+-";
    pub const S_ADD_DIV: &'static str = "+/";
    pub const S_IF_ELSE: &'static str = "?:";
    pub const S_ABS: &'static str = "abs";
    pub const S_AT2: &'static str = "at2";
    pub const S_CAT2: &'static str = "cat2";
    pub const S_COS: &'static str = "cos";
    pub const S_MAX: &'static str = "max";
    pub const S_MIN: &'static str = "min";
    pub const S_MOD: &'static str = "mod";
    pub const S_PIN: &'static str = "pin";
    pub const S_SAT2: &'static str = "sat2";
    pub const S_SIN: &'static str = "sin";
    pub const S_SQRT: &'static str = "sqrt";
    pub const S_TAN: &'static str = "tan";
    pub const S_VAL: &'static str = "val";
    // Java: public static int toType(String value)
    pub fn toType(value: &str) -> i32 {
        match value {
            "*/" => 1, "+-" => 2, "+/" => 3, "?:" => 4, "abs" => 5,
            "at2" => 6, "cat2" => 7, "cos" => 8, "max" => 9, "min" => 10,
            "mod" => 11, "pin" => 12, "sat2" => 13, "sin" => 14,
            "sqrt" => 15, "tan" => 16, "val" => 17, _ => 0,
        }
    }
}
