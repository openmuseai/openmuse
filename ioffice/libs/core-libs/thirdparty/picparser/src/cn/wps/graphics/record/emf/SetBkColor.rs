#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetBkColor extends EMFRecord
pub struct SetBkColor {
    // Java field: int color
    color: i32,
}

impl SetBkColor {
    // Java: SetBkColor()
    pub fn new() -> Self {
        // Java: super();
        SetBkColor {
            color: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetBkColor tag = new SetBkColor();
        // Java: tag.color = emf.readColor();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setBkColor(this.color);
    }

}
