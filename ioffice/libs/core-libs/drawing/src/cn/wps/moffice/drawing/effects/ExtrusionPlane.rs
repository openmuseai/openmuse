#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

// Java: import java.util.HashMap
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::Mutex;

// Java field: HashMap<Integer, ExtrusionPlane> sMap
static sMap: OnceLock<Mutex<HashMap<i32, ExtrusionPlane>>> = OnceLock::new();

// Java: public enum ExtrusionPlane
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ExtrusionPlane {
    // Java field: int mData
    mData: i32,
}

impl ExtrusionPlane {
    pub const XY: ExtrusionPlane = ExtrusionPlane { mData: 0 };
    pub const ZX: ExtrusionPlane = ExtrusionPlane { mData: 1 };
    pub const YZ: ExtrusionPlane = ExtrusionPlane { mData: 2 };

    // Java: ExtrusionPlane(int value)
    fn init(&mut self, value: i32) {
        // Java: this.mData = value;
        self.mData = value;
        // Java: ExtrusionPlane.MAP.sMap.put(value, this);
        sMap.get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .insert(value, *self);
    }

    fn new(value: i32) -> Self {
        let mut instance = Self { mData: value };
        instance.init(value);
        instance
    }

    // Java: int getData()
    pub fn getData(&self) -> i32 {
        // Java: return this.mData;
        self.mData
    }

    // Java: static ExtrusionPlane getExtrusionPlane(int value)
    pub fn getExtrusionPlane(value: i32) -> Option<ExtrusionPlane> {
        // Java: return (ExtrusionPlane)ExtrusionPlane.MAP.sMap.get(value);
        sMap.get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .get(&value)
            .copied()
    }
}

// Java: public class MAP
pub struct MAP;

// Java: public class MAP
impl MAP {
    // Java: MAP()
    fn new() -> Self {
        // Java: super();
        Self
    }
}
