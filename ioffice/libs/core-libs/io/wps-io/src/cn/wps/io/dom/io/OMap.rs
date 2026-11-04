#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.StringIntMap
use crate::io::simple::StringIntMap::StringIntMap;
// Java: import cn.wps.io.simple.XmlName2Id

// Java field: String PREFIX
pub const PREFIX: &str = "o";
// Java field: String NAME_SPACING
pub const NAME_SPACING: &str = "urn:schemas-microsoft-com:office:office";
// Java field: String CALLOUT
const CALLOUT: &str = "callout";
// Java field: String LOCK
const LOCK: &str = "lock";
// Java field: String FILL
const FILL: &str = "fill";
// Java field: String LEFT
const LEFT: &str = "left";
// Java field: String RIGHT
const RIGHT: &str = "right";
// Java field: String TOP
const TOP: &str = "top";
// Java field: String BOTTOM
const BOTTOM: &str = "bottom";
// Java field: String COLUMN
const COLUMN: &str = "column";
// Java field: String EXTRUSION
const EXTRUSION: &str = "extrusion";
// Java field: String INK
const INK: &str = "ink";
// Java field: String EQUATIONXML
const EQUATIONXML: &str = "equationxml";
// Java field: String DIAGRAM
const DIAGRAM: &str = "diagram";
// Java field: String COMPLEX
const COMPLEX: &str = "complex";

// Java field: OMap obj
static mut OBJ: Option<OMap> = None;

// Java: public class OMap implements XmlName2Id
pub struct OMap {
    // Java field: StringIntMap mapobj
    mapobj: StringIntMap,
}

// Java: public class OMap implements XmlName2Id
impl OMap {
    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("callout", 548646960);
        self.mapobj.put("callout", 548646960);
        // Java: this.mapobj.put("lock", 3327275);
        self.mapobj.put("lock", 3327275);
        // Java: this.mapobj.put("fill", 3143043);
        self.mapobj.put("fill", 3143043);
        // Java: this.mapobj.put("left", 3317767);
        self.mapobj.put("left", 3317767);
        // Java: this.mapobj.put("right", 108511772);
        self.mapobj.put("right", 108511772);
        // Java: this.mapobj.put("top", 115029);
        self.mapobj.put("top", 115029);
        // Java: this.mapobj.put("bottom", -1383228885);
        self.mapobj.put("bottom", -1383228885);
        // Java: this.mapobj.put("extrusion", -233914151);
        self.mapobj.put("extrusion", -233914151);
        // Java: this.mapobj.put("column", -1354837162);
        self.mapobj.put("column", -1354837162);
        // Java: this.mapobj.put("ink", 104422);
        self.mapobj.put("ink", 104422);
        // Java: this.mapobj.put("equationxml", 1678632267);
        self.mapobj.put("equationxml", 1678632267);
        // Java: this.mapobj.put("diagram", 1654878419);
        self.mapobj.put("diagram", 1654878419);
        // Java: this.mapobj.put("complex", 950494384);
        self.mapobj.put("complex", 950494384);
    }

    // Java: OMap()
    fn new() -> Self {
        // Java: super();
        let mut m = OMap {
            mapobj: StringIntMap::new(5),
        };
        // Java: this.init();
        m.init();
        m
    }

    // Java: static OMap getInstance()
    pub fn getInstance() -> &'static OMap {
        // Java: if (obj == null)
        // Java: obj = new OMap();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() {
                OBJ = Some(OMap::new());
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
