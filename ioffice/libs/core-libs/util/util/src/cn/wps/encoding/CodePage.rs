#![allow(non_snake_case, dead_code, unused_variables)]

use std::collections::HashMap;
use std::sync::OnceLock;

// Java field: HashMap<Integer, Encoding> sMap
static S_MAP: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();
// Java field: int UTF8
pub(crate) const UTF8: i32 = 65001;

// Java: public class CodePage
pub struct CodePage;

// Java: public class CodePage
impl CodePage {
    // Java: CodePage()
    pub fn new() -> Self {
        // Java: super();
        Self
    }

    pub fn get_map() -> &'static HashMap<i32, &'static str> {
        S_MAP.get_or_init(|| {
            let mut map = HashMap::new();
            // Java: sMap.put(65001, Encoding.UTF8);
            map.insert(65001, "UTF-8");
            map
        })
    }
}
