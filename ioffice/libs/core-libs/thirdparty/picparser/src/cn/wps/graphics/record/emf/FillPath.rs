#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class FillPath extends EMFRecord
pub struct FillPath {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl FillPath {
    // Java: FillPath()
    pub fn new() -> Self {
        // Java: super();
        FillPath {
            bounds: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: FillPath tag = new FillPath();
        // Java: tag.bounds = emf.readRect();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (renderer.beginningPath())
        {
        // Java: renderer.fillPath(renderer.getDC().getPath());
        // Java: renderer.getDC().setPath((GdiPath)null);
        }
    }

}
