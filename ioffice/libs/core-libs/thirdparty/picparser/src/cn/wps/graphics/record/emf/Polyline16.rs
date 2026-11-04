#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Polyline16 extends EMFRecord
pub struct Polyline16 {
    // Java field: int count
    count: i32,
    // Java field: int[] points
    points: Option<Vec<i32>>,
}

impl Polyline16 {
    // Java: Polyline16()
    pub fn new() -> Self {
        // Java: super();
        Polyline16 {
            count: 0,
            points: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Polyline16 poly16 = new Polyline16();
        // Java: emf.skip(16);
        // Java: poly16.count = emf.readUnsignedInt();
        // Java: poly16.points = emf.readShort(poly16.count * 2);
        // Java: return poly16;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.polyLine(this.points, this.count);
    }

}
