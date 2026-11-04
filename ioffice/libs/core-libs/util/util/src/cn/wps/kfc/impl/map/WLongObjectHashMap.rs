#![allow(dead_code, non_snake_case, unused_variables)]

// Java: import cn.wps.kfc.Constants
// Java: import cn.wps.kfc.HashCode
// Java: import cn.wps.kfc.api.iterator.ILongIterator
// Java: import cn.wps.kfc.api.iterator.ILongObjectIterator
// Java: import cn.wps.kfc.api.map.ILongObjectMap
// Java: import cn.wps.kfc.api.procedure.ILongObjectProcedure
// Java: import cn.wps.kfc.api.procedure.ILongProcedure
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
// Java: import cn.wps.kfc.api.set.ILongSet
// Java: import cn.wps.kfc.impl.WHashPrimitiveIterator
// Java: import cn.wps.kfc.impl.WLongHash
// Java: import java.io.Serializable
// Java: import java.lang.reflect.Array
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.Map

use std::collections::HashMap;

// Java: public class WLongObjectHashMap extends WLongHash implements ILongObjectMap, Serializable
#[derive(Clone, Debug)]
pub struct WLongObjectHashMap<V> {
    // Java field: long serialVersionUID
    // Java field: ILongObjectProcedure<V> PUT_ALL_PROC
    // Java field: V[] _values
    // Java field: long no_entry_key
    inner: HashMap<i64, V>,
    no_entry_key: i64,
}

impl<V> Default for WLongObjectHashMap<V> {
    fn default() -> Self {
        Self { inner: HashMap::new(), no_entry_key: 0 }
    }
}

