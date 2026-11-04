#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.graphics.data.MetaFileRecord
use crate::cn::wps::graphics::data::MetaFileRecord::MetaFileRecord;
// Java: import cn.wps.graphics.data.reader.EMFParser
use crate::cn::wps::graphics::data::reader::EMFParser::EMFParser;
// Java: import cn.wps.graphics.data.reader.MetaFileParser
use crate::cn::wps::graphics::data::reader::MetaFileParser::MetaFileParser;

// Java: public class EMFRecord extends MetaFileRecord
pub struct EMFRecord {
}

// Java: public class EMFRecord extends MetaFileRecord
impl EMFRecord {
    // Java: EMFRecord()
    pub fn new() -> Self {
        // Java: super();
        EMFRecord {}
    }

    // Java: MetaFileRecord read(MetaFileParser parser, int len)
    pub fn read(&self, parser: &MetaFileParser, len: i32) -> i32 {
        // Java: return this.read((EMFParser)parser, len);
        self.read_emf(len)
    }

    // Java: abstract EMFRecord read(EMFParser var1, int var2)
    pub fn read_emf(&self, var2: i32) -> i32 {
        0
    }
}
