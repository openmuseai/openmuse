#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class LOGLUV extends TIFFImageType
pub struct LOGLUV {
    _private: (),
}

impl LOGLUV {
    // Java: LOGLUV(TIFFImage tiff)
    pub fn new() -> Self {
        // Java: super(tiff);
        LOGLUV { _private: () }
    }

    // Java: public void rendered(TIFFTile tile)
    pub fn rendered(&self, _tile: i32) {
    }
}
