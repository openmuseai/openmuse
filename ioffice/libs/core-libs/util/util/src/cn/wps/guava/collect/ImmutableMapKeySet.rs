#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableSet::ImmutableSet;
use super::ImmutableMap::ImmutableMap;

// Java: public class ImmutableMapKeySet extends ImmutableSet
pub struct ImmutableMapKeySet {
    pub base: ImmutableSet,
    // Java field: ImmutableMap<K, V> map
    pub map: Box<ImmutableMap>,
}

// Java: public class ImmutableMapKeySet extends ImmutableSet
impl ImmutableMapKeySet {
    // Java: ImmutableMapKeySet(ImmutableMap map)
    pub fn new(map: ImmutableMap) -> Self {
        // Java: super();
        // Java: this.map = map;
        ImmutableMapKeySet {
            base: ImmutableSet::new(),
            map: Box::new(map),
        }
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.map.size();
        self.map.size()
    }

    // Java: UnmodifiableIterator<K> iterator()
    pub fn iterator(&self) {
        // Java: return this.asList().iterator();
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: Option<&dyn Any>) -> bool {
        // Java: return this.map.containsKey(object);
        self.map.containsKey(object)
    }

    // Java: ImmutableList<K> createAsList()
    pub fn createAsList(&self) {
        // Java: final ImmutableList<Map.Entry<K, V>> entryList = this.map.entrySet().asList();
        // Java: return new ImmutableAsList<K>()
        {
            // Java: public K get(int index)
            {
                // Java: return (K)((Map.Entry)entryList.get(index)).getKey();
            }
            // Java: ImmutableCollection<K> delegateCollection()
            {
                // Java: return ImmutableMapKeySet.this;
            }
        }
    }
}
