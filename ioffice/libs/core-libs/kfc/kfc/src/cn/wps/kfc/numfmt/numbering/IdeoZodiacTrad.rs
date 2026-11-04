#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class IdeoZodiacTrad extends NumberingBase
pub struct IdeoZodiacTrad {
    pub(crate) base: NumberingBase,
}

// Java: public class IdeoZodiacTrad extends NumberingBase
impl IdeoZodiacTrad {
    // Java: IdeoZodiacTrad()
    pub fn new() -> Self {
        // Java: super();
        IdeoZodiacTrad {
            base: NumberingBase::new(),
        }
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: String[] traditional = NumberDef.IDEOGRAPH_TRADITIONAL;
        // Java: String[] zodiac = NumberDef.IDEOGRAPH_ZODIAC;
        // Java: int n = (int)((page - 1L) % (long)traditional.length);
        // Java: this.mBuffer.append(traditional[n]);
        // Java: n = (int)((page - 1L) % (long)zodiac.length);
        // Java: this.mBuffer.append(zodiac[n]);
        // Java: return this.mBuffer.toString();
        todo!("IdeoZodiacTrad::convert")
    }
}

impl INumbering for IdeoZodiacTrad {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
