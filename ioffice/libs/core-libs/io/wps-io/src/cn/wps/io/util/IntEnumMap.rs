#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: import cn.wps.base.io.css.EnumMap

use std::collections::HashMap;

// Java: public class IntEnumMap extends EnumMap<Integer, T>
pub struct IntEnumMap<T> {
    // Java field: Map<K, V> sMap
    pub(crate) sMap: HashMap<i32, T>,
}

// Java: public class IntEnumMap extends EnumMap<Integer, T>
impl<T> IntEnumMap<T> {
    // Java: IntEnumMap()
    pub fn new() -> Self {
        // Java: super();
        IntEnumMap {
            sMap: HashMap::new(),
        }
    }

    // Java: void put(int k, T v)
    pub fn put_int(&mut self, k: i32, v: T) {
        // Java: this.sMap.put(k, v);
        self.sMap.insert(k, v);
    }

    // Java: void put(Integer k, T v)
    pub fn put(&mut self, k: i32, v: T) {
        // Java: this.sMap.put(k, v);
        self.sMap.insert(k, v);
    }

    // Java: T get(int k)
    pub fn get_int(&self, k: i32) -> Option<&T> {
        // Java: return (T)super.get(k);
        self.sMap.get(&k)
    }

    // Java: T get(Integer k)
    pub fn get(&self, k: &i32) -> Option<&T> {
        // Java: return (T)super.get(k);
        self.sMap.get(k)
    }
}
