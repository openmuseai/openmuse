#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class NumberShape extends NumberingBase
pub struct NumberShape {
    pub(crate) base: NumberingBase,
}

// Java: public class NumberShape extends NumberingBase
impl NumberShape {
    // Java: NumberShape()
    pub fn new() -> Self {
        // Java: super();
        NumberShape {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 10:
            // Java: case 17:
            // Java: case 36:
            // Java: case 37:
            // Java: this.mPageCode = NumberDef.IDEOGRAPH_DIGITAL;
            // Java: break;
            // Java: case 14:
            // Java: case 19:
            // Java: this.mPageCode = NumberDef.DECIMAL_FULL_WIDTH;
            // Java: break;
            // Java: case 41:
            // Java: this.mPageCode = NumberDef.KOREAN_DIGITAL;
            // Java: break;
            // Java: case 44:
            // Java: this.mPageCode = NumberDef.KOREAN_DIGITAL2;
            // Java: break;
            // Java: case 51:
            // Java: this.mPageCode = NumberDef.HINDI_NUMBERS;
            // Java: break;
            // Java: case 54:
            // Java: this.mPageCode = NumberDef.THAI_NUMBERS;
        }
        todo!("NumberShape::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: String strValue = this.longToStr(page);
        // Java: for(int i = 0; i < strValue.length(); ++i)
        {
            // Java: int n = strValue.charAt(i) - 48;
            // Java: this.mBuffer.append(this.mPageCode[n]);
        }
        // Java: return this.mBuffer.toString();
        todo!("NumberShape::convert")
    }
}

impl INumbering for NumberShape {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
