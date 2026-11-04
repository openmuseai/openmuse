#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import java.util.HashMap
use std::collections::HashMap;
// Java: import java.util.Map

// Java: public class EnumMap
pub struct EnumMap<K: std::hash::Hash + Eq, V> {
    // Java field: Map<K, V> sMap
    pub(crate) sMap: HashMap<K, V>,
}

// Java: public class EnumMap
impl<K: std::hash::Hash + Eq, V> EnumMap<K, V> {
    // Java: EnumMap()
    pub fn new() -> Self {
        // Java: super();
        EnumMap {
            sMap: HashMap::new(),
        }
    }

    // Java: void put(K k, V v)
    pub fn put(&mut self, k: K, v: V) {
        // Java: this.sMap.put(k, v);
        self.sMap.insert(k, v);
    }

    // Java: V get(K k)
    pub fn get(&self, k: &K) -> Option<&V> {
        // Java: return (V)this.sMap.get(k);
        self.sMap.get(k)
    }
}
