#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_CreatePenIndirect extends WMFRecord
pub struct WMF_CreatePenIndirect {
    // Java field: WMF_Pen pen
    pen: Option<i32>,
}

impl WMF_CreatePenIndirect {
    // Java: WMF_CreatePenIndirect()
    pub fn new() -> Self {
        // Java: super();
        WMF_CreatePenIndirect {
            pen: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_CreatePenIndirect cfi = new WMF_CreatePenIndirect();
        // Java: cfi.pen = new WMF_Pen(wmf);
        // Java: if (len == 12)
        {
        // Java: wmf.skip(2);
        }
        // Java: return cfi;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(-1, this.pen);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "CreatePenIndirect";
        String::new()
    }

}
