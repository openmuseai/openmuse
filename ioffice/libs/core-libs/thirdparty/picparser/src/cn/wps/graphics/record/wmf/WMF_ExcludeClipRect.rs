#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_ExcludeClipRect extends WMFRecord
pub struct WMF_ExcludeClipRect {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: Path path
    path: Option<i32>,
}

impl WMF_ExcludeClipRect {
    // Java: WMF_ExcludeClipRect()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.path = null;
        WMF_ExcludeClipRect {
            bounds: None,
            path: None,
        }
    }

    // Java: WMF_ExcludeClipRect(Rectangle bounds)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.bounds = bounds;
        WMF_ExcludeClipRect {
            bounds: None,
            path: None,
        }
    }

    // Java: public void add(WMF_ExcludeClipRect clipRect)
    pub fn add(&self, _clipRect: i32) {
        // Java: if (clipRect != null && clipRect.bounds != null)
        {
        // Java: Rectangle bounds = clipRect.bounds;
        // Java: if (this.path == null)
        {
        // Java: this.path = new Path();
        // Java: if (this.bounds != null)
        {
        // Java: this.path.addRect((float)this.bounds.x, (float)this.bounds.y, (float)(this.bounds.x + this.bounds.width), (float)(this.bounds.y + this.bounds.height), Direction.CW);
        }
        }
        // Java: this.path.addRect((float)bounds.x, (float)bounds.y, (float)(bounds.x + bounds.width), (float)(bounds.y + bounds.height), Direction.CW);
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int bottom = wmf.readShort();
        // Java: int right = wmf.readShort();
        // Java: int top = wmf.readShort();
        // Java: int left = wmf.readShort();
        // Java: Rectangle bounds = new Rectangle(left, top, right - left, bottom - top);
        // Java: return new WMF_ExcludeClipRect(bounds);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (this.path != null)
        {
        // Java: renderer.getDrawer().setClip((Path)this.path, 4);
        }
        // Java: else if (this.bounds != null)
        {
        // Java: renderer.getDrawer().setClip((Rectangle)this.bounds, 4);
        }
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "WMF_ExcludeClipRect";
        String::new()
    }

}
