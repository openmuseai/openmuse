#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Series extends NumberingBase
pub struct Series {
    pub(crate) base: NumberingBase,
}

// Java: public class Series extends NumberingBase
impl Series {
    // Java: Series()
    pub fn new() -> Self {
        // Java: super();
        Series {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 12:
            // Java: this.mPageCode = NumberDef.AIUEO;
            // Java: break;
            // Java: case 13:
            // Java: this.mPageCode = NumberDef.IROHA;
            // Java: case 14:
            // Java: case 15:
            // Java: case 16:
            // Java: case 17:
            // Java: case 19:
            // Java: case 22:
            // Java: case 23:
            // Java: case 24:
            // Java: case 25:
            // Java: default:
            // Java: break;
            // Java: case 18:
            // Java: this.mPageCode = NumberDef.DECIMAL_ENCLOSED_CIRCLE;
            // Java: break;
            // Java: case 20:
            // Java: this.mPageCode = NumberDef.AIUEO_FULL_WIDTH;
            // Java: break;
            // Java: case 21:
            // Java: this.mPageCode = NumberDef.IROHA_FULL_WIDTH;
            // Java: break;
            // Java: case 26:
            // Java: this.mPageCode = NumberDef.DECIMAL_ENCLOSED_FULL_STOP;
            // Java: break;
            // Java: case 27:
            // Java: this.mPageCode = NumberDef.DECIMAL_ENCLOSED_PAREN;
            // Java: break;
            // Java: case 28:
            // Java: this.mPageCode = NumberDef.DECIMAL_ENCLOSED_CIRCLE_CHINESE;
            // Java: break;
            // Java: case 29:
            // Java: this.mPageCode = NumberDef.IDEOGRAPH_ENCLOSED_CIRCLE;
            // Java: break;
            // Java: case 30:
            // Java: this.mPageCode = NumberDef.IDEOGRAPH_TRADITIONAL;
            // Java: break;
            // Java: case 31:
            // Java: this.mPageCode = NumberDef.IDEOGRAPH_ZODIAC;
        }
        todo!("Series::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&self, _page: i64, _nfc: i32) -> String {
        // Java: if (page <= (long)this.mPageCode.length && page > 0L)
        {
            // Java: String str = this.getPageNum((int)(page - 1L));
            // Java: return str;
        }
        // Java: else
        {
            // Java: return super.convert(page, nfc);
        }
        todo!("Series::convert")
    }
}

impl INumbering for Series {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
