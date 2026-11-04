#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class LOGL extends TIFFImageType
pub struct LOGL {
    _private: (),
}

impl LOGL {
    // Java: LOGL(TIFFImage tiff)
    pub fn new() -> Self {
        // Java: super(tiff);
        LOGL { _private: () }
    }

    // Java: public void rendered(TIFFTile tile)
    pub fn rendered(&self, _tile: i32) {
    }
}
