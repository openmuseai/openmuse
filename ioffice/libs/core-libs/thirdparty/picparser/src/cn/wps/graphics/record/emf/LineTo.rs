#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class LineTo extends EMFRecord
pub struct LineTo {
    // Java field: PointF value
    value: Option<i32>,
}

impl LineTo {
    // Java: LineTo()
    pub fn new() -> Self {
        // Java: super();
        LineTo {
            value: None,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: LineTo tag = new LineTo();
        // Java: tag.value = emf.readIntPoint();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.lineTo((int)this.value.x, (int)this.value.y);
    }

}
