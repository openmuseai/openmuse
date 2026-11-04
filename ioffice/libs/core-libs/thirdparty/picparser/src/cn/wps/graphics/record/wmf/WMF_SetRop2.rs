#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetRop2 extends WMFRecord
pub struct WMF_SetRop2 {
    // Java field: int rop
    rop: i32,
}

impl WMF_SetRop2 {
    // Java: WMF_SetRop2()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetRop2 {
            rop: 0,
        }
    }

    // Java: WMF_SetRop2(int rop)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.rop = rop;
        WMF_SetRop2 {
            rop: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int rop = wmf.readUnsignedShort();
        // Java: if (len > 2)
        {
        // Java: wmf.skip(len - 2);
        }
        // Java: return new WMF_SetRop2(rop);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setRop2(this.rop);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "SetRop2";
        String::new()
    }

}
