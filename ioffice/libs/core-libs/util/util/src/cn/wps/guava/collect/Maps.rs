#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;

// Java: public class Maps
pub struct Maps;

// Java: public class Maps
impl Maps {
    // Java: Maps()
    // Java: super();

    // Java: static <K, V> UnmodifiableIterator<V> valueIterator(final UnmodifiableIterator<Map.Entry<K, V>> entryIterator)
    pub fn valueIterator(_entry_iterator: &dyn Any) {
        // Java: return new UnmodifiableIterator<V>()
        {
            // Java: public boolean hasNext()
            {
                // Java: return entryIterator.hasNext();
            }
            // Java: public V next()
            {
                // Java: return (V)((Map.Entry)entryIterator.next()).getValue();
            }
        }
    }
}
