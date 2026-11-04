#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Hebrew1 extends NumberingBase
pub struct Hebrew1 {
    pub(crate) base: NumberingBase,
}

// Java: public class Hebrew1 extends NumberingBase
impl Hebrew1 {
    // Java: Hebrew1()
    pub fn new() -> Self {
        // Java: super();
        Hebrew1 {
            base: NumberingBase::new(),
        }
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: int maxLimit = 392;
        // Java: if (page > (long)maxLimit)
        {
            // Java: page %= (long)maxLimit;
        }
        // Java: int temp = (int)(page / 100L % 10L);
        // Java: if (temp != 0)
        {
            // Java: this.mBuffer.append(NumberDef.HEBREW1_X00[temp - 1]);
        }
        // Java: temp = (int)(page / 10L % 10L);
        // Java: if (temp != 0)
        {
            // Java: this.mBuffer.append(NumberDef.HEBREW1_X0[temp - 1]);
        }
        // Java: temp = (int)(page % 10L);
        // Java: if (temp != 0)
        {
            // Java: this.mBuffer.append(NumberDef.HEBREW1_X[temp - 1]);
        }
        // Java: return this.mBuffer.toString();
        todo!("Hebrew1::convert")
    }
}

impl INumbering for Hebrew1 {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
