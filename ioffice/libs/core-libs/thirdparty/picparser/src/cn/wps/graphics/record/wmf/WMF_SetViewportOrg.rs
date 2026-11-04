#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetViewportOrg extends WMFRecord
pub struct WMF_SetViewportOrg {
    // Java field: float dx
    dx: f32,
    // Java field: float dy
    dy: f32,
}

impl WMF_SetViewportOrg {
    // Java: WMF_SetViewportOrg()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetViewportOrg {
            dx: 0.0,
            dy: 0.0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_SetViewportOrg tag = new WMF_SetViewportOrg();
        // Java: tag.dy = (float)wmf.readShort();
        // Java: tag.dx = (float)wmf.readShort();
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
