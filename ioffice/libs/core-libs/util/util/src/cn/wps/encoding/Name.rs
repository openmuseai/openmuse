#![allow(non_snake_case, dead_code, unused_variables)]

use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: HashMap<String, Encoding> sMap
static S_MAP: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
// Java field: String UTF8
pub(crate) const UTF8: &str = "utf-8";
// Java field: String GB2312
pub(crate) const GB2312: &str = "gb2312";
// Java field: String UNICODE
pub(crate) const UNICODE: &str = "utf-16be";

// Java: public class Name
pub struct Name;

// Java: public class Name
impl Name {
    // Java: Name()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    pub fn get_map() -> &'static HashMap<String, &'static str> {
        S_MAP.get_or_init(|| {
            let mut map = HashMap::new();
            // Java: sMap.put("utf-8", Encoding.UTF8);
            map.insert("utf-8".to_string(), "UTF-8");
            // Java: sMap.put("gb2312", Encoding.GB2312);
            map.insert("gb2312".to_string(), "GB2312");
            // Java: sMap.put("utf-16be", Encoding.Unicode);
            map.insert("utf-16be".to_string(), "Unicode");
            map
        })
    }
}
