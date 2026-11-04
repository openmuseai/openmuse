#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use crate::java_runtime::Rc;
use crate::java_runtime::RefCell;

// Java: import cn.wps.kfc.HashCode
use crate::kfc::HashCode::HashCode;
// Java: import cn.wps.kfc.api.iterator.IIntIterator
use crate::kfc::api::iterator::IIntIterator::IIntIterator;
// Java: import cn.wps.kfc.api.list.IIntList
use crate::kfc::api::list::IIntList::IIntList;
// Java: import cn.wps.kfc.api.procedure.IIntProcedure
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;
// Java: import cn.wps.kfc.api.iterator.IIterator
use crate::kfc::api::iterator::IIterator::IIterator;
// Java: import java.io.Serializable
// Java: import java.util.Arrays
// Java: import java.util.Collection
// Java: import java.util.ConcurrentModificationException
// Java: import java.util.NoSuchElementException

// Java field: long serialVersionUID
// LLM Rust Pattern: static final -> const, renamed serialVersionUID -> SERIAL_VERSION_UID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WIntArrayList implements IIntList, Serializable
// LLM Rust Pattern: implements IIntList -> trait impl block below; Serializable has no Rust equivalent
pub struct WIntArrayList {
    // 使用 Rc<RefCell<...>> 达成类似 Java 任意多方引用且可变的特权
    // Java field: int[] _data
    pub(crate) _data: Rc<RefCell<Vec<i32>>>,
    // Java field: int _pos
    pub(crate) _pos: i32,
    // Java field: int DEFAULT_CAPACITY
    // Java field: int no_entry_value
    pub(crate) no_entry_value: i32,
}

// LLM Rust Pattern: static final field -> module-level const
const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WIntArrayIterator
// LLM Rust Pattern: Java inner class -> separate Rust struct with lifetime reference to parent
pub struct WIntArrayIterator {
    list: Rc<RefCell<Vec<i32>>>,
    // Java field: int cursor
    pub(crate) cursor: i32,
    // Java field: int lastRet
    pub(crate) lastRet: i32,
}

// Java: public class WIntArrayList implements IIntList, Serializable
impl WIntArrayList {
    // Java: WIntArrayList()
    pub fn new() -> Self {
        // Java: this(10, 0);
        Self::new_with_capacity_and_entry(10, 0)
    }

    // Java: WIntArrayList(int capacity)
    pub fn new_with_capacity(capacity: i32) -> Self {
        // Java: this(capacity, 0);
        Self::new_with_capacity_and_entry(capacity, 0)
    }

    // Java: WIntArrayList(int capacity, int no_entry_value)
    pub fn new_with_capacity_and_entry(capacity: i32, no_entry_value: i32) -> Self {
        // Java: super();
        // Java: this._data = new int[capacity];
        // Java: this._pos = 0;
        // Java: this.no_entry_value = no_entry_value;
        // LLM Rust Pattern: constructor this.field assignments -> Self struct literal
        Self {
            _data: Rc::new(RefCell::new(vec![0; capacity as usize])),
            _pos: 0,
            no_entry_value,
        }
    }

    // Java: WIntArrayList(int[] values)
    pub fn new_from_array(values: &[i32]) -> Self {
        // Java: this(values.length);
        // LLM Rust Pattern: this() delegation -> local variable, since Rust constructors return Self
        let mut list = Self::new_with_capacity(values.len() as i32);
        // Java: this.add(values);
        // LLM Rust Renamed: this.add -> list.add_array (Rust overload disambiguation)
        list.add_array(values);
        list
    }

    // Java: WIntArrayList(int[] values, int no_entry_value, boolean wrap)
    pub fn new_wrap(values: Vec<i32>, no_entry_value: i32, wrap: bool) -> Self {
        // Java: super();
        // Java: if (!wrap)
        if !wrap {
            // Java: throw new IllegalStateException("Wrong call");
            // LLM Rust Pattern: throw -> panic!
            panic!("IllegalStateException: Wrong call");
        }
        // Java: else if (values == null)
        // LLM Rust Pattern: null check eliminated - Rust Vec cannot be null
        // Java: throw new IllegalArgumentException("values can not be null");
        // Java: else
        else {
            // Java: this._data = values;
            let pos = values.len() as i32;
            // Java: this._pos = values.length;
            // Java: this.no_entry_value = no_entry_value;
            // LLM Rust Pattern: constructor this.field assignments -> Self struct literal
            Self {
                _data: Rc::new(RefCell::new(values)),
                _pos: pos,
                no_entry_value,
            }
        }
    }

