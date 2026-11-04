#![allow(non_snake_case)]
#![allow(dead_code)]

// Java field: int MAX_CHAR_WIDTH
pub const MAX_CHAR_WIDTH: i32 = 256;

// Java: public interface IWPSFormatter
pub trait IWPSFormatter {
    // Java: abstract void resetLCID(short var1)
    fn resetLCID(&mut self, var1: i16);

    // Java: abstract void destroy()
    fn destroy(&mut self);

}

pub struct IWPSFormatterStub;
