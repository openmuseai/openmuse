#![allow(non_snake_case, dead_code, unused_variables)]

use std::any::Any;

// Java: public class AbstractMapEntry implements Map.Entry
pub trait AbstractMapEntry {
    // Java: AbstractMapEntry()
    // Java: super();

    // Java: abstract K getKey()
    fn getKey(&self) -> &dyn Any;

    // Java: abstract V getValue()
    fn getValue(&self) -> &dyn Any;

    // Java: V setValue(V value)
    fn setValue(&mut self, _value: Box<dyn Any>) -> Option<Box<dyn Any>> {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean equals(Object object)
    fn equals(&self, _object: &dyn Any) -> bool {
        // Java: if (!(object instanceof Map.Entry))
        // {
            // Java: return false;
            // return false;
        // }
        // Java: else
        // {
            // Java: Map.Entry<?, ?> that = (Map.Entry)object;
            // Java: return Objects.equal(this.getKey(), that.getKey()) && Objects.equal(this.getValue(), that.getValue());
            false
        // }
    }

    // Java: int hashCode()
    fn hashCode(&self) -> i32 {
        // Java: K k = (K)this.getKey();
        // Java: V v = (V)this.getValue();
        // Java: return (k == null ? 0 : k.hashCode()) ^ (v == null ? 0 : v.hashCode());
        0
    }

    // Java: String toString()
    fn toString(&self) -> String {
        // Java: return this.getKey() + "=" + this.getValue();
        String::from("entry")
    }
}
