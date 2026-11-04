#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class ScaleViewportExtEx extends EMFRecord
pub struct ScaleViewportExtEx {
    // Java field: int xNum
    xNum: i32,
    // Java field: int xDenom
    xDenom: i32,
    // Java field: int yNum
    yNum: i32,
    // Java field: int yDenom
    yDenom: i32,
}

impl ScaleViewportExtEx {
    // Java: ScaleViewportExtEx()
    pub fn new() -> Self {
        // Java: super();
        ScaleViewportExtEx {
            xNum: 0,
            xDenom: 0,
            yNum: 0,
            yDenom: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: ScaleViewportExtEx scale = new ScaleViewportExtEx();
        // Java: scale.xNum = emf.readInt();
        // Java: scale.xDenom = emf.readInt();
        // Java: scale.yNum = emf.readInt();
        // Java: scale.yDenom = emf.readInt();
        // Java: return scale;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getContext().scaleViewportSize(this.xNum, this.xDenom, this.yNum, this.yDenom);
    }

}
