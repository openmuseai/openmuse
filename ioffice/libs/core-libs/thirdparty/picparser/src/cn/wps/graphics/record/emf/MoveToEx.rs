#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class MoveToEx extends EMFRecord
pub struct MoveToEx {
    // Java field: int x
    x: i32,
    // Java field: int y
    y: i32,
}

impl MoveToEx {
    // Java: MoveToEx()
    pub fn new() -> Self {
        // Java: super();
        MoveToEx {
            x: 0,
            y: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: MoveToEx tag = new MoveToEx();
        // Java: tag.x = emf.readInt();
        // Java: tag.y = emf.readInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.moveTo(this.x, this.y);
    }

}
