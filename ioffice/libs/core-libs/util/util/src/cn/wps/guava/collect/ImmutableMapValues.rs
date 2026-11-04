#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableCollection::ImmutableCollection;
use super::ImmutableMap::ImmutableMap;

// Java: public class ImmutableMapValues extends ImmutableCollection
pub struct ImmutableMapValues {
    pub base: ImmutableCollection,
    // Java field: ImmutableMap<K, V> map
    pub map: Box<ImmutableMap>,
}

// Java: public class ImmutableMapValues extends ImmutableCollection
impl ImmutableMapValues {
    // Java: ImmutableMapValues(ImmutableMap map)
    pub fn new(map: ImmutableMap) -> Self {
        // Java: super();
        // Java: this.map = map;
        ImmutableMapValues {
            base: ImmutableCollection::new(),
            map: Box::new(map),
        }
    }

    // Java: int size()
    pub fn size(&self) -> usize {
        // Java: return this.map.size();
        self.map.size()
    }

    // Java: UnmodifiableIterator<V> iterator()
    pub fn iterator(&self) {
        // Java: return Maps.valueIterator(this.map.entrySet().iterator());
    }

    // Java: boolean contains(Object object)
    pub fn contains(&self, object: Option<&dyn Any>) -> bool {
        // Java: return object != null && Iterators.contains(this.iterator(), object);
        object.is_some()
    }

    // Java: ImmutableList<V> createAsList()
    pub fn createAsList(&self) {
        // Java: final ImmutableList<Map.Entry<K, V>> entryList = this.map.entrySet().asList();
        // Java: return new ImmutableAsList<V>()
        {
            // Java: public V get(int index)
            {
                // Java: return (V)((Map.Entry)entryList.get(index)).getValue();
            }
            // Java: ImmutableCollection<V> delegateCollection()
            {
                // Java: return ImmutableMapValues.this;
            }
        }
    }

    // Java: static public static <T> T checkNotNull(T reference)
    pub fn checkNotNull(reference: Option<&dyn Any>) -> &dyn Any {
        // Java: if (reference == null)
        if reference.is_none() {
            // Java: throw new NullPointerException();
            panic!("NullPointerException");
        }
        // Java: else
        {
            // Java: return reference;
            reference.unwrap()
        }
    }
}
