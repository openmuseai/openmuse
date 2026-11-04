#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class DefaultRecord extends MetaFileRecord
pub struct DefaultRecord {
    _private: (),
}

impl DefaultRecord {
    // Java: DefaultRecord()
    pub fn new() -> Self {
        // Java: super();
        DefaultRecord { _private: () }
    }

    // Java: public MetaFileRecord read(MetaFileParser parser, int len)
    pub fn read(&self, _parser: i32, _len: i32) -> i32 {
        // Java: parser.skip(len);
        // Java: return this;
        0
    }

    // Java: public void render(MetaFilePainter renderer)
    pub fn render(&self, _renderer: i32) {
    }

}
