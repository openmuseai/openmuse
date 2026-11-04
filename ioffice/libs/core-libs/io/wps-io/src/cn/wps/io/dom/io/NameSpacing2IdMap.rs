#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.io.simple.XmlName2Id
// Java: import java.util.HashMap
use std::collections::HashMap;

// Java: public class NameSpacing2IdMap
pub struct NameSpacing2IdMap {
    // Java field: HashMap<Integer, XmlName2Id> mName2IdMap
    mName2IdMap: HashMap<i32, Box<dyn std::any::Any + Send + Sync>>,
}

// Java field: NameSpacing2IdMap obj
static mut OBJ: Option<NameSpacing2IdMap> = None;

// Java: public class NameSpacing2IdMap
impl NameSpacing2IdMap {
    // Java: NameSpacing2IdMap()
    fn new() -> Self {
        // Java: super();
        let mut inst = NameSpacing2IdMap {
            mName2IdMap: HashMap::new(),
        };
        // Java: this.init();
        inst.init();
        inst
    }

    // Java: void init()
    fn init(&mut self) {
        // Java: this.mName2IdMap.put(WMap.MAIN_NAMESPACING.hashCode(), WMap.getInstance());
        todo!()
    }

    // Java: static NameSpacing2IdMap getInstance()
    pub fn getInstance() -> &'static NameSpacing2IdMap {
        // Java: if (null == obj)
        // Java: obj = new NameSpacing2IdMap();
        // Java: return obj;
        unsafe {
            if OBJ.is_none() {
                OBJ = Some(NameSpacing2IdMap::new());
            }
            OBJ.as_ref().unwrap()
        }
    }
}
