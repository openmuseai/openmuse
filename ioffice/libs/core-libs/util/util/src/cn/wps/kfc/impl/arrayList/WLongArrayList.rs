#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.HashCode
use crate::kfc::HashCode::HashCode;
// Java: import cn.wps.kfc.api.iterator.ILongIterator
use crate::kfc::api::iterator::ILongIterator::ILongIterator;
// Java: import cn.wps.kfc.api.list.ILongList
use crate::kfc::api::list::ILongList::ILongList;
// Java: import cn.wps.kfc.api.procedure.ILongProcedure
use crate::kfc::api::procedure::ILongProcedure::ILongProcedure;
// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.NoSuchElementException

// Java: public class WLongArrayList implements ILongList, Serializable
pub struct WLongArrayList {
    // Java field: long serialVersionUID
    // Java field: long[] _data
    pub(crate) _data: Vec<i64>,
    // Java field: int _pos
    pub(crate) _pos: i32,
    // Java field: int DEFAULT_CAPACITY
    // Java field: long no_entry_value
    pub(crate) no_entry_value: i64,
}
const SERIAL_VERSION_UID: i64 = 1;
const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WLongArrayIterator
pub struct WLongArrayIterator<'a> {
    list: &'a WLongArrayList,
    // Java field: int cursor
    pub(crate) cursor: i32,
    // Java field: int lastRet
    pub(crate) lastRet: i32,
}

// Java: public class WLongArrayList implements ILongList, Serializable
impl WLongArrayList {
    // Java: WLongArrayList()
    pub fn new() -> Self {
        // Java: this(10, 0L);
        Self::new_with_capacity_and_entry(10, 0)
    }

