#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.HashCode
use crate::kfc::HashCode::HashCode;
// Java: import cn.wps.kfc.api.iterator.ILongIterator
use crate::kfc::api::iterator::ILongIterator::ILongIterator;
// Java: import cn.wps.kfc.api.iterator.IIterator::IIterator;
use crate::kfc::api::iterator::IIterator::IIterator;
// Java: import cn.wps.kfc.api.set.ILongSet
use crate::kfc::api::set::ILongSet::ILongSet;
// Java: import cn.wps.kfc.impl.WHashPrimitiveIterator
use crate::kfc::r#impl::WHashPrimitiveIterator::WHashPrimitiveIterator;
// Java: import cn.wps.kfc.impl.WLongHash
use crate::kfc::r#impl::WLongHash::WLongHash;
// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection

use std::collections::HashSet;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WLongHashSet extends WLongHash implements ILongSet, Serializable
pub struct WLongHashSet {
    data: HashSet<i64>,
    no_entry_value: i64,
}

// Java: public class WLongHashIterator
pub struct WLongHashIterator {
    items: Vec<i64>,
    cursor: usize,
    // Java field: WLongHash _hash
}

// Java: public class WLongHashSet extends WLongHash implements ILongSet, Serializable
impl WLongHashSet {
    // Java: WLongHashSet()
    pub fn new() -> Self {
        // Java: super();
        Self { data: HashSet::new(), no_entry_value: 0 }
    }

    // Java: WLongHashSet(int initialCapacity)
    pub fn new_with_capacity(initial_capacity: i32) -> Self {
        // Java: super(initialCapacity);
        Self { data: HashSet::with_capacity(initial_capacity as usize), no_entry_value: 0 }
    }

    // Java: WLongHashSet(int initialCapacity, float load_factor)
    pub fn new_with_capacity_and_load(initial_capacity: i32, _load_factor: f32) -> Self {
        // Java: super(initialCapacity, load_factor);
        Self { data: HashSet::with_capacity(initial_capacity as usize), no_entry_value: 0 }
    }

    // Java: WLongHashSet(int initial_capacity, float load_factor, long no_entry_value)
    pub fn new_with_capacity_load_entry(initial_capacity: i32, _load_factor: f32, no_entry_value: i64) -> Self {
        // Java: super(initial_capacity, load_factor, no_entry_value);
        // Java: if (no_entry_value != 0L)
        // Java: Arrays.fill(this._set, no_entry_value);
        Self { data: HashSet::with_capacity(initial_capacity as usize), no_entry_value }
    }

    // Java: WLongHashSet(Collection<? extends Long> collection)
    pub fn new_from_collection(collection: &[i64]) -> Self {
        // Java: this(Math.max(collection.size(), 10));
        let mut s = Self::new_with_capacity(std::cmp::max(collection.len() as i32, 10));
        // Java: this.addAll(collection);
        s.addAll_collection(collection);
        s
    }

    // Java: WLongHashSet(long[] array)
    pub fn new_from_array(array: &[i64]) -> Self {
        // Java: this(Math.max(array.length, 10));
        let mut s = Self::new_with_capacity(std::cmp::max(array.len() as i32, 10));
        // Java: this.addAll(array);
        s.addAll_array(array);
        s
    }

    // Java: WLongHashSet(ILongSet longSet)
    pub fn new_from_set(long_set: &dyn ILongSet) -> Self {
        // Java: this(Math.max(longSet.size(), 10));
        let mut s = Self::new_with_capacity(std::cmp::max(long_set.size(), 10));
        // Java: if (longSet instanceof WLongHashSet)
        // Java: WLongHashSet hashset = (WLongHashSet)longSet;
        // Java: this._loadFactor = hashset._loadFactor;
        // Java: this.no_entry_value = hashset.no_entry_value;
        // Java: if (this.no_entry_value != 0L)
        // Java: Arrays.fill(this._set, this.no_entry_value);
        // Java: this.setUp((int)Math.ceil((double)(10.0F / this._loadFactor)));
        // Java: this.addAll(longSet);
        s.addAll_set(long_set);
        s
    }

    // Java: ILongIterator iterator()
    pub fn iterator(&self) -> WLongHashIterator {
        // Java: return new WLongHashIterator(this);
        WLongHashIterator {
            items: self.data.iter().copied().collect(),
            cursor: 0,
        }
    }

