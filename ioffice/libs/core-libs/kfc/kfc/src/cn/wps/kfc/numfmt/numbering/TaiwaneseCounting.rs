#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class TaiwaneseCounting extends NumberingBase
pub struct TaiwaneseCounting {
    pub(crate) base: NumberingBase,
}

// Java: public class TaiwaneseCounting extends NumberingBase
impl TaiwaneseCounting {
    // Java: TaiwaneseCounting()
    pub fn new() -> Self {
        // Java: super();
        TaiwaneseCounting {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 33:
            // Java: this.mPageCode = NumberDef.TAIWANESE_COUNTING;
            // Java: default:
        }
        todo!("TaiwaneseCounting::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: if (page >= 10L && page <= 99L)
        {
            // Java: int ten = (int)(page / 10L);
            // Java: int unit = (int)(page % 10L);
            // Java: int posTen = 10;
            // Java: if (ten != 1)
            {
                // Java: this.mBuffer.append(this.mPageCode[ten]);
            }
            // Java: this.mBuffer.append(this.mPageCode[10]);
            // Java: if (unit != 0)
            {
                // Java: this.mBuffer.append(this.mPageCode[unit]);
            }
        }
        // Java: else
        {
            // Java: String strValue = this.longToStr(page);
            // Java: for(int i = 0; i < strValue.length(); ++i)
            {
                // Java: int n = strValue.charAt(i) - 48;
                // Java: this.mBuffer.append(this.mPageCode[n]);
            }
        }
        // Java: return this.mBuffer.toString();
        todo!("TaiwaneseCounting::convert")
    }
}

impl INumbering for TaiwaneseCounting {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
