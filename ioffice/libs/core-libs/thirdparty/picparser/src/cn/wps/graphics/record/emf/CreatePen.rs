#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class CreatePen extends EMFRecord
pub struct CreatePen {
    // Java field: int index
    index: i32,
    // Java field: GdiPen pen
    pen: Option<i32>,
}

impl CreatePen {
    // Java: CreatePen()
    pub fn new() -> Self {
        // Java: super();
        CreatePen {
            index: 0,
            pen: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: CreatePen tag = new CreatePen();
        // Java: tag.index = emf.readUnsignedInt();
        // Java: tag.pen = GdiPen.readLogPen(emf);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(this.index, this.pen);
    }

}
