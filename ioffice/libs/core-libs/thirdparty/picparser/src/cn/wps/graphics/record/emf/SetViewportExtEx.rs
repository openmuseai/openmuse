#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetViewportExtEx extends EMFRecord
pub struct SetViewportExtEx {
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
}

impl SetViewportExtEx {
    // Java: SetViewportExtEx()
    pub fn new() -> Self {
        // Java: super();
        SetViewportExtEx {
            width: 0,
            height: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetViewportExtEx tag = new SetViewportExtEx();
        // Java: tag.width = emf.readInt();
        // Java: tag.height = emf.readInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setViewportSize(this.width, this.height);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.width + ", " + this.height + ")";
        String::new()
    }

}
