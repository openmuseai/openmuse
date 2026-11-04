#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.HashCode
use crate::kfc::HashCode::HashCode;
// Java: import cn.wps.kfc.api.iterator.IFloatIterator
use crate::kfc::api::iterator::IFloatIterator::IFloatIterator;
// Java: import cn.wps.kfc.api.iterator.IIterator::IIterator;
use crate::kfc::api::iterator::IIterator::IIterator;
// Java: import cn.wps.kfc.api.list.IFloatList
use crate::kfc::api::list::IFloatList::IFloatList;
// Java: import cn.wps.kfc.api.procedure.IFloatProcedure
use crate::kfc::api::procedure::IFloatProcedure::IFloatProcedure;
// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.NoSuchElementException

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WFloatArrayList implements IFloatList, Serializable
pub struct WFloatArrayList {
    // Java field: float[] _data
    pub(crate) _data: Vec<f32>,
    // Java field: int _pos
    pub(crate) _pos: i32,
    // Java field: int DEFAULT_CAPACITY
    // Java field: float no_entry_value
    pub(crate) no_entry_value: f32,
}

const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WFloatArrayIterator
pub struct WFloatArrayIterator<'a> {
    list: &'a WFloatArrayList,
    // Java field: int cursor
    pub(crate) cursor: i32,
    // Java field: int lastRet
    pub(crate) lastRet: i32,
}

// Java: public class WFloatArrayList implements IFloatList, Serializable
impl WFloatArrayList {
    // Java: WFloatArrayList()
    pub fn new() -> Self {
        // Java: this(10, 0.0F);
        Self::new_with_capacity_and_entry(10, 0.0)
    }

