#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code)]

use std::sync::Mutex;

// Java field: int ETO_OPAQUE
pub static ETO_OPAQUE: Mutex<i32> = Mutex::new(2);
// Java field: int ETO_CLIPPED
pub static ETO_CLIPPED: Mutex<i32> = Mutex::new(4);
// Java field: int ETO_GLYPH_INDEX
pub static ETO_GLYPH_INDEX: Mutex<i32> = Mutex::new(16);
// Java field: int ETO_RTLREADING
pub static ETO_RTLREADING: Mutex<i32> = Mutex::new(128);
// Java field: int ETO_NUMERICSLOCAL
pub static ETO_NUMERICSLOCAL: Mutex<i32> = Mutex::new(1024);
// Java field: int ETO_NUMERICSLATIN
pub static ETO_NUMERICSLATIN: Mutex<i32> = Mutex::new(2048);
// Java field: int ETO_PDY
pub static ETO_PDY: Mutex<i32> = Mutex::new(8192);

// Java: public class WMF_ExtTextOutOptions
pub struct WMF_ExtTextOutOptions {
}

// Java: public class WMF_ExtTextOutOptions
impl WMF_ExtTextOutOptions {
    // Java: WMF_ExtTextOutOptions()
    pub fn new() -> Self {
        // Java: super();
        WMF_ExtTextOutOptions {}
    }
}
