#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Polyline extends EMFRecord
pub struct Polyline {
    // Java field: int count
    count: i32,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl Polyline {
    // Java: Polyline()
    pub fn new() -> Self {
        // Java: super();
        Polyline {
            count: 0,
            points: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Polyline poly = new Polyline();
        // Java: emf.skip(16);
        // Java: poly.count = emf.readUnsignedInt();
        // Java: poly.points = emf.readInt(poly.count * 2);
        // Java: return poly;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.polyLine(this.points, this.count);
    }

}
