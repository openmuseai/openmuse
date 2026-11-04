#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java field: int posOne
pub(crate) const posOne: i32 = 1;
// Java field: int posTen
pub(crate) const posTen: i32 = 10;
// Java field: int posHundred
pub(crate) const posHundred: i32 = 11;
// Java field: int posThousand
pub(crate) const posThousand: i32 = 12;
// Java field: int posMyriad
pub(crate) const posMyriad: i32 = 13;
// Java field: int posHundredMillion
pub(crate) const posHundredMillion: i32 = 14;
// Java field: int posMega
pub(crate) const posMega: i32 = 15;

// Java: public class Tongue extends NumberingBase
pub struct Tongue {
    pub(crate) base: NumberingBase,
}

// Java: public class Tongue extends NumberingBase
impl Tongue {
    // Java: Tongue()
    pub fn new() -> Self {
        // Java: super();
        Tongue {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 34:
            // Java: this.mPageCode = NumberDef.IDEOGRAPH_LEGAL_TRADITIONANL;
            // Java: break;
            // Java: case 35:
            // Java: this.mPageCode = NumberDef.TAIWANESE_COUNTING_THOUSAND;
            // Java: case 36:
            // Java: case 37:
            // Java: default:
            // Java: break;
            // Java: case 38:
            // Java: this.mPageCode = NumberDef.CHINESE_LEGAL_SIMPLIFIED;
            // Java: break;
            // Java: case 39:
            // Java: this.mPageCode = NumberDef.CHINESE_COUNTING_THOUSAND;
        }
        todo!("Tongue::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: switch (nfc)
        {
            // Java: case 34:
            // Java: case 36:
            // Java: case 37:
            // Java: case 38:
            // Java: default:
            // Java: this.convertTongueChinese(page, false, this.mPageCode, this.mBuffer);
            // Java: break;
            // Java: case 35:
            // Java: case 39:
            // Java: this.convertTongueChinese(page, true, this.mPageCode, this.mBuffer);
        }
        // Java: return this.mBuffer.toString();
        todo!("Tongue::convert")
    }

    // Java: void convertTongueChinese(long value, boolean bEraseTen, String[] pageCode, StringBuffer result)
    pub fn convertTongueChinese(&self, _value: i64, _b_erase_ten: bool, _page_code: &[String], _result: &mut String) {
        // Java: String strValue = this.longToStr(value);
        // Java: int MEGE_DIG_COUNT = 12;
        // Java: int lSet = strValue.length() % 12;
        // Java: int lMegaCount = (strValue.length() - 1) / 12;
        // Java: int lPos = -1;
        // Java: if (lMegaCount > 0 && (lMegaCount != 1 || lSet > 4))
        {
            // Java: while(lMegaCount > 0)
            {
                // Java: while(lSet > 0)
                {
                    // Java: ++lPos;
                    // Java: char ch = strValue.charAt(lPos);
                    // Java: result.append(this.getPageNum(ch));
                    // Java: --lSet;
                }
                // Java: --lMegaCount;
                // Java: lSet = 12;
            }
            // Java: result.append(pageCode[15]);
        }
        // Java: boolean bSect = false;
        // Java: while(true)
        {
            // Java: while(lSet <= 0)
            {
                // Java: --lMegaCount;
                // Java: if (lMegaCount >= 0)
                {
                    // Java: lSet = 12;
                }
                // Java: else
                {
                    // Java: lSet = 0;
                }
                // Java: if (lSet <= 0)
                {
                    // Java: lPos = 0;
                    // Java: if (result.length() == lPos)
                    {
                        // Java: result.append(pageCode[0]);
                    }
                    // Java: if (bEraseTen && result.charAt(lPos) == pageCode[1].charAt(0) && result.length() >= lPos + 2 && result.charAt(lPos + 1) == pageCode[10].charAt(0))
                    {
                        // Java: result.replace(lPos, lPos + 1, "");
                    }
                    // Java: return;
                }
            }
            // Java: ++lPos;
            // Java: char ch = strValue.charAt(lPos);
            // Java: switch (lSet)
            {
                // Java: case 1:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                }
                // Java: if (lMegaCount > 0)
                {
                    // Java: result.append(pageCode[15]);
                }
                // Java: bSect = false;
                // Java: break;
                // Java: case 2:
                // Java: case 6:
                // Java: case 10:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(pageCode[10]);
                    // Java: bSect = true;
                }
                // Java: else if ('0' != strValue.charAt(lPos + 1))
                {
                    // Java: result.append(pageCode[0]);
                }
                // Java: break;
                // Java: case 3:
                // Java: case 7:
                // Java: case 11:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(pageCode[11]);
                    // Java: bSect = true;
                }
                // Java: else if ('0' != strValue.charAt(lPos + 1))
                {
                    // Java: result.append(pageCode[0]);
                }
                // Java: break;
                // Java: case 4:
                // Java: case 8:
                // Java: case 12:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(pageCode[12]);
                    // Java: bSect = true;
                }
                // Java: else if ('0' != strValue.charAt(lPos + 1))
                {
                    // Java: result.append(pageCode[0]);
                }
                // Java: break;
                // Java: case 5:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(pageCode[13]);
                }
                // Java: else if (bSect)
                {
                    // Java: result.append(pageCode[13]);
                }
                // Java: bSect = false;
                // Java: break;
                // Java: case 9:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(pageCode[14]);
                }
                // Java: else if (bSect)
                {
                    // Java: result.append(pageCode[14]);
                }
                // Java: bSect = false;
            }
            // Java: --lSet;
        }
        todo!("Tongue::convertTongueChinese")
    }
}

impl INumbering for Tongue {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
