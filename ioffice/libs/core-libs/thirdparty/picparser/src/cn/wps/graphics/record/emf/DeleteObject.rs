#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class DeleteObject extends EMFRecord
pub struct DeleteObject {
    // Java field: int index
    index: i32,
}

impl DeleteObject {
    // Java: DeleteObject()
    pub fn new() -> Self {
        // Java: super();
        DeleteObject {
            index: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: DeleteObject tag = new DeleteObject();
        // Java: tag.index = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().deleteObject(this.index);
    }

}
