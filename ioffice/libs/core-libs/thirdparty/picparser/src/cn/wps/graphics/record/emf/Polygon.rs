#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Polygon extends EMFRecord
pub struct Polygon {
    // Java field: int count
    count: i32,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl Polygon {
    // Java: Polygon()
    pub fn new() -> Self {
        // Java: super();
        Polygon {
            count: 0,
            points: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Polygon poly = new Polygon();
        // Java: emf.skip(16);
        // Java: poly.count = emf.readUnsignedInt();
        // Java: poly.points = emf.readInt(poly.count * 2);
        // Java: return poly;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.polyGon(this.points, this.count);
    }

}
