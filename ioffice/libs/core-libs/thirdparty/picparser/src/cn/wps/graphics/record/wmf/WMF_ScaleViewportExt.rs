#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_ScaleViewportExt extends WMFRecord
pub struct WMF_ScaleViewportExt {
    // Java field: int yDenom
    yDenom: i32,
    // Java field: int yNum
    yNum: i32,
    // Java field: int xDenom
    xDenom: i32,
    // Java field: int xNum
    xNum: i32,
}

impl WMF_ScaleViewportExt {
    // Java: WMF_ScaleViewportExt()
    pub fn new() -> Self {
        // Java: super();
        WMF_ScaleViewportExt {
            yDenom: 0,
            yNum: 0,
            xDenom: 0,
            xNum: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: WMF_ScaleViewportExt tag = new WMF_ScaleViewportExt();
        // Java: tag.yDenom = wmf.readShort();
        // Java: tag.yNum = wmf.readShort();
        // Java: tag.xDenom = wmf.readShort();
        // Java: tag.xNum = wmf.readShort();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().scaleViewportSize(this.xNum, this.xDenom, this.yNum, this.yDenom);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.xNum + ", " + this.xDenom + " - " + this.yNum + ", " + this.yDenom + ")";
        String::new()
    }

}
