#![allow(non_snake_case)]
use std::sync::OnceLock;

// Java: public class X15Map implements XmlName2Id
pub struct X15Map;
static OBJ: OnceLock<X15Map> = OnceLock::new();

impl X15Map {
    // Java: public static X15Map getInstance()
    pub fn getInstance() -> &'static Self {
        OBJ.get_or_init(|| X15Map)
    }

    // Java: public int getId(int prefix, int name)
    pub fn getId(&self, _prefix: i32, name: i32) -> i32 {
        // Java: this.mapobj.put("workbookPr", 6422529);
        // Java: this.mapobj.put("chartTrackingRefBase", 6422530);
        match name {
            -472144612 => 6422529,
            -1166481521 => 6422530,
            _ => -1,
        }
    }
}
