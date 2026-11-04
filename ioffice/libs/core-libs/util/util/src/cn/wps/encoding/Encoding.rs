#![allow(dead_code, non_snake_case)]

use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: UTF8Encoding UTF8
// Java field: GB2312Encoding GB2312
// Java field: UnicodeEncoding Unicode

static NAME_MAP: OnceLock<HashMap<String, Box<dyn Encoding + Send + Sync>>> = OnceLock::new();
static CODEPAGE_MAP: OnceLock<HashMap<i32, Box<dyn Encoding + Send + Sync>>> = OnceLock::new();

// Java: public class Encoding implements Cloneable
pub trait Encoding {
    fn bodyName(&self) -> &str;
}

// Java: public class Encoding implements Cloneable
pub struct EncodingBase;

impl EncodingBase {
    // Java: Encoding()
    pub fn new() -> Self {
      // Java: super();
      EncodingBase
    }

    // Java: abstract String bodyName()
    pub fn bodyName(&self) -> &str { "" }

    // Java: static Encoding getEncoding(String name)
    pub fn getEncoding(_name: &str) -> Option<&'static (dyn Encoding + Send + Sync)> {
      // Java: Assert.assertNotNull("name should not be null!", name);
      // Java: Assert.assertNotNull("Name.sMap should not be null!", Name.sMap);
      // Java: return (Encoding)Name.sMap.get(name);
      todo!();
    }

    // Java: static Encoding getEncoding(int codePage)
    pub fn getEncoding_by_codepage(_codePage: i32) -> Option<&'static (dyn Encoding + Send + Sync)> {
      // Java: Assert.assertNotNull("CodePage.sMap should not be null!", CodePage.sMap);
      // Java: return (Encoding)CodePage.sMap.get(codePage);
      todo!();
    }
}
