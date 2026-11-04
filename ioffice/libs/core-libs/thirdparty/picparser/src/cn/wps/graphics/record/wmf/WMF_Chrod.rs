#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_Chrod extends WMFRecord
pub struct WMF_Chrod {
    // Java field: PointF radial1
    radial1: Option<i32>,
    // Java field: PointF radial2
    radial2: Option<i32>,
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl WMF_Chrod {
    // Java: WMF_Chrod()
    pub fn new() -> Self {
        // Java: super();
        WMF_Chrod {
            radial1: None,
            radial2: None,
            bounds: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_Chrod chrod = new WMF_Chrod();
        // Java: int yRadial2 = wmf.readShort();
        // Java: int xRadial2 = wmf.readShort();
        // Java: chrod.radial2 = new PointF((float)xRadial2, (float)yRadial2);
        // Java: int yRadial1 = wmf.readShort();
        // Java: int xRadial1 = wmf.readShort();
        // Java: chrod.radial1 = new PointF((float)xRadial1, (float)yRadial1);
        // Java: chrod.bounds = wmf.readRect();
        // Java: return chrod;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.addPath(GdiTool.createPath(this.bounds, this.radial1, this.radial2));
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "Chrod";
        String::new()
    }

}
