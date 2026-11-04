#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java field: long MAX_PAGE
pub(crate) const MAX_PAGE: i64 = 780;

// Java: public class Alphabet extends NumberingBase
pub struct Alphabet {
    pub(crate) base: NumberingBase,
}

// Java: public class Alphabet extends NumberingBase
impl Alphabet {
    // Java: Alphabet()
    pub fn new() -> Self {
        // Java: super();
        Alphabet {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        // Java: case 3:
        // Java: this.mPageCode = NumberDef.UPPER_LETTER;
        // Java: break;
        // Java: case 4:
        // Java: this.mPageCode = NumberDef.LOWER_LETTER;
        // Java: break;
        // Java: case 9:
        // Java: this.mPageCode = NumberDef.CHICAGO;
        // Java: break;
        // Java: case 46:
        // Java: this.mPageCode = NumberDef.ARABIC_ALPHA;
        // Java: break;
        // Java: case 48:
        // Java: this.mPageCode = NumberDef.ARABIC_ABJAD;
        // Java: break;
        // Java: case 49:
        // Java: this.mPageCode = NumberDef.HINDI_VOWELS;
        // Java: break;
        // Java: case 50:
        // Java: this.mPageCode = NumberDef.HINDI_CONSONANTS;
        // Java: break;
        // Java: case 53:
        // Java: this.mPageCode = NumberDef.THAI_LETTERS;
        // Java: break;
        // Java: case 58:
        // Java: this.mPageCode = NumberDef.RUSSIAN_LOWER;
        // Java: break;
        // Java: case 59:
        // Java: this.mPageCode = NumberDef.RUSSIAN_UPPER;
        todo!("Alphabet::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: page = (page - 1L) % 780L + 1L;
        // Java: int remainder = (int)((page - 1L) % (long)this.mPageCode.length);
        // Java: int times = (int)((page - 1L) / (long)this.mPageCode.length + 1L);
        // Java: this.appendRepeatStr(this.mPageCode[remainder], times, this.mBuffer);
        // Java: return this.mBuffer.toString();
        todo!("Alphabet::convert")
    }

    // Java: void appendRepeatStr(String str, int n, StringBuffer result)
    fn appendRepeatStr(&self, _str: &str, _n: i32, _result: &mut String) {
        // Java: while(n-- > 0)
        {
            // Java: result.append(str);
        }
        todo!("Alphabet::appendRepeatStr")
    }
}

impl INumbering for Alphabet {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
