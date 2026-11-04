#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SelectObject extends EMFRecord
pub struct SelectObject {
    // Java field: int index
    index: i32,
}

impl SelectObject {
    // Java: SelectObject()
    pub fn new() -> Self {
        // Java: super();
        SelectObject {
            index: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SelectObject tag = new SelectObject();
        // Java: tag.index = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.selectObject(this.index);
    }

}
