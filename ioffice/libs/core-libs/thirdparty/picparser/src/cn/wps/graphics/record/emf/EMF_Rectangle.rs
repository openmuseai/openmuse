#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class EMF_Rectangle extends EMFRecord
pub struct EMF_Rectangle {
    // Java field: Rectangle value
    value: Option<i32>,
}

impl EMF_Rectangle {
    // Java: EMF_Rectangle()
    pub fn new() -> Self {
        // Java: super();
        EMF_Rectangle {
            value: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: EMF_Rectangle tag = new EMF_Rectangle();
        // Java: tag.value = emf.readRect();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = new GdiPath();
        // Java: path.addRect(this.value);
        // Java: renderer.addPath(path);
    }

}