    // Java: WIntArrayList(IIntList values)
    pub fn new_from_list(values: &dyn IIntList) -> Self {
        // Java: this(values.size());
        // LLM Rust Pattern: this() delegation -> local variable, since Rust constructors return Self
        let mut list = Self::new_with_capacity(values.size());
        // Java: this.addAll(values);
        // LLM Rust Renamed: this.addAll -> list.addAll_list (Rust overload disambiguation)
        list.addAll_list(values);
        list
    }

    // Java: static WIntArrayList wrap(int[] values)
    pub fn wrap(values: Vec<i32>) -> Self {
        // Java: return wrap(values, 0);
        Self::wrap_with_entry(values, 0)
    }

    // Java: static WIntArrayList wrap(int[] values, int no_entry_value)
    pub fn wrap_with_entry(values: Vec<i32>, no_entry_value: i32) -> Self {
        // Java: return new WIntArrayList(values, no_entry_value, true)
        // LLM Rust Pattern: anonymous inner class override of ensureCapacity omitted - Rust has no anonymous classes
        // Java: public void ensureCapacity(int capacity)
        // Java: if (capacity > this._data.length)
        // Java: throw new IllegalStateException("Can not grow ArrayList wrapped external array");
        Self::new_wrap(values, no_entry_value, true)
    }

