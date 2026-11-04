#![allow(non_snake_case, dead_code)]

pub struct GdiPainter;

// Java: public interface IGdiObject
pub trait IGdiObject {
    // Java: abstract void render(GdiPainter var1)
    fn render(&self, var1: &GdiPainter);
}
