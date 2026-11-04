#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_MoveTo extends WMFRecord
pub struct WMF_MoveTo {
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
}

impl WMF_MoveTo {
    // Java: WMF_MoveTo()
    pub fn new() -> Self {
        // Java: super();
        WMF_MoveTo {
            x: 0,
            y: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_MoveTo moveTo = new WMF_MoveTo();
        // Java: moveTo.y = wmf.readShort();
        // Java: moveTo.x = wmf.readShort();
        // Java: return moveTo;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.moveTo(this.x, this.y);
    }

}
