#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetViewportOrgEx extends EMFRecord
pub struct SetViewportOrgEx {
    // Java field: float dx
    dx: f32,
    // Java field: float dy
    dy: f32,
}

impl SetViewportOrgEx {
    // Java: SetViewportOrgEx()
    pub fn new() -> Self {
        // Java: super();
        SetViewportOrgEx {
            dx: 0.0,
            dy: 0.0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetViewportOrgEx tag = new SetViewportOrgEx();
        // Java: tag.dx = (float)emf.readInt();
        // Java: tag.dy = (float)emf.readInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().setViewportOffset(this.dx, this.dy);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.dx + ", " + this.dy + ")";
        String::new()
    }

}
