#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.util.NumberingUtil
use crate::cn::wps::kfc::numfmt::util::NumberingUtil::NumberingUtil;
use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class OrdinalText extends NumberingBase
pub struct OrdinalText {
    pub(crate) base: NumberingBase,
}

// Java: public class OrdinalText extends NumberingBase
impl OrdinalText {
    // Java: OrdinalText()
    pub fn new() -> Self {
        // Java: super();
        OrdinalText {
            base: NumberingBase::new(),
        }
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: if (page > 999999L)
        {
            // Java: return this.longToStr(page);
        }
        // Java: else
        {
            // Java: NumberingUtil.convertEnglishOne(page, false, this.mBuffer);
            // Java: String TH = "th";
            // Java: String FTH = "fth";
            // Java: String IETH = "ieth";
            // Java: String FIRST = "first";
            // Java: String SECOND = "second";
            // Java: String THIRD = "third";
            // Java: int temp = 0;
            // Java: int lenght = this.mBuffer.length();
            // Java: char lastChar = this.mBuffer.charAt(lenght - 1);
            // Java: label29:
            // Java: switch (lastChar)
            {
                // Java: case 'e':
                // Java: temp = (int)(page % 10L);
                // Java: switch (temp)
                {
                    // Java: case 1:
                    // Java: this.mBuffer.replace(lenght - 3, lenght, "first");
                    // Java: break label29;
                    // Java: case 2:
                    // Java: case 5:
                    // Java: this.mBuffer.replace(lenght - 2, lenght, "fth");
                    // Java: break label29;
                    // Java: case 3:
                    // Java: this.mBuffer.replace(lenght - 5, lenght, "third");
                    // Java: break label29;
                    // Java: case 4:
                    // Java: case 6:
                    // Java: case 7:
                    // Java: case 8:
                    // Java: default:
                    // Java: assert false;
                    // Java: break label29;
                    // Java: case 9:
                    // Java: this.mBuffer.replace(lenght - 1, lenght, "th");
                    // Java: break label29;
                }
                // Java: case 'o':
                // Java: temp = (int)(page % 10L);
                // Java: if (temp == 2)
                {
                    // Java: this.mBuffer.replace(lenght - 3, lenght, "second");
                }
                // Java: else if (temp == 0)
                {
                    // Java: this.mBuffer.append("th");
                }
                // Java: else
                {
                    // Java: assert false;
                }
                // Java: break;
                // Java: case 't':
                // Java: this.mBuffer.replace(lenght - 1, lenght, "th");
                // Java: break;
                // Java: case 'y':
                // Java: this.mBuffer.replace(lenght - 1, lenght, "ieth");
                // Java: break;
                // Java: default:
                // Java: this.mBuffer.append("th");
            }
            // Java: NumberingUtil.convertCapital(true, this.mBuffer);
            // Java: return this.mBuffer.toString();
        }
        todo!("OrdinalText::convert")
    }
}

impl INumbering for OrdinalText {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
