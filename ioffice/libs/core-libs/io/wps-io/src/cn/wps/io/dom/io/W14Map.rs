#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.StringIntMap
use crate::io::simple::StringIntMap::StringIntMap;
// Java: import cn.wps.io.simple.XmlName2Id

// Java field: String PREFIX
pub static PREFIX: &str = "w14";
// Java field: String NAMESPACING
pub const NAMESPACING: &str = "http://schemas.microsoft.com/office/word/2010/wordml";
// Java field: String CHILD_NODE_GLOW
const CHILD_NODE_GLOW: &str = "glow";
// Java field: String CHILD_NODE_SHADOW
const CHILD_NODE_SHADOW: &str = "shadow";
// Java field: String CHILD_NODE_REFLECTION
const CHILD_NODE_REFLECTION: &str = "reflection";
// Java field: String CHILD_NODE_TEXTOUTLINE
const CHILD_NODE_TEXTOUTLINE: &str = "textOutline";
// Java field: String CHILD_NODE_TEXTFILL
const CHILD_NODE_TEXTFILL: &str = "textFill";
// Java field: String CHILD_NODE_SCENE3D
const CHILD_NODE_SCENE3D: &str = "scene3d";
// Java field: String CHILD_NODE_PROPS3D
const CHILD_NODE_PROPS3D: &str = "props3d";
// Java field: String CHILD_NODE_LIGATURES
const CHILD_NODE_LIGATURES: &str = "ligatures";
// Java field: String CHILD_NODE_NUMFORM
const CHILD_NODE_NUMFORM: &str = "numForm";
// Java field: String CHILD_NODE_NUMSPACING
const CHILD_NODE_NUMSPACING: &str = "numSpacing";
// Java field: String CHILD_NODE_STYLISTICSETS
const CHILD_NODE_STYLISTICSETS: &str = "stylisticSets";

// Java field: W14Map obj
static mut OBJ: Option<W14Map> = None;

// Java: public class W14Map implements XmlName2Id
pub struct W14Map {
    // Java field: StringIntMap mapobj
    mapobj: StringIntMap,
}

// Java: public class W14Map implements XmlName2Id
impl W14Map {
    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("textOutline", 14);
        self.mapobj.put("textOutline", 14);
        // Java: this.mapobj.put("textFill", 15);
        self.mapobj.put("textFill", 15);
        // Java: this.mapobj.put("ligatures", 18);
        self.mapobj.put("ligatures", 18);
        // Java: this.mapobj.put("numForm", 19);
        self.mapobj.put("numForm", 19);
        // Java: this.mapobj.put("numSpacing", 20);
        self.mapobj.put("numSpacing", 20);
        // Java: this.mapobj.put("stylisticSets", 21);
        self.mapobj.put("stylisticSets", 21);
    }

    // Java: W14Map()
    fn new() -> Self {
        // Java: super();
        let mut m = W14Map { mapobj: StringIntMap::new(11) };
        // Java: this.init();
        m.init();
        m
    }

    // Java: static W14Map getInstance()
    pub fn getInstance() -> &'static W14Map {
        // Java: if (obj == null)
        // Java: obj = new W14Map();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() { OBJ = Some(W14Map::new()); }
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
