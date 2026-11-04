#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class BeginPath extends EMFRecord
pub struct BeginPath {
    _private: (),
}

impl BeginPath {
    // Java: BeginPath()
    pub fn new() -> Self {
        // Java: super();
        BeginPath { _private: () }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: emf.skip(size);
        // Java: return this;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.beginPath();
    }

}
