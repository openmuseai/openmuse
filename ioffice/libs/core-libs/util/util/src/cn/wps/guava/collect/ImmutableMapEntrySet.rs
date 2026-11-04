#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableSet::ImmutableSet;
use super::ImmutableMap::ImmutableMap;

// Java: public class ImmutableMapEntrySet extends ImmutableSet
pub struct ImmutableMapEntrySet {
    pub base: ImmutableSet,
}

// Java: public class ImmutableMapEntrySet extends ImmutableSet
impl ImmutableMapEntrySet {
    // Java: ImmutableMapEntrySet()
    pub fn new() -> Self {
        // Java: super();
        ImmutableMapEntrySet {
            base: ImmutableSet::new(),
        }
    }

    // Java: abstract abstract ImmutableMap<K, V> map()

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.map().size();
        0
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: Option<&dyn Any>) -> bool {
        // Java: if (!(object instanceof Map.Entry))
        if object.is_none() {
            // Java: return false;
            return false;
        }
        // Java: else
        {
            // Java: Map.Entry<?, ?> entry = (Map.Entry)object;
            // Java: V value = (V)this.map().get(entry.getKey());
            // Java: return value != null && value.equals(entry.getValue());
            false
        }
    }
}
