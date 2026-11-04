#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::any::Any;
use super::ImmutableSet::ImmutableSet;
use super::ImmutableCollection::ImmutableCollection;
use super::ImmutableCollection::ImmutableCollectionBuilder;
use super::CollectPreconditions::CollectPreconditions;

// Java: public class ImmutableMap implements Map
pub struct ImmutableMap {
    // Java field: ImmutableSet<Map.Entry<K, V>> entrySet
    pub entrySet: Option<Box<ImmutableSet>>,
    // Java field: ImmutableSet<K> keySet
    pub keySet: Option<Box<ImmutableSet>>,
    // Java field: ImmutableCollection<V> values
    pub values: Option<Box<ImmutableCollection>>,
}

// Java: public class ImmutableMap implements Map
impl ImmutableMap {
    // Java: ImmutableMap()
    pub fn new() -> Self {
        // Java: super();
        ImmutableMap {
            entrySet: None,
            keySet: None,
            values: None,
        }
    }

    // Java: static public static <K, V> Builder<K, V> builder()
    pub fn builder() -> ImmutableMapBuilder {
        // Java: return new Builder<K, V>();
        ImmutableMapBuilder::new()
    }

    // Java: static void checkNoConflict(boolean safe, String conflictDescription, Map.Entry<?, ?> entry1, Map.Entry<?, ?> entry2)
    pub fn checkNoConflict(safe: bool, conflict_description: &str, _entry1: &dyn Any, _entry2: &dyn Any) {
        // Java: if (!safe)
        if !safe {
            // Java: throw new IllegalArgumentException("Multiple entries with same " + conflictDescription + ": " + entry1 + " and " + entry2);
            panic!("Multiple entries with same {}", conflict_description);
        }
    }

    // Java: static ImmutableMapEntry.TerminalEntry entryOf(K key, V value)
    pub fn entryOf(key: &dyn Any, value: &dyn Any) -> (Option<Box<dyn Any>>, Option<Box<dyn Any>>) {
        // Java: CollectPreconditions.checkEntryNotNull(key, value);
        CollectPreconditions::checkEntryNotNull(key, value);
        // Java: return new ImmutableMapEntry.TerminalEntry(key, value);
        (None, None)
    }

