#![allow(non_snake_case, dead_code)]

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use super::BitField::BitField;

fn get_instances() -> &'static Mutex<HashMap<i32, BitField>> {
    static INSTANCES: OnceLock<Mutex<HashMap<i32, BitField>>> = OnceLock::new();
    INSTANCES.get_or_init(|| Mutex::new(HashMap::new()))
}

// Java: public class BitFieldFactory
pub struct BitFieldFactory {
    // Java field: Map instances
    instances: HashMap<i32, BitField>,
}

// Java: public class BitFieldFactory
impl BitFieldFactory {
    // Java: BitFieldFactory()
    pub fn new() -> Self {
        // Java: super();
        Self { instances: HashMap::new() }
    }

    // Java: static BitField getInstance(int mask)
    pub fn getInstance(mask: i32) -> BitField {
        let mut instances = get_instances().lock().unwrap();
        // Java: BitField f = (BitField)instances.get(mask);
        // LLM JavaLine Aggregation: merged into contains_key check below
        // Java: if (f == null)
        if !instances.contains_key(&mask) {
            // Java: f = new BitField(mask);
            let f = BitField::new(mask);
            // Java: instances.put(mask, f);
            instances.insert(mask, f);
        }
        // Java: return f;
        instances.get(&mask).unwrap().clone()
    }
}
