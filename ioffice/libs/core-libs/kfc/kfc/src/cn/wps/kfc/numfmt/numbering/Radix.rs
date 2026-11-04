#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: import java.util.ArrayList
use super::NumberingBase::NumberingBase;
use super::INumbering::INumbering;

// Java: public class Radix extends NumberingBase
pub struct Radix {
    pub(crate) base: NumberingBase,
    // Java field: ArrayList<Integer> mNumDigits
    mNumDigits: Vec<i32>,
}

// Java: public class Radix extends NumberingBase
impl Radix {
    // Java: Radix()
    pub fn new() -> Self {
        // Java: super();
        Radix {
            base: NumberingBase::new(),
            mNumDigits: Vec::new(),
        }
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        // Java: switch (nfc)
        {
            // Java: case 8:
            // Java: this.mPageCode = NumberDef.HEX;
            // Java: default:
        }
        todo!("Radix::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&mut self, _page: i64, _nfc: i32) -> String {
        // Java: this.mNumDigits.clear();
        // Java: int power = this.mPageCode.length;
        // Java: if (page == 0L)
        {
            // Java: this.mNumDigits.add(0);
        }
        // Java: else
        {
            // Java: int iNum = !this.startFromZero(nfc) ? 1 : 0;
            // Java: int digit = 0;
            // Java: while(page >= (long)power)
            {
                // Java: page -= (long)iNum;
                // Java: digit = (int)(page % (long)power);
                // Java: page /= (long)power;
                // Java: this.mNumDigits.add(digit);
            }
            // Java: if (page != 0L)
            {
                // Java: this.mNumDigits.add((int)(page - (long)iNum));
            }
        }
        // Java: for(int i = this.mNumDigits.size() - 1; i >= 0; --i)
        {
            // Java: this.mBuffer.append(this.getPageNum((Integer)this.mNumDigits.get(i)));
        }
        // Java: return this.mBuffer.toString();
        todo!("Radix::convert")
    }

    // Java: boolean startFromZero(int nfc)
    pub fn startFromZero(&self, _nfc: i32) -> bool {
        // Java: switch (nfc)
        {
            // Java: case 8:
            // Java: return true;
            // Java: default:
            // Java: return false;
        }
        todo!("Radix::startFromZero")
    }
}

impl INumbering for Radix {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.base.getNumbering_impl(var1, var3)
    }
}
