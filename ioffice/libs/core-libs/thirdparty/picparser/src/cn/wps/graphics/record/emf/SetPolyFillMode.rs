#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetPolyFillMode extends EMFRecord
pub struct SetPolyFillMode {
    // Java field: int value
    value: i32,
}

impl SetPolyFillMode {
    // Java: SetPolyFillMode()
    pub fn new() -> Self {
        // Java: super();
        SetPolyFillMode {
            value: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetPolyFillMode tag = new SetPolyFillMode();
        // Java: tag.value = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.setPolyFillMode(this.value);
    }

}
