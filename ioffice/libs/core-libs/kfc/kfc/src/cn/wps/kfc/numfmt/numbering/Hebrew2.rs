#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Hebrew2 extends NumberingBase
pub struct Hebrew2 {
    pub(crate) base: NumberingBase,
}

// Java: public class Hebrew2 extends NumberingBase
impl Hebrew2 {
    // Java: Hebrew2()
    pub fn new() -> Self {
        // Java: super();
        Hebrew2 {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 47:
            // Java: this.mPageCode = NumberDef.HEBREW2;
            // Java: default:
        }
        todo!("Hebrew2::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: int maxLimit = 392;
        // Java: if (page > (long)maxLimit)
        {
            // Java: page %= (long)maxLimit;
        }
        // Java: int size = this.mPageCode.length;
        // Java: int times = (int)((page - 1L) / (long)size);
        // Java: String str = this.mPageCode[size - 1];
        // Java: while(times-- > 0)
        {
            // Java: this.mBuffer.append(str);
        }
        // Java: int remainder = (int)((page - 1L) % (long)size);
        // Java: this.mBuffer.append(this.mPageCode[remainder]);
        // Java: return this.mBuffer.toString();
        todo!("Hebrew2::convert")
    }
}

impl INumbering for Hebrew2 {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
