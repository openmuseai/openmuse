#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_SetTextColor extends WMFRecord
pub struct WMF_SetTextColor {
    // Java field: int color
    color: i32,
}

impl WMF_SetTextColor {
    // Java: WMF_SetTextColor()
    pub fn new() -> Self {
        // Java: super();
        WMF_SetTextColor {
            color: 0,
        }
    }

    // Java: WMF_SetTextColor(int color)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.color = color;
        WMF_SetTextColor {
            color: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: return new WMF_SetTextColor(wmf.readColor());
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDrawer().setTextColor(ReColorMap.getReColor(this.color));
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "setTextColor";
        String::new()
    }

}
