#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_RoundRect extends WMFRecord
pub struct WMF_RoundRect {
    // Java field: int height
    height: i32,
    // Java field: int width
    width: i32,
    // Java field: Rectangle rectangle
    rectangle: Option<i32>,
}

impl WMF_RoundRect {
    // Java: WMF_RoundRect()
    pub fn new() -> Self {
        // Java: super();
        WMF_RoundRect {
            height: 0,
            width: 0,
            rectangle: None,
        }
    }

    // Java: WMF_RoundRect(Rectangle bounds, int width, int height)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.rectangle = bounds;
        // Java: this.width = width;
        // Java: this.height = height;
        WMF_RoundRect {
            height: 0,
            width: 0,
            rectangle: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int height = wmf.readShort();
        // Java: int width = wmf.readShort();
        // Java: int bottomRect = wmf.readShort();
        // Java: int rightRect = wmf.readShort();
        // Java: int topRect = wmf.readShort();
        // Java: int leftRect = wmf.readShort();
        // Java: Rectangle bounds = new Rectangle(leftRect, topRect, rightRect - leftRect, bottomRect - topRect);
        // Java: return new WMF_RoundRect(bounds, width, height);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = new GdiPath(renderer.getDC().getFillType());
        // Java: path.addRoundRect(this.rectangle, (float)this.width, (float)this.height);
        // Java: renderer.addPath(path);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "RoundRect\n rectangle  " + this.rectangle.toString();
        String::new()
    }

}
