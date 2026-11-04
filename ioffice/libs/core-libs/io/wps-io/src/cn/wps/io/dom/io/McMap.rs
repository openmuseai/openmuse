#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.StringIntMap
use crate::io::simple::StringIntMap::StringIntMap;
// Java: import cn.wps.io.simple.XmlName2Id

// Java field: String PREFIX
pub static PREFIX: &str = "mc";
// Java field: String NAMESPACING
pub const NAMESPACING: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
// Java field: String ALTERNATE_CONTENT
const ALTERNATE_CONTENT: &str = "AlternateContent";
// Java field: String FALLBACK
const FALLBACK: &str = "Fallback";
// Java field: String CHOICE
const CHOICE: &str = "Choice";
// Java field: McMap obj
static mut OBJ: Option<McMap> = None;

// Java: public class McMap implements XmlName2Id
pub struct McMap {
    // Java field: StringIntMap mapobj
    mapobj: StringIntMap,
}

// Java: public class McMap implements XmlName2Id
impl McMap {
    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("AlternateContent", 1590178495);
        self.mapobj.put(ALTERNATE_CONTENT, 1590178495);
        // Java: this.mapobj.put("Fallback", 825887490);
        self.mapobj.put(FALLBACK, 825887490);
        // Java: this.mapobj.put("Choice", 2017610177);
        self.mapobj.put(CHOICE, 2017610177);
    }

    // Java: McMap()
    fn new() -> Self {
        // Java: super();
        let mut m = McMap {
            mapobj: StringIntMap::new(5),
        };
        // Java: this.init();
        m.init();
        m
    }

    // Java: static McMap getInstance()
    pub fn getInstance() -> &'static McMap {
        // Java: if (obj == null)
        // Java: obj = new McMap();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() {
                OBJ = Some(McMap::new());
            }
            OBJ.as_ref().unwrap()
        }
    }

    // Java: int getId(String uri, String name)
    pub fn getId_str(&self, _uri: &str, name: &str) -> i32 {
        // Java: return this.mapobj.get(name);
        self.mapobj.get(name)
    }

    // Java: int getId(int prefix, int name)
    pub fn getId_int(&self, _prefix: i32, name: i32) -> i32 {
        // Java: return this.mapobj.get(name);
        self.mapobj.get_by_int(name)
    }
}
