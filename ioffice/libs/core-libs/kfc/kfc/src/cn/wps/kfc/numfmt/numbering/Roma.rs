#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java field: int MAX_PAGE
pub(crate) const ROMA_MAX_PAGE: i32 = 1000;

// Java: public class Roma extends NumberingBase
pub struct Roma {
    pub(crate) base: NumberingBase,
}

// Java: public class Roma extends NumberingBase
impl Roma {
    // Java: Roma()
    pub fn new() -> Self {
        // Java: super();
        Roma {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 1:
            // Java: this.mPageCode = NumberDef.UPPER_ROMAS;
            // Java: break;
            // Java: case 2:
            // Java: this.mPageCode = NumberDef.LOWER_ROMAS;
        }
        todo!("Roma::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: int romaThousand = 27;
        // Java: int romaHundred = 18;
        // Java: int romaTen = 9;
        // Java: String[] pageCode = this.mPageCode;
        // Java: long digit = page / 1000L;
        // Java: if (digit > 0L)
        {
            // Java: String code = pageCode[27];
            // Java: for(int i = 0; (long)i < digit; ++i)
            {
                // Java: this.mBuffer.append(code);
            }
        }
        // Java: page %= 1000L;
        // Java: digit = page / 100L;
        // Java: if (digit != 0L)
        {
            // Java: this.mBuffer.append(pageCode[(int)(digit + 18L - 1L)]);
        }
        // Java: page %= 100L;
        // Java: digit = page / 10L;
        // Java: if (digit != 0L)
        {
            // Java: this.mBuffer.append(pageCode[(int)(digit + 9L - 1L)]);
        }
        // Java: digit = page % 10L;
        // Java: if (digit != 0L)
        {
            // Java: this.mBuffer.append(pageCode[(int)(digit - 1L)]);
        }
        // Java: return this.mBuffer.toString();
        todo!("Roma::convert")
    }
}

impl INumbering for Roma {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
