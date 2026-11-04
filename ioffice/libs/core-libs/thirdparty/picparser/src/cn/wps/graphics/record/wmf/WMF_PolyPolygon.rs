#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_PolyPolygon extends WMFRecord
pub struct WMF_PolyPolygon {
    // Java field: int[] counts
    counts: Option<Vec<i32>>,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl WMF_PolyPolygon {
    // Java: WMF_PolyPolygon()
    pub fn new() -> Self {
        // Java: super();
        WMF_PolyPolygon {
            counts: None,
            points: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_PolyPolygon poly = new WMF_PolyPolygon();
        // Java: int count = wmf.readUnsignedShort();
        // Java: poly.counts = wmf.readShort(count);
        // Java: int sum = 0;
        // Java: for(int i = 0; i < count; ++i)
        {
        // Java: sum += poly.counts[i];
        }
        // Java: poly.points = wmf.readShort(sum * 2);
        // Java: return poly;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.polyPolyGon(this.points, this.counts, true);
    }

}
