#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetWindowExt extends WMFRecord
pub struct WMF_SetWindowExt {
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
}

impl WMF_SetWindowExt {
    // Java: WMF_SetWindowExt()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetWindowExt {
            width: 0,
            height: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SetWindowExt tag = new WMF_SetWindowExt();
        // Java: tag.height = wmf.readShort();
        // Java: tag.width = wmf.readShort();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setWindowSize(Math.abs(this.width), this.height);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.width + ", " + this.height + ")";
        String::new()
    }

    // Java: public Size getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return new Size((float)Math.abs(this.width), (float)Math.abs(this.height));
        0
    }

}
