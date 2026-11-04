#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ExtCreatePen extends EMFRecord
pub struct ExtCreatePen {
    // Java field: int index
    index: i32,
    // Java field: GdiPen pen
    pen: Option<i32>,
}

impl ExtCreatePen {
    // Java: ExtCreatePen()
    pub fn new() -> Self {
        // Java: super();
        ExtCreatePen {
            index: 0,
            pen: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: ExtCreatePen tag = new ExtCreatePen();
        // Java: tag.index = emf.readUnsignedInt();
        // Java: emf.skip(16);
        // Java: tag.pen = GdiPen.readExtLogPen(emf);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(this.index, this.pen);
    }

}
