#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetROP2 extends EMFRecord
pub struct SetROP2 {
    // Java field: int value
    value: i32,
}

impl SetROP2 {
    // Java: SetROP2()
    pub fn new() -> Self {
        // Java: super();
        SetROP2 {
            value: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetROP2 tag = new SetROP2();
        // Java: tag.value = emf.readUnsignedInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setRop2(this.value);
    }

}
