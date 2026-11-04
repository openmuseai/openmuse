#![allow(dead_code, non_snake_case)]

// Java: import cn.wps.kfc.Constants
// Java: import cn.wps.kfc.HashCode
// Java: import cn.wps.kfc.api.iterator.IIntIterator
// Java: import cn.wps.kfc.api.iterator.IIntObjectIterator
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.api.procedure.IIntObjectProcedure
// Java: import cn.wps.kfc.api.procedure.IIntProcedure
// Java: import cn.wps.kfc.api.procedure.IObjectProcedure
// Java: import cn.wps.kfc.api.set.IIntSet
// Java: import cn.wps.kfc.impl.WHashPrimitiveIterator
// Java: import cn.wps.kfc.impl.WIntHash
// Java: import java.io.Serializable
// Java: import java.lang.reflect.Array
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.Map

use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WIntObjectHashMap extends WIntHash implements IIntObjectMap, Serializable
#[derive(Clone, Debug)]
pub struct WIntObjectHashMap {
    // Java field: IIntObjectProcedure<V> PUT_ALL_PROC
    // Java field: V[] _values
    // Java field: int no_entry_key
    pub(crate) no_entry_key: i32,
    inner: HashMap<i32, Element>,
}

// Java: public class WIntObjectHashMap extends WIntHash implements IIntObjectMap, Serializable
impl WIntObjectHashMap {
    // Java: WIntObjectHashMap()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.PUT_ALL_PROC = new IIntObjectProcedure<V>()
        {
            // Java: public boolean execute(int key, V value)
            {
                // Java: WIntObjectHashMap.this.put(key, value);
                // Java: return true;
            }
        }
        WIntObjectHashMap { no_entry_key: 0, inner: HashMap::new() }
    }

    // Java: WIntObjectHashMap(int initialCapacity)
    pub fn new_with_capacity(initialCapacity: usize) -> Self {
        // Java: super(initialCapacity);
        // Java: this.PUT_ALL_PROC = new IIntObjectProcedure<V>()
        {
            // Java: public boolean execute(int key, V value)
            {
                // Java: WIntObjectHashMap.this.put(key, value);
                // Java: return true;
            }
        }
        // Java: this.no_entry_key = Constants.DEFAULT_INT_NO_ENTRY_VALUE;
        WIntObjectHashMap { no_entry_key: 0, inner: HashMap::with_capacity(initialCapacity) }
    }

    // Java: WIntObjectHashMap(int initialCapacity, float loadFactor)
    pub fn new_with_load(initialCapacity: usize, loadFactor: f32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.PUT_ALL_PROC = new IIntObjectProcedure<V>()
        {
            // Java: public boolean execute(int key, V value)
            {
                // Java: WIntObjectHashMap.this.put(key, value);
                // Java: return true;
            }
        }
        // Java: this.no_entry_key = Constants.DEFAULT_INT_NO_ENTRY_VALUE;
        WIntObjectHashMap { no_entry_key: 0, inner: HashMap::with_capacity(initialCapacity) }
    }

    // Java: WIntObjectHashMap(int initialCapacity, float loadFactor, int noEntryKey)
    pub fn new_full(initialCapacity: usize, loadFactor: f32, noEntryKey: i32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.PUT_ALL_PROC = new IIntObjectProcedure<V>()
        {
            // Java: public boolean execute(int key, V value)
            {
                // Java: WIntObjectHashMap.this.put(key, value);
                // Java: return true;
            }
        }
        // Java: this.no_entry_key = noEntryKey;
        WIntObjectHashMap { no_entry_key: noEntryKey, inner: HashMap::with_capacity(initialCapacity) }
    }

    // Java: WIntObjectHashMap(IIntObjectMap<? extends V> map)
    pub fn from_map(other: &WIntObjectHashMap) -> Self {
        // Java: this(map.size(), 0.5F, map.getNoEntryKey());
        // Java: this.putAll(map);
        let mut m = Self::new_full(other.size(), 0.5, other.no_entry_key);
        m.inner = other.inner.clone();
        m
    }

    // Java: int setUp(int initialCapacity)
    fn setUp(&mut self, initialCapacity: usize) -> usize {
        // Java: int capacity = super.setUp(initialCapacity);
        // Java: this._values = (V[])(new Object[capacity]);
        // Java: return capacity;
        initialCapacity
    }

    // Java: void rehash(int newCapacity)
    fn rehash(&mut self, _newCapacity: usize) {
        // Java: int oldCapacity = this._set.length;
        // Java: _oldKeys: &[i32] = this._set;
        // Java: V[] oldVals = this._values;
        // Java: _oldStates: &[u8] = this._states;
        // Java: this._set = new int[newCapacity];
        // Java: this._values = (V[])(new Object[newCapacity]);
        // Java: this._states = new byte[newCapacity];
        // Java: int i = oldCapacity;
        // Java: while(i-- > 0)
        // Java: if (oldStates[i] == 1)
        // Java: int o = oldKeys[i];
        // Java: int index = this.insertKey(o);
        // Java: this._values[index] = oldVals[i];
    }

    // Java: int getNoEntryKey()
    pub fn getNoEntryKey(&self) -> i32 {
        // Java: return this.no_entry_key;
        self.no_entry_key
    }

    // Java: boolean containsKey(int key)
    pub fn containsKey(&self, key: i32) -> bool {
        // Java: return this.contains(key);
        self.inner.contains_key(&key)
    }

    // Java: boolean containsValue(Object val)
    pub fn containsValue(&self, val: &Element) -> bool {
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
        false
    }

    // Java: V get(int key)
    pub fn get(&self, key: i32) -> &Element {
        // Java: int index = this.index(key);
        // Java: return (V)(index < 0 ? null : this._values[index]);
        static NONE: Element = None;
        self.inner.get(&key).unwrap_or(&NONE)
    }

    // Java: V put(int key, V value)
    pub fn put(&mut self, key: i32, value: Element) -> Element {
        // Java: int index = this.insertKey(key);
        // Java: return (V)this.doPut(value, index);
        self.inner.insert(key, value).flatten()
    }

    // Java: V doPut(V value, int index)
    fn doPut(&mut self, _value: Element, _index: i32) -> Element {
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
        None
    }

    // Java: V remove(int key)
    pub fn remove(&mut self, key: i32) -> Element {
        // Java: V prev = null;
        // Java: int index = this.index(key);
        // Java: if (index >= 0)
        // Java: prev = (V)this._values[index];
        // Java: this.removeAt(index);
        // Java: return prev;
        self.inner.remove(&key).flatten()
    }

    // Java: void removeAt(int index)
    fn removeAt(&mut self, _index: usize) {
        // Java: this._values[index] = null;
        // Java: super.removeAt(index);
    }

    // Java: void putAll(Map<? extends Integer, ? extends V> map)
    pub fn putAll_map(&mut self, map: &HashMap<i32, Element>) {
        // Java: for(Map.Entry<? extends Integer, ? extends V> entry : map.entrySet())
        for (&key, value) in map.iter() {
            // Java: this.put((Integer)entry.getKey(), entry.getValue());
            self.inner.insert(key, value.clone());
        }
    }

    // Java: void putAll(IIntObjectMap<? extends V> map)
    pub fn putAll(&mut self, other: &WIntObjectHashMap) {
        // Java: map.forEachEntry(this.PUT_ALL_PROC);
        for (&key, value) in other.inner.iter() {
            self.inner.insert(key, value.clone());
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

    // Java: IIntSet keySet()
    // Java: return new KeyIntSet();

    // Java: _keys: &[i32]()
    pub fn keys(&self) -> Vec<i32> {
        // Java: _keys: &[i32] = new int[this.size()];
        // Java: _k: &[i32] = this._set;
        // Java: _states: &[u8] = this._states;
        // Java: int i = k.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: keys[j++] = k[i];
        // Java: return keys;
        self.inner.keys().copied().collect()
    }

    // Java: _keys: &[i32](_dest: &[i32])
    pub fn keys_into(&self, dest: &mut Vec<i32>) -> Vec<i32> {
        // Java: if (dest.length < this._size)
        // Java: dest = new int[this._size];
        // Java: _k: &[i32] = this._set;
        // Java: _states: &[u8] = this._states;
        // Java: int i = k.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: dest[j++] = k[i];
        // Java: return dest;
        *dest = self.inner.keys().copied().collect();
        dest.clone()
    }

    // Java: Collection<V> valueCollection()
    // Java: return new WIntObjectHashMapValueView();

    // Java: Object[] values()
    pub fn values(&self) -> Vec<Element> {
        // Java: Object[] vals = new Object[this.size()];
        // Java: V[] v = this._values;
        // Java: _states: &[u8] = this._states;
        // Java: int i = v.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1)
        // Java: vals[j++] = v[i];
        // Java: return vals;
        self.inner.values().cloned().collect()
    }

    // Java: V[] values(V[] dest)
    pub fn values_into(&self, dest: &mut Vec<Element>) -> Vec<Element> {
        // Java: if (dest.length < this._size)
        {
            // Java: dest = (V[])((Object[])Array.newInstance(dest.getClass().getComponentType(), this._size));
        }
        // Java: V[] v = this._values;
        // Java: _states: &[u8] = this._states;
        // Java: int i = v.length;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        {
            // Java: if (states[i] == 1)
            {
                // Java: dest[j++] = v[i];
            }
        }
        // Java: return dest;
        self.values()
    }

    // Java: IIntObjectIterator<V> iterator()
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, i32, Element> {
        // Java: return new WIntObjectHashIterator<V>(this);
        self.inner.iter()
    }

    // Java: boolean forEachKey(IIntProcedure procedure)
    pub fn forEachKey(&self, mut procedure: impl FnMut(i32) -> bool) -> bool {
        // Java: return this.forEach(procedure);
        for &key in self.inner.keys() {
            if !procedure(key) { return false; }
        }
        true
    }

    // Java: boolean forEachValue(IObjectProcedure<? super V> procedure)
    pub fn forEachValue(&self, mut procedure: impl FnMut(&Element) -> bool) -> bool {
        // Java: _states: &[u8] = this._states;
        // Java: V[] values = this._values;
        // Java: int i = values.length;
        // Java: while(i-- > 0)
        for value in self.inner.values() {
            // Java: if (states[i] == 1 && !procedure.execute(values[i]))
            if !procedure(value) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: boolean forEachEntry(IIntObjectProcedure<? super V> procedure)
    pub fn forEachEntry(&self, mut procedure: impl FnMut(i32, &Element) -> bool) -> bool {
        // Java: _states: &[u8] = this._states;
        // Java: _keys: &[i32] = this._set;
        // Java: V[] values = this._values;
        // Java: int i = keys.length;
        // Java: while(i-- > 0)
        for (&key, value) in self.inner.iter() {
            // Java: if (states[i] == 1 && !procedure.execute(keys[i], values[i]))
            if !procedure(key, value) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &WIntObjectHashMap) -> bool {
        // Java: if (!(other instanceof IIntObjectMap))
        {
            // Java: return false;
        }
        // Java: else
        {
            // Java: IIntObjectMap that = (IIntObjectMap)other;
            // Java: if (that.size() != this.size())
            if other.size() != self.size() {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: try
                {
                    // Java: IIntObjectIterator iter = this.iterator();
                    // Java: while(iter.hasNext())
                    {
                        // Java: iter.advance();
                        // Java: int key = iter.key();
                        // Java: Object value = iter.value();
                        // Java: if (value == null)
                        {
                            // Java: if (that.get(key) != null || !that.containsKey(key))
                            {
                                // Java: return false;
                            }
                        }
                        // Java: else if (!value.equals(that.get(key)))
                        {
                            // Java: return false;
                        }
                    }
                }
                // Java: catch (ClassCastException var6)
                {
                }
                // Java: return true;
                true
            }
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int hashcode = 0;
        let mut hashcode: i32 = 0;
        // Java: V[] values = this._values;
        // Java: _states: &[u8] = this._states;
        // Java: int i = values.length;
        // Java: while(i-- > 0)
        for (&key, _value) in self.inner.iter() {
            // Java: if (states[i] == 1)
            // Java: hashcode += HashCode.hash(this._set[i]) ^ (values[i] == null ? 0 : values[i].hashCode());
            hashcode = hashcode.wrapping_add(key);
        }
        // Java: return hashcode;
        hashcode
    }

    // Java: String toString()
    pub fn to_string_repr(&self) -> String {
        // Java: final StringBuilder buf = new StringBuilder("{");
        let mut buf = String::from("{");
        // Java: this.forEachEntry(new IIntObjectProcedure<V>()
        // Java: private boolean first = true;
        let mut first = true;
        // Java: public boolean execute(int key, Object value)
        for (&key, _value) in self.inner.iter() {
            // Java: if (this.first)
            if first {
                // Java: this.first = false;
                first = false;
            }
            // Java: else
            else {
                // Java: buf.append(",");
                buf.push(',');
            }
            // Java: buf.append(key);
            buf.push_str(&key.to_string());
            // Java: buf.append("=");
            buf.push('=');
            // Java: buf.append(value);
            buf.push_str("?");
            // Java: return true;
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

// Java: public class KeyIntSet
// Java: KeyIntSet()
// Java: super();
// Java: int getNoEntryValue()
// Java: return WIntObjectHashMap.this.no_entry_key;
// Java: int size()
// Java: return WIntObjectHashMap.this._size;
// Java: boolean isEmpty()
// Java: return WIntObjectHashMap.this._size == 0;
// Java: boolean contains(int entry)
// Java: return WIntObjectHashMap.this.containsKey(entry);
// Java: IIntIterator iterator()
// Java: return new WIntHashIterator(WIntObjectHashMap.this);
// Java: _toArray: &[i32]()
// Java: return WIntObjectHashMap.this.keys();
// Java: _toArray: &[i32](_dest: &[i32])
// Java: return WIntObjectHashMap.this.keys(dest);
// Java: boolean add(int entry)
// Java: throw new UnsupportedOperationException();
// Java: boolean remove(int entry)
// Java: return null != WIntObjectHashMap.this.remove(entry);
// Java: boolean containsAll(Collection<?> collection)
// Java: for(Object element : collection)
// Java: if (!WIntObjectHashMap.this.containsKey((Integer)element))
// Java: return false;
// Java: return true;
// Java: boolean containsAll(_array: &[i32])
// Java: for(int element : array)
// Java: if (!WIntObjectHashMap.this.containsKey(element))
// Java: return false;
// Java: return true;
// Java: boolean addAll(Collection<? extends Integer> collection)
// Java: throw new UnsupportedOperationException();
// Java: boolean addAll(_array: &[i32])
// Java: throw new UnsupportedOperationException();
// Java: boolean addAll(IIntSet intSet)
// Java: throw new UnsupportedOperationException();
// Java: boolean retainAll(Collection<?> collection)
// Java: boolean modified = false;
// Java: IIntIterator iter = this.iterator();
// Java: while(iter.hasNext())
// Java: if (!collection.contains(iter.next()))
// Java: iter.remove();
// Java: modified = true;
// Java: return modified;
// Java: boolean retainAll(_array: &[i32])
// Java: boolean changed = false;
// Java: Arrays.sort(array);
// Java: _set: &[i32] = WIntObjectHashMap.this._set;
// Java: _states: &[u8] = WIntObjectHashMap.this._states;
// Java: int i = set.length;
// Java: while(i-- > 0)
// Java: if (states[i] == 1 && Arrays.binarySearch(array, set[i]) < 0)
// Java: WIntObjectHashMap.this.removeAt(i);
// Java: changed = true;
// Java: return changed;
// Java: boolean removeAll(Collection<?> collection)
// Java: boolean changed = false;
// Java: for(Object element : collection)
// Java: if (element instanceof Integer)
// Java: int c = (Integer)element;
// Java: if (this.remove(c))
// Java: changed = true;
// Java: return changed;
// Java: boolean removeAll(_array: &[i32])
// Java: boolean changed = false;
// Java: int i = array.length;
// Java: while(i-- > 0)
// Java: if (this.remove(array[i]))
// Java: changed = true;
// Java: return changed;
// Java: void clear()
// Java: WIntObjectHashMap.this.clear();
// Java: boolean forEach(IIntProcedure procedure)
// Java: return WIntObjectHashMap.this.forEachKey(procedure);
// Java: boolean equals(Object other)
// Java: if (!(other instanceof IIntSet))
// Java: return false;
// Java: else
// Java: IIntSet that = (IIntSet)other;
// Java: if (that.size() != this.size())
// Java: return false;
// Java: else
// Java: int i = WIntObjectHashMap.this._states.length;
// Java: while(i-- > 0)
// Java: if (WIntObjectHashMap.this._states[i] == 1 && !that.contains(WIntObjectHashMap.this._set[i]))
// Java: return false;
// Java: return true;
// Java: int hashCode()
// Java: int hashcode = 0;
// Java: int i = WIntObjectHashMap.this._states.length;
// Java: while(i-- > 0)
// Java: if (WIntObjectHashMap.this._states[i] == 1)
// Java: hashcode += HashCode.hash(WIntObjectHashMap.this._set[i]);
// Java: return hashcode;
// Java: String toString()
// Java: StringBuilder buf = new StringBuilder("{");
// Java: boolean first = true;
// Java: int i = WIntObjectHashMap.this._states.length;
// Java: while(i-- > 0)
// Java: if (WIntObjectHashMap.this._states[i] == 1)
// Java: if (first)
// Java: first = false;
// Java: else
// Java: buf.append(",");
// Java: buf.append(WIntObjectHashMap.this._set[i]);
// Java: return buf.toString();

// Java: class WIntHashIterator extends WHashPrimitiveIterator implements IIntIterator
// Java: private final WIntHash _hash;
// Java: public WIntHashIterator(WIntHash hash)
// Java: super(hash);
// Java: this._hash = hash;
// Java: public int next()
// Java: this.moveToNextIndex();
// Java: return this._hash._set[this._index];

// Java: public class WIntObjectHashMapValueView
// Java: WIntObjectHashMapValueView()
// Java: super();
// Java: Iterator<V> iterator()
// Java: return new WIntObjectHashMap<V>.WIntObjectHashMapValueView.WIntObjectValueHashIterator(WIntObjectHashMap.this)
// Java: protected V objectAtIndex(int index)
// Java: return (V)WIntObjectHashMap.this._values[index];
// Java: boolean containsElement(V value)
// Java: return WIntObjectHashMap.this.containsValue(value);
// Java: void clear()
// Java: WIntObjectHashMap.this.clear();
// Java: int size()
// Java: return WIntObjectHashMap.this.size();
// Java: boolean isEmpty()
// Java: return WIntObjectHashMap.this.isEmpty();
// Java: boolean removeElement(V value)
// Java: V[] values = WIntObjectHashMap.this._values;
// Java: _states: &[u8] = WIntObjectHashMap.this._states;
// Java: int i = values.length;
// Java: while(i-- > 0)
// Java: if (states[i] == 1 && (value == values[i] || null != values[i] && values[i].equals(value)))
// Java: WIntObjectHashMap.this.removeAt(i);
// Java: return true;
// Java: return false;

// Java: public class WIntObjectValueHashIterator
// Java field: WIntObjectHashMap _map
// Java: WIntObjectValueHashIterator(WIntObjectHashMap map)
// Java: super(map);
// Java: this._map = map;
// Java: V objectAtIndex(int index)
// Java: _states: &[u8] = WIntObjectHashMap.this._states;
// Java: Object value = this._map._values[index];
// Java: return (V)(states[index] != 1 ? null : value);
// Java: V next()
// Java: this.moveToNextIndex();
// Java: return (V)this._map._values[this._index];

// Java: public class WIntObjectHashIterator
// Java field: WIntObjectHashMap<V> _map
// Java: WIntObjectHashIterator(WIntObjectHashMap<V> map)
// Java: super(map);
// Java: this._map = map;
// Java: void advance()
// Java: this.moveToNextIndex();
// Java: int key()
// Java: return this._map._set[this._index];
// Java: V value()
// Java: return (V)this._map._values[this._index];
// Java: V setValue(V val)
// Java: V old = (V)this.value();
// Java: this._map._values[this._index] = val;
// Java: return old;
