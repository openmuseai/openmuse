#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_RestoreDC extends WMFRecord
pub struct WMF_RestoreDC {
    // Java field: int index
    index: i32,
}

impl WMF_RestoreDC {
    // Java: WMF_RestoreDC()
    pub fn new() -> Self {
        // Java: super();
        WMF_RestoreDC {
            index: 0,
        }
    }

    // Java: WMF_RestoreDC(int index)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.index = index;
        WMF_RestoreDC {
            index: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int index = wmf.readShort();
        // Java: return new WMF_RestoreDC(index);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.restoreDC();
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "RestoreDC";
        String::new()
    }

}
