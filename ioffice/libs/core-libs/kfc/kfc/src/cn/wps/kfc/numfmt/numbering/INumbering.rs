#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code)]

// Java: public interface INumbering
pub trait INumbering {
    // Java: abstract String getNumbering(long var1, int var3)
    fn getNumbering(&self, var1: i64, var3: i32) -> String;
}
