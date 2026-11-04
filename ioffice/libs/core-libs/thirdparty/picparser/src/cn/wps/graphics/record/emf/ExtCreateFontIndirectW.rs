#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ExtCreateFontIndirectW extends EMFRecord
pub struct ExtCreateFontIndirectW {
    // Java field: int index
    index: i32,
    // Java field: LogFont font
    font: Option<i32>,
}

impl ExtCreateFontIndirectW {
    // Java: ExtCreateFontIndirectW()
    pub fn new() -> Self {
        // Java: super();
        ExtCreateFontIndirectW {
            index: 0,
            font: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: ExtCreateFontIndirectW tag = new ExtCreateFontIndirectW();
        // Java: tag.index = emf.readUnsignedInt();
        // Java: tag.font = new LogFont(emf, size - 4);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(this.index, this.font);
    }

}
