#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.HashCode
use crate::kfc::HashCode::HashCode;
// Java: import cn.wps.kfc.api.iterator.IIntIterator
use crate::kfc::api::iterator::IIntIterator::IIntIterator;
// Java: import cn.wps.kfc.api.iterator.IIterator::IIterator;
use crate::kfc::api::iterator::IIterator::IIterator;
// Java: import cn.wps.kfc.api.set.IIntSet
use crate::kfc::api::set::IIntSet::IIntSet;
// Java: import cn.wps.kfc.impl.WHashPrimitiveIterator
use crate::kfc::r#impl::WHashPrimitiveIterator::WHashPrimitiveIterator;
// Java: import cn.wps.kfc.impl.WIntHash
use crate::kfc::r#impl::WIntHash::WIntHash;
// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WIntHashSet extends WIntHash implements IIntSet, Serializable
pub struct WIntHashSet {
    pub(crate) base: WIntHash,
}

// Java: public class WIntHashIterator
pub struct WIntHashIterator<'a> {
    base: WHashPrimitiveIterator,
    // Java field: WIntHash _hash
    _hash: &'a WIntHash,
}

// Java: public class WIntHashSet extends WIntHash implements IIntSet, Serializable
impl WIntHashSet {
    // Java: WIntHashSet()
    pub fn new() -> Self {
        // Java: super();
        Self { base: WIntHash::new() }
    }

    // Java: WIntHashSet(int initialCapacity)
    pub fn new_with_capacity(initial_capacity: i32) -> Self {
        // Java: super(initialCapacity);
        Self { base: WIntHash::new_with_capacity(initial_capacity) }
    }

    // Java: WIntHashSet(int initialCapacity, float load_factor)
    pub fn new_with_capacity_and_load(initial_capacity: i32, load_factor: f32) -> Self {
        // Java: super(initialCapacity, load_factor);
        Self { base: WIntHash::new_with_capacity_and_load(initial_capacity, load_factor) }
    }

    // Java: WIntHashSet(int initial_capacity, float load_factor, int no_entry_value)
    pub fn new_with_capacity_load_entry(initial_capacity: i32, load_factor: f32, no_entry_value: i32) -> Self {
        // Java: super(initial_capacity, load_factor, no_entry_value);
        let mut s = Self { base: WIntHash::new_with_all(initial_capacity, load_factor, no_entry_value) };
        // Java: if (no_entry_value != 0)
        if no_entry_value != 0
        {
            // Java: Arrays.fill(this._set, no_entry_value);
            s.base._set.fill(no_entry_value);
        }
        s
    }

    // Java: WIntHashSet(Collection<? extends Integer> collection)
    pub fn new_from_collection(collection: &[i32]) -> Self {
        // Java: this(Math.max(collection.size(), 10));
        let mut s = Self::new_with_capacity(std::cmp::max(collection.len() as i32, 10));
        // Java: this.addAll(collection);
        s.addAll_collection(collection);
        s
    }

    // Java: WIntHashSet(int[] array)
    pub fn new_from_array(array: &[i32]) -> Self {
        // Java: this(Math.max(array.length, 10));
        let mut s = Self::new_with_capacity(std::cmp::max(array.len() as i32, 10));
        // Java: this.addAll(array);
        s.addAll_array(array);
        s
    }

    // Java: WIntHashSet(IIntSet intSet)
    pub fn new_from_set(int_set: &dyn IIntSet) -> Self {
        // Java: this(Math.max(intSet.size(), 10));
        let mut s = Self::new_with_capacity(std::cmp::max(int_set.size(), 10));
        // Java: if (intSet instanceof WIntHashSet)
        // Java: WIntHashSet hashset = (WIntHashSet)intSet;
        // Java: this._loadFactor = hashset._loadFactor;
        // Java: this.no_entry_value = hashset.no_entry_value;
        // Java: if (this.no_entry_value != 0)
        // Java: Arrays.fill(this._set, this.no_entry_value);
        // Java: this.setUp((int)Math.ceil((double)(10.0F / this._loadFactor)));
        // Java: this.addAll(intSet);
        s.addAll_set(int_set);
        s
    }

    // Java: IIntIterator iterator()
    pub fn iterator(&self) -> WIntHashIterator {
        // Java: return new WIntHashIterator(this);
        WIntHashIterator {
            base: WHashPrimitiveIterator::WHashPrimitiveIterator_new(
                self.base._size as usize,
                self.base._set.len(),
            ),
            _hash: &self.base,
        }
    }