impl<V> WLongObjectHashMap<V> {
    // Java: WLongObjectHashMap()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.PUT_ALL_PROC = new ILongObjectProcedure<V>()
        // Java: public boolean execute(long key, V value)
        // Java: WLongObjectHashMap.this.put(key, value);
        // Java: return true;
        Self::default()
    }

    // Java: WLongObjectHashMap(int initialCapacity)
    pub fn WLongObjectHashMap_with_capacity(initial_capacity: usize) -> Self {
        // Java: super(initialCapacity);
        // Java: this.PUT_ALL_PROC = new ILongObjectProcedure<V>()
        // Java: public boolean execute(long key, V value)
        // Java: WLongObjectHashMap.this.put(key, value);
        // Java: return true;
        // Java: this.no_entry_key = Constants.DEFAULT_LONG_NO_ENTRY_VALUE;
        Self { inner: HashMap::with_capacity(initial_capacity), no_entry_key: 0 }
    }

    // Java: WLongObjectHashMap(int initialCapacity, float loadFactor)
    pub fn WLongObjectHashMap_with_load(initial_capacity: usize, _load_factor: f32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.PUT_ALL_PROC = new ILongObjectProcedure<V>()
        // Java: public boolean execute(long key, V value)
        // Java: WLongObjectHashMap.this.put(key, value);
        // Java: return true;
        // Java: this.no_entry_key = Constants.DEFAULT_LONG_NO_ENTRY_VALUE;
        Self { inner: HashMap::with_capacity(initial_capacity), no_entry_key: 0 }
    }

    // Java: WLongObjectHashMap(int initialCapacity, float loadFactor, long noEntryKey)
    pub fn WLongObjectHashMap_with_no_entry(initial_capacity: usize, _load_factor: f32, no_entry_key: i64) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.PUT_ALL_PROC = new ILongObjectProcedure<V>()
        // Java: public boolean execute(long key, V value)
        // Java: WLongObjectHashMap.this.put(key, value);
        // Java: return true;
        // Java: this.no_entry_key = noEntryKey;
        Self { inner: HashMap::with_capacity(initial_capacity), no_entry_key }
    }

    // Java: WLongObjectHashMap(ILongObjectMap<? extends V> map)
    pub fn WLongObjectHashMap_from_map(map: &WLongObjectHashMap<V>, no_entry_key: i64) -> Self where V: Clone {
        // Java: this(map.size(), 0.5F, map.getNoEntryKey());
        // Java: this.putAll(map);
        Self { inner: map.inner.clone(), no_entry_key }
    }

    // Java: int setUp(int initialCapacity)
    pub fn setUp(&mut self, initial_capacity: usize) -> usize {
        // Java: int capacity = super.setUp(initialCapacity);
        // Java: this._values = (V[])(new Object[capacity]);
        // Java: return capacity;
        self.inner.reserve(initial_capacity);
        initial_capacity
    }

    // Java: void rehash(int newCapacity)
    pub fn rehash(&mut self, _new_capacity: usize) {
        // Java: int oldCapacity = this._set.length;
        // Java: long[] oldKeys = this._set;
        // Java: V[] oldVals = this._values;
        // Java: _oldStates: &[u8] = this._states;
        // Java: this._set = new long[newCapacity];
        // Java: this._values = (V[])(new Object[newCapacity]);
        // Java: this._states = new byte[newCapacity];
        // Java: int i = oldCapacity;
        // Java: while(i-- > 0)
        // Java: if (oldStates[i] == 1)
        // Java: long o = oldKeys[i];
        // Java: int index = this.insertKey(o);
        // Java: this._values[index] = oldVals[i];
    }

    // Java: long getNoEntryKey()
    pub fn getNoEntryKey(&self) -> i64 {
        // Java: return this.no_entry_key;
        self.no_entry_key
    }

    // Java: boolean containsKey(long key)
    pub fn containsKey(&self, key: i64) -> bool {
        // Java: return this.contains(key);
        self.inner.contains_key(&key)
    }

    // Java: boolean containsValue(Object val)
    pub fn containsValue(&self, val: &V) -> bool where V: PartialEq {
        // Java: _states: &[u8] = this._states;
        // Java: V[] vals = this._values;
        // Java: if (null == val)
        // Java: int i = vals.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && null == vals[i])
        // Java: return true;
        // Java: else
        // Java: int i = vals.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && (val == vals[i] || val.equals(vals[i])))
        // Java: return true;
        // Java: return false;
        self.inner.values().any(|v| v == val)
    }

    // Java: V get(long key)
    pub fn get(&self, key: i64) -> Option<&V> {
        // Java: int index = this.index(key);
        // Java: return (V)(index < 0 ? null : this._values[index]);
        self.inner.get(&key)
    }

    // Java: V put(long key, V value)
    pub fn put(&mut self, key: i64, value: V) -> Option<V> {
        // Java: int index = this.insertKey(key);
        // Java: return (V)this.doPut(value, index);
        self.inner.insert(key, value)
    }

    // Java: V doPut(V value, int index)
    fn doPut(&mut self, key: i64, value: V) -> Option<V> {
        // Java: V previous = null;
        // Java: boolean isNewMapping = true;
        // Java: if (index < 0)
        // Java: index = -index - 1;
        // Java: previous = (V)this._values[index];
        // Java: isNewMapping = false;
        // Java: this._values[index] = value;
        // Java: if (isNewMapping)
        // Java: this.postInsertHook(this.consumeFreeSlot);
        // Java: return previous;
        self.inner.insert(key, value)
    }

    // Java: V remove(long key)
    pub fn remove(&mut self, key: i64) -> Option<V> {
        // Java: V prev = null;
        // Java: int index = this.index(key);
        // Java: if (index >= 0)
        // Java: prev = (V)this._values[index];
        // Java: this.removeAt(index);
        // Java: return prev;
        self.inner.remove(&key)
    }

    // Java: void removeAt(int index)
    fn removeAt_impl(&mut self, key: i64) {
        // Java: this._values[index] = null;
        // Java: super.removeAt(index);
        self.inner.remove(&key);
    }

    // Java: void putAll(Map<? extends Long, ? extends V> map)
    pub fn putAll_map(&mut self, map: &HashMap<i64, V>) where V: Clone {
        // Java: for(Map.Entry<? extends Long, ? extends V> entry : map.entrySet())
        // Java: this.put((Long)entry.getKey(), entry.getValue());
        for (&k, v) in map.iter() {
            self.inner.insert(k, v.clone());
        }
    }

    // Java: void putAll(ILongObjectMap<? extends V> map)
    pub fn putAll(&mut self, map: &WLongObjectHashMap<V>) where V: Clone {
        // Java: map.forEachEntry(this.PUT_ALL_PROC);
        for (&k, v) in map.inner.iter() {
            self.inner.insert(k, v.clone());
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: super.clear();
        // Java: Arrays.fill(this._set, 0, this._set.length, this.no_entry_key);
        // Java: Arrays.fill(this._states, 0, this._states.length, (byte)0);
        // Java: Arrays.fill(this._values, 0, this._values.length, null);
        self.inner.clear();
    }

    // Java: ILongSet keySet()
    pub fn keySet(&self) -> Vec<i64> {
        // Java: return new KeyLongSet();
        self.inner.keys().copied().collect()
    }

    // Java: long[] keys()
    pub fn keys(&self) -> Vec<i64> {
        // Java: long[] keys = new long[this.size()];
        // Java: long[] k = this._set;
        // Java: _states: &[u8] = this._states;
        // Java: int i = k.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: keys[j++] = k[i];
        // Java: return keys;
        self.inner.keys().copied().collect()
    }

    // Java: long[] keys(long[] dest)
    pub fn keys_into(&self, dest: &mut Vec<i64>) {
        // Java: if (dest.length < this._size)
        // Java: dest = new long[this._size];
        // Java: long[] k = this._set;
        // Java: _states: &[u8] = this._states;
        // Java: int i = k.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: dest[j++] = k[i];
        // Java: return dest;
        dest.clear();
        dest.extend(self.inner.keys().copied());
    }

    // Java: Collection<V> valueCollection()
    pub fn valueCollection(&self) -> Vec<&V> {
        // Java: return new WLongObjectHashMapValueView();
        self.inner.values().collect()
    }

    // Java: Object[] values()
    pub fn values(&self) -> Vec<&V> {
        // Java: Object[] vals = new Object[this.size()];
        // Java: V[] v = this._values;
        // Java: _states: &[u8] = this._states;
        // Java: int i = v.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: vals[j++] = v[i];
        // Java: return vals;
        self.inner.values().collect()
    }

    // Java: V[] values(V[] dest)
    pub fn values_into(&self, dest: &mut Vec<V>) where V: Clone {
        // Java: if (dest.length < this._size)
        // Java: dest = (V[])((Object[])Array.newInstance(dest.getClass().getComponentType(), this._size));
        // Java: V[] v = this._values;
        // Java: _states: &[u8] = this._states;
        // Java: int i = v.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: dest[j++] = v[i];
        // Java: return dest;
        dest.clear();
        dest.extend(self.inner.values().cloned());
    }

    // Java: ILongObjectIterator<V> iterator()
    pub fn iterator(&self) -> std::collections::hash_map::Iter<'_, i64, V> {
        // Java: return new WLongObjectHashIterator<V>(this);
        self.inner.iter()
    }

    // Java: boolean forEachKey(ILongProcedure procedure)
    pub fn forEachKey(&self, procedure: &mut dyn FnMut(i64) -> bool) -> bool {
        // Java: return this.forEach(procedure);
        for &key in self.inner.keys() {
            if !procedure(key) { return false; }
        }
        true
    }

    // Java: boolean forEachValue(IObjectProcedure<? super V> procedure)
    pub fn forEachValue(&self, procedure: &mut dyn FnMut(&V) -> bool) -> bool {
        // Java: _states: &[u8] = this._states;
        // Java: V[] values = this._values;
        // Java: int i = values.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && !procedure.execute(values[i]))
        // Java: return false;
        // Java: return true;
        for value in self.inner.values() {
            if !procedure(value) { return false; }
        }
        true
    }

    // Java: boolean forEachEntry(ILongObjectProcedure<? super V> procedure)
    pub fn forEachEntry(&self, procedure: &mut dyn FnMut(i64, &V) -> bool) -> bool {
        // Java: _states: &[u8] = this._states;
        // Java: long[] keys = this._set;
        // Java: V[] values = this._values;
        // Java: int i = keys.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && !procedure.execute(keys[i], values[i]))
        // Java: return false;
        // Java: return true;
        for (&key, value) in self.inner.iter() {
            if !procedure(key, value) { return false; }
        }
        true
    }

    // Java: boolean retainEntries(ILongObjectProcedure<? super V> procedure)
    pub fn retainEntries(&mut self, procedure: &mut dyn FnMut(i64, &V) -> bool) -> bool {
        // Java: boolean modified = false;
        // Java: _states: &[u8] = this._states;
        // Java: long[] keys = this._set;
        // Java: V[] values = this._values;
        // Java: this.tempDisableAutoCompaction();
        // Java: try
        // Java: int i = keys.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && !procedure.execute(keys[i], values[i]))
        // Java: this.removeAt(i);
        // Java: modified = true;
        // Java: finally
        // Java: this.reenableAutoCompaction(true);
        // Java: return modified;
        let keys_to_remove: Vec<i64> = self.inner.iter()
            .filter(|(&k, v)| !procedure(k, v))
            .map(|(&k, _)| k)
            .collect();
        let modified = !keys_to_remove.is_empty();
        for key in keys_to_remove { self.inner.remove(&key); }
        modified
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &WLongObjectHashMap<V>) -> bool where V: PartialEq {
        // Java: if (!(other instanceof ILongObjectMap))
        // Java: return false;
        // Java: else
        // Java: ILongObjectMap that = (ILongObjectMap)other;
        // Java: if (that.size() != this.size())
        // Java: return false;
        // Java: else
        // Java: try
        // Java: ILongObjectIterator iter = this.iterator();
        // Java: while(iter.hasNext())
        // Java: iter.advance();
        // Java: long key = iter.key();
        // Java: Object value = iter.value();
        // Java: if (value == null)
        // Java: if (that.get(key) != null || !that.containsKey(key))
        // Java: return false;
        // Java: else if (!value.equals(that.get(key)))
        // Java: return false;
        // Java: catch (ClassCastException var7)
        // Java: return true;
        self.inner == other.inner
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 where V: std::hash::Hash {
        // Java: int hashcode = 0;
        // Java: V[] values = this._values;
        // Java: _states: &[u8] = this._states;
        // Java: int i = values.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: hashcode += HashCode.hash(this._set[i]) ^ (values[i] == null ? 0 : values[i].hashCode());
        // Java: return hashcode;
        use std::hash::{Hash, Hasher};
        let mut hashcode: i32 = 0;
        for (&key, value) in self.inner.iter() {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            value.hash(&mut hasher);
            let vh = hasher.finish() as i32;
            hashcode = hashcode.wrapping_add((key ^ (key >> 32)) as i32 ^ vh);
        }
        hashcode
    }

    // Java: String toString()
    pub fn toString(&self) -> String where V: std::fmt::Display {
        // Java: final StringBuilder buf = new StringBuilder("{");
        // Java: this.forEachEntry(new ILongObjectProcedure<V>()
        // Java: private boolean first = true;
        // Java: public boolean execute(long key, Object value)
        // Java: if (this.first)
        // Java: this.first = false;
        // Java: else
        // Java: buf.append(",");
        // Java: buf.append(key);
        // Java: buf.append("=");
        // Java: buf.append(value);
        // Java: return true;
        // Java: buf.append("}");
        // Java: return buf.toString();
        let mut buf = String::from("{");
        let mut first = true;
        for (&key, value) in self.inner.iter() {
            if first { first = false; } else { buf.push(','); }
            buf.push_str(&format!("{}={}", key, value));
        }
        buf.push('}');
        buf
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub fn isEmpty(&self) -> bool {
        self.inner.is_empty()
    }
}

// Java: public class KeyLongSet
impl<V> WLongObjectHashMap<V> {
    // Java: long getNoEntryValue()
    pub fn keySet_getNoEntryValue(&self) -> i64 {
        // Java: return WLongObjectHashMap.this.no_entry_key;
        self.no_entry_key
    }

    // Java: int size()
    pub fn keySet_size(&self) -> i32 {
        // Java: return WLongObjectHashMap.this._size;
        self.inner.len() as i32
    }

    // Java: boolean isEmpty()
    pub fn keySet_isEmpty(&self) -> bool {
        // Java: return WLongObjectHashMap.this._size == 0;
        self.inner.is_empty()
    }

    // Java: boolean contains(long entry)
    pub fn keySet_contains(&self, entry: i64) -> bool {
        // Java: return WLongObjectHashMap.this.containsKey(entry);
        self.inner.contains_key(&entry)
    }

    // Java: ILongIterator iterator()
    pub fn keySet_iterator(&self) -> impl Iterator<Item = &i64> {
        // Java: return new WLongHashIterator(WLongObjectHashMap.this);
        self.inner.keys()
    }

    // Java: long[] toArray()
    pub fn keySet_toArray(&self) -> Vec<i64> {
        // Java: return WLongObjectHashMap.this.keys();
        self.keys()
    }

    // Java: long[] toArray(long[] dest)
    pub fn keySet_toArray_into(&self, dest: &mut Vec<i64>) {
        // Java: return WLongObjectHashMap.this.keys(dest);
        self.keys_into(dest);
    }

    // Java: boolean add(long entry)
    pub fn keySet_add(&self, _entry: i64) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException")
    }

    // Java: boolean remove(long entry)
    pub fn keySet_remove(&mut self, entry: i64) -> bool {
        // Java: return null != WLongObjectHashMap.this.remove(entry);
        self.inner.remove(&entry).is_some()
    }

    // Java: boolean containsAll(Collection<?> collection)
    pub fn keySet_containsAll(&self, collection: &[i64]) -> bool {
        // Java: for(Object element : collection)
        // Java: if (!WLongObjectHashMap.this.containsKey((Long)element))
        // Java: return false;
        // Java: return true;
        collection.iter().all(|e| self.inner.contains_key(e))
    }

    // Java: boolean containsAll(long[] array)
    pub fn keySet_containsAll_array(&self, array: &[i64]) -> bool {
        // Java: for(long element : array)
        // Java: if (!WLongObjectHashMap.this.containsKey(element))
        // Java: return false;
        // Java: return true;
        array.iter().all(|e| self.inner.contains_key(e))
    }

    // Java: boolean addAll(Collection<? extends Long> collection)
    pub fn keySet_addAll(&self, _collection: &[i64]) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException")
    }

    // Java: boolean addAll(long[] array)
    pub fn keySet_addAll_array(&self, _array: &[i64]) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException")
    }

    // Java: boolean addAll(ILongSet longSet)
    pub fn keySet_addAll_set(&self, _long_set: &[i64]) -> bool {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException")
    }

    // Java: boolean retainAll(Collection<?> collection)
    pub fn keySet_retainAll(&mut self, collection: &[i64]) -> bool {
        // Java: boolean modified = false;
        // Java: ILongIterator iter = this.iterator();
        // Java: while(iter.hasNext())
        // Java: if (!collection.contains(iter.next()))
        // Java: iter.remove();
        // Java: modified = true;
        // Java: return modified;
        let before = self.inner.len();
        self.inner.retain(|k, _| collection.contains(k));
        self.inner.len() != before
    }

    // Java: boolean retainAll(long[] array)
    pub fn keySet_retainAll_array(&mut self, array: &[i64]) -> bool {
        // Java: boolean changed = false;
        // Java: Arrays.sort(array);
        // Java: long[] set = WLongObjectHashMap.this._set;
        // Java: _states: &[u8] = WLongObjectHashMap.this._states;
        // Java: int i = set.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && Arrays.binarySearch(array, set[i]) < 0)
        // Java: WLongObjectHashMap.this.removeAt(i);
        // Java: changed = true;
        // Java: return changed;
        let mut sorted = array.to_vec();
        sorted.sort();
        let before = self.inner.len();
        self.inner.retain(|k, _| sorted.binary_search(k).is_ok());
        self.inner.len() != before
    }

    // Java: boolean removeAll(Collection<?> collection)
    pub fn keySet_removeAll(&mut self, collection: &[i64]) -> bool {
        // Java: boolean changed = false;
        // Java: for(Object element : collection)
        // Java: if (element instanceof Long)
        // Java: long c = (Long)element;
        // Java: if (this.remove(c))
        // Java: changed = true;
        // Java: return changed;
        let mut changed = false;
        for &e in collection {
            if self.inner.remove(&e).is_some() { changed = true; }
        }
        changed
    }

    // Java: boolean removeAll(long[] array)
    pub fn keySet_removeAll_array(&mut self, array: &[i64]) -> bool {
        // Java: boolean changed = false;
        // Java: int i = array.length;
        // Java: while(i-- > 0)
        // Java: if (this.remove(array[i]))
        // Java: changed = true;
        // Java: return changed;
        let mut changed = false;
        for &e in array {
            if self.inner.remove(&e).is_some() { changed = true; }
        }
        changed
    }

    // Java: void clear()
    pub fn keySet_clear(&mut self) {
        // Java: WLongObjectHashMap.this.clear();
        self.inner.clear();
    }

    // Java: boolean forEach(ILongProcedure procedure)
    pub fn keySet_forEach(&self, procedure: &mut dyn FnMut(i64) -> bool) -> bool {
        // Java: return WLongObjectHashMap.this.forEachKey(procedure);
        self.forEachKey(procedure)
    }

    // Java: boolean equals(Object other)
    pub fn keySet_equals(&self, other: &WLongObjectHashMap<V>) -> bool {
        // Java: if (!(other instanceof ILongSet))
        // Java: return false;
        // Java: else
        // Java: ILongSet that = (ILongSet)other;
        // Java: if (that.size() != this.size())
        // Java: return false;
        // Java: else
        // Java: int i = WLongObjectHashMap.this._states.length;
        // Java: while(i-- > 0)
        // Java: if (WLongObjectHashMap.this._states[i] == 1 && !that.contains(WLongObjectHashMap.this._set[i]))
        // Java: return false;
        // Java: return true;
        self.inner.len() == other.inner.len()
            && self.inner.keys().all(|k| other.inner.contains_key(k))
    }

    // Java: int hashCode()
    pub fn keySet_hashCode(&self) -> i32 {
        // Java: int hashcode = 0;
        // Java: int i = WLongObjectHashMap.this._states.length;
        // Java: while(i-- > 0)
        // Java: if (WLongObjectHashMap.this._states[i] == 1)
        // Java: hashcode += HashCode.hash(WLongObjectHashMap.this._set[i]);
        // Java: return hashcode;
        let mut hashcode: i32 = 0;
        for &key in self.inner.keys() {
            hashcode = hashcode.wrapping_add((key ^ (key >> 32)) as i32);
        }
        hashcode
    }

    // Java: String toString()
    pub fn keySet_toString(&self) -> String {
        // Java: StringBuilder buf = new StringBuilder("{");
        // Java: boolean first = true;
        // Java: int i = WLongObjectHashMap.this._states.length;
        // Java: while(i-- > 0)
        // Java: if (WLongObjectHashMap.this._states[i] == 1)
        // Java: if (first)
        // Java: first = false;
        // Java: else
        // Java: buf.append(",");
        // Java: buf.append(WLongObjectHashMap.this._set[i]);
        // Java: return buf.toString();
        let mut buf = String::from("{");
        let mut first = true;
        for &key in self.inner.keys() {
            if first { first = false; } else { buf.push(','); }
            buf.push_str(&key.to_string());
        }
        buf.push('}');
        buf
    }
}

