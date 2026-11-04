#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

use super::INumbering::INumbering;

// Java: public class NumberingBase implements INumbering
pub struct NumberingBase {
    // Java field: String[] mPageCode
    pub(crate) mPageCode: Vec<String>,
    // Java field: StringBuffer mBuffer
    pub(crate) mBuffer: String,
}

// Java: public class NumberingBase implements INumbering
impl NumberingBase {
    // Java: NumberingBase()
    pub fn new() -> Self {
        // Java: super();
        NumberingBase {
            mPageCode: Vec::new(),
            mBuffer: String::new(),
        }
    }

    // Java: String getNumbering(long page, int nfc)
    pub fn getNumbering_impl(&self, _page: i64, _nfc: i32) -> String {
        // Java: this.reset(nfc);
        // Java: this.setPageCode(nfc);
        // Java: return this.convert(page, nfc);
        todo!("NumberingBase::getNumbering")
    }

    // Java: void setPageCode(int nfc)
    pub fn setPageCode(&mut self, _nfc: i32) {
        todo!("NumberingBase::setPageCode")
    }

    // Java: String convert(long page, int nfc)
    pub fn convert(&self, _page: i64, _nfc: i32) -> String {
        // Java: return this.longToStr(page);
        todo!("NumberingBase::convert")
    }

    // Java: void reset(int nfc)
    pub fn reset(&mut self, _nfc: i32) {
        // Java: this.mBuffer.setLength(0);
        todo!("NumberingBase::reset")
    }

    // Java: String longToStr(long page)
    pub fn longToStr(&self, _page: i64) -> String {
        // Java: return Long.toString(page);
        todo!("NumberingBase::longToStr")
    }

    // Java: String getPageNum(int n)
    pub fn getPageNum_int(&self, _n: i32) -> String {
        // Java: return this.mPageCode[n];
        todo!("NumberingBase::getPageNum_int")
    }

    // Java: String getPageNum(char c)
    pub fn getPageNum(&self, _c: char) -> String {
        // Java: return this.mPageCode[c - 48];
        todo!("NumberingBase::getPageNum")
    }
}

impl INumbering for NumberingBase {
    fn getNumbering(&self, var1: i64, var3: i32) -> String {
        self.getNumbering_impl(var1, var3)
    }
}