    // Java: WFloatArrayList(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: this(capacity, 0.0F);
        Self::new_with_capacity_and_entry(capacity, 0.0)
    }

    // Java: WFloatArrayList(int capacity, float no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: f32) -> Self {
        // Java: super();
        // Java: this._data = new float[capacity];
        // Java: this._pos = 0;
        // Java: this.no_entry_value = no_entry_value;
        Self {
            _data: vec![0.0; capacity as usize],
            _pos: 0,
            no_entry_value,
        }
    }

    // Java: WFloatArrayList(IFloatList floatList)
    pub fn new_from_list(float_list: &dyn IFloatList) -> Self {
        // Java: this(floatList.size());
        let mut list = Self::new_with_capacity(float_list.size());
        // Java: this.addAll(floatList);
        list.addAll_list(float_list);
        list
    }

    // Java: WFloatArrayList(float[] values)
    pub fn new_from_array(values: &[f32]) -> Self {
        // Java: this(values.length);
        let mut list = Self::new_with_capacity(values.len() as i32);
        // Java: this.add(values);
        list.add_array(values);
        list
    }

    // Java: WFloatArrayList(float[] values, float no_entry_value, boolean wrap)
    pub fn new_wrap(values: Vec<f32>, no_entry_value: f32, wrap: bool) -> Self {
        // Java: super();
        // Java: if (!wrap)
        if !wrap
        {
            // Java: throw new IllegalStateException("Wrong call");
            panic!("IllegalStateException: Wrong call");
        }
        // Java: else if (values == null)
        // Java: throw new IllegalArgumentException("values can not be null");
        // Java: else
        {
            // Java: this._data = values;
            let pos = values.len() as i32;
            // Java: this._pos = values.length;
            // Java: this.no_entry_value = no_entry_value;
            Self {
                _data: values,
                _pos: pos,
                no_entry_value,
            }
        }
    }

    // Java: static WFloatArrayList wrap(float[] values)
    pub fn wrap(values: Vec<f32>) -> Self {
        // Java: return wrap(values, 0.0F);
        Self::wrap_with_entry(values, 0.0)
    }

    // Java: static WFloatArrayList wrap(float[] values, float no_entry_value)
    pub fn wrap_with_entry(values: Vec<f32>, no_entry_value: f32) -> Self {
        // Java: return new WFloatArrayList(values, no_entry_value, true)
        Self::new_wrap(values, no_entry_value, true)
        // Java: public void ensureCapacity(int capacity)
        // Java: if (capacity > this._data.length)
        // Java: throw new IllegalStateException("Can not grow ArrayList wrapped external array");
    }

    // Java: float getNoEntryValue()
    pub fn getNoEntryValue(&self) -> f32 {
        // Java: return this.no_entry_value;
        self.no_entry_value
    }

    // Java: void ensureCapacity(int capacity)
    pub fn ensureCapacity(&mut self, capacity: i32) {
        // Java: if (capacity > this._data.length)
        if capacity > self._data.len() as i32
        {
            // Java: int newCap = Math.max(this._data.length << 1, capacity);
            let new_cap = std::cmp::max((self._data.len() << 1) as i32, capacity);
            // Java: float[] tmp = new float[newCap];
            // Java: System.arraycopy(this._data, 0, tmp, 0, this._data.length);
            // Java: this._data = tmp;
            self._data.resize(new_cap as usize, 0.0);
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
        if self._data.len() as i32 > self.size()
        {
            // Java: float[] tmp = new float[this.size()];
            // Java: this.toArray(tmp, 0, tmp.length);
            // Java: this._data = tmp;
            self._data.truncate(self._pos as usize);
        }
    }

    // Java: boolean add(float val)
    pub fn add(&mut self, val: f32) -> bool {
        // Java: this.ensureCapacity(this._pos + 1);
        self.ensureCapacity(self._pos + 1);
        // Java: this._data[this._pos++] = val;
        self._data[self._pos as usize] = val;
        self._pos += 1;
        // Java: return true;
        true
    }

    // Java: void add(float[] vals)
    pub fn add_array(&mut self, vals: &[f32]) {
        // Java: this.add(vals, 0, vals.length);
        self.add_array_range(vals, 0, vals.len() as i32);
    }

    // Java: void add(float[] vals, int offset, int length)
    pub fn add_array_range(&mut self, vals: &[f32], offset: i32, length: i32) {
        // Java: this.ensureCapacity(this._pos + length);
        self.ensureCapacity(self._pos + length);
        // Java: System.arraycopy(vals, offset, this._data, this._pos, length);
        let src = &vals[offset as usize..(offset + length) as usize];
        self._data[self._pos as usize..(self._pos + length) as usize].copy_from_slice(src);
        // Java: this._pos += length;
        self._pos += length;
    }

    // Java: void insert(int offset, float value)
    pub fn insert(&mut self, offset: i32, value: f32) {
        // Java: if (offset == this._pos)
        if offset == self._pos
        {
            // Java: this.add(value);
            self.add(value);
        }
        // Java: else
        else
        {
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

    // Java: void insert(int offset, float[] values)
    pub fn insert_array(&mut self, offset: i32, values: &[f32]) {
        // Java: this.insert(offset, values, 0, values.length);
        self.insert_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void insert(int offset, float[] values, int valOffset, int len)
    pub fn insert_array_range(&mut self, offset: i32, values: &[f32], val_offset: i32, len: i32) {
        // Java: if (offset == this._pos)
        if offset == self._pos
        {
            // Java: this.add(values, valOffset, len);
            self.add_array_range(values, val_offset, len);
        }
        // Java: else
        else
        {
            // Java: this.ensureCapacity(this._pos + len);
            self.ensureCapacity(self._pos + len);
            // Java: System.arraycopy(this._data, offset, this._data, offset + len, this._pos - offset);
            self._data.copy_within(offset as usize..self._pos as usize, (offset + len) as usize);
            // Java: System.arraycopy(values, valOffset, this._data, offset, len);
            let src = &values[val_offset as usize..(val_offset + len) as usize];
            self._data[offset as usize..(offset + len) as usize].copy_from_slice(src);
            // Java: this._pos += len;
            self._pos += len;
        }
    }

    // Java: float get(int offset)
    pub fn get(&self, offset: i32) -> f32 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos
        {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        {
            // Java: return this._data[offset];
            self._data[offset as usize]
        }
    }

    // Java: float getQuick(int offset)
    pub fn getQuick(&self, offset: i32) -> f32 {
        // Java: return this._data[offset];
        self._data[offset as usize]
    }

    // Java: float set(int offset, float val)
    pub fn set(&mut self, offset: i32, val: f32) -> f32 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos
        {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        {
            // Java: float prev_val = this._data[offset];
            let prev_val = self._data[offset as usize];
            // Java: this._data[offset] = val;
            self._data[offset as usize] = val;
            // Java: return prev_val;
            prev_val
        }
    }

    // Java: void set(int offset, float[] values)
    pub fn set_array(&mut self, offset: i32, values: &[f32]) {
        // Java: this.set(offset, values, 0, values.length);
        self.set_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void set(int offset, float[] values, int valOffset, int length)
    pub fn set_array_range(&mut self, offset: i32, values: &[f32], val_offset: i32, length: i32) {
        // Java: if (offset >= 0 && offset + length <= this._pos)
        if offset >= 0 && offset + length <= self._pos
        {
            // Java: System.arraycopy(values, valOffset, this._data, offset, length);
            let src = &values[val_offset as usize..(val_offset + length) as usize];
            self._data[offset as usize..(offset + length) as usize].copy_from_slice(src);
        }
        // Java: else
        else
        {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: void setQuick(int offset, float val)
    pub fn setQuick(&mut self, offset: i32, val: f32) {
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
        // Java: this._data = new float[capacity];
        self._data = vec![0.0; capacity as usize];
        // Java: this._pos = 0;
        self._pos = 0;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this._pos = 0;
        self._pos = 0;
        // Java: Arrays.fill(this._data, this.no_entry_value);
        let nev = self.no_entry_value;
        self._data.fill(nev);
    }

    // Java: void resetQuick()
    pub fn resetQuick(&mut self) {
        // Java: this._pos = 0;
        self._pos = 0;
    }

    // Java: boolean remove(float value)
    pub fn remove_value(&mut self, value: f32) -> bool {
        // Java: for(int index = 0; index < this._pos; ++index)
        for index in 0..self._pos
        {
            // Java: if (value == this._data[index])
            if value == self._data[index as usize]
            {
                // Java: this.remove(index, 1);
                self.remove_range(index, 1);
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: float removeAt(int offset)
    pub fn removeAt(&mut self, offset: i32) -> f32 {
        // Java: float old = this.get(offset);
        let old = self.get(offset);
        // Java: this.remove(offset, 1);
        self.remove_range(offset, 1);
        // Java: return old;
        old
    }

    // Java: void remove(int offset, int length)
    pub fn remove_range(&mut self, offset: i32, length: i32) {
        // Java: if (length != 0)
        if length != 0
        {
            // Java: if (offset >= 0 && offset < this._pos)
            if offset >= 0 && offset < self._pos
            {
                // Java: if (offset == 0)
                if offset == 0
                {
                    // Java: System.arraycopy(this._data, length, this._data, 0, this._pos - length);
                    self._data.copy_within(length as usize..self._pos as usize, 0);
                }
                // Java: else if (this._pos - length != offset)
                else if self._pos - length != offset
                {
                    // Java: System.arraycopy(this._data, offset + length, this._data, offset, this._pos - (offset + length));
                    self._data.copy_within((offset + length) as usize..self._pos as usize, offset as usize);
                }
                // Java: this._pos -= length;
                self._pos -= length;
            }
            // Java: else
            else
            {
                // Java: throw new ArrayIndexOutOfBoundsException(offset);
                panic!("ArrayIndexOutOfBoundsException: {}", offset);
            }
        }
    }

    // Java: IFloatIterator iterator()
    pub fn iterator(&self) -> WFloatArrayIterator {
        // Java: return new WFloatArrayIterator(0);
        WFloatArrayIterator {
            list: self,
            cursor: 0,
            lastRet: -1,
        }
    }

    // Java: boolean addAll(Collection<? extends Float> collection)
    pub fn addAll_collection(&mut self, collection: &[f32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Float element : collection)
        for &element in collection
        {
            // Java: float e = element;
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

    // Java: boolean addAll(IFloatList floatList)
    pub fn addAll_list(&mut self, float_list: &dyn IFloatList) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: IFloatIterator iter = floatList.iterator();
        let mut iter = float_list.iterator();
        // Java: while(iter.hasNext())
        while iter.hasNext()
        {
            // Java: float element = iter.next();
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

    // Java: boolean addAll(float[] array)
    pub fn addAll_array(&mut self, array: &[f32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(float element : array)
        for &element in array
        {
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

    // Java: boolean removeAll(Collection<?> collection)
    pub fn removeAll_collection(&mut self, collection: &[f32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Object element : collection)
        for &element in collection
        {
            // Java: if (element instanceof Float)
            {
                // Java: float c = (Float)element;
                // Java: if (this.remove(c))
                if self.remove_value(element)
                {
                    // Java: changed = true;
                    changed = true;
                }
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean removeAll(IFloatList floatList)
    pub fn removeAll_list(&mut self, float_list: &dyn IFloatList) -> bool {
        // Java: if (floatList == this)
        // Java: this.clear();
        // Java: return true;
        // Java: else
        {
            // Java: boolean changed = false;
            let mut changed = false;
            // Java: IFloatIterator iter = floatList.iterator();
            let mut iter = float_list.iterator();
            // Java: while(iter.hasNext())
            while iter.hasNext()
            {
                // Java: float element = iter.next();
                let element = iter.next();
                // Java: if (this.remove(element))
                if self.remove_value(element)
                {
                    // Java: changed = true;
                    changed = true;
                }
            }
            // Java: return changed;
            changed
        }
    }

    // Java: boolean removeAll(float[] array)
    pub fn removeAll_array(&mut self, array: &[f32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: int i = array.length;
        let mut i = array.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (this.remove(array[i]))
            if self.remove_value(array[i as usize])
            {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: void reverse()
    pub fn reverse(&mut self) {
        // Java: this.reverse(0, this._pos);
        self.reverse_range(0, self._pos);
    }

    // Java: void reverse(int from, int to)
    pub fn reverse_range(&mut self, from: i32, to: i32) {
        // Java: if (from != to)
        if from != to
        {
            // Java: if (from > to)
            if from > to
            {
                // Java: throw new IllegalArgumentException("from cannot be greater than to");
                panic!("IllegalArgumentException: from cannot be greater than to");
            }
            // Java: else
            {
                // Java: int i = from;
                let mut i = from;
                // Java: for(int j = to - 1; i < j; --j)
                let mut j = to - 1;
                while i < j
                {
                    // Java: this.swap(i, j);
                    self.swap(i, j);
                    // Java: ++i;
                    i += 1;
                    j -= 1;
                }
            }
        }
    }

    // Java: void swap(int i, int j)
    fn swap(&mut self, i: i32, j: i32) {
        // Java: float tmp = this._data[i];
        let tmp = self._data[i as usize];
        // Java: this._data[i] = this._data[j];
        self._data[i as usize] = self._data[j as usize];
        // Java: this._data[j] = tmp;
        self._data[j as usize] = tmp;
    }

    // Java: IFloatList subList(int begin, int end)
    pub fn subList(&self, begin: i32, end: i32) -> WFloatArrayList {
        // Java: if (end < begin)
        if end < begin
        {
            // Java: throw new IllegalArgumentException("end index " + end + " greater than begin index " + begin);
            panic!("IllegalArgumentException: end index {} greater than begin index {}", end, begin);
        }
        // Java: else if (begin < 0)
        else if begin < 0
        {
            // Java: throw new IndexOutOfBoundsException("begin index can not be < 0");
            panic!("IndexOutOfBoundsException: begin index can not be < 0");
        }
        // Java: else if (end > this._data.length)
        else if end > self._data.len() as i32
        {
            // Java: throw new IndexOutOfBoundsException("end index < " + this._data.length);
            panic!("IndexOutOfBoundsException: end index < {}", self._data.len());
        }
        // Java: else
        {
            // Java: WFloatArrayList list = new WFloatArrayList(end - begin);
            let mut list = WFloatArrayList::new_with_capacity(end - begin);
            // Java: for(int i = begin; i < end; ++i)
            for i in begin..end
            {
                // Java: list.add(this._data[i]);
                list.add(self._data[i as usize]);
            }
            // Java: return list;
            list
        }
    }

    // Java: float[] toArray()
    pub fn toArray(&self) -> Vec<f32> {
        // Java: return this.toArray(0, this._pos);
        self.toArray_range(0, self._pos)
    }

    // Java: float[] toArray(int offset, int len)
    pub fn toArray_range(&self, offset: i32, len: i32) -> Vec<f32> {
        // Java: float[] rv = new float[len];
        let mut rv = vec![0.0f32; len as usize];
        // Java: this.toArray(rv, offset, len);
        self.toArray_dest_range_into(&mut rv, offset, len);
        // Java: return rv;
        rv
    }

    // Java: float[] toArray(float[] dest)
    pub fn toArray_dest(&self, dest: &mut [f32]) -> Vec<f32> {
        // Java: int len = dest.length;
        let mut len = dest.len() as i32;
        // Java: if (dest.length > this._pos)
        if dest.len() as i32 > self._pos
        {
            // Java: len = this._pos;
            len = self._pos;
            // Java: dest[len] = this.no_entry_value;
            dest[len as usize] = self.no_entry_value;
        }
        // Java: this.toArray(dest, 0, len);
        self.toArray_dest_range_into(dest, 0, len);
        // Java: return dest;
        dest.to_vec()
    }

    // Java: float[] toArray(float[] dest, int offset, int len)
    pub fn toArray_dest_range(&self, dest: &mut [f32], offset: i32, len: i32) -> Vec<f32> {
        // Java: if (len == 0)
        if len == 0
        {
            // Java: return dest;
            return dest.to_vec();
        }
        // Java: else if (offset >= 0 && offset < this._pos)
        else if offset >= 0 && offset < self._pos
        {
            // Java: System.arraycopy(this._data, offset, dest, 0, len);
            dest[..len as usize].copy_from_slice(&self._data[offset as usize..(offset + len) as usize]);
            // Java: return dest;
            dest.to_vec()
        }
        // Java: else
        else
        {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    fn toArray_dest_range_into(&self, dest: &mut [f32], offset: i32, len: i32) {
        if len == 0 {
            return;
        }
        if offset >= 0 && offset < self._pos {
            dest[..len as usize].copy_from_slice(&self._data[offset as usize..(offset + len) as usize]);
        } else {
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: float[] toArray(float[] dest, int source_pos, int dest_pos, int len)
    pub fn toArray_dest_range_offset(&self, dest: &mut [f32], source_pos: i32, dest_pos: i32, len: i32) -> Vec<f32> {
        // Java: if (len == 0)
        if len == 0
        {
            // Java: return dest;
            return dest.to_vec();
        }
        // Java: else if (source_pos >= 0 && source_pos < this._pos)
        else if source_pos >= 0 && source_pos < self._pos
        {
            // Java: System.arraycopy(this._data, source_pos, dest, dest_pos, len);
            dest[dest_pos as usize..(dest_pos + len) as usize].copy_from_slice(&self._data[source_pos as usize..(source_pos + len) as usize]);
            // Java: return dest;
            dest.to_vec()
        }
        // Java: else
        else
        {
            // Java: throw new ArrayIndexOutOfBoundsException(source_pos);
            panic!("ArrayIndexOutOfBoundsException: {}", source_pos);
        }
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &WFloatArrayList) -> bool {
        // Java: if (other == this)
        if std::ptr::eq(self, other)
        {
            // Java: return true;
            return true;
        }
        // Java: else if (other instanceof WFloatArrayList)
        {
            // Java: WFloatArrayList that = (WFloatArrayList)other;
            // Java: if (that.size() != this.size())
            if other.size() != self.size()
            {
                // Java: return false;
                return false;
            }
            // Java: else
            {
                // Java: int i = this._pos;
                let mut i = self._pos;
                // Java: while(i-- > 0)
                while { i -= 1; i >= 0 }
                {
                    // Java: if (this._data[i] != that._data[i])
                    if self._data[i as usize] != other._data[i as usize]
                    {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
        // Java: else
        {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int h = 0;
        let mut h: i32 = 0;
        // Java: for(int i = this._pos; i-- > 0; h += HashCode.hash(this._data[i]))
        let mut i = self._pos;
        while { i -= 1; i >= 0 }
        {
            h = h.wrapping_add(HashCode::hash_float(self._data[i as usize]));
        }
        // Java: return h;
        h
    }

    // Java: boolean forEach(IFloatProcedure procedure)
    pub fn forEach(&self, procedure: &mut dyn IFloatProcedure) -> bool {
        // Java: for(int i = 0; i < this._pos; ++i)
        for i in 0..self._pos
        {
            // Java: if (!procedure.execute(this._data[i]))
            if !procedure.execute(self._data[i as usize])
            {
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
        self._data[..self._pos as usize].sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Java: void sort(int fromIndex, int toIndex)
    pub fn sort_range(&mut self, from_index: i32, to_index: i32) {
        // Java: Arrays.sort(this._data, fromIndex, toIndex);
        self._data[from_index as usize..to_index as usize].sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Java: void fill(float val)
    pub fn fill(&mut self, val: f32) {
        // Java: Arrays.fill(this._data, 0, this._pos, val);
        self._data[..self._pos as usize].fill(val);
    }

    // Java: void fill(int fromIndex, int toIndex, float val)
    pub fn fill_range(&mut self, from_index: i32, to_index: i32, val: f32) {
        // Java: if (toIndex > this._pos)
        if to_index > self._pos
        {
            // Java: this.ensureCapacity(toIndex);
            self.ensureCapacity(to_index);
            // Java: this._pos = toIndex;
            self._pos = to_index;
        }
        // Java: Arrays.fill(this._data, fromIndex, toIndex, val);
        self._data[from_index as usize..to_index as usize].fill(val);
    }

    // Java: int binarySearch(float value)
    pub fn binarySearch(&self, value: f32) -> i32 {
        // Java: return this.binarySearch(value, 0, this._pos);
        self.binarySearch_range(value, 0, self._pos)
    }

    // Java: int binarySearch(float value, int fromIndex, int toIndex)
    pub fn binarySearch_range(&self, value: f32, from_index: i32, to_index: i32) -> i32 {
        // Java: if (fromIndex < 0)
        if from_index < 0
        {
            // Java: throw new ArrayIndexOutOfBoundsException(fromIndex);
            panic!("ArrayIndexOutOfBoundsException: {}", from_index);
        }
        // Java: else if (toIndex > this._pos)
        else if to_index > self._pos
        {
            // Java: throw new ArrayIndexOutOfBoundsException(toIndex);
            panic!("ArrayIndexOutOfBoundsException: {}", to_index);
        }
        // Java: else
        {
            // Java: int low = fromIndex;
            let mut low = from_index;
            // Java: int high = toIndex - 1;
            let mut high = to_index - 1;
            // Java: while(low <= high)
            while low <= high
            {
                // Java: int mid = low + high >>> 1;
                let mid = ((low as u32 + high as u32) >> 1) as i32;
                // Java: float midVal = this._data[mid];
                let mid_val = self._data[mid as usize];
                // Java: if (midVal < value)
                if mid_val < value
                {
                    // Java: low = mid + 1;
                    low = mid + 1;
                }
                // Java: else
                else
                {
                    // Java: if (!(midVal > value))
                    if !(mid_val > value)
                    {
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

    // Java: int indexOf(float value)
    pub fn indexOf(&self, value: f32) -> i32 {
        // Java: return this.indexOf(0, value);
        self.indexOf_from(0, value)
    }

    // Java: int indexOf(int offset, float value)
    pub fn indexOf_from(&self, offset: i32, value: f32) -> i32 {
        // Java: for(int i = offset; i < this._pos; ++i)
        for i in offset..self._pos
        {
            // Java: if (this._data[i] == value)
            if self._data[i as usize] == value
            {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: int lastIndexOf(float value)
    pub fn lastIndexOf(&self, value: f32) -> i32 {
        // Java: return this.lastIndexOf(this._pos, value);
        self.lastIndexOf_from(self._pos, value)
    }

    // Java: int lastIndexOf(int offset, float value)
    pub fn lastIndexOf_from(&self, offset: i32, value: f32) -> i32 {
        // Java: int i = offset;
        let mut i = offset;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 }
        {
            // Java: if (this._data[i] == value)
            if self._data[i as usize] == value
            {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: boolean contains(float value)
    pub fn contains(&self, value: f32) -> bool {
        // Java: return this.lastIndexOf(value) >= 0;
        self.lastIndexOf(value) >= 0
    }

    // Java: float max()
    pub fn max(&self) -> f32 {
        // Java: if (this.size() == 0)
        if self.size() == 0
        {
            // Java: throw new IllegalStateException("cannot find maximum of an empty list");
            panic!("IllegalStateException: cannot find maximum of an empty list");
        }
        // Java: else
        {
            // Java: float max = Float.NEGATIVE_INFINITY;
            let mut max = f32::NEG_INFINITY;
            // Java: for(int i = 0; i < this._pos; ++i)
            for i in 0..self._pos
            {
                // Java: if (this._data[i] > max)
                if self._data[i as usize] > max
                {
                    // Java: max = this._data[i];
                    max = self._data[i as usize];
                }
            }
            // Java: return max;
            max
        }
    }

    // Java: float min()
    pub fn min(&self) -> f32 {
        // Java: if (this.size() == 0)
        if self.size() == 0
        {
            // Java: throw new IllegalStateException("cannot find minimum of an empty list");
            panic!("IllegalStateException: cannot find minimum of an empty list");
        }
        // Java: else
        {
            // Java: float min = Float.POSITIVE_INFINITY;
            let mut min = f32::INFINITY;
            // Java: for(int i = 0; i < this._pos; ++i)
            for i in 0..self._pos
            {
                // Java: if (this._data[i] < min)
                if self._data[i as usize] < min
                {
                    // Java: min = this._data[i];
                    min = self._data[i as usize];
                }
            }
            // Java: return min;
            min
        }
    }

    // Java: float sum()
    pub fn sum(&self) -> f32 {
        // Java: float sum = 0.0F;
        let mut sum = 0.0f32;
        // Java: for(int i = 0; i < this._pos; ++i)
        for i in 0..self._pos
        {
            // Java: sum += this._data[i];
            sum += self._data[i as usize];
        }
        // Java: return sum;
        sum
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuilder buf = new StringBuilder("{");
        let mut buf = String::from("{");
        // Java: int i = 0;
        let mut i = 0;
        // Java: for(int end = this._pos - 1; i < end; ++i)
        let end = self._pos - 1;
        while i < end
        {
            // Java: buf.append(this._data[i]);
            buf.push_str(&self._data[i as usize].to_string());
            // Java: buf.append(", ");
            buf.push_str(", ");
            i += 1;
        }
        // Java: if (this.size() > 0)
        if self.size() > 0
        {
            // Java: buf.append(this._data[this._pos - 1]);
            buf.push_str(&self._data[(self._pos - 1) as usize].to_string());
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }
}

// Java: public class WFloatArrayIterator
impl<'a> WFloatArrayIterator<'a> {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.cursor < WFloatArrayList.this.size();
        self.cursor < self.list.size()
    }

    // Java: float next()
    pub fn next(&mut self) -> f32 {
        // Java: try
        if self.cursor < self.list.size()
        {
            // Java: float next = WFloatArrayList.this.get(this.cursor);
            let next = self.list.get(self.cursor);
            // Java: this.lastRet = this.cursor++;
            self.lastRet = self.cursor;
            self.cursor += 1;
            // Java: return next;
            next
        }
        // Java: catch (IndexOutOfBoundsException var2)
        else
        {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
        }
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: if (this.lastRet == -1)
        if self.lastRet == -1
        {
            // Java: throw new IllegalStateException();
            panic!("IllegalStateException");
        }
        // Java: else
        {
            // Java: try
            {
                // Java: WFloatArrayList.this.remove(this.lastRet, 1);
                // Java: if (this.lastRet < this.cursor)
                if self.lastRet < self.cursor
                {
                    // Java: --this.cursor;
                    self.cursor -= 1;
                }
                // Java: this.lastRet = -1;
                self.lastRet = -1;
            }
            // Java: catch (IndexOutOfBoundsException var2)
            {
                // Java: throw new ConcurrentModificationException();
            }
        }
    }
}
