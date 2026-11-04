#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class StrokePath extends EMFRecord
pub struct StrokePath {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl StrokePath {
    // Java: StrokePath()
    pub fn new() -> Self {
        // Java: super();
        StrokePath {
            bounds: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: StrokePath tag = new StrokePath();
        // Java: tag.bounds = emf.readRect();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.drawPath(renderer.getDC().getPath());
        // Java: renderer.getDC().setPath((GdiPath)null);
    }

}
