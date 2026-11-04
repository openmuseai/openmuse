#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Pie extends EMFRecord
pub struct Pie {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: PointF start
    start: Option<i32>,
    // Java field: PointF end
    end: Option<i32>,
}

impl Pie {
    // Java: Pie()
    pub fn new() -> Self {
        // Java: super();
        Pie {
            bounds: None,
            start: None,
            end: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Pie pie = new Pie();
        // Java: pie.bounds = emf.readRect();
        // Java: pie.start = emf.readIntPoint();
        // Java: pie.end = emf.readIntPoint();
        // Java: return pie;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = GdiTool.createPath(this.bounds, this.start, this.end, true);
        // Java: renderer.addPath(path);
    }

}
