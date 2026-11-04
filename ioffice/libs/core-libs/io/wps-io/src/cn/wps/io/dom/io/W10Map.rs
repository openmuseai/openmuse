#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.StringIntMap
use crate::io::simple::StringIntMap::StringIntMap;
// Java: import cn.wps.io.simple.XmlName2Id

// Java field: String PREFIX
pub const PREFIX: &str = "w10";
// Java field: String NAME_SPACING
pub const NAME_SPACING: &str = "urn:schemas-microsoft-com:office:word";
// Java field: String WRAP
const WRAP: &str = "wrap";
// Java field: String ANCHORLOCK
const ANCHORLOCK: &str = "anchorlock";

// Java field: W10Map obj
static mut OBJ: Option<W10Map> = None;

// Java: public class W10Map implements XmlName2Id
pub struct W10Map {
    // Java field: StringIntMap mapobj
    mapobj: StringIntMap,
}

// Java: public class W10Map implements XmlName2Id
impl W10Map {
    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("wrap", 3657802);
        self.mapobj.put("wrap", 3657802);
        // Java: this.mapobj.put("anchorlock", -1296358048);
        self.mapobj.put("anchorlock", -1296358048);
    }

    // Java: W10Map()
    fn new() -> Self {
        // Java: super();
        let mut m = W10Map { mapobj: StringIntMap::new(5) };
        // Java: this.init();
        m.init();
        m
    }

    // Java: static W10Map getInstance()
    pub fn getInstance() -> &'static W10Map {
        // Java: if (obj == null)
        // Java: obj = new W10Map();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() { OBJ = Some(W10Map::new()); }
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
