#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_CreatePatternBrush extends WMFRecord
pub struct WMF_CreatePatternBrush {
    // Java field: WMF_PatternBrush brush
    brush: Option<i32>,
}

impl WMF_CreatePatternBrush {
    // Java: WMF_CreatePatternBrush()
    pub fn new() -> Self {
        // Java: super();
        WMF_CreatePatternBrush {
            brush: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_CreatePatternBrush cpb = new WMF_CreatePatternBrush();
        // Java: wmf.skip(len);
        // Java: cpb.brush = new WMF_PatternBrush();
        // Java: return cpb;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(-1, this.brush);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "createPatternBrush";
        String::new()
    }

}
