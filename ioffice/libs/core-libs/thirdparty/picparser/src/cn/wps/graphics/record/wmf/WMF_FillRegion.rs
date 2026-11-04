#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_FillRegion extends WMFRecord
pub struct WMF_FillRegion {
    // Java field: int region
    region: i32,
    // Java field: int brush
    brush: i32,
}

impl WMF_FillRegion {
    // Java: WMF_FillRegion()
    pub fn new() -> Self {
        // Java: super();
        WMF_FillRegion {
            region: 0,
            brush: 0,
        }
    }

    // Java: WMF_FillRegion(int region, int brush)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.region = region;
        // Java: this.brush = brush;
        WMF_FillRegion {
            region: 0,
            brush: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int region = wmf.readUnsignedShort();
        // Java: int brush = wmf.readUnsignedShort();
        // Java: return new WMF_FillRegion(region, brush);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "FillRegion";
        String::new()
    }

}
