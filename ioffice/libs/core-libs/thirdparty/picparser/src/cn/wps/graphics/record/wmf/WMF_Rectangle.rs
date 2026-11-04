#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Rectangle extends WMFRecord
pub struct WMF_Rectangle {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl WMF_Rectangle {
    // Java: WMF_Rectangle()
    pub fn new() -> Self {
        // Java: super();
        WMF_Rectangle {
            bounds: None,
        }
    }

    // Java: WMF_Rectangle(Rectangle rectangle)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.bounds = rectangle;
        WMF_Rectangle {
            bounds: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int bottomRect = wmf.readShort();
        // Java: int rightRect = wmf.readShort();
        // Java: int topRect = wmf.readShort();
        // Java: int leftRect = wmf.readShort();
        // Java: Rectangle bounds = new Rectangle(leftRect, topRect, rightRect - leftRect, bottomRect - topRect);
        // Java: return new WMF_Rectangle(bounds);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = new GdiPath(renderer.getDC().getFillType());
        // Java: path.addRect(this.bounds);
        // Java: renderer.addPath(path);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "Rectangle\n " + this.bounds.toString();
        String::new()
    }

}