    // Java: long[] toArray()
    pub fn toArray(&self) -> Vec<i64> {
        // Java: long[] result = new long[this.size()];
        let mut result = Vec::with_capacity(self.size() as usize);
        // Java: long[] set = this._set;
        let set: Vec<i64> = self.data.iter().copied().collect();
        // Java: byte[] states = this._states;
        // Java: int i = states.length;
        let mut i = set.len() as i32;
        // Java: int j = 0;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (states[i] == 1)
            {
                // Java: result[j++] = set[i];
                result.push(set[i as usize]);
            }
        }
        // Java: return result;
        result
    }

    // Java: long[] toArray(long[] dest)
    pub fn toArray_into(&self, dest: &mut [i64]) -> Vec<i64> {
        // Java: long[] set = this._set;
        // Java: byte[] states = this._states;
        // Java: int i = states.length;
        // Java: int j = 0;
        let mut j = 0usize;
        // Java: while(i-- > 0)
        for &val in &self.data
        {
            // Java: if (states[i] == 1)
            // Java: dest[j++] = set[i];
            if j < dest.len() {
                dest[j] = val;
            }
            j += 1;
        }
        // Java: if (dest.length > this._size)
        if dest.len() as i32 > self.size()
        {
            // Java: dest[this._size] = this.no_entry_value;
            dest[self.size() as usize] = self.no_entry_value;
        }
        // Java: return dest;
        dest.to_vec()
    }

    // Java: boolean add(long val)
    pub fn add(&mut self, val: i64) -> bool {
        // Java: int index = this.insertKey(val);
        // Java: if (index < 0)
        // Java: return false;
        // Java: else
        // Java: this.postInsertHook(this.consumeFreeSlot);
        // Java: return true;
        self.data.insert(val)
    }

    // Java: boolean remove(long val)
    pub fn remove(&mut self, val: i64) -> bool {
        // Java: int index = this.index(val);
        // Java: if (index >= 0)
        // Java: this.removeAt(index);
        // Java: return true;
        // Java: else
        // Java: return false;
        self.data.remove(&val)
    }

    pub fn contains(&self, val: i64) -> bool {
        self.data.contains(&val)
    }

    pub fn size(&self) -> i32 {
        self.data.len() as i32
    }

    // Java: boolean containsAll(Collection<?> collection)
    pub fn containsAll_collection(&self, collection: &[i64]) -> bool {
        // Java: for(Object element : collection)
        for &element in collection
        {
            // Java: if (!(element instanceof Long))
            // Java: return false;
            // Java: long c = (Long)element;
            // Java: if (!this.contains(c))
            if !self.contains(element)
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: boolean containsAll(long[] array)
    pub fn containsAll_array(&self, array: &[i64]) -> bool {
        // Java: int i = array.length;
        let mut i = array.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (!this.contains(array[i]))
            if !self.contains(array[i as usize])
            {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: boolean addAll(Collection<? extends Long> collection)
    pub fn addAll_collection(&mut self, collection: &[i64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Long element : collection)
        for &element in collection
        {
            // Java: long e = element;
            // Java: if (this.add(e))
            if self.add(element)
            {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean addAll(long[] array)
    pub fn addAll_array(&mut self, array: &[i64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: int i = array.length;
        let mut i = array.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (this.add(array[i]))
            if self.add(array[i as usize])
            {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean addAll(ILongSet longSet)
    pub fn addAll_set(&mut self, long_set: &dyn ILongSet) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: ILongIterator iter = longSet.iterator();
        let arr = long_set.toArray();
        // Java: while(iter.hasNext())
        for &element in &arr
        {
            // Java: long element = iter.next();
            // Java: if (this.add(element))
            if self.add(element)
            {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean retainAll(Collection<?> collection)
    pub fn retainAll_collection(&mut self, collection: &[i64]) -> bool {
        // Java: boolean modified = false;
        let mut modified = false;
        // Java: ILongIterator iter = this.iterator();
        let arr = self.toArray();
        // Java: while(iter.hasNext())
        for &val in &arr
        {
            // Java: if (!collection.contains(iter.next()))
            if !collection.contains(&val)
            {
                // Java: iter.remove();
                self.remove(val);
                // Java: modified = true;
                modified = true;
            }
        }
        // Java: return modified;
        modified
    }

    // Java: boolean retainAll(long[] array)
    pub fn retainAll_array(&mut self, array: &mut [i64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: Arrays.sort(array);
        array.sort();
        // Java: long[] set = this._set;
        // Java: byte[] states = this._states;
        // Java: this._autoCompactTemporaryDisable = true;
        // Java: int i = set.length;
        let arr = self.toArray();
        // Java: while(i-- > 0)
        for &val in &arr
        {
            // Java: if (states[i] == 1 && Arrays.binarySearch(array, set[i]) < 0)
            if array.binary_search(&val).is_err()
            {
                // Java: this.removeAt(i);
                self.remove(val);
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: this._autoCompactTemporaryDisable = false;
        // Java: return changed;
        changed
    }

    // Java: boolean removeAll(Collection<?> collection)
    pub fn removeAll_collection(&mut self, collection: &[i64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Object element : collection)
        for &element in collection
        {
            // Java: if (element instanceof Long)
            {
                // Java: long c = (Long)element;
                // Java: if (this.remove(c))
                if self.remove(element)
                {
                    // Java: changed = true;
                    changed = true;
                }
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean removeAll(long[] array)
    pub fn removeAll_array(&mut self, array: &[i64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: int i = array.length;
        let mut i = array.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (this.remove(array[i]))
            if self.remove(array[i as usize])
            {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: super.clear();
        // Java: long[] set = this._set;
        // Java: byte[] states = this._states;
        // Java: for(int i = set.length; i-- > 0; states[i] = 0)
        // Java: set[i] = this.no_entry_value;
        self.data.clear();
    }

    // Java: void rehash(int newCapacity)
    pub fn rehash(&mut self, _new_capacity: i32) {
        // Java: int oldCapacity = this._set.length;
        // Java: long[] oldSet = this._set;
        // Java: byte[] oldStates = this._states;
        // Java: this._set = new long[newCapacity];
        // Java: this._states = new byte[newCapacity];
        // Java: int i = oldCapacity;
        // Java: while(i-- > 0)
        // Java: if (oldStates[i] == 1)
        // Java: long o = oldSet[i];
        // Java: this.insertKey(o);
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &dyn ILongSet) -> bool {
        // Java: if (!(other instanceof ILongSet))
        // Java: return false;
        // Java: else
        {
            // Java: ILongSet that = (ILongSet)other;
            // Java: if (that.size() != this.size())
            if other.size() != self.size()
            {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: int i = this._states.length;
                // Java: while(i-- > 0)
                for &val in &self.data
                {
                    // Java: if (this._states[i] == 1 && !that.contains(this._set[i]))
                    if !other.contains(val)
                    {
                        // Java: return false;
                        return false;
                    }
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
        // Java: int i = this._states.length;
        // Java: while(i-- > 0)
        for &val in &self.data
        {
            // Java: if (this._states[i] == 1)
            // Java: hashcode += HashCode.hash(this._set[i]);
            hashcode = hashcode.wrapping_add(HashCode::hash_long(val));
        }
        // Java: return hashcode;
        hashcode
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder buffy = new StringBuilder(this._size * 2 + 2);
        let mut buffy = String::with_capacity((self.size() * 2 + 2) as usize);
        // Java: buffy.append("{");
        buffy.push('{');
        // Java: int i = this._states.length;
        // Java: int j = 1;
        let mut j = 1;
        let size = self.size();
        // Java: while(i-- > 0)
        for &val in &self.data
        {
            // Java: if (this._states[i] == 1)
            {
                // Java: buffy.append(this._set[i]);
                buffy.push_str(&val.to_string());
                // Java: if (j++ < this._size)
                if j < size
                {
                    // Java: buffy.append(",");
                    buffy.push(',');
                }
                j += 1;
            }
        }
        // Java: buffy.append("}");
        buffy.push('}');
        // Java: return buffy.toString();
        buffy
    }
}

// Java: public class WLongHashIterator
impl WLongHashIterator {
    // Java: WLongHashIterator(WLongHash hash)
    pub fn new(items: Vec<i64>) -> Self {
        // Java: super(hash);
        // Java: this._hash = hash;
        WLongHashIterator { items, cursor: 0 }
    }

    // Java: long next()
    pub fn next(&mut self) -> i64 {
        // Java: this.moveToNextIndex();
        // Java: return this._hash._set[this._index];
        let val = self.items[self.cursor];
        self.cursor += 1;
        val
    }

    pub fn hasNext(&self) -> bool {
        self.cursor < self.items.len()
    }

    pub fn remove(&mut self) {
        // no-op
    }
}
