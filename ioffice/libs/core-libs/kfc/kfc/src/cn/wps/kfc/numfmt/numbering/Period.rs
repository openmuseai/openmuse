#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Period extends NumberingBase
pub struct Period {
    pub(crate) base: NumberingBase,
}

// Java: public class Period extends NumberingBase
impl Period {
    // Java: Period()
    pub fn new() -> Self {
        // Java: super();
        Period {
            base: NumberingBase::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 24:
            // Java: this.mPageCode = NumberDef.GANADA;
            // Java: break;
            // Java: case 25:
            // Java: this.mPageCode = NumberDef.CHOSUNG;
        }
        todo!("Period::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&self, _page: i64, _nfc: i32) -> String {
        // Java: page = (page - 1L) % (long)this.mPageCode.length;
        // Java: return this.mPageCode[(int)page];
        todo!("Period::convert")
    }
}

impl INumbering for Period {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