    // Java: int[] toArray()
    pub fn toArray(&self) -> Vec<i32> {
        // Java: int[] result = new int[this.size()];
        let mut result = vec![0i32; self.size() as usize];
        // Java: int[] set = this._set;
        let set = &self.base._set;
        // Java: byte[] states = this._states;
        let states = &self.base._states;
        // Java: int i = states.length;
        let mut i = states.len() as i32;
        // Java: int j = 0;
        let mut j = 0usize;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (states[i] == 1)
            if states[i as usize] == 1
            {
                // Java: result[j++] = set[i];
                result[j] = set[i as usize];
                j += 1;
            }
        }
        // Java: return result;
        result
    }

    // Java: int[] toArray(int[] dest)
    pub fn toArray_into(&self, dest: &mut [i32]) -> Vec<i32> {
        // Java: int[] set = this._set;
        let set = &self.base._set;
        // Java: byte[] states = this._states;
        let states = &self.base._states;
        // Java: int i = states.length;
        let mut i = states.len() as i32;
        // Java: int j = 0;
        let mut j = 0usize;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (states[i] == 1)
            if states[i as usize] == 1
            {
                // Java: dest[j++] = set[i];
                dest[j] = set[i as usize];
                j += 1;
            }
        }
        // Java: if (dest.length > this._size)
        if dest.len() as i32 > self.base._size
        {
            // Java: dest[this._size] = this.no_entry_value;
            dest[self.base._size as usize] = self.base.no_entry_value;
        }
        // Java: return dest;
        dest.to_vec()
    }

    // Java: boolean add(int val)
    pub fn add(&mut self, val: i32) -> bool {
        // Java: int index = this.insertKey(val);
        let index = self.base.insertKey(val);
        // Java: if (index < 0)
        if index < 0
        {
            // Java: return false;
            false
        }
        // Java: else
        else
        {
            // Java: this.postInsertHook(this.consumeFreeSlot);
            let consume = self.base.consumeFreeSlot;
            self.base.postInsertHook(consume);
            // Java: return true;
            true
        }
    }

    // Java: boolean remove(int val)
    pub fn remove(&mut self, val: i32) -> bool {
        // Java: int index = this.index(val);
        let index = self.base.index(val);
        // Java: if (index >= 0)
        if index >= 0
        {
            // Java: this.removeAt(index);
            self.base.removeAt(index);
            // Java: return true;
            true
        }
        // Java: else
        else
        {
            // Java: return false;
            false
        }
    }

    pub fn contains(&self, val: i32) -> bool {
        self.base.contains(val)
    }

    pub fn size(&self) -> i32 {
        self.base._size
    }

    // Java: boolean containsAll(Collection<?> collection)
    pub fn containsAll_collection(&self, collection: &[i32]) -> bool {
        // Java: for(Object element : collection)
        for &element in collection
        {
            // Java: if (!(element instanceof Integer))
            // Java: return false;
            // Java: int c = (Integer)element;
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

    // Java: boolean containsAll(int[] array)
    pub fn containsAll_array(&self, array: &[i32]) -> bool {
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

    // Java: boolean addAll(Collection<? extends Integer> collection)
    pub fn addAll_collection(&mut self, collection: &[i32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Integer element : collection)
        for &element in collection
        {
            // Java: int e = element;
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

    // Java: boolean addAll(int[] array)
    pub fn addAll_array(&mut self, array: &[i32]) -> bool {
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

    // Java: boolean addAll(IIntSet intSet)
    pub fn addAll_set(&mut self, int_set: &dyn IIntSet) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: IIntIterator iter = intSet.iterator();
        let mut iter = int_set.iterator();
        // Java: while(iter.hasNext())
        while iter.hasNext()
        {
            // Java: int element = iter.next();
            let element = iter.next();
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
    pub fn retainAll_collection(&mut self, collection: &[i32]) -> bool {
        // Java: boolean modified = false;
        let mut modified = false;
        // Java: IIntIterator iter = this.iterator();
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

    // Java: boolean retainAll(int[] array)
    pub fn retainAll_array(&mut self, array: &mut [i32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: Arrays.sort(array);
        array.sort();
        // Java: int[] set = this._set;
        // Java: byte[] states = this._states;
        // Java: this._autoCompactTemporaryDisable = true;
        self.base._autoCompactTemporaryDisable = true;
        // Java: int i = set.length;
        let mut i = self.base._set.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (states[i] == 1 && Arrays.binarySearch(array, set[i]) < 0)
            if self.base._states[i as usize] == 1 && array.binary_search(&self.base._set[i as usize]).is_err()
            {
                // Java: this.removeAt(i);
                self.base.removeAt(i);
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: this._autoCompactTemporaryDisable = false;
        self.base._autoCompactTemporaryDisable = false;
        // Java: return changed;
        changed
    }

    // Java: boolean removeAll(Collection<?> collection)
    pub fn removeAll_collection(&mut self, collection: &[i32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Object element : collection)
        for &element in collection
        {
            // Java: if (element instanceof Integer)
            {
                // Java: int c = (Integer)element;
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

    // Java: boolean removeAll(int[] array)
    pub fn removeAll_array(&mut self, array: &[i32]) -> bool {
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
        self.base._size = 0;
        self.base._free = self.base._set.len() as i32;
        // Java: int[] set = this._set;
        // Java: byte[] states = this._states;
        // Java: for(int i = set.length; i-- > 0; states[i] = 0)
        let mut i = self.base._set.len() as i32;
        let nev = self.base.no_entry_value;
        while { i -= 1; i >= 0 }
        {
            // Java: set[i] = this.no_entry_value;
            self.base._set[i as usize] = nev;
            self.base._states[i as usize] = 0;
        }
    }

    // Java: void rehash(int newCapacity)
    pub fn rehash(&mut self, new_capacity: i32) {
        // Java: int oldCapacity = this._set.length;
        let old_capacity = self.base._set.len();
        // Java: int[] oldSet = this._set;
        let old_set = std::mem::take(&mut self.base._set);
        // Java: byte[] oldStates = this._states;
        let old_states = std::mem::take(&mut self.base._states);
        // Java: this._set = new int[newCapacity];
        self.base._set = vec![0; new_capacity as usize];
        // Java: this._states = new byte[newCapacity];
        self.base._states = vec![0; new_capacity as usize];
        // Java: int i = oldCapacity;
        let mut i = old_capacity as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (oldStates[i] == 1)
            if old_states[i as usize] == 1
            {
                // Java: int o = oldSet[i];
                let o = old_set[i as usize];
                // Java: this.insertKey(o);
                self.base.insertKey(o);
            }
        }
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &dyn IIntSet) -> bool {
        // Java: if (!(other instanceof IIntSet))
        // Java: return false;
        // Java: else
        {
            // Java: IIntSet that = (IIntSet)other;
            // Java: if (that.size() != this.size())
            if other.size() != self.size()
            {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: int i = this._states.length;
                let mut i = self.base._states.len() as i32;
                // Java: while(i-- > 0)
                while { i -= 1; i >= 0 }
                {
                    // Java: if (this._states[i] == 1 && !that.contains(this._set[i]))
                    if self.base._states[i as usize] == 1 && !other.contains(self.base._set[i as usize])
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
        let mut i = self.base._states.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (this._states[i] == 1)
            if self.base._states[i as usize] == 1
            {
                // Java: hashcode += HashCode.hash(this._set[i]);
                hashcode = hashcode.wrapping_add(HashCode::hash_int(self.base._set[i as usize]));
            }
        }
        // Java: return hashcode;
        hashcode
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder buffy = new StringBuilder(this._size * 2 + 2);
        let mut buffy = String::with_capacity((self.base._size * 2 + 2) as usize);
        // Java: buffy.append("{");
        buffy.push('{');
        // Java: int i = this._states.length;
        let mut i = self.base._states.len() as i32;
        // Java: int j = 1;
        let mut j = 1;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (this._states[i] == 1)
            if self.base._states[i as usize] == 1
            {
                // Java: buffy.append(this._set[i]);
                buffy.push_str(&self.base._set[i as usize].to_string());
                // Java: if (j++ < this._size)
                if j < self.base._size
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

// Java: public class WIntHashIterator
impl<'a> WIntHashIterator<'a> {
    // Java: WIntHashIterator(WIntHash hash)
    pub fn new(hash: &'a WIntHash) -> Self {
        // Java: super(hash);
        // Java: this._hash = hash;
        WIntHashIterator {
            base: WHashPrimitiveIterator::WHashPrimitiveIterator_new(
                hash._size as usize,
                hash._set.len(),
            ),
            _hash: hash,
        }
    }

    // Java: int next()
    pub fn next(&mut self) -> i32 {
        // Java: this.moveToNextIndex();
        self.base.moveToNextIndex();
        // Java: return this._hash._set[this._index];
        self._hash._set[self.base._index as usize]
    }

    pub fn hasNext(&self) -> bool {
        self.base.hasNext()
    }

    pub fn remove(&mut self) {
        self.base.remove();
    }
}
