#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetStretchBltMode extends EMFRecord
pub struct SetStretchBltMode {
    // Java field: int mode
    mode: i32,
}

impl SetStretchBltMode {
    // Java: SetStretchBltMode()
    pub fn new() -> Self {
        // Java: super();
        SetStretchBltMode {
            mode: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetStretchBltMode tag = new SetStretchBltMode();
        // Java: tag.mode = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setStretchMode(this.mode);
    }

}
