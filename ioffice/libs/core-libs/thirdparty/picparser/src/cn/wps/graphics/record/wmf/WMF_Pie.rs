#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Pie extends WMFRecord
pub struct WMF_Pie {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: PointF start
    start: Option<i32>,
    // Java field: PointF end
    end: Option<i32>,
}

impl WMF_Pie {
    // Java: WMF_Pie()
    pub fn new() -> Self {
        // Java: super();
        WMF_Pie {
            bounds: None,
            start: None,
            end: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_Pie tag = new WMF_Pie();
        // Java: int yRadial2 = wmf.readShort();
        // Java: int xRadial2 = wmf.readShort();
        // Java: int yRadial1 = wmf.readShort();
        // Java: int xRadial1 = wmf.readShort();
        // Java: int bottomRect = wmf.readShort();
        // Java: int rightRect = wmf.readShort();
        // Java: int topRect = wmf.readShort();
        // Java: int leftRect = wmf.readShort();
        // Java: tag.start = new PointF((float)xRadial1, (float)yRadial1);
        // Java: tag.end = new PointF((float)xRadial2, (float)yRadial2);
        // Java: tag.bounds = new Rectangle(leftRect, topRect, rightRect - leftRect, bottomRect - topRect);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = GdiTool.createPath(this.bounds, this.start, this.end, true);
        // Java: renderer.addPath(path);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "Pie";
        String::new()
    }

}
