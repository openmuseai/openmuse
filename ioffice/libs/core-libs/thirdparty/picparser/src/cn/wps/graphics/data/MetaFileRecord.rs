#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// Java: public class MetaFileRecord
pub struct MetaFileRecord {
    // Java field: int DEFAULT_RECORD
    _private: (),
}

pub(crate) const DEFAULT_RECORD: i32 = -1;

impl MetaFileRecord {
    // Java: MetaFileRecord()
    pub fn new() -> Self {
        // Java: super();
        MetaFileRecord { _private: () }
    }

    // Java: public MetaFileRecord parser(MetaFileParser parser, int len)
    pub fn parser(&mut self, _parser: i32, _len: i32) -> i32 {
        // Java: int finalPos = parser.getCurrentPos() + len;
        let finalPos: i32 = 0;
        // Java: MetaFileRecord record = this.read(parser, len);
        let record = self.read(_parser, _len);
        // Java: parser.setCurrentPos(finalPos);
        // Java: return record;
        record
    }

    // Java: public String toString()
    pub fn toString(&self) -> String {
        // Java: return this.getClass().getSimpleName();
        String::from("MetaFileRecord")
    }

    // Java: public void dispose()
    pub fn dispose(&mut self) {
    }

    // Java: public abstract MetaFileRecord read(MetaFileParser var1, int var2)
    pub fn read(&mut self, _parser: i32, _len: i32) -> i32 {
        0
    }

    // Java: public abstract void render(MetaFilePainter var1)
    pub fn render(&self, _renderer: i32) {
    }
}
