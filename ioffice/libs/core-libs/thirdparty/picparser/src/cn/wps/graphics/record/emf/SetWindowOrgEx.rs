#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetWindowOrgEx extends EMFRecord
pub struct SetWindowOrgEx {
    // Java field: float dx
    dx: f32,
    // Java field: float dy
    dy: f32,
}

impl SetWindowOrgEx {
    // Java: SetWindowOrgEx()
    pub fn new() -> Self {
        // Java: super();
        SetWindowOrgEx {
            dx: 0.0,
            dy: 0.0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetWindowOrgEx tag = new SetWindowOrgEx();
        // Java: tag.dx = (float)emf.readInt();
        // Java: tag.dy = (float)emf.readInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setWindowOffset(this.dx, this.dy);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.dx + ", " + this.dy + ")";
        String::new()
    }

}
