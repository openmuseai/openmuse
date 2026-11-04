#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_CreateFont_Indirect extends WMFRecord
pub struct WMF_CreateFont_Indirect {
    // Java field: LogFont font
    font: Option<i32>,
}

impl WMF_CreateFont_Indirect {
    // Java: WMF_CreateFont_Indirect()
    pub fn new() -> Self {
        // Java: super();
        WMF_CreateFont_Indirect {
            font: None,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_CreateFont_Indirect cf = new WMF_CreateFont_Indirect();
        // Java: cf.font = new LogFont(wmf, len);
        // Java: return cf;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getGdiObjects().setObject(-1, this.font);
    }

}
