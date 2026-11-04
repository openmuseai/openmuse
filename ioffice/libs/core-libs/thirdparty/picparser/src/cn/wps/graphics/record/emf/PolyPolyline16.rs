#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class PolyPolyline16 extends EMFRecord
pub struct PolyPolyline16 {
    // Java field: int[] counts
    counts: Option<Vec<i32>>,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl PolyPolyline16 {
    // Java: PolyPolyline16()
    pub fn new() -> Self {
        // Java: super();
        PolyPolyline16 {
            counts: None,
            points: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: PolyPolyline16 polyline16 = new PolyPolyline16();
        // Java: emf.skip(16);
        // Java: int count = emf.readUnsignedInt();
        // Java: emf.skip(4);
        // Java: polyline16.counts = emf.readUnsignedInt(count);
        // Java: int sum = 0;
        // Java: for(int i = 0; i < count; ++i)
        {
        // Java: sum += polyline16.counts[i];
        }
        // Java: polyline16.points = emf.readShort(sum * 2);
        // Java: return polyline16;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.polyPolyGon(this.points, this.counts, false);
    }

}
