#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class RestoreDC extends EMFRecord
pub struct RestoreDC {
    _private: (),
}

impl RestoreDC {
    // Java: RestoreDC()
    pub fn new() -> Self {
        // Java: super();
        RestoreDC { _private: () }
    }

    // Java: public EMFRecord read(EMFParser emf, int size)
    pub fn read(&self, _emf: i32, _size: i32) -> i32 {
        // Java: RestoreDC tag = new RestoreDC();
        // Java: emf.skip(4);
        // Java: return tag;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
        // Java: renderer.restoreDC();
    }

}
