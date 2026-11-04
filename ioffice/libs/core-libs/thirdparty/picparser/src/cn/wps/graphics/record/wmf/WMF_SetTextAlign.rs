#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetTextAlign extends WMFRecord
pub struct WMF_SetTextAlign {
    // Java field: int textAlign
    textAlign: i32,
}

impl WMF_SetTextAlign {
    // Java: WMF_SetTextAlign()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetTextAlign {
            textAlign: 0,
        }
    }

    // Java: WMF_SetTextAlign(int textAlign)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.textAlign = textAlign;
        WMF_SetTextAlign {
            textAlign: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: this.textAlign = wmf.readUnsignedShort();
        // Java: if (len == 4)
        {
        // Java: wmf.readUnsignedShort();
        }
        // Java: return new WMF_SetTextAlign(this.textAlign);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setTextAlign(this.textAlign);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "SetTextAlign";
        String::new()
    }

}
