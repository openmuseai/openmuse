#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SelectClipPath extends EMFRecord
pub struct SelectClipPath {
    // Java field: int value
    value: i32,
}

impl SelectClipPath {
    // Java: SelectClipPath()
    pub fn new() -> Self {
        // Java: super();
        SelectClipPath {
            value: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SelectClipPath tag = new SelectClipPath();
        // Java: tag.value = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.clipPath(renderer.getDC().getPath(), this.value);
    }

}
