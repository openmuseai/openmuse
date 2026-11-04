#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_EOF extends WMFRecord
pub struct WMF_EOF {
    _private: (),
}

impl WMF_EOF {
    // Java: WMF_EOF()
    pub fn new() -> Self {
        // Java: super();
        WMF_EOF { _private: () }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: return null;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
    }

}
