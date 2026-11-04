#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Arc extends WMFRecord
pub struct WMF_Arc {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: PointF start
    start: Option<i32>,
    // Java field: PointF end
    end: Option<i32>,
}

impl WMF_Arc {
    // Java: WMF_Arc()
    pub fn new() -> Self {
        // Java: super();
        WMF_Arc {
            bounds: None,
            start: None,
            end: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_Arc arc = new WMF_Arc();
        // Java: int yEnd = wmf.readShort();
        // Java: int xEnd = wmf.readShort();
        // Java: arc.end = new PointF((float)xEnd, (float)yEnd);
        // Java: int yStart = wmf.readShort();
        // Java: int xStart = wmf.readShort();
        // Java: arc.start = new PointF((float)xStart, (float)yStart);
        // Java: arc.bounds = wmf.readRect();
        // Java: return arc;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.addPath(GdiTool.createPath(this.bounds, this.end, this.start), false);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "WMF_Arc";
        String::new()
    }

}