// Java: class WLongHashIterator extends WHashPrimitiveIterator implements ILongIterator
// Java: private final WLongHash _hash;

// Java: public WLongHashIterator(WLongHash hash)
// Java: super(hash);
// Java: this._hash = hash;

// Java: public long next()
// Java: this.moveToNextIndex();
// Java: return this._hash._set[this._index];

// Java: public class WLongObjectHashMapValueView

// Java: WLongObjectHashMapValueView()
// Java: super();

// Java: Iterator<V> iterator()
// Java: return new WLongObjectHashMap<V>.WLongObjectHashMapValueView.WLongObjectValueHashIterator(WLongObjectHashMap.this)
// Java: protected V objectAtIndex(int index)
// Java: return (V)WLongObjectHashMap.this._values[index];

// Java: boolean containsElement(V value)
// Java: return WLongObjectHashMap.this.containsValue(value);

// Java: void clear()
// Java: WLongObjectHashMap.this.clear();

// Java: int size()
// Java: return WLongObjectHashMap.this.size();

// Java: boolean isEmpty()
// Java: return WLongObjectHashMap.this.isEmpty();

// Java: boolean removeElement(V value)
// Java: V[] values = WLongObjectHashMap.this._values;
// Java: _states: &[u8] = WLongObjectHashMap.this._states;
// Java: int i = values.length;
// Java: while(i-- > 0)
// Java: if (states[i] == 1 && (value == values[i] || null != values[i] && values[i].equals(value)))
// Java: WLongObjectHashMap.this.removeAt(i);
// Java: return true;
// Java: return false;

// Java: public class WLongObjectValueHashIterator
// Java field: WLongObjectHashMap _map

// Java: WLongObjectValueHashIterator(WLongObjectHashMap map)
// Java: super(map);
// Java: this._map = map;

// Java: V objectAtIndex(int index)
// Java: _states: &[u8] = WLongObjectHashMap.this._states;
// Java: Object value = this._map._values[index];
// Java: return (V)(states[index] != 1 ? null : value);

// Java: V next()
// Java: this.moveToNextIndex();
// Java: return (V)this._map._values[this._index];

// Java: public class WLongObjectHashIterator
// Java field: WLongObjectHashMap<V> _map

// Java: WLongObjectHashIterator(WLongObjectHashMap<V> map)
// Java: super(map);
// Java: this._map = map;

// Java: void advance()
// Java: this.moveToNextIndex();

// Java: long key()
// Java: return this._map._set[this._index];

// Java: V value()
// Java: return (V)this._map._values[this._index];

// Java: V setValue(V val)
// Java: V old = (V)this.value();
// Java: this._map._values[this._index] = val;
// Java: return old;
