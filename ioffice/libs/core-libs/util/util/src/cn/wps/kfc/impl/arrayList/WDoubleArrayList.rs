#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.kfc.HashCode
use crate::kfc::HashCode::HashCode;
// Java: import cn.wps.kfc.api.iterator.IDoubleIterator
use crate::kfc::api::iterator::IDoubleIterator::IDoubleIterator;
// Java: import cn.wps.kfc.api.iterator.IIterator::IIterator;
use crate::kfc::api::iterator::IIterator::IIterator;
// Java: import cn.wps.kfc.api.list.IDoubleList
use crate::kfc::api::list::IDoubleList::IDoubleList;
// Java: import cn.wps.kfc.api.procedure.IDoubleProcedure
use crate::kfc::api::procedure::IDoubleProcedure::IDoubleProcedure;
// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.NoSuchElementException

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WDoubleArrayList implements IDoubleList, Serializable
pub struct WDoubleArrayList {
    // Java field: double[] _data
    pub(crate) _data: Vec<f64>,
    // Java field: int _pos
    pub(crate) _pos: i32,
    // Java field: int DEFAULT_CAPACITY
    // Java field: double no_entry_value
    pub(crate) no_entry_value: f64,
}

const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WDoubleArrayIterator
pub struct WDoubleArrayIterator<'a> {
    list: &'a WDoubleArrayList,
    // Java field: int cursor
    pub(crate) cursor: i32,
    // Java field: int lastRet
    pub(crate) lastRet: i32,
}

// Java: public class WDoubleArrayList implements IDoubleList, Serializable
impl WDoubleArrayList {
    // Java: WDoubleArrayList()
    pub fn new() -> Self {
        // Java: this(10, (double)0.0F);
        Self::new_with_capacity_and_entry(10, 0.0)
    }

