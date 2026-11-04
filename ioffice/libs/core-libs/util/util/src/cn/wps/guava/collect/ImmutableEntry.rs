#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public class ImmutableEntry extends AbstractMapEntry
pub struct ImmutableEntry {
    // Java field: K key
    pub key: Option<Box<dyn Any>>,
    // Java field: V value
    pub value: Option<Box<dyn Any>>,
}

// Java: public class ImmutableEntry extends AbstractMapEntry
impl ImmutableEntry {
    // Java: ImmutableEntry(K key, V value)
    pub fn new(key: Option<Box<dyn Any>>, value: Option<Box<dyn Any>>) -> Self {
        // Java: super();
        // Java: this.key = key;
        // Java: this.value = value;
        ImmutableEntry { key, value }
    }

    // Java: K getKey()
    pub fn getKey(&self) -> Option<&dyn Any> {
        // Java: return this.key;
        self.key.as_deref()
    }

    // Java: V getValue()
    pub fn getValue(&self) -> Option<&dyn Any> {
        // Java: return this.value;
        self.value.as_deref()
    }

    // Java: V setValue(V value)
    pub fn setValue(&self, _value: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}