    // Java: WLongArrayList(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: this(capacity, 0L);
        Self::new_with_capacity_and_entry(capacity, 0)
    }

    // Java: WLongArrayList(int capacity, long no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: i64) -> Self {
        // Java: super();
        // Java: this._data = new long[capacity];
        // Java: this._pos = 0;
        // Java: this.no_entry_value = no_entry_value;
        Self {
            _data: vec![0i64; capacity as usize],
            _pos: 0,
            no_entry_value,
        }
    }

    // Java: WLongArrayList(long[] values)
    pub fn new_from_array(values: &[i64]) -> Self {
        // Java: this(values.length);
        let mut list = Self::new_with_capacity(values.len() as i32);
        // Java: this.add(values);
        list.add_array(values);
        list
    }

    // Java: WLongArrayList(long[] values, long no_entry_value, boolean wrap)
    pub fn new_wrap(values: Vec<i64>, no_entry_value: i64, wrap: bool) -> Self {
        // Java: super();
        // Java: if (!wrap)
        if !wrap {
            // Java: throw new IllegalStateException("Wrong call");
            panic!("Wrong call");
        }
        // Java: else if (values == null)
        // Java: throw new IllegalArgumentException("values can not be null");
        // Java: else
        {
            let pos = values.len() as i32;
            // Java: this._data = values;
            // Java: this._pos = values.length;
            // Java: this.no_entry_value = no_entry_value;
            Self { _data: values, _pos: pos, no_entry_value }
        }
    }

    // Java: WLongArrayList(ILongList values)
    pub fn new_from_list(values: &dyn ILongList) -> Self {
        // Java: this(values.size());
        let mut list = Self::new_with_capacity(values.size());
        // Java: this.addAll(values);
        list.addAll_list(values);
        list
    }

    // Java: static WLongArrayList wrap(long[] values)
    pub fn wrap(values: Vec<i64>) -> Self {
        // Java: return wrap(values, 0L);
        Self::wrap_with_entry(values, 0)
    }

    // Java: static WLongArrayList wrap(long[] values, long no_entry_value)
    pub fn wrap_with_entry(values: Vec<i64>, no_entry_value: i64) -> Self {
        // Java: return new WLongArrayList(values, no_entry_value, true)
        // Java: public void ensureCapacity(int capacity)
        // Java: if (capacity > this._data.length)
        // Java: throw new IllegalStateException("Can not grow ArrayList wrapped external array");
        Self::new_wrap(values, no_entry_value, true)
    }

    // Java: long getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i64 {
        // Java: return this.no_entry_value;
        self.no_entry_value
    }

    // Java: void ensureCapacity(int capacity)
    pub fn ensureCapacity(&mut self, capacity: i32) {
        // Java: if (capacity > this._data.length)
        if capacity > self._data.len() as i32 {
            // Java: int newCap = Math.max(this._data.length << 1, capacity);
            let new_cap = std::cmp::max((self._data.len() << 1) as i32, capacity);
            // Java: long[] tmp = new long[newCap];
            let mut tmp = vec![0i64; new_cap as usize];
            // Java: System.arraycopy(this._data, 0, tmp, 0, this._data.length);
            tmp[..self._data.len()].copy_from_slice(&self._data);
            // Java: this._data = tmp;
            self._data = tmp;
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._pos;
        self._pos
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this._pos == 0;
        self._pos == 0
    }

    // Java: void trimToSize()
    pub fn trimToSize(&mut self) {
        // Java: if (this._data.length > this.size())
        if self._data.len() as i32 > self.size() {
            // Java: long[] tmp = new long[this.size()];
            let sz = self.size() as usize;
            let mut tmp = vec![0i64; sz];
            // Java: this.toArray(tmp, 0, tmp.length);
            tmp.copy_from_slice(&self._data[..sz]);
            // Java: this._data = tmp;
            self._data = tmp;
        }
    }

    // Java: boolean add(long val)
    pub fn add(&mut self, val: i64) -> bool {
        // Java: this.ensureCapacity(this._pos + 1);
        self.ensureCapacity(self._pos + 1);
        // Java: this._data[this._pos++] = val;
        self._data[self._pos as usize] = val;
        self._pos += 1;
        // Java: return true;
        true
    }

    // Java: void add(long[] vals)
    pub fn add_array(&mut self, vals: &[i64]) {
        // Java: this.add(vals, 0, vals.length);
        self.add_array_range(vals, 0, vals.len() as i32);
    }

    // Java: void add(long[] vals, int offset, int length)
    pub fn add_array_range(&mut self, vals: &[i64], offset: i32, length: i32) {
        // Java: this.ensureCapacity(this._pos + length);
        self.ensureCapacity(self._pos + length);
        // Java: System.arraycopy(vals, offset, this._data, this._pos, length);
        self._data[self._pos as usize..(self._pos + length) as usize]
            .copy_from_slice(&vals[offset as usize..(offset + length) as usize]);
        // Java: this._pos += length;
        self._pos += length;
    }

    // Java: void insert(int offset, long value)
    pub fn insert(&mut self, offset: i32, value: i64) {
        // Java: if (offset == this._pos)
        if offset == self._pos {
            // Java: this.add(value);
            self.add(value);
        }
        // Java: else
        else {
            // Java: this.ensureCapacity(this._pos + 1);
            self.ensureCapacity(self._pos + 1);
            // Java: System.arraycopy(this._data, offset, this._data, offset + 1, this._pos - offset);
            self._data.copy_within(offset as usize..self._pos as usize, (offset + 1) as usize);
            // Java: this._data[offset] = value;
            self._data[offset as usize] = value;
            // Java: ++this._pos;
            self._pos += 1;
        }
    }

    // Java: void insert(int offset, long[] values)
    pub fn insert_array(&mut self, offset: i32, values: &[i64]) {
        // Java: this.insert(offset, values, 0, values.length);
        self.insert_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void insert(int offset, long[] values, int valOffset, int len)
    pub fn insert_array_range(&mut self, offset: i32, values: &[i64], val_offset: i32, len: i32) {
        // Java: if (offset == this._pos)
        if offset == self._pos {
            // Java: this.add(values, valOffset, len);
            self.add_array_range(values, val_offset, len);
        }
        // Java: else
        else {
            // Java: this.ensureCapacity(this._pos + len);
            self.ensureCapacity(self._pos + len);
            // Java: System.arraycopy(this._data, offset, this._data, offset + len, this._pos - offset);
            self._data.copy_within(offset as usize..self._pos as usize, (offset + len) as usize);
            // Java: System.arraycopy(values, valOffset, this._data, offset, len);
            self._data[offset as usize..(offset + len) as usize]
                .copy_from_slice(&values[val_offset as usize..(val_offset + len) as usize]);
            // Java: this._pos += len;
            self._pos += len;
        }
    }

    // Java: long get(int offset)
    pub fn get(&self, offset: i32) -> i64 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        // Java: return this._data[offset];
        self._data[offset as usize]
    }

    // Java: long getQuick(int offset)
    pub fn getQuick(&self, offset: i32) -> i64 {
        // Java: return this._data[offset];
        self._data[offset as usize]
    }

    // Java: long set(int offset, long val)
    pub fn set(&mut self, offset: i32, val: i64) -> i64 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        {
            // Java: long prev_val = this._data[offset];
            let prev_val = self._data[offset as usize];
            // Java: this._data[offset] = val;
            self._data[offset as usize] = val;
            // Java: return prev_val;
            prev_val
        }
    }

    // Java: void set(int offset, long[] values)
    pub fn set_array(&mut self, offset: i32, values: &[i64]) {
        // Java: this.set(offset, values, 0, values.length);
        self.set_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void set(int offset, long[] values, int valOffset, int length)
    pub fn set_array_range(&mut self, offset: i32, values: &[i64], val_offset: i32, length: i32) {
        // Java: if (offset >= 0 && offset + length <= this._pos)
        if offset >= 0 && offset + length <= self._pos {
            // Java: System.arraycopy(values, valOffset, this._data, offset, length);
            self._data[offset as usize..(offset + length) as usize]
                .copy_from_slice(&values[val_offset as usize..(val_offset + length) as usize]);
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: void setQuick(int offset, long val)
    pub fn setQuick(&mut self, offset: i32, val: i64) {
        // Java: this._data[offset] = val;
        self._data[offset as usize] = val;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.clear(10);
        self.clear_with_capacity(10);
    }

    // Java: void clear(int capacity)
    pub fn clear_with_capacity(&mut self, capacity: i32) {
        // Java: this._data = new long[capacity];
        self._data = vec![0i64; capacity as usize];
        // Java: this._pos = 0;
        self._pos = 0;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this._pos = 0;
        self._pos = 0;
        // Java: Arrays.fill(this._data, this.no_entry_value);
        self._data.fill(self.no_entry_value);
    }

    // Java: void resetQuick()
    pub fn resetQuick(&mut self) {
        // Java: this._pos = 0;
        self._pos = 0;
    }

    // Java: boolean remove(long value)
    pub fn remove_value(&mut self, value: i64) -> bool {
        // Java: for(int index = 0; index < this._pos; ++index)
        for index in 0..self._pos {
            // Java: if (value == this._data[index])
            if value == self._data[index as usize] {
                // Java: this.remove(index, 1);
                self.remove_range(index, 1);
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: long removeAt(int offset)
    pub fn removeAt(&mut self, offset: i32) -> i64 {
        // Java: long old = this.get(offset);
        let old = self.get(offset);
        // Java: this.remove(offset, 1);
        self.remove_range(offset, 1);
        // Java: return old;
        old
    }

    // Java: void remove(int offset, int length)
    pub fn remove_range(&mut self, offset: i32, length: i32) {
        // Java: if (length != 0)
        if length != 0 {
            // Java: if (offset >= 0 && offset < this._pos)
            if offset >= 0 && offset < self._pos {
                // Java: if (offset == 0)
                if offset == 0 {
                    // Java: System.arraycopy(this._data, length, this._data, 0, this._pos - length);
                    self._data.copy_within(length as usize..self._pos as usize, 0);
                }
                // Java: else if (this._pos - length != offset)
                else if self._pos - length != offset {
                    // Java: System.arraycopy(this._data, offset + length, this._data, offset, this._pos - (offset + length));
                    self._data.copy_within((offset + length) as usize..self._pos as usize, offset as usize);
                }
                // Java: this._pos -= length;
                self._pos -= length;
            }
            // Java: else
            else {
                // Java: throw new ArrayIndexOutOfBoundsException(offset);
                panic!("ArrayIndexOutOfBoundsException: {}", offset);
            }
        }
    }

    // Java: ILongIterator iterator()
    pub fn iterator(&self) -> WLongArrayIterator {
        // Java: return new WLongArrayIterator(0);
        WLongArrayIterator { list: self, cursor: 0, lastRet: -1 }
    }

    // Java: boolean addAll(Collection<? extends Long> collection)
    pub fn addAll_collection(&mut self, collection: &[i64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Long element : collection)
        for &element in collection {
            // Java: long e = element;
            let e = element;
            // Java: if (this.add(e))
            if self.add(e) {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean addAll(ILongList list)
    pub fn addAll_list(&mut self, list: &dyn ILongList) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: ILongIterator iter = list.iterator();
        let mut iter = list.iterator();
        // Java: while(iter.hasNext())
        while iter.hasNext() {
            // Java: long element = iter.next();
            let element = iter.next();
            // Java: if (this.add(element))
            if self.add(element) {
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
        // Java: for(long element : array)
        for &element in array {
            // Java: if (this.add(element))
            if self.add(element) {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: void swap(int i, int j)
    pub fn swap(&mut self, i: i32, j: i32) {
        // Java: long tmp = this._data[i];
        let tmp = self._data[i as usize];
        // Java: this._data[i] = this._data[j];
        self._data[i as usize] = self._data[j as usize];
        // Java: this._data[j] = tmp;
        self._data[j as usize] = tmp;
    }

    // Java: ILongList subList(int begin, int end)
    pub fn subList(&self, begin: i32, end: i32) -> WLongArrayList {
        // Java: if (end < begin)
        if end < begin {
            // Java: throw new IllegalArgumentException("end index " + end + " greater than begin index " + begin);
            panic!("end index {} greater than begin index {}", end, begin);
        }
        // Java: else if (begin < 0)
        else if begin < 0 {
            // Java: throw new IndexOutOfBoundsException("begin index can not be < 0");
            panic!("begin index can not be < 0");
        }
        // Java: else if (end > this._data.length)
        else if end > self._data.len() as i32 {
            // Java: throw new IndexOutOfBoundsException("end index < " + this._data.length);
            panic!("end index < {}", self._data.len());
        }
        // Java: else
        else {
            // Java: WLongArrayList list = new WLongArrayList(end - begin);
            let mut list = WLongArrayList::new_with_capacity(end - begin);
            // Java: for(int i = begin; i < end; ++i)
            for i in begin..end {
                // Java: list.add(this._data[i]);
                list.add(self._data[i as usize]);
            }
            // Java: return list;
            list
        }
    }

    // Java: long[] toArray()
    pub fn toArray(&self) -> Vec<i64> {
        // Java: return this.toArray(0, this._pos);
        self.toArray_range(0, self._pos)
    }

    // Java: long[] toArray(int offset, int len)
    pub fn toArray_range(&self, offset: i32, len: i32) -> Vec<i64> {
        // Java: long[] rv = new long[len];
        let mut rv = vec![0i64; len as usize];
        // Java: this.toArray(rv, offset, len);
        self.toArray_dest_range(&mut rv, offset, len);
        // Java: return rv;
        rv
    }

    // Java: long[] toArray(long[] dest)
    pub fn toArray_dest(&self, dest: &mut Vec<i64>) {
        // Java: int len = dest.length;
        let mut len = dest.len() as i32;
        // Java: if (dest.length > this._pos)
        if dest.len() as i32 > self._pos {
            // Java: len = this._pos;
            len = self._pos;
            // Java: dest[len] = this.no_entry_value;
            dest[len as usize] = self.no_entry_value;
        }
        // Java: this.toArray(dest, 0, len);
        self.toArray_dest_range(dest, 0, len);
        // Java: return dest;
    }

    // Java: long[] toArray(long[] dest, int offset, int len)
    pub fn toArray_dest_range(&self, dest: &mut [i64], offset: i32, len: i32) {
        // Java: if (len == 0)
        if len == 0 {
            // Java: return dest;
            return;
        }
        // Java: else if (offset >= 0 && offset < this._pos)
        else if offset >= 0 && offset < self._pos {
            // Java: System.arraycopy(this._data, offset, dest, 0, len);
            dest[..len as usize].copy_from_slice(&self._data[offset as usize..(offset + len) as usize]);
            // Java: return dest;
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: long[] toArray(long[] dest, int source_pos, int dest_pos, int len)
    pub fn toArray_dest_range2(&self, dest: &mut [i64], source_pos: i32, dest_pos: i32, len: i32) {
        // Java: if (len == 0)
        if len == 0 {
            // Java: return dest;
            return;
        }
        // Java: else if (source_pos >= 0 && source_pos < this._pos)
        else if source_pos >= 0 && source_pos < self._pos {
            // Java: System.arraycopy(this._data, source_pos, dest, dest_pos, len);
            dest[dest_pos as usize..(dest_pos + len) as usize]
                .copy_from_slice(&self._data[source_pos as usize..(source_pos + len) as usize]);
            // Java: return dest;
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException(source_pos);
            panic!("ArrayIndexOutOfBoundsException: {}", source_pos);
        }
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &WLongArrayList) -> bool {
        // Java: if (other == this)
        if std::ptr::eq(self, other) {
            // Java: return true;
            return true;
        }
        // Java: else if (other instanceof WLongArrayList)
        {
            // Java: WLongArrayList that = (WLongArrayList)other;
            // Java: if (that.size() != this.size())
            if other.size() != self.size() {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: int i = this._pos;
                let mut i = self._pos;
                // Java: while(i-- > 0)
                loop {
                    i -= 1;
                    if i < 0 { break; }
                    // Java: if (this._data[i] != that._data[i])
                    if self._data[i as usize] != other._data[i as usize] {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
        // Java: else
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int h = 0;
        let mut h: i32 = 0;
        // Java: for(int i = this._pos; i-- > 0; h += HashCode.hash(this._data[i]))
        let mut i = self._pos;
        loop {
            i -= 1;
            if i < 0 { break; }
            h = h.wrapping_add(HashCode::hash_long(self._data[i as usize]));
        }
        // Java: return h;
        h
    }

    // Java: boolean forEach(ILongProcedure procedure)
    pub fn forEach(&self, procedure: &mut dyn ILongProcedure) -> bool {
        // Java: for(int i = 0; i < this._pos; ++i)
        for i in 0..self._pos {
            // Java: if (!procedure.execute(this._data[i]))
            if !procedure.execute(self._data[i as usize]) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: void sort()
    pub fn sort(&mut self) {
        // Java: Arrays.sort(this._data, 0, this._pos);
        self._data[..self._pos as usize].sort();
    }

    // Java: void sort(int fromIndex, int toIndex)
    pub fn sort_range(&mut self, from_index: i32, to_index: i32) {
        // Java: Arrays.sort(this._data, fromIndex, toIndex);
        self._data[from_index as usize..to_index as usize].sort();
    }

    // Java: void fill(long val)
    pub fn fill(&mut self, val: i64) {
        // Java: Arrays.fill(this._data, 0, this._pos, val);
        self._data[..self._pos as usize].fill(val);
    }

    // Java: void fill(int fromIndex, int toIndex, long val)
    pub fn fill_range(&mut self, from_index: i32, to_index: i32, val: i64) {
        // Java: if (toIndex > this._pos)
        if to_index > self._pos {
            // Java: this.ensureCapacity(toIndex);
            self.ensureCapacity(to_index);
            // Java: this._pos = toIndex;
            self._pos = to_index;
        }
        // Java: Arrays.fill(this._data, fromIndex, toIndex, val);
        self._data[from_index as usize..to_index as usize].fill(val);
    }

    // Java: int binarySearch(long value)
    pub fn binarySearch(&self, value: i64) -> i32 {
        // Java: return this.binarySearch(value, 0, this._pos);
        self.binarySearch_range(value, 0, self._pos)
    }

    // Java: int binarySearch(long value, int fromIndex, int toIndex)
    pub fn binarySearch_range(&self, value: i64, from_index: i32, to_index: i32) -> i32 {
        // Java: if (fromIndex < 0)
        if from_index < 0 {
            // Java: throw new ArrayIndexOutOfBoundsException(fromIndex);
            panic!("ArrayIndexOutOfBoundsException: {}", from_index);
        }
        // Java: else if (toIndex > this._pos)
        else if to_index > self._pos {
            // Java: throw new ArrayIndexOutOfBoundsException(toIndex);
            panic!("ArrayIndexOutOfBoundsException: {}", to_index);
        }
        // Java: else
        else {
            // Java: int low = fromIndex;
            let mut low = from_index;
            // Java: int high = toIndex - 1;
            let mut high = to_index - 1;
            // Java: while(low <= high)
            while low <= high {
                // Java: int mid = low + high >>> 1;
                let mid = ((low + high) as u32 >> 1) as i32;
                // Java: long midVal = this._data[mid];
                let mid_val = self._data[mid as usize];
                // Java: if (midVal < value)
                if mid_val < value {
                    // Java: low = mid + 1;
                    low = mid + 1;
                }
                // Java: else
                else {
                    // Java: if (midVal <= value)
                    if mid_val <= value {
                        // Java: return mid;
                        return mid;
                    }
                    // Java: high = mid - 1;
                    high = mid - 1;
                }
            }
            // Java: return -(low + 1);
            -(low + 1)
        }
    }

    // Java: int indexOf(long value)
    pub fn indexOf(&self, value: i64) -> i32 {
        // Java: return this.indexOf(0, value);
        self.indexOf_from(0, value)
    }

    // Java: int indexOf(int offset, long value)
    pub fn indexOf_from(&self, offset: i32, value: i64) -> i32 {
        // Java: for(int i = offset; i < this._pos; ++i)
        for i in offset..self._pos {
            // Java: if (this._data[i] == value)
            if self._data[i as usize] == value {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: int lastIndexOf(long value)
    pub fn lastIndexOf(&self, value: i64) -> i32 {
        // Java: return this.lastIndexOf(this._pos, value);
        self.lastIndexOf_from(self._pos, value)
    }

    // Java: int lastIndexOf(int offset, long value)
    pub fn lastIndexOf_from(&self, offset: i32, value: i64) -> i32 {
        // Java: int i = offset;
        let mut i = offset;
        // Java: while(i-- > 0)
        loop {
            i -= 1;
            if i < 0 { break; }
            // Java: if (this._data[i] == value)
            if self._data[i as usize] == value {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: boolean contains(long value)
    pub fn contains(&self, value: i64) -> bool {
        // Java: return this.lastIndexOf(value) >= 0;
        self.lastIndexOf(value) >= 0
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder buf = new StringBuilder("{");
        let mut buf = String::from("{");
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int end = this._pos - 1; i < end; ++i)
        let end = self._pos - 1;
        while i < end {
            // Java: buf.append(this._data[i]);
            buf.push_str(&self._data[i as usize].to_string());
            // Java: buf.append(", ");
            buf.push_str(", ");
            i += 1;
        }
        // Java: if (this.size() > 0)
        if self.size() > 0 {
            // Java: buf.append(this._data[this._pos - 1]);
            buf.push_str(&self._data[(self._pos - 1) as usize].to_string());
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }
}

// Java: public class WLongArrayIterator
impl<'a> WLongArrayIterator<'a> {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.cursor < WLongArrayList.this.size();
        self.cursor < self.list.size()
    }

    // Java: long next()
    pub fn next(&mut self) -> i64 {
        // Java: try
        {
            // Java: long next = WLongArrayList.this.get(this.cursor);
            if self.cursor >= self.list.size() {
                panic!("NoSuchElementException");
            }
            let next = self.list.get(self.cursor);
            // Java: this.lastRet = this.cursor++;
            self.lastRet = self.cursor;
            self.cursor += 1;
            // Java: return next;
            return next;
        }
        // Java: catch (IndexOutOfBoundsException var3)
        // Java: throw new NoSuchElementException();
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: if (this.lastRet == -1)
        if self.lastRet == -1 {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
        // Java: else
        {
            // Java: try
            {
                // Java: WLongArrayList.this.remove(this.lastRet, 1);
                // Java: if (this.lastRet < this.cursor)
                if self.lastRet < self.cursor {
                    // Java: --this.cursor;
                    self.cursor -= 1;
                }
                // Java: this.lastRet = -1;
                self.lastRet = -1;
            }
            // Java: catch (IndexOutOfBoundsException var2)
            // Java: throw new ConcurrentModificationException();
        }
    }
}
