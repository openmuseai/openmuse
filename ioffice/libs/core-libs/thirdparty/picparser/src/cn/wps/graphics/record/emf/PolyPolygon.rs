#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class PolyPolygon extends EMFRecord
pub struct PolyPolygon {
    // Java field: int[] counts
    counts: Option<Vec<i32>>,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl PolyPolygon {
    // Java: PolyPolygon()
    pub fn new() -> Self {
        // Java: super();
        PolyPolygon {
            counts: None,
            points: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: PolyPolygon poly = new PolyPolygon();
        // Java: emf.skip(16);
        // Java: int count = emf.readUnsignedInt();
        // Java: emf.skip(4);
        // Java: poly.counts = emf.readUnsignedInt(count);
        // Java: int sum = 0;
        // Java: for(int i = 0; i < count; ++i)
        {
        // Java: sum += poly.counts[i];
        }
        // Java: poly.points = emf.readInt(sum * 2);
        // Java: return poly;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (null != this.points && null != this.counts)
        {
        // Java: renderer.polyPolyGon(this.points, this.counts, true);
        }
    }

}
