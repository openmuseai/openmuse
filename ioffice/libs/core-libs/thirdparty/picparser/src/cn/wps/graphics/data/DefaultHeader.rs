#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_variables, unused_imports)]

use crate::cn::wps::graphics::data::MetaFileHeader::MetaFileHeader;

// Java: public class DefaultHeader implements MetaFileHeader
pub struct DefaultHeader {
}

// Java: public class DefaultHeader implements MetaFileHeader
impl DefaultHeader {
    // Java: DefaultHeader()
    pub fn new() -> Self {
      // Java: super();
      DefaultHeader {}
    }

    // Java: boolean isValid()
    pub fn isValid(&self) -> bool {
      // Java: return true;
      true
    }

    // Java: int getHandles()
    pub fn getHandles(&self) -> i32 {
      // Java: return 0;
      0
    }

}

impl MetaFileHeader for DefaultHeader {
    fn isValid(&self) -> bool { DefaultHeader::isValid(self) }
    fn getHandles(&self) -> i32 { DefaultHeader::getHandles(self) }
}
