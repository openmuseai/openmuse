#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class SetWindowExtEx extends EMFRecord
pub struct SetWindowExtEx {
    // Java field: int width
    width: i32,
    // Java field: int height
    height: i32,
}

impl SetWindowExtEx {
    // Java: SetWindowExtEx()
    pub fn new() -> Self {
        // Java: super();
        SetWindowExtEx {
            width: 0,
            height: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: SetWindowExtEx tag = new SetWindowExtEx();
        // Java: tag.width = emf.readInt();
        // Java: tag.height = emf.readInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: int mapMode = renderer.getContext().getMapMode();
        // Java: if (mapMode == 8)
        {
        // Java: renderer.getContext().setWindowSize(this.width, this.height);
        }
        // Java: else
        {
        // Java: renderer.getContext().setWindowSize(Math.abs(this.width), Math.abs(this.height));
        }
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.width + ", " + this.height + ")";
        String::new()
    }

}
