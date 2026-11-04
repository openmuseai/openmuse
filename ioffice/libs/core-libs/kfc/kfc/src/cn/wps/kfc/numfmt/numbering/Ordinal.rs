#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Ordinal extends NumberingBase
pub struct Ordinal {
    pub(crate) base: NumberingBase,
}

// Java: public class Ordinal extends NumberingBase
impl Ordinal {
    // Java: Ordinal()
    pub fn new() -> Self {
        // Java: super();
        Ordinal {
            base: NumberingBase::new(),
        }
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: this.mBuffer.append(this.longToStr(page));
        // Java: int index = 0;
        // Java: switch ((int)(page % 10L))
        {
            // Java: case 1:
            // Java: case 2:
            // Java: case 3:
            // Java: if (page % 100L / 10L != 1L)
            {
                // Java: index = (int)(page % 10L);
                // Java: break;
            }
            // Java: default:
            // Java: index = 0;
        }
        // Java: String[] EN_NUM_Th = new String[]{"th", "st", "nd", "rd"};
        // Java: this.mBuffer.append(EN_NUM_Th[index]);
        // Java: return this.mBuffer.toString();
        todo!("Ordinal::convert")
    }
}

impl INumbering for Ordinal {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
