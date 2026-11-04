#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_CreateBrush_Indirect extends WMFRecord
pub struct WMF_CreateBrush_Indirect {
    // Java field: WMF_Brush brush
    brush: Option<i32>,
}

impl WMF_CreateBrush_Indirect {
    // Java: WMF_CreateBrush_Indirect()
    pub fn new() -> Self {
        // Java: super();
        WMF_CreateBrush_Indirect {
            brush: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_CreateBrush_Indirect cb = new WMF_CreateBrush_Indirect();
        // Java: cb.brush = new WMF_Brush(wmf);
        // Java: return cb;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(-1, this.brush);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "createBrush_Indirect";
        String::new()
    }

}
