#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class AbortPath extends EMFRecord
pub struct AbortPath {
    _private: (),
}

impl AbortPath {
    // Java: AbortPath()
    pub fn new() -> Self {
        // Java: super();
        AbortPath { _private: () }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: return new AbortPath();
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.getDC().abortPath();
    }

}
