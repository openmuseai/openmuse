// java package cn.wps.kfc.xsd.shared;

// java import cn.wps.kfc.xpxml.reader.XmlName2Id;
// java import cn.wps.kfc.xsd.StringIntMap;
use std::collections::HashMap;
use std::sync::OnceLock;

// java public class RMap implements XmlName2Id
pub struct RMap {
    // java private StringIntMap mapobj = new StringIntMap(4);
    mapobj: HashMap<i32, i32>,
}

static OBJ: OnceLock<RMap> = OnceLock::new();

// java public class RMap implements XmlName2Id
impl RMap {
    // java public static final String URI = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    pub const URI: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
    // java public static final String PREFIX = "r";
    pub const PREFIX: &str = "r";
    // java public static final String PREFIX_1 = "d3p1";
    pub const PREFIX_1: &str = "d3p1";
    // java public static final String id = "id";
    pub const id: &str = "id";
    // java public static final String embed = "embed";
    pub const embed: &str = "embed";
    // java public static final String link = "link";
    pub const link: &str = "link";
    // java public static final String dm = "dm";
    pub const dm: &str = "dm";
    // java public static final String lo = "lo";
    pub const lo: &str = "lo";
    // java public static final String qs = "qs";
    pub const qs: &str = "qs";
    // java public static final String cs = "cs";
    pub const cs: &str = "cs";
    // java public static final String blip = "blip";
    pub const blip: &str = "blip";
    // java public static final String pict = "pict";
    pub const pict: &str = "pict";
    // java public static final String href = "href";
    pub const href: &str = "href";
    // java public static final String topLeft = "topLeft";
    pub const topLeft: &str = "topLeft";
    // java public static final String topRight = "topRight";
    pub const topRight: &str = "topRight";
    // java public static final String bottomLeft = "bottomLeft";
    pub const bottomLeft: &str = "bottomLeft";
    // java public static final String bottomRight = "bottomRight";
    pub const bottomRight: &str = "bottomRight";

    // java private RMap()
    fn RMap() -> Self {
        // java super();
        let mut map = Self {
            mapobj: HashMap::new(),
        };
        // java this.init();
        map.init();
        map
    }

    // java private void init()
    fn init(&mut self) {
        // java this.mapobj.put("id", 4259841);
        self.mapobj.insert(hash_str("id"), 4259841);
        // java this.mapobj.put("embed", 4259842);
        self.mapobj.insert(hash_str("embed"), 4259842);
        // java this.mapobj.put("link", 4259843);
        self.mapobj.insert(hash_str("link"), 4259843);
        // java this.mapobj.put("dm", 4259844);
        self.mapobj.insert(hash_str("dm"), 4259844);
        // java this.mapobj.put("lo", 4259845);
        self.mapobj.insert(hash_str("lo"), 4259845);
        // java this.mapobj.put("qs", 4259846);
        self.mapobj.insert(hash_str("qs"), 4259846);
        // java this.mapobj.put("cs", 4259847);
        self.mapobj.insert(hash_str("cs"), 4259847);
        // java this.mapobj.put("blip", 4259848);
        self.mapobj.insert(hash_str("blip"), 4259848);
        // java this.mapobj.put("pict", 4259849);
        self.mapobj.insert(hash_str("pict"), 4259849);
        // java this.mapobj.put("href", 4259850);
        self.mapobj.insert(hash_str("href"), 4259850);
        // java this.mapobj.put("topLeft", 4259851);
        self.mapobj.insert(hash_str("topLeft"), 4259851);
        // java this.mapobj.put("topRight", 4259852);
        self.mapobj.insert(hash_str("topRight"), 4259852);
        // java this.mapobj.put("bottomLeft", 4259853);
        self.mapobj.insert(hash_str("bottomLeft"), 4259853);
        // java this.mapobj.put("bottomRight", 4259854);
        self.mapobj.insert(hash_str("bottomRight"), 4259854);
    }

    // java public static RMap getInstance()
    pub fn getInstance() -> &'static RMap {
        // java if (obj == null)
        // java obj = new RMap();
        let obj = OBJ.get_or_init(Self::RMap);
        // java return obj;
        obj
    }

    // java public int getId(int prefix, int name)
    pub fn getId(&self, _prefix: i32, name: i32) -> i32 {
        // java return this.mapobj.get(name);
        *self.mapobj.get(&name).unwrap_or(&-1)
    }
}

fn hash_str(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
