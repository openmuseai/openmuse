#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Chord extends EMFRecord
pub struct Chord {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: PointF start
    start: Option<i32>,
    // Java field: PointF end
    end: Option<i32>,
}

impl Chord {
    // Java: Chord()
    pub fn new() -> Self {
        // Java: super();
        Chord {
            bounds: None,
            start: None,
            end: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Chord tag = new Chord();
        // Java: tag.bounds = emf.readRect();
        // Java: tag.start = emf.readIntPoint();
        // Java: tag.end = emf.readIntPoint();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = GdiTool.createPath(this.bounds, this.start, this.end);
        // Java: renderer.addPath(path);
    }

}
