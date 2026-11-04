#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_IntersectClipRect extends WMFRecord
pub struct WMF_IntersectClipRect {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl WMF_IntersectClipRect {
    // Java: WMF_IntersectClipRect()
    pub fn new() -> Self {
        // Java: super();
        WMF_IntersectClipRect {
            bounds: None,
        }
    }

    // Java: WMF_IntersectClipRect(Rectangle bounds)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.bounds = bounds;
        WMF_IntersectClipRect {
            bounds: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int bottom = wmf.readShort();
        // Java: int right = wmf.readShort();
        // Java: int top = wmf.readShort();
        // Java: int left = wmf.readShort();
        // Java: Rectangle bounds = new Rectangle(left, top, right - left, bottom - top);
        // Java: return new WMF_IntersectClipRect(bounds);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().clip(this.bounds);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "IntersectClipRect";
        String::new()
    }

}
