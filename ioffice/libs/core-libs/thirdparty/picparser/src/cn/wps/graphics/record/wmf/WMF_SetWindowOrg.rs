#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetWindowOrg extends WMFRecord
pub struct WMF_SetWindowOrg {
    // Java field: float dx
    dx: f32,
    // Java field: float dy
    dy: f32,
}

impl WMF_SetWindowOrg {
    // Java: WMF_SetWindowOrg()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetWindowOrg {
            dx: 0.0,
            dy: 0.0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SetWindowOrg tag = new WMF_SetWindowOrg();
        // Java: tag.dy = (float)wmf.readShort();
        // Java: tag.dx = (float)wmf.readShort();
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
