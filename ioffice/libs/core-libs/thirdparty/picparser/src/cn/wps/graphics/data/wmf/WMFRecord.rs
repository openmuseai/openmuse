#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.graphics.data.MetaFileRecord
// Java: import cn.wps.graphics.data.reader.MetaFileParser
// Java: import cn.wps.graphics.data.reader.WMFParser

// Java: public class WMFRecord extends MetaFileRecord
pub struct WMFRecord {
    _private: (),
}

// Java: public class WMFRecord extends MetaFileRecord
impl WMFRecord {
    // Java: WMFRecord()
    pub fn new() -> Self {
        // Java: super();
        WMFRecord { _private: () }
    }

    // Java: MetaFileRecord read(MetaFileParser parser, int len)
    pub fn read(&self, _parser: i32, _len: i32) -> i32 {
        // Java: return this.read((WMFParser)parser, len);
        0
    }

    // Java: abstract WMFRecord read(WMFParser var1, int var2)
    pub fn read_wmf(&self, _var1: i32, _var2: i32) -> i32 {
        0
    }
}