    // Java: V put(K k, V v)
    pub fn put(&self, _k: Option<Box<dyn Any>>, _v: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: V remove(Object o)
    pub fn remove(&self, _o: Option<&dyn Any>) -> Option<Box<dyn Any>> {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: void putAll(Map<? extends K, ? extends V> map)
    pub fn putAll(&self, _map: &dyn Any) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: void clear()
    pub fn clear(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this.size() == 0;
        self.size() == 0
    }

    // Java: boolean containsKey(Object key)
    pub fn containsKey(&self, key: Option<&dyn Any>) -> bool {
        // Java: return this.get(key) != null;
        self.get(key).is_some()
    }

    // Java: boolean containsValue(Object value)
    pub fn containsValue(&self, _value: Option<&dyn Any>) -> bool {
        // Java: return this.values().contains(value);
        false
    }

    // Java: abstract V get(Object var1)
    pub fn get(&self, _var1: Option<&dyn Any>) -> Option<&dyn Any> {
        None
    }

    // Java: ImmutableSet<Map.Entry<K, V>> entrySet()
    pub fn entrySet(&mut self) -> &ImmutableSet {
        // Java: ImmutableSet<Map.Entry<K, V>> result = this.entrySet;
        // Java: return result == null ? (this.entrySet = this.createEntrySet()) : result;
        if self.entrySet.is_none() {
            self.entrySet = Some(Box::new(ImmutableSet::new()));
        }
        self.entrySet.as_ref().unwrap()
    }

    // Java: abstract ImmutableSet<Map.Entry<K, V>> createEntrySet()

    // Java: ImmutableSet<K> keySet()
    pub fn keySet(&mut self) -> &ImmutableSet {
        // Java: ImmutableSet<K> result = this.keySet;
        // Java: return result == null ? (this.keySet = this.createKeySet()) : result;
        if self.keySet.is_none() {
            self.keySet = Some(Box::new(self.createKeySet()));
        }
        self.keySet.as_ref().unwrap()
    }

    // Java: ImmutableSet<K> createKeySet()
    pub fn createKeySet(&self) -> ImmutableSet {
        // Java: return new ImmutableMapKeySet(this);
        ImmutableSet::new()
    }

    // Java: ImmutableCollection<V> values()
    pub fn values_collection(&mut self) -> &ImmutableCollection {
        // Java: ImmutableCollection<V> result = this.values;
        // Java: return result == null ? (this.values = new ImmutableMapValues(this)) : result;
        if self.values.is_none() {
            self.values = Some(Box::new(ImmutableCollection::new()));
        }
        self.values.as_ref().unwrap()
    }

    // Java: boolean equals(Object object)
    pub fn equals(&self, _object: Option<&dyn Any>) -> bool {
        // Java: if (this == object)
        {
            // Java: return true;
        }
        // Java: else if (object instanceof Map)
        {
            // Java: Map<?, ?> o = (Map)object;
            // Java: return this.entrySet().equals(o.entrySet());
        }
        // Java: else
        {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.entrySet().hashCode();
        0
    }

    pub fn size(&self) -> usize {
        0
    }
}

// Java: public class Builder
pub struct ImmutableMapBuilder {
    // Java field: int DEFAULT_INITIAL_CAPACITY
    // Java field: ImmutableMapEntry.TerminalEntry<K, V>[] entries
    pub entries: Vec<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)>,
    // Java field: int size
    pub size: usize,
}

// Java: public class Builder
impl ImmutableMapBuilder {
    // Java field: int DEFAULT_INITIAL_CAPACITY
    pub const DEFAULT_INITIAL_CAPACITY: i32 = 4;

    // Java: Builder()
    pub fn new() -> Self {
        // Java: this(4);
        Self::with_capacity(4)
    }

    // Java: Builder(int initialCapacity)
    pub fn with_capacity(initial_capacity: i32) -> Self {
        // Java: super();
        // Java: this.entries = new ImmutableMapEntry.TerminalEntry[initialCapacity];
        // Java: this.size = 0;
        ImmutableMapBuilder {
            entries: Vec::with_capacity(initial_capacity as usize),
            size: 0,
        }
    }

    // Java: void ensureCapacity(int minCapacity)
    fn ensureCapacity(&mut self, min_capacity: usize) {
        // Java: if (minCapacity > this.entries.length)
        if min_capacity > self.entries.len() {
            // Java: this.entries = (ImmutableMapEntry.TerminalEntry[])ObjectArrays.arraysCopyOf(this.entries, ImmutableCollection.Builder.expandedCapacity(this.entries.length, minCapacity));
            let new_cap = ImmutableCollectionBuilder::expandedCapacity(self.entries.len() as i32, min_capacity as i32);
            self.entries.reserve(new_cap as usize - self.entries.len());
        }
    }

    // Java: public Builder<K, V> put(K key, V value)
    pub fn put(&mut self, key: Option<Box<dyn Any>>, value: Option<Box<dyn Any>>) -> &mut Self {
        // Java: this.ensureCapacity(this.size + 1);
        self.ensureCapacity(self.size + 1);
        // Java: ImmutableMapEntry.TerminalEntry<K, V> entry = ImmutableMap.<K, V>entryOf(key, value);
        let entry = (key, value);
        // Java: this.entries[this.size++] = entry;
        self.entries.push(entry);
        self.size += 1;
        // Java: return this;
        self
    }

    // Java: public Builder<K, V> put(Map.Entry<? extends K, ? extends V> entry)
    pub fn put_entry(&mut self, entry: (Option<Box<dyn Any>>, Option<Box<dyn Any>>)) -> &mut Self {
        // Java: return this.put(entry.getKey(), entry.getValue());
        let (k, v) = entry;
        self.put(k, v)
    }

    // Java: public Builder<K, V> putAll(Map<? extends K, ? extends V> map)
    pub fn putAll(&mut self, map: Vec<(Option<Box<dyn Any>>, Option<Box<dyn Any>>)>) -> &mut Self {
        // Java: this.ensureCapacity(this.size + map.size());
        self.ensureCapacity(self.size + map.len());
        // Java: for(Map.Entry<? extends K, ? extends V> entry : map.entrySet())
        for entry in map {
            // Java: this.put(entry);
            self.put_entry(entry);
        }
        // Java: return this;
        self
    }

    // Java: public ImmutableMap<K, V> build()
    pub fn build(&self) -> ImmutableMap {
        // Java: return new RegularImmutableMap<K, V>(this.size, this.entries);
        ImmutableMap::new()
    }
}
