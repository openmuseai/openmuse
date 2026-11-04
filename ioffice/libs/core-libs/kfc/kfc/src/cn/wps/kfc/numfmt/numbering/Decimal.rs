#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Decimal extends NumberingBase
pub struct Decimal {
    pub(crate) base: NumberingBase,
}

// Java: public class Decimal extends NumberingBase
impl Decimal {
    // Java: Decimal()
    pub fn new() -> Self {
        // Java: super();
        Decimal {
            base: NumberingBase::new(),
        }
    }
}

impl INumbering for Decimal {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
