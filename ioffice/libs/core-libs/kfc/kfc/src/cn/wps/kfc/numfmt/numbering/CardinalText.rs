#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import cn.wps.kfc.numfmt.util.NumberingUtil
use crate::cn::wps::kfc::numfmt::util::NumberingUtil::NumberingUtil;
use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class CardinalText extends NumberingBase
pub struct CardinalText {
    pub(crate) base: NumberingBase,
}

// Java: public class CardinalText extends NumberingBase
impl CardinalText {
    // Java: CardinalText()
    pub fn new() -> Self {
        // Java: super();
        CardinalText {
            base: NumberingBase::new(),
        }
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: NumberingUtil.convertEnglishOne(page, true, this.mBuffer);
        // Java: return this.mBuffer.toString();
        todo!("CardinalText::convert")
    }
}

impl INumbering for CardinalText {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
