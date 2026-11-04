#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.StringIntMap
use crate::io::simple::StringIntMap::StringIntMap;
// Java: import cn.wps.io.simple.XmlName2Id

// Java field: String PREFIX
pub const PREFIX: &str = "v";
// Java field: String NAME_SPACING
pub const NAME_SPACING: &str = "urn:schemas-microsoft-com:vml";
// Java field: String RECT
const RECT: &str = "rect";
// Java field: String ROUNDRECT
const ROUNDRECT: &str = "roundrect";
// Java field: String GROUP
const GROUP: &str = "group";
// Java field: String IMAGEDATA
const IMAGEDATA: &str = "imagedata";
// Java field: String FILL
const FILL: &str = "fill";
// Java field: String PATH
const PATH: &str = "path";
// Java field: String TEXTPATH
const TEXTPATH: &str = "textpath";
// Java field: String STROKE
const STROKE: &str = "stroke";
// Java field: String SHADOW
const SHADOW: &str = "shadow";
// Java field: String SHAPE
const SHAPE: &str = "shape";
// Java field: String SHAPETYPE
const SHAPETYPE: &str = "shapetype";
// Java field: String F
const F: &str = "f";
// Java field: String FORMULAS
const FORMULAS: &str = "formulas";
// Java field: String OVAL
const OVAL: &str = "oval";
// Java field: String HANDLES
const HANDLES: &str = "handles";
// Java field: String TEXTBOX
const TEXTBOX: &str = "textbox";
// Java field: String POLYLINE
const POLYLINE: &str = "polyline";
// Java field: String ARC
const ARC: &str = "arc";
// Java field: String CURVE
const CURVE: &str = "curve";
// Java field: String IMAGE
const IMAGE: &str = "image";
// Java field: String LINE
const LINE: &str = "line";
// Java field: String BACKGROUND
const BACKGROUND: &str = "background";

// Java field: VMap obj
static mut OBJ: Option<VMap> = None;

// Java: public class VMap implements XmlName2Id
pub struct VMap {
    // Java field: StringIntMap mapobj
    mapobj: StringIntMap,
}

// Java: public class VMap implements XmlName2Id
impl VMap {
    // Java: void init()
    fn init(&mut self) {
        // Java: this.mapobj.put("rect", 3496420);
        self.mapobj.put("rect", 3496420);
        // Java: this.mapobj.put("roundrect", -4156302);
        self.mapobj.put("roundrect", -4156302);
        // Java: this.mapobj.put("group", 98629247);
        self.mapobj.put("group", 98629247);
        // Java: this.mapobj.put("imagedata", -877694075);
        self.mapobj.put("imagedata", -877694075);
        // Java: this.mapobj.put("fill", 3143043);
        self.mapobj.put("fill", 3143043);
        // Java: this.mapobj.put("path", 3433509);
        self.mapobj.put("path", 3433509);
        // Java: this.mapobj.put("textpath", -1002812718);
        self.mapobj.put("textpath", -1002812718);
        // Java: this.mapobj.put("stroke", -891980232);
        self.mapobj.put("stroke", -891980232);
        // Java: this.mapobj.put("shadow", -903579360);
        self.mapobj.put("shadow", -903579360);
        // Java: this.mapobj.put("shape", 109399969);
        self.mapobj.put("shape", 109399969);
        // Java: this.mapobj.put("shapetype", -1638324645);
        self.mapobj.put("shapetype", -1638324645);
        // Java: this.mapobj.put("f", 102);
        self.mapobj.put("f", 102);
        // Java: this.mapobj.put("formulas", 474667981);
        self.mapobj.put("formulas", 474667981);
        // Java: this.mapobj.put("oval", 3423314);
        self.mapobj.put("oval", 3423314);
        // Java: this.mapobj.put("handles", 692803403);
        self.mapobj.put("handles", 692803403);
        // Java: this.mapobj.put("textbox", -1417835138);
        self.mapobj.put("textbox", -1417835138);
        // Java: this.mapobj.put("polyline", 561938880);
        self.mapobj.put("polyline", 561938880);
        // Java: this.mapobj.put("arc", 96850);
        self.mapobj.put("arc", 96850);
        // Java: this.mapobj.put("curve", 95027439);
        self.mapobj.put("curve", 95027439);
        // Java: this.mapobj.put("image", 100313435);
        self.mapobj.put("image", 100313435);
        // Java: this.mapobj.put("line", 3321844);
        self.mapobj.put("line", 3321844);
    }

    // Java: VMap()
    fn new() -> Self {
        // Java: super();
        let mut m = VMap { mapobj: StringIntMap::new(5) };
        // Java: this.init();
        m.init();
        m
    }

    // Java: static VMap getInstance()
    pub fn getInstance() -> &'static VMap {
        // Java: if (obj == null)
        // Java: obj = new VMap();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() { OBJ = Some(VMap::new()); }
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
