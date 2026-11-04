#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::Tongue::Tongue;
use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class TongueJapanese extends Tongue
pub struct TongueJapanese {
    pub(crate) base: Tongue,
}

// Java: public class TongueJapanese extends Tongue
impl TongueJapanese {
    // Java: TongueJapanese()
    pub fn new() -> Self {
        // Java: super();
        TongueJapanese {
            base: Tongue::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 11:
            // Java: this.mPageCode = NumberDef.JAPANESE_COUNTING;
            // Java: break;
            // Java: case 16:
            // Java: this.mPageCode = NumberDef.JAPANESE_LEGAL;
        }
        todo!("TongueJapanese::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: switch (nfc)
        {
            // Java: case 11:
            // Java: this.convertTongueJp(page, false, this.mPageCode, this.mBuffer);
            // Java: break;
            // Java: case 16:
            // Java: this.convertTongueJp(page, true, this.mPageCode, this.mBuffer);
        }
        // Java: return this.mBuffer.toString();
        todo!("TongueJapanese::convert")
    }

    // Java: void convertTongueJp(long value, boolean bHead, String[] pageCode, StringBuffer result)
    pub fn convertTongueJp(&self, _value: i64, _b_head: bool, _page_code: &[String], _result: &mut String) {
        // Java: String strValue = this.longToStr(value);
        // Java: int MEGE_DIG_COUNT = 12;
        // Java: int lSet = strValue.length() % 12;
        // Java: int lMegaCount = (strValue.length() - 1) / 12;
        // Java: if (lSet == 0)
        {
            // Java: lSet = 12;
        }
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
        // Java: String chThouand = pageCode[12];
        // Java: String chHundred = pageCode[11];
        // Java: String chTen = pageCode[10];
        // Java: String chHundredMillion = pageCode[14];
        // Java: String chMyriad = pageCode[13];
        // Java: String chMega = pageCode[15];
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
                    // Java: result.append(chMega);
                }
                // Java: bSect = false;
                // Java: break;
                // Java: case 2:
                // Java: case 6:
                // Java: case 10:
                // Java: if ('0' != ch)
                {
                    // Java: if (bHead || '1' != ch)
                    {
                        // Java: result.append(this.getPageNum(ch));
                    }
                    // Java: result.append(chTen);
                    // Java: bSect = true;
                }
                // Java: else if ('0' != strValue.charAt(lPos + 1))
                {
                }
                // Java: break;
                // Java: case 3:
                // Java: case 7:
                // Java: case 11:
                // Java: if ('0' != ch)
                {
                    // Java: if (bHead || '1' != ch)
                    {
                        // Java: result.append(this.getPageNum(ch));
                    }
                    // Java: result.append(chHundred);
                    // Java: bSect = true;
                }
                // Java: else if ('0' != strValue.charAt(lPos + 1))
                {
                }
                // Java: break;
                // Java: case 4:
                // Java: case 8:
                // Java: case 12:
                // Java: if ('0' != ch)
                {
                    // Java: if (bHead || '1' != ch)
                    {
                        // Java: result.append(this.getPageNum(ch));
                    }
                    // Java: result.append(chThouand);
                    // Java: bSect = true;
                }
                // Java: else if ('0' != strValue.charAt(lPos + 1))
                {
                }
                // Java: break;
                // Java: case 5:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(chMyriad);
                }
                // Java: else if (bSect)
                {
                    // Java: result.append(chMyriad);
                }
                // Java: bSect = false;
                // Java: break;
                // Java: case 9:
                // Java: if ('0' != ch)
                {
                    // Java: result.append(this.getPageNum(ch));
                    // Java: result.append(chHundredMillion);
                }
                // Java: else if (bSect)
                {
                    // Java: result.append(chHundredMillion);
                }
                // Java: bSect = false;
            }
            // Java: --lSet;
        }
        todo!("TongueJapanese::convertTongueJp")
    }
}

impl INumbering for TongueJapanese {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.base.getNumbering_impl(var1, var3)
    }
}
