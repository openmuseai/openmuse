#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Polyline extends WMFRecord
pub struct WMF_Polyline {
    // Java field: int count
    count: i32,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl WMF_Polyline {
    // Java: WMF_Polyline()
    pub fn new() -> Self {
        // Java: super();
        WMF_Polyline {
            count: 0,
            points: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_Polyline poly = new WMF_Polyline();
        // Java: poly.count = wmf.readUnsignedShort();
        // Java: poly.points = wmf.readShort(poly.count * 2);
        // Java: return poly;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.polyLine(this.points, this.count);
    }

}
