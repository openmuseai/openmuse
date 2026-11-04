#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class EOF extends EMFRecord
pub struct EOF {
    _private: (),
}

impl EOF {
    // Java: EOF()
    pub fn new() -> Self {
        // Java: super();
        EOF { _private: () }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: return null;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
    }

}
