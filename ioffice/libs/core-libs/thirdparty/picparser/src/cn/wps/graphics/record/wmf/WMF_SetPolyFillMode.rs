#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetPolyFillMode extends WMFRecord
pub struct WMF_SetPolyFillMode {
    // Java field: int mode
    mode: i32,
    // Java field: int spare1
    spare1: i32,
}

impl WMF_SetPolyFillMode {
    // Java: WMF_SetPolyFillMode()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetPolyFillMode {
            mode: 0,
            spare1: 0,
        }
    }

    // Java: WMF_SetPolyFillMode(int mode, int spare1)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.mode = mode;
        // Java: this.spare1 = spare1;
        WMF_SetPolyFillMode {
            mode: 0,
            spare1: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int mode = wmf.readUnsignedShort();
        // Java: int spare1 = len == 4 ? wmf.readUnsignedShort() : 0;
        // Java: return new WMF_SetPolyFillMode(mode, spare1);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.setPolyFillMode(this.mode);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "SetPolyFillMode";
        String::new()
    }

}
