#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class DecimalZero extends NumberingBase
pub struct DecimalZero {
    pub(crate) base: NumberingBase,
}

// Java: public class DecimalZero extends NumberingBase
impl DecimalZero {
    // Java: DecimalZero()
    pub fn new() -> Self {
        // Java: super();
        DecimalZero {
            base: NumberingBase::new(),
        }
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: return page <= 9L && page > 0L ? this.mBuffer.append('0').append(this.longToStr(page)).toString() : this.longToStr(page);
        todo!("DecimalZero::convert")
    }
}

impl INumbering for DecimalZero {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
