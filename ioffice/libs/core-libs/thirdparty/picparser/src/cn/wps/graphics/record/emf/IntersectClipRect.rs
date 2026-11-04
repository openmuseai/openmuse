#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class IntersectClipRect extends EMFRecord
pub struct IntersectClipRect {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl IntersectClipRect {
    // Java: IntersectClipRect()
    pub fn new() -> Self {
        // Java: super();
        IntersectClipRect {
            bounds: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: IntersectClipRect tag = new IntersectClipRect();
        // Java: tag.bounds = emf.readRect();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().clip(this.bounds);
    }

}
