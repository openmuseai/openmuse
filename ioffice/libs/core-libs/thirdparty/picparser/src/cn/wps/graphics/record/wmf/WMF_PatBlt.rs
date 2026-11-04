#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_PatBlt extends WMFRecord
pub struct WMF_PatBlt {
    // Java field: int rop
    rop: i32,
    // Java field: Rectangle bounds
    bounds: Option<i32>,
}

impl WMF_PatBlt {
    // Java: WMF_PatBlt()
    pub fn new() -> Self {
        // Java: super();
        WMF_PatBlt {
            rop: 0,
            bounds: None,
        }
    }

    // Java: WMF_PatBlt(int rop, Rectangle bounds)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.rop = rop;
        // Java: this.bounds = bounds;
        WMF_PatBlt {
            rop: 0,
            bounds: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int rop = wmf.readUnsignedInt();
        // Java: int height = wmf.readShort();
        // Java: int width = wmf.readShort();
        // Java: int yLeft = wmf.readShort();
        // Java: int xLeft = wmf.readShort();
        // Java: Rectangle bounds = new Rectangle(xLeft, yLeft, width, height);
        // Java: return new WMF_PatBlt(rop, bounds);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: if (this.rop == 15728673)
        {
        // Java: GdiPath path = new GdiPath(renderer.getDC().getFillType());
        // Java: path.addRect(this.bounds);
        // Java: renderer.fillPath(path);
        }
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "PatBlt";
        String::new()
    }

}
