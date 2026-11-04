#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Vietnamese extends NumberingBase
pub struct Vietnamese {
    pub(crate) base: NumberingBase,
}

// Java: public class Vietnamese extends NumberingBase
impl Vietnamese {
    // Java: Vietnamese()
    pub fn new() -> Self {
        // Java: super();
        Vietnamese {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 56:
            // Java: this.mPageCode = NumberDef.VIETNAMESE_COUNTING;
            // Java: default:
        }
        todo!("Vietnamese::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: int posZero = 0;
        // Java: int posOne = 1;
        // Java: int posTen = 10;
        // Java: int posHundred = 11;
        // Java: int posThousand = 12;
        // Java: String[] pages = this.mPageCode;
        // Java: if (page == 1000L)
        {
            // Java: this.append(pages[1], this.mBuffer);
            // Java: this.append(pages[12], this.mBuffer);
            // Java: return this.mBuffer.toString();
        }
        // Java: else
        {
            // Java: page %= 1000L;
            // Java: int hundred = (int)(page / 100L % 10L);
            // Java: if (hundred != 0)
            {
                // Java: this.append(pages[hundred], this.mBuffer);
                // Java: this.append(pages[11], this.mBuffer);
            }
            // Java: int ten = (int)(page / 10L % 10L);
            // Java: int one = (int)(page % 10L);
            // Java: if (ten != 0 || ten == 0 && one != 0)
            {
                // Java: if (ten != 0)
                {
                    // Java: if (ten != 1)
                    {
                        // Java: this.append(pages[ten], this.mBuffer);
                    }
                    // Java: this.append(pages[10], this.mBuffer);
                }
                // Java: if (ten == 0 && one != 0 && hundred != 0)
                {
                    // Java: this.append(pages[0], this.mBuffer);
                }
            }
            // Java: if (one != 0)
            {
                // Java: this.append(pages[one], this.mBuffer);
            }
            // Java: return this.mBuffer.toString();
        }
        todo!("Vietnamese::convert")
    }

    // Java: void append(String data, StringBuffer buffer)
    fn append(&self, _data: &str, _buffer: &mut String) {
        // Java: if (buffer.length() > 0)
        {
            // Java: buffer.append(' ');
        }
        // Java: buffer.append(data);
        todo!("Vietnamese::append")
    }
}

impl INumbering for Vietnamese {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
