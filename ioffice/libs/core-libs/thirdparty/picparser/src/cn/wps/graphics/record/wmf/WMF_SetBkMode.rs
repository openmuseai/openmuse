#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetBkMode extends WMFRecord
pub struct WMF_SetBkMode {
    // Java field: int mode
    mode: i32,
}

impl WMF_SetBkMode {
    // Java: WMF_SetBkMode()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetBkMode {
            mode: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SetBkMode tag = new WMF_SetBkMode();
        // Java: tag.mode = wmf.readUnsignedShort();
        // Java: if (len > 2)
        {
        // Java: wmf.skip(len - 2);
        }
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setBkMode(this.mode);
    }

}
