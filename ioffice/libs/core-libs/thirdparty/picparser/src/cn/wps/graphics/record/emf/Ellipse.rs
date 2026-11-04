#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class Ellipse extends EMFRecord
pub struct Ellipse {
    // Java field: Rectangle value
    value: Option<i32>,
}

impl Ellipse {
    // Java: Ellipse()
    pub fn new() -> Self {
        // Java: super();
        Ellipse {
            value: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: Ellipse ellipse = new Ellipse();
        // Java: ellipse.value = emf.readRect();
        // Java: return ellipse;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = new GdiPath();
        // Java: path.arcTo(this.value);
        // Java: renderer.addPath(path);
    }

}