    // Java: WDoubleArrayList(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: this(capacity, (double)0.0F);
        Self::new_with_capacity_and_entry(capacity, 0.0)
    }

    // Java: WDoubleArrayList(int capacity, double no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: f64) -> Self {
        // Java: super();
        // Java: this._data = new double[capacity];
        // Java: this._pos = 0;
        // Java: this.no_entry_value = no_entry_value;
        Self {
            _data: vec![0.0; capacity as usize],
            _pos: 0,
            no_entry_value,
        }
    }

    // Java: WDoubleArrayList(double[] values)
    pub fn new_from_array(values: &[f64]) -> Self {
        // Java: this(values.length);
        let mut list = Self::new_with_capacity(values.len() as i32);
        // Java: this.add(values);
        list.add_array(values);
        list
    }

    // Java: WDoubleArrayList(double[] values, double no_entry_value, boolean wrap)
    pub fn new_wrap(values: Vec<f64>, no_entry_value: f64, wrap: bool) -> Self {
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

    // Java: WDoubleArrayList(IDoubleList values)
    pub fn new_from_list(values: &dyn IDoubleList) -> Self {
        // Java: this(values.size());
        let mut list = Self::new_with_capacity(values.size());
        // Java: this.addAll(values);
        list.addAll_list(values);
        list
    }

    // Java: static WDoubleArrayList wrap(double[] values)
    pub fn wrap(values: Vec<f64>) -> Self {
        // Java: return wrap(values, (double)0.0F);
        Self::wrap_with_entry(values, 0.0)
    }

    // Java: static WDoubleArrayList wrap(double[] values, double no_entry_value)
    pub fn wrap_with_entry(values: Vec<f64>, no_entry_value: f64) -> Self {
        // Java: return new WDoubleArrayList(values, no_entry_value, true)
        Self::new_wrap(values, no_entry_value, true)
        // Java: public void ensureCapacity(int capacity)
        // Java: if (capacity > this._data.length)
        // Java: throw new IllegalStateException("Can not grow ArrayList wrapped external array");
    }

    // Java: double getNoEntryValue()
    pub fn getNoEntryValue(&self) -> f64 {
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
            // Java: double[] tmp = new double[newCap];
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
            // Java: double[] tmp = new double[this.size()];
            // Java: this.toArray(tmp, 0, tmp.length);
            // Java: this._data = tmp;
            self._data.truncate(self._pos as usize);
        }
    }

    // Java: boolean add(double val)
    pub fn add(&mut self, val: f64) -> bool {
        // Java: this.ensureCapacity(this._pos + 1);
        self.ensureCapacity(self._pos + 1);
        // Java: this._data[this._pos++] = val;
        self._data[self._pos as usize] = val;
        self._pos += 1;
        // Java: return true;
        true
    }

    // Java: void add(double[] vals)
    pub fn add_array(&mut self, vals: &[f64]) {
        // Java: this.add(vals, 0, vals.length);
        self.add_array_range(vals, 0, vals.len() as i32);
    }

    // Java: void add(double[] vals, int offset, int length)
    pub fn add_array_range(&mut self, vals: &[f64], offset: i32, length: i32) {
        // Java: this.ensureCapacity(this._pos + length);
        self.ensureCapacity(self._pos + length);
        // Java: System.arraycopy(vals, offset, this._data, this._pos, length);
        let src = &vals[offset as usize..(offset + length) as usize];
        self._data[self._pos as usize..(self._pos + length) as usize].copy_from_slice(src);
        // Java: this._pos += length;
        self._pos += length;
    }

    // Java: void insert(int offset, double value)
    pub fn insert(&mut self, offset: i32, value: f64) {
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

    // Java: void insert(int offset, double[] values)
    pub fn insert_array(&mut self, offset: i32, values: &[f64]) {
        // Java: this.insert(offset, values, 0, values.length);
        self.insert_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void insert(int offset, double[] values, int valOffset, int len)
    pub fn insert_array_range(&mut self, offset: i32, values: &[f64], val_offset: i32, len: i32) {
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

    // Java: double get(int offset)
    pub fn get(&self, offset: i32) -> f64 {
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

    // Java: double getQuick(int offset)
    pub fn getQuick(&self, offset: i32) -> f64 {
        // Java: return this._data[offset];
        self._data[offset as usize]
    }

    // Java: double set(int offset, double val)
    pub fn set(&mut self, offset: i32, val: f64) -> f64 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos
        {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        {
            // Java: double prev_val = this._data[offset];
            let prev_val = self._data[offset as usize];
            // Java: this._data[offset] = val;
            self._data[offset as usize] = val;
            // Java: return prev_val;
            prev_val
        }
    }

    // Java: void set(int offset, double[] values)
    pub fn set_array(&mut self, offset: i32, values: &[f64]) {
        // Java: this.set(offset, values, 0, values.length);
        self.set_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void set(int offset, double[] values, int valOffset, int length)
    pub fn set_array_range(&mut self, offset: i32, values: &[f64], val_offset: i32, length: i32) {
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

    // Java: void setQuick(int offset, double val)
    pub fn setQuick(&mut self, offset: i32, val: f64) {
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
        // Java: this._data = new double[capacity];
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

    // Java: boolean remove(double value)
    pub fn remove_value(&mut self, value: f64) -> bool {
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

    // Java: double removeAt(int offset)
    pub fn removeAt(&mut self, offset: i32) -> f64 {
        // Java: double old = this.get(offset);
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

    // Java: IDoubleIterator iterator()
    pub fn iterator(&self) -> WDoubleArrayIterator {
        // Java: return new WDoubleArrayIterator(0);
        WDoubleArrayIterator {
            list: self,
            cursor: 0,
            lastRet: -1,
        }
    }

    // Java: boolean addAll(Collection<? extends Double> collection)
    pub fn addAll_collection(&mut self, collection: &[f64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Double element : collection)
        for &element in collection
        {
            // Java: double e = element;
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

    // Java: boolean addAll(IDoubleList list)
    pub fn addAll_list(&mut self, list: &dyn IDoubleList) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: IDoubleIterator iter = list.iterator();
        let mut iter = list.iterator();
        // Java: while(iter.hasNext())
        while iter.hasNext()
        {
            // Java: double element = iter.next();
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

    // Java: boolean addAll(double[] array)
    pub fn addAll_array(&mut self, array: &[f64]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(double element : array)
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

    // Java: IDoubleList subList(int begin, int end)
    pub fn subList(&self, begin: i32, end: i32) -> WDoubleArrayList {
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
            // Java: WDoubleArrayList list = new WDoubleArrayList(end - begin);
            let mut list = WDoubleArrayList::new_with_capacity(end - begin);
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

    // Java: double[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: return this.toArray(0, this._pos);
        self.toArray_range(0, self._pos)
    }

    // Java: double[] toArray(int offset, int len)
    pub fn toArray_range(&self, offset: i32, len: i32) -> Vec<f64> {
        // Java: double[] rv = new double[len];
        let mut rv = vec![0.0f64; len as usize];
        // Java: this.toArray(rv, offset, len);
        self.toArray_dest_range_into(&mut rv, offset, len);
        // Java: return rv;
        rv
    }

    // Java: double[] toArray(double[] dest)
    pub fn toArray_dest(&self, dest: &mut [f64]) -> Vec<f64> {
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

    // Java: double[] toArray(double[] dest, int offset, int len)
    pub fn toArray_dest_range(&self, dest: &mut [f64], offset: i32, len: i32) -> Vec<f64> {
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

    fn toArray_dest_range_into(&self, dest: &mut [f64], offset: i32, len: i32) {
        if len == 0 {
            return;
        }
        if offset >= 0 && offset < self._pos {
            dest[..len as usize].copy_from_slice(&self._data[offset as usize..(offset + len) as usize]);
        } else {
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: double[] toArray(double[] dest, int source_pos, int dest_pos, int len)
    pub fn toArray_dest_range_offset(&self, dest: &mut [f64], source_pos: i32, dest_pos: i32, len: i32) -> Vec<f64> {
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
    pub fn equals(&self, other: &WDoubleArrayList) -> bool {
        // Java: if (other == this)
        if std::ptr::eq(self, other)
        {
            // Java: return true;
            return true;
        }
        // Java: else if (other instanceof WDoubleArrayList)
        {
            // Java: WDoubleArrayList that = (WDoubleArrayList)other;
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
            h = h.wrapping_add(HashCode::hash_double(self._data[i as usize]));
        }
        // Java: return h;
        h
    }

    // Java: boolean forEach(IDoubleProcedure procedure)
    pub fn forEach(&self, procedure: &mut dyn IDoubleProcedure) -> bool {
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

    // Java: void fill(double val)
    pub fn fill(&mut self, val: f64) {
        // Java: Arrays.fill(this._data, 0, this._pos, val);
        self._data[..self._pos as usize].fill(val);
    }

    // Java: void fill(int fromIndex, int toIndex, double val)
    pub fn fill_range(&mut self, from_index: i32, to_index: i32, val: f64) {
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

    // Java: int binarySearch(double value)
    pub fn binarySearch(&self, value: f64) -> i32 {
        // Java: return this.binarySearch(value, 0, this._pos);
        self.binarySearch_range(value, 0, self._pos)
    }

    // Java: int binarySearch(double value, int fromIndex, int toIndex)
    pub fn binarySearch_range(&self, value: f64, from_index: i32, to_index: i32) -> i32 {
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
                // Java: double midVal = this._data[mid];
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

    // Java: int indexOf(double value)
    pub fn indexOf(&self, value: f64) -> i32 {
        // Java: return this.indexOf(0, value);
        self.indexOf_from(0, value)
    }

    // Java: int indexOf(int offset, double value)
    pub fn indexOf_from(&self, offset: i32, value: f64) -> i32 {
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

    // Java: int lastIndexOf(double value)
    pub fn lastIndexOf(&self, value: f64) -> i32 {
        // Java: return this.lastIndexOf(this._pos, value);
        self.lastIndexOf_from(self._pos, value)
    }

    // Java: int lastIndexOf(int offset, double value)
    pub fn lastIndexOf_from(&self, offset: i32, value: f64) -> i32 {
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

    // Java: boolean contains(double value)
    pub fn contains(&self, value: f64) -> bool {
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

// Java: public class WDoubleArrayIterator
impl<'a> WDoubleArrayIterator<'a> {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.cursor < WDoubleArrayList.this.size();
        self.cursor < self.list.size()
    }

    // Java: double next()
    pub fn next(&mut self) -> f64 {
        // Java: try
        if self.cursor < self.list.size()
        {
            // Java: double next = WDoubleArrayList.this.get(this.cursor);
            let next = self.list.get(self.cursor);
            // Java: this.lastRet = this.cursor++;
            self.lastRet = self.cursor;
            self.cursor += 1;
            // Java: return next;
            next
        }
        // Java: catch (IndexOutOfBoundsException var3)
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
                // Java: WDoubleArrayList.this.remove(this.lastRet, 1);
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
