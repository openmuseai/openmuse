#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class CreateBrushIndirect extends EMFRecord
pub struct CreateBrushIndirect {
    // Java field: int index
    index: i32,
    // Java field: GDILogBrush brush
    brush: Option<i32>,
}

impl CreateBrushIndirect {
    // Java: CreateBrushIndirect()
    pub fn new() -> Self {
        // Java: super();
        CreateBrushIndirect {
            index: 0,
            brush: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: CreateBrushIndirect tag = new CreateBrushIndirect();
        // Java: tag.index = emf.readUnsignedInt();
        // Java: tag.brush = GDILogBrush.readLogBrush(emf);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(this.index, this.brush);
    }

}
