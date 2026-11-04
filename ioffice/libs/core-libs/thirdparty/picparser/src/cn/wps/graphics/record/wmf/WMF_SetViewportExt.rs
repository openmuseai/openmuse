#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetViewportExt extends WMFRecord
pub struct WMF_SetViewportExt {
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
}

impl WMF_SetViewportExt {
    // Java: WMF_SetViewportExt()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetViewportExt {
            width: 0,
            height: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SetViewportExt tag = new WMF_SetViewportExt();
        // Java: tag.height = wmf.readShort();
        // Java: tag.width = wmf.readShort();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setViewportSize(this.width, this.height);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.width + ", " + this.height + ")";
        String::new()
    }

    // Java: public Size getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return new Size((float)this.width, (float)this.height);
        0
    }

}
