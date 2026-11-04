// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.Map
use std::collections::HashMap;
use std::hash::Hash;

pub(crate) const DEFAULT_SIZE: i32 = 10;

// Java: public class IntMapper
pub struct IntMapper<T: Eq + Hash + Clone> {
    // Java field: List<T> elements
    pub(crate) elements: Vec<T>,
    // Java field: Map<T, Integer> valueKeyMap
    value_key_map: HashMap<T, i32>,
    // Java field: int _default_size
}

// Java: public class IntMapper
impl<T: Eq + Hash + Clone> IntMapper<T> {
    // Java: IntMapper()
    pub fn new() -> Self {
        // Java: this(10);
        Self::new_with_capacity(10)
    }

    // Java: IntMapper(int initialCapacity)
    pub fn new_with_capacity(initial_capacity: usize) -> Self {
        // Java: super();
        // Java: this.elements = new ArrayList(initialCapacity);
        // Java: this.valueKeyMap = new HashMap(initialCapacity);
        IntMapper {
            elements: Vec::with_capacity(initial_capacity),
            value_key_map: HashMap::with_capacity(initial_capacity),
        }
    }

    // Java: boolean add(T value)
    pub fn add(&mut self, value: T) -> bool {
        // Java: int index = this.elements.size();
        let index = self.elements.len() as i32;
        // Java: this.elements.add(value);
        self.elements.push(value.clone());
        // Java: this.valueKeyMap.put(value, index);
        self.value_key_map.insert(value, index);
        // Java: return true;
        true
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.elements.size();
        self.elements.len() as i32
    }

    // Java: T get(int index)
    pub fn get(&self, index: i32) -> Option<&T> {
        // Java: return (T)this.elements.get(index);
        self.elements.get(index as usize)
    }

    // Java: int getIndex(T o)
    pub fn get_index(&self, o: &T) -> i32 {
        // Java: Integer i = (Integer)this.valueKeyMap.get(o);
        // Java: return i == null ? -1 : i;
        match self.value_key_map.get(o) {
            Some(&i) => i,
            None => -1,
        }
    }

    // Java: Iterator<T> iterator()
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        // Java: return this.elements.iterator();
        self.elements.iter()
    }
}
