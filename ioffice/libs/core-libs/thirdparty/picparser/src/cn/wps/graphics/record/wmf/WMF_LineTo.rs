#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_LineTo extends WMFRecord
pub struct WMF_LineTo {
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
}

impl WMF_LineTo {
    // Java: WMF_LineTo()
    pub fn new() -> Self {
        // Java: super();
        WMF_LineTo {
            x: 0,
            y: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_LineTo lineTo = new WMF_LineTo();
        // Java: lineTo.y = wmf.readShort();
        // Java: lineTo.x = wmf.readShort();
        // Java: return lineTo;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.lineTo(this.x, this.y);
    }

}