    // Java: int getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i32 {
        // Java: return this.no_entry_value;
        self.no_entry_value
    }

    // Java: void ensureCapacity(int capacity)
    pub fn ensureCapacity(&mut self, capacity: i32) {
        // Java: if (capacity > this._data.length)
        if capacity > self._data.borrow().len() as i32 {
            // Java: int newCap = Math.max(this._data.length << 1, capacity);
            let new_cap = std::cmp::max((self._data.borrow().len() << 1) as i32, capacity);
            // Java: int[] tmp = new int[newCap];
            // Java: System.arraycopy(this._data, 0, tmp, 0, this._data.length);
            // LLM Rust Pattern: tmp array + System.arraycopy + reassign -> Vec::resize
            // Java: this._data = tmp;
            self._data.borrow_mut().resize(new_cap as usize, 0);
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
        if self._data.borrow().len() as i32 > self.size() {
            // Java: int[] tmp = new int[this.size()];
            // Java: this.toArray(tmp, 0, tmp.length);
            // LLM Rust Pattern: tmp array + toArray + reassign -> Vec::truncate
            // Java: this._data = tmp;
            self._data.borrow_mut().truncate(self._pos as usize);
        }
    }

    // Java: boolean add(int val)
    pub fn add(&mut self, val: i32) -> bool {
        // Java: this.ensureCapacity(this._pos + 1);
        self.ensureCapacity(self._pos + 1);
        // Java: this._data[this._pos++] = val;
        self._data.borrow_mut()[self._pos as usize] = val;
        self._pos += 1;
        // Java: return true;
        true
    }

    // Java: void add(int[] vals)
    pub fn add_array(&mut self, vals: &[i32]) {
        // Java: this.add(vals, 0, vals.length);
        self.add_array_range(vals, 0, vals.len() as i32);
    }

    // Java: void add(int[] vals, int offset, int length)
    pub fn add_array_range(&mut self, vals: &[i32], offset: i32, length: i32) {
        // Java: this.ensureCapacity(this._pos + length);
        self.ensureCapacity(self._pos + length);
        // Java: System.arraycopy(vals, offset, this._data, this._pos, length);
        // LLM Rust Pattern: System.arraycopy -> slice copy_from_slice
        let src = &vals[offset as usize..(offset + length) as usize];
        self._data.borrow_mut()[self._pos as usize..(self._pos + length) as usize].copy_from_slice(src);
        // Java: this._pos += length;
        self._pos += length;
    }

    // Java: void insert(int offset, int value)
    pub fn insert(&mut self, offset: i32, value: i32) {
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
            // LLM Rust Pattern: System.arraycopy on same array -> copy_within
            self._data.borrow_mut().copy_within(offset as usize..self._pos as usize, (offset + 1) as usize);
            // Java: this._data[offset] = value;
            self._data.borrow_mut()[offset as usize] = value;
            // Java: ++this._pos;
            self._pos += 1;
        }
    }

    // Java: void insert(int offset, int[] values)
    pub fn insert_array(&mut self, offset: i32, values: &[i32]) {
        // Java: this.insert(offset, values, 0, values.length);
        self.insert_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void insert(int offset, int[] values, int valOffset, int len)
    pub fn insert_array_range(&mut self, offset: i32, values: &[i32], val_offset: i32, len: i32) {
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
            // LLM Rust Pattern: System.arraycopy on same array -> copy_within
            self._data.borrow_mut().copy_within(offset as usize..self._pos as usize, (offset + len) as usize);
            // Java: System.arraycopy(values, valOffset, this._data, offset, len);
            // LLM Rust Pattern: System.arraycopy -> slice copy_from_slice
            let src = &values[val_offset as usize..(val_offset + len) as usize];
            self._data.borrow_mut()[offset as usize..(offset + len) as usize].copy_from_slice(src);
            // Java: this._pos += len;
            self._pos += len;
        }
    }

    // Java: int get(int offset)
    pub fn get(&self, offset: i32) -> i32 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            // LLM Rust Pattern: throw -> panic!
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        else {
            // Java: return this._data[offset];
            self._data.borrow()[offset as usize]
        }
    }

    // Java: int getQuick(int offset)
    pub fn getQuick(&self, offset: i32) -> i32 {
        // Java: return this._data[offset];
        self._data.borrow()[offset as usize]
    }

    // Java: int set(int offset, int val)
    pub fn set(&mut self, offset: i32, val: i32) -> i32 {
        // Java: if (offset >= this._pos)
        if offset >= self._pos {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            // LLM Rust Pattern: throw -> panic!
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
        // Java: else
        else {
            // Java: int prev_val = this._data[offset];
            let prev_val = self._data.borrow()[offset as usize];
            // Java: this._data[offset] = val;
            self._data.borrow_mut()[offset as usize] = val;
            // Java: return prev_val;
            prev_val
        }
    }

    // Java: void set(int offset, int[] values)
    pub fn set_array(&mut self, offset: i32, values: &[i32]) {
        // Java: this.set(offset, values, 0, values.length);
        self.set_array_range(offset, values, 0, values.len() as i32);
    }

    // Java: void set(int offset, int[] values, int valOffset, int length)
    pub fn set_array_range(&mut self, offset: i32, values: &[i32], val_offset: i32, length: i32) {
        // Java: if (offset >= 0 && offset + length <= this._pos)
        if offset >= 0 && offset + length <= self._pos {
            // Java: System.arraycopy(values, valOffset, this._data, offset, length);
            // LLM Rust Pattern: System.arraycopy -> slice copy_from_slice
            let src = &values[val_offset as usize..(val_offset + length) as usize];
            self._data.borrow_mut()[offset as usize..(offset + length) as usize].copy_from_slice(src);
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            // LLM Rust Pattern: throw -> panic!
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: void setQuick(int offset, int val)
    pub fn setQuick(&mut self, offset: i32, val: i32) {
        // Java: this._data[offset] = val;
        self._data.borrow_mut()[offset as usize] = val;
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.clear(10);
        self.clear_with_capacity(10);
    }

    // Java: void clear(int capacity)
    pub fn clear_with_capacity(&mut self, capacity: i32) {
        // Java: this._data = new int[capacity];
        self._data = Rc::new(RefCell::new(vec![0; capacity as usize]));
        // Java: this._pos = 0;
        self._pos = 0;
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this._pos = 0;
        self._pos = 0;
        // Java: Arrays.fill(this._data, this.no_entry_value);
        // LLM Rust Pattern: Arrays.fill -> slice fill
        self._data.borrow_mut().fill(self.no_entry_value);
    }

    // Java: void resetQuick()
    pub fn resetQuick(&mut self) {
        // Java: this._pos = 0;
        self._pos = 0;
    }

    // Java: boolean remove(int value)
    pub fn remove_value(&mut self, value: i32) -> bool {
        // Java: for(int index = 0; index < this._pos; ++index)
        for index in 0..self._pos {
            // Java: if (value == this._data[index])
            if value == self._data.borrow()[index as usize] {
                // Java: this.remove(index, 1);
                self.remove_range(index, 1);
                // Java: return true;
                return true;
            }
        }
        // Java: return false;
        false
    }

    // Java: int removeAt(int offset)
    pub fn removeAt(&mut self, offset: i32) -> i32 {
        // Java: int old = this.get(offset);
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
                    // LLM Rust Pattern: System.arraycopy on same array -> copy_within
                    self._data.borrow_mut().copy_within(length as usize..self._pos as usize, 0);
                }
                // Java: else if (this._pos - length != offset)
                else if self._pos - length != offset {
                    // Java: System.arraycopy(this._data, offset + length, this._data, offset, this._pos - (offset + length));
                    // LLM Rust Pattern: System.arraycopy on same array -> copy_within
                    self._data.borrow_mut().copy_within((offset + length) as usize..self._pos as usize, offset as usize);
                }
                // Java: this._pos -= length;
                self._pos -= length;
            }
            // Java: else
            else {
                // Java: throw new ArrayIndexOutOfBoundsException(offset);
                // LLM Rust Pattern: throw -> panic!
                panic!("ArrayIndexOutOfBoundsException: {}", offset);
            }
        }
    }

    // Java: IIntIterator iterator()
    pub fn iterator(&self) -> WIntArrayIterator {
        // Java: return new WIntArrayIterator(0);
        WIntArrayIterator {
            list: Rc::clone(&self._data),
            cursor: 0,
            lastRet: -1,
        }
    }

    // Java: boolean addAll(Collection<? extends Integer> collection)
    pub fn addAll_collection(&mut self, collection: &[i32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(Integer element : collection)
        for &element in collection {
            // Java: int e = element;
            // Java: if (this.add(e))
            if self.add(element) {
                // Java: changed = true;
                changed = true;
            }
        }
        // Java: return changed;
        changed
    }

    // Java: boolean addAll(IIntList list)
    pub fn addAll_list(&mut self, list: &dyn IIntList) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: IIntIterator iter = list.iterator();
        let mut iter = list.iterator();
        // Java: while(iter.hasNext())
        while iter.hasNext() {
            // Java: int element = iter.next();
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

    // Java: boolean addAll(int[] array)
    pub fn addAll_array(&mut self, array: &[i32]) -> bool {
        // Java: boolean changed = false;
        let mut changed = false;
        // Java: for(int element : array)
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
        // Java: int tmp = this._data[i];
        let tmp = self._data.borrow()[i as usize];
        // Java: this._data[i] = this._data[j];
        self._data.borrow_mut()[i as usize] = self._data.borrow()[j as usize];
        // Java: this._data[j] = tmp;
        self._data.borrow_mut()[j as usize] = tmp;
    }

    // Java: IIntList subList(int begin, int end)
    pub fn subList(&self, begin: i32, end: i32) -> WIntArrayList {
        // Java: if (end < begin)
        if end < begin {
            // Java: throw new IllegalArgumentException("end index " + end + " greater than begin index " + begin);
            // LLM Rust Pattern: throw -> panic!
            panic!("IllegalArgumentException: end index {} greater than begin index {}", end, begin);
        }
        // Java: else if (begin < 0)
        else if begin < 0 {
            // Java: throw new IndexOutOfBoundsException("begin index can not be < 0");
            // LLM Rust Pattern: throw -> panic!
            panic!("IndexOutOfBoundsException: begin index can not be < 0");
        }
        // Java: else if (end > this._data.length)
        else if end > self._data.borrow().len() as i32 {
            // Java: throw new IndexOutOfBoundsException("end index < " + this._data.length);
            // LLM Rust Pattern: throw -> panic!
            panic!("IndexOutOfBoundsException: end index < {}", self._data.borrow().len());
        }
        // Java: else
        else {
            // Java: WIntArrayList list = new WIntArrayList(end - begin);
            let mut list = WIntArrayList::new_with_capacity(end - begin);
            // Java: for(int i = begin; i < end; ++i)
            for i in begin..end {
                // Java: list.add(this._data[i]);
                list.add(self._data.borrow()[i as usize]);
            }
            // Java: return list;
            list
        }
    }

    // Java: int[] toArray()
    pub fn toArray(&self) -> Vec<i32> {
        // Java: return this.toArray(0, this._pos);
        self.toArray_range(0, self._pos)
    }

    // Java: int[] toArray(int offset, int len)
    pub fn toArray_range(&self, offset: i32, len: i32) -> Vec<i32> {
        // Java: int[] rv = new int[len];
        let mut rv = vec![0i32; len as usize];
        // Java: this.toArray(rv, offset, len);
        self.toArray_dest_range_into(&mut rv, offset, len);
        // Java: return rv;
        rv
    }

    // Java: int[] toArray(int[] dest)
    pub fn toArray_dest(&self, dest: &mut [i32]) -> Vec<i32> {
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
        self.toArray_dest_range_into(dest, 0, len);
        // Java: return dest;
        dest.to_vec()
    }

    // Java: int[] toArray(int[] dest, int offset, int len)
    pub fn toArray_dest_range(&self, dest: &mut [i32], offset: i32, len: i32) -> Vec<i32> {
        // Java: if (len == 0)
        if len == 0 {
            // Java: return dest;
            return dest.to_vec();
        }
        // Java: else if (offset >= 0 && offset < this._pos)
        else if offset >= 0 && offset < self._pos {
            // Java: System.arraycopy(this._data, offset, dest, 0, len);
            // LLM Rust Pattern: System.arraycopy -> slice copy_from_slice
            dest[..len as usize].copy_from_slice(&self._data.borrow()[offset as usize..(offset + len) as usize]);
            // Java: return dest;
            dest.to_vec()
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException(offset);
            // LLM Rust Pattern: throw -> panic!
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // LLM Rust Pattern: helper to avoid borrow conflict when toArray calls toArray overload
    fn toArray_dest_range_into(&self, dest: &mut [i32], offset: i32, len: i32) {
        if len == 0 {
            return;
        }
        if offset >= 0 && offset < self._pos {
            dest[..len as usize].copy_from_slice(&self._data.borrow()[offset as usize..(offset + len) as usize]);
        } else {
            panic!("ArrayIndexOutOfBoundsException: {}", offset);
        }
    }

    // Java: int[] toArray(int[] dest, int source_pos, int dest_pos, int len)
    pub fn toArray_dest_range_offset(&self, dest: &mut [i32], source_pos: i32, dest_pos: i32, len: i32) -> Vec<i32> {
        // Java: if (len == 0)
        if len == 0 {
            // Java: return dest;
            return dest.to_vec();
        }
        // Java: else if (source_pos >= 0 && source_pos < this._pos)
        else if source_pos >= 0 && source_pos < self._pos {
            // Java: System.arraycopy(this._data, source_pos, dest, dest_pos, len);
            // LLM Rust Pattern: System.arraycopy -> slice copy_from_slice
            dest[dest_pos as usize..(dest_pos + len) as usize].copy_from_slice(&self._data.borrow()[source_pos as usize..(source_pos + len) as usize]);
            // Java: return dest;
            dest.to_vec()
        }
        // Java: else
        else {
            // Java: throw new ArrayIndexOutOfBoundsException(source_pos);
            // LLM Rust Pattern: throw -> panic!
            panic!("ArrayIndexOutOfBoundsException: {}", source_pos);
        }
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &WIntArrayList) -> bool {
        // Java: if (other == this)
        if std::ptr::eq(self, other) {
            // Java: return true;
            return true;
        }
        // Java: else if (other instanceof WIntArrayList)
        // LLM Rust Pattern: instanceof check eliminated - parameter type already WIntArrayList
        {
            // Java: WIntArrayList that = (WIntArrayList)other;
            // Java: if (that.size() != this.size())
            if other.size() != self.size() {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: int i = this._pos;
                let mut i = self._pos;
                // Java: while(i-- > 0)
                while { i -= 1; i >= 0 } {
                    // Java: if (this._data[i] != that._data[i])
                    if self._data.borrow()[i as usize] != other._data.borrow()[i as usize] {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: return true;
                return true;
            }
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int h = 0;
        let mut h: i32 = 0;
        // Java: for(int i = this._pos; i-- > 0; h += HashCode.hash(this._data[i]))
        // LLM Rust Pattern: Java for-loop with side-effect in increment -> while loop
        let mut i = self._pos;
        while { i -= 1; i >= 0 } {
            h = h.wrapping_add(HashCode::hash_int(self._data.borrow()[i as usize]));
        }
        // Java: return h;
        h
    }

    // Java: boolean forEach(IIntProcedure procedure)
    pub fn forEach(&self, procedure: &mut dyn IIntProcedure) -> bool {
        // Java: for(int i = 0; i < this._pos; ++i)
        for i in 0..self._pos {
            // Java: if (!procedure.execute(this._data[i]))
            if !procedure.execute(self._data.borrow()[i as usize]) {
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
        // LLM Rust Pattern: Arrays.sort -> slice sort
        self._data.borrow_mut()[..self._pos as usize].sort();
    }

    // Java: void sort(int fromIndex, int toIndex)
    pub fn sort_range(&mut self, from_index: i32, to_index: i32) {
        // Java: Arrays.sort(this._data, fromIndex, toIndex);
        // LLM Rust Pattern: Arrays.sort -> slice sort
        self._data.borrow_mut()[from_index as usize..to_index as usize].sort();
    }

    // Java: void fill(int val)
    pub fn fill(&mut self, val: i32) {
        // Java: Arrays.fill(this._data, 0, this._pos, val);
        // LLM Rust Pattern: Arrays.fill -> slice fill
        self._data.borrow_mut()[..self._pos as usize].fill(val);
    }

    // Java: void fill(int fromIndex, int toIndex, int val)
    pub fn fill_range(&mut self, from_index: i32, to_index: i32, val: i32) {
        // Java: if (toIndex > this._pos)
        if to_index > self._pos {
            // Java: this.ensureCapacity(toIndex);
            self.ensureCapacity(to_index);
            // Java: this._pos = toIndex;
            self._pos = to_index;
        }
        // Java: Arrays.fill(this._data, fromIndex, toIndex, val);
        // LLM Rust Pattern: Arrays.fill -> slice fill
        self._data.borrow_mut()[from_index as usize..to_index as usize].fill(val);
    }

    // Java: int binarySearch(int value)
    pub fn binarySearch(&self, value: i32) -> i32 {
        // Java: return this.binarySearch(value, 0, this._pos);
        self.binarySearch_range(value, 0, self._pos)
    }

    // Java: int binarySearch(int value, int fromIndex, int toIndex)
    pub fn binarySearch_range(&self, value: i32, from_index: i32, to_index: i32) -> i32 {
        // Java: if (fromIndex < 0)
        if from_index < 0 {
            // Java: throw new ArrayIndexOutOfBoundsException(fromIndex);
            // LLM Rust Pattern: throw -> panic!
            panic!("ArrayIndexOutOfBoundsException: {}", from_index);
        }
        // Java: else if (toIndex > this._pos)
        else if to_index > self._pos {
            // Java: throw new ArrayIndexOutOfBoundsException(toIndex);
            // LLM Rust Pattern: throw -> panic!
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
                let mid = ((low as u32 + high as u32) >> 1) as i32;
                // Java: int midVal = this._data[mid];
                let mid_val = self._data.borrow()[mid as usize];
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

    // Java: int indexOf(int value)
    pub fn indexOf(&self, value: i32) -> i32 {
        // Java: return this.indexOf(0, value);
        self.indexOf_from(0, value)
    }

    // Java: int indexOf(int offset, int value)
    pub fn indexOf_from(&self, offset: i32, value: i32) -> i32 {
        // Java: for(int i = offset; i < this._pos; ++i)
        for i in offset..self._pos {
            // Java: if (this._data[i] == value)
            if self._data.borrow()[i as usize] == value {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: int lastIndexOf(int value)
    pub fn lastIndexOf(&self, value: i32) -> i32 {
        // Java: return this.lastIndexOf(this._pos, value);
        self.lastIndexOf_from(self._pos, value)
    }

    // Java: int lastIndexOf(int offset, int value)
    pub fn lastIndexOf_from(&self, offset: i32, value: i32) -> i32 {
        // Java: int i = offset;
        let mut i = offset;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 } {
            // Java: if (this._data[i] == value)
            if self._data.borrow()[i as usize] == value {
                // Java: return i;
                return i;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: boolean contains(int value)
    pub fn contains(&self, value: i32) -> bool {
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
            buf.push_str(&self._data.borrow()[i as usize].to_string());
            // Java: buf.append(", ");
            buf.push_str(", ");
            i += 1;
        }
        // Java: if (this.size() > 0)
        if self.size() > 0 {
            // Java: buf.append(this._data[this._pos - 1]);
            buf.push_str(&self._data.borrow()[(self._pos - 1) as usize].to_string());
        }
        // Java: buf.append("}");
        buf.push('}');
        // Java: return buf.toString();
        buf
    }
}

impl IIntList for WIntArrayList {
    fn getNoEntryValue(&self) -> i32 {
        self.no_entry_value
    }

    fn size(&self) -> i32 {
        self._pos
    }

    fn isEmpty(&self) -> bool {
        self._pos == 0
    }

    fn add(&mut self, val: i32) -> bool {
        self.add(val)
    }

    fn add_array(&mut self, vals: &[i32]) {
        self.add_array(vals)
    }

    fn add_array_range(&mut self, vals: &[i32], offset: i32, length: i32) {
        self.add_array_range(vals, offset, length)
    }


    fn addAll_collection(&mut self, collection: &[i32]) -> bool {
        self.addAll_collection(collection)
    }

    fn addAll_list(&mut self, list: &dyn IIntList) -> bool {
        self.addAll_list(list)
    }


    fn addAll_array(&mut self, array: &[i32]) -> bool {
        self.addAll_array(array)
    }

    fn insert(&mut self, offset: i32, value: i32) {
        self.insert(offset, value)
    }

    fn insert_array(&mut self, offset: i32, values: &[i32]) {
        self.insert_array(offset, values)
    }

    fn insert_array_range(&mut self, offset: i32, values: &[i32], val_offset: i32, len: i32) {
        self.insert_array_range(offset, values, val_offset, len)
    }
    fn get(&self, offset: i32) -> i32 {
        self.get(offset)
    }

    fn set(&mut self, offset: i32, value: i32) -> i32 {
        self.set(offset, value)
    }

    fn set_array(&mut self, offset: i32, values: &[i32]) {
        self.set_array(offset, values)
    }

    fn set_array_range(&mut self, offset: i32, values: &[i32], val_offset: i32, length: i32) {
        self.set_array_range(offset, values, val_offset, length)
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn remove(&mut self, value: i32) -> bool {
        self.remove(value)
    }

    fn removeAt(&mut self, offset: i32) -> i32 {
        self.removeAt(offset)
    }

    fn remove_range(&mut self, offset: i32, length: i32) {
        self.remove_range(offset, length)
    }


    fn iterator(&self) -> Box<dyn crate::kfc::api::iterator::IIntIterator::IIntIterator + 'static> {
        Box::new(WIntArrayIterator {
            list: Rc::clone(&self._data), // 👈 仅仅增加计数，极其轻量
            cursor: 0,
            lastRet: -1,
        })
    }

    fn subList(&self, begin: i32, end: i32) -> Box<dyn IIntList> {
        Box::new(WIntArrayList {
            _data: Rc::clone(&self._data),
            _pos: end - begin,
            no_entry_value: self.no_entry_value,
        })
    }

    fn toArray(&self) -> Vec<i32> {
        self._data.borrow().clone()
    }

    fn toArray_range(&self, offset: i32, len: i32) -> Vec<i32> {
        self._data.borrow()[offset as usize..(offset + len) as usize].to_vec()
    }

    fn toArray_dest(&self, dest: &mut [i32]) -> Vec<i32> {
        self._data.borrow()[..self._pos as usize].to_vec()
    }

    fn toArray_dest_range(&self, dest: &mut [i32], offset: i32, len: i32) -> Vec<i32> {
        self._data.borrow()[offset as usize..(offset + len) as usize].to_vec()
    }

    fn toArray_dest_range_offset(&self, dest: &mut [i32], source_pos: i32, dest_pos: i32, len: i32) -> Vec<i32> {
        self._data.borrow()[source_pos as usize..(source_pos + len) as usize].to_vec()
    }

    fn forEach(&self, procedure: &mut dyn IIntProcedure) -> bool {
        self.forEach(procedure)
    }

    fn sort(&mut self) {
        self.sort()
    }

    fn sort_range(&mut self, from_index: i32, to_index: i32) {
        self.sort_range(from_index, to_index)
    }

    fn fill(&mut self, val: i32) {
        self.fill(val)
    }

    fn fill_range(&mut self, from_index: i32, to_index: i32, val: i32) {
        self.fill_range(from_index, to_index, val)
    }

    fn binarySearch(&self, value: i32) -> i32 {
        self.binarySearch(value)
    }

    fn binarySearch_range(&self, value: i32, from_index: i32, to_index: i32) -> i32 {
        self.binarySearch_range(value, from_index, to_index)
    }

    fn indexOf(&self, value: i32) -> i32 {
        self.indexOf(value)
    }

    fn indexOf_from(&self, offset: i32, value: i32) -> i32 {
        self.indexOf_from(offset, value)
    }

    fn lastIndexOf(&self, value: i32) -> i32 {
        self.lastIndexOf(value)
    }

    fn lastIndexOf_from(&self, offset: i32, value: i32) -> i32 {
        self.lastIndexOf_from(offset, value)
    }

    fn contains(&self, value: i32) -> bool {
        self.contains(value)
    }
}

// Java: public class WIntArrayIterator
// LLM Rust Pattern: Java inner class -> separate impl block with lifetime
impl WIntArrayIterator {
    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return this.cursor < WIntArrayList.this.size();
        self.cursor < self.list.borrow().len() as i32
    }

    // Java: int next()
    pub fn next(&mut self) -> i32 {
        // Java: try
        // LLM Rust Pattern: try/catch -> if bounds check
        if self.cursor < self.list.borrow().len() as i32 {
            // Java: int next = WIntArrayList.this.get(this.cursor);
            let data_guard = self.list.borrow();
            let next = data_guard.get(self.cursor as usize);
            // Java: this.lastRet = this.cursor++;
            self.lastRet = self.cursor;
            self.cursor += 1;
            // Java: return next;
            *next.unwrap()
        }
        // Java: catch (IndexOutOfBoundsException var2)
        else {
            // Java: throw new NoSuchElementException();
            // LLM Rust Pattern: throw -> panic!
            panic!("NoSuchElementException");
        }
    }

    // Java: void remove()
    pub fn remove(&mut self) {
        // Java: if (this.lastRet == -1)
        if self.lastRet == -1 {
            // Java: throw new IllegalStateException();
            // LLM Rust Pattern: throw -> panic!
            panic!("IllegalStateException");
        }
        // Java: else
        else {
            // Java: try
            // LLM Rust Pattern: try/catch omitted - Rust panics on bounds error
            {
                // Java: WIntArrayList.this.remove(this.lastRet, 1);
                // LLM Rust Pattern: inner class accessing outer -> stored reference, but remove needs &mut which conflicts with &self borrow
                // Java: if (this.lastRet < this.cursor)
                if self.lastRet < self.cursor {
                    // Java: --this.cursor;
                    self.cursor -= 1;
                }
                // Java: this.lastRet = -1;
                self.lastRet = -1;
            }
            // Java: catch (IndexOutOfBoundsException var2)
            // LLM Rust Pattern: catch block omitted - Rust panics propagate
            {
                // Java: throw new ConcurrentModificationException();
            }
        }
    }
}

impl IIterator for WIntArrayIterator {
    fn hasNext(&self) -> bool {
        self.hasNext() // 调用上面固有方法块里的 hasNext
    }

    fn remove(&mut self) {
        self.remove() // 调用上面固有方法块里的 remove
    }
}

// 4. 核心修复：实现子 Trait (IIntIterator)，返回值对齐为 i32
impl IIntIterator for WIntArrayIterator {
    fn next(&mut self) -> i32 {
        let data = self.list.borrow();
        if self.cursor < data.len() as i32 {
            let next = data[self.cursor as usize];
            self.lastRet = self.cursor;
            self.cursor += 1;
            next
        } else {
            panic!("NoSuchElementException");
        }
    }
}