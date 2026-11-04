#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetBkColor extends WMFRecord
pub struct WMF_SetBkColor {
    // Java field: int color
    color: i32,
}

impl WMF_SetBkColor {
    // Java: WMF_SetBkColor()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetBkColor {
            color: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SetBkColor tag = new WMF_SetBkColor();
        // Java: tag.color = wmf.readColor();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setBkColor(this.color);
    }

}
