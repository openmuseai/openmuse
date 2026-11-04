#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class WMF_ScaleWindowExt extends WMFRecord
pub struct WMF_ScaleWindowExt {
    // Java field: int yDenom
    yDenom: i32,
    // Java field: int yNum
    yNum: i32,
    // Java field: int xDenom
    xDenom: i32,
    // Java field: int xNum
    xNum: i32,
}

impl WMF_ScaleWindowExt {
    // Java: WMF_ScaleWindowExt()
    pub fn new() -> Self {
        // Java: super();
        WMF_ScaleWindowExt {
            yDenom: 0,
            yNum: 0,
            xDenom: 0,
            xNum: 0,
        }
    }

    // Java: WMF_ScaleWindowExt(int yDenom, int yNum, int xDenom, int xNum)
    pub fn new_2() -> Self {
        // Java: this();
        // Java: this.yDenom = yDenom;
        // Java: this.yNum = yNum;
        // Java: this.xDenom = xDenom;
        // Java: this.xNum = xNum;
        WMF_ScaleWindowExt {
            yDenom: 0,
            yNum: 0,
            xDenom: 0,
            xNum: 0,
        }
    }

    // Java: public WMFRecord read(WMFParser wmf, int len)
    pub fn read(&self, _wmf: i32, _len: i32) -> i32 {
        // Java: int yDenom = wmf.readShort();
        // Java: int yNum = wmf.readShort();
        // Java: int xDenom = wmf.readShort();
        // Java: int xNum = wmf.readShort();
        // Java: return new WMF_ScaleWindowExt(yDenom, yNum, xDenom, xNum);
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().scaleWindowSize(this.xNum, this.xDenom, this.yNum, this.yDenom);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return "ScaleWindowExt";
        String::new()
    }

}
