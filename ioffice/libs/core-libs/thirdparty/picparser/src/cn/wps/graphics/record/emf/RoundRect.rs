#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class RoundRect extends EMFRecord
pub struct RoundRect {
    // Java field: Rectangle bounds
    bounds: Option<i32>,
    // Java field: int cornerWidth
    cornerWidth: i32,
    // Java field: int cornerHeight
    cornerHeight: i32,
}

impl RoundRect {
    // Java: RoundRect()
    pub fn new() -> Self {
        // Java: super();
        RoundRect {
            bounds: None,
            cornerWidth: 0,
            cornerHeight: 0,
        }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: RoundRect tag = new RoundRect();
        // Java: tag.bounds = emf.readRect();
        // Java: tag.cornerWidth = emf.readInt();
        // Java: tag.cornerHeight = emf.readInt();
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: GdiPath path = new GdiPath();
        // Java: path.addRoundRect(this.bounds, (float)this.cornerWidth / 2.0F, (float)this.cornerHeight / 2.0F);
        // Java: renderer.addPath(path);
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return super.toString() + "(" + this.bounds.toString() + ", " + this.cornerWidth + ", " + this.cornerHeight + ")";
        String::new()
    }

}
