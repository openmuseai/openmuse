#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class KoreanCounting extends NumberingBase
pub struct KoreanCounting {
    pub(crate) base: NumberingBase,
}

// Java: public class KoreanCounting extends NumberingBase
impl KoreanCounting {
    // Java: KoreanCounting()
    pub fn new() -> Self {
        // Java: super();
        KoreanCounting {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 42:
            // Java: this.mPageCode = NumberDef.KOREAN_COUNTING;
            // Java: default:
        }
        todo!("KoreanCounting::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: if (page > 99999L)
        {
            // Java: return this.longToStr(page);
        }
        // Java: else
        {
            // Java: int posTen = 10;
            // Java: int posHundred = 11;
            // Java: int posThousand = 12;
            // Java: int posMyriad = 13;
            // Java: int temp = (int)(page / 10000L % 10L);
            // Java: if (temp > 0)
            {
                // Java: if (temp != 1)
                {
                    // Java: this.mBuffer.append(this.mPageCode[temp]);
                }
                // Java: this.mBuffer.append(this.mPageCode[13]);
            }
            // Java: temp = (int)(page / 1000L % 10L);
            // Java: if (temp > 0)
            {
                // Java: if (temp != 1)
                {
                    // Java: this.mBuffer.append(this.mPageCode[temp]);
                }
                // Java: this.mBuffer.append(this.mPageCode[12]);
            }
            // Java: temp = (int)(page / 100L % 10L);
            // Java: if (temp > 0)
            {
                // Java: if (temp != 1)
                {
                    // Java: this.mBuffer.append(this.mPageCode[temp]);
                }
                // Java: this.mBuffer.append(this.mPageCode[11]);
            }
            // Java: temp = (int)(page / 10L % 10L);
            // Java: if (temp > 0)
            {
                // Java: if (temp != 1)
                {
                    // Java: this.mBuffer.append(this.mPageCode[temp]);
                }
                // Java: this.mBuffer.append(this.mPageCode[10]);
            }
            // Java: temp = (int)(page % 10L);
            // Java: if (temp > 0)
            {
                // Java: this.mBuffer.append(this.mPageCode[temp]);
            }
            // Java: return this.mBuffer.toString();
        }
        todo!("KoreanCounting::convert")
    }
}

impl INumbering for KoreanCounting {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
