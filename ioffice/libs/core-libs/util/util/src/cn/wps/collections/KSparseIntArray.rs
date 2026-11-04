#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.util.Arrays

// Java: public class KSparseIntArray implements Cloneable
pub struct KSparseIntArray {
    // Java field: _mKeys: &[i32]
    mKeys: Vec<i32>,
    // Java field: _mValues: &[i32]
    mValues: Vec<i32>,
    // Java field: int mSize
    mSize: usize,
}

// Java: public class KSparseIntArray implements Cloneable
impl KSparseIntArray {
    // Java: KSparseIntArray()
    pub fn new() -> Self {
        // Java: this(10);
        Self::new_with_capacity(10)
    }

    // Java: KSparseIntArray(int initialCapacity)
    pub fn new_with_capacity(mut initial_capacity: usize) -> Self {
        // Java: super();
        // Java: if (initialCapacity <= 5)
        if initial_capacity <= 5
        {
            // Java: initialCapacity = 5;
            initial_capacity = 5;
        }
        // Java: this.mKeys = new int[initialCapacity];
        let mKeys = vec![0i32; initial_capacity];
        // Java: this.mValues = new int[this.mKeys.length];
        let mValues = vec![0i32; initial_capacity];
        // Java: this.mSize = 0;
        Self {
            mKeys,
            mValues,
            mSize: 0,
        }
    }

    // Java: static _ensureCapacity: &[i32](_values: &[i32], int capacity)
    fn ensure_capacity(values: &mut Vec<i32>, capacity: usize) {
        // Java: if (values.length < capacity)
        if values.len() < capacity
        {
            // Java: int length = values.length + values.length >> 1;
            let mut length = values.len() + (values.len() >> 1);
            // Java: if (capacity > length)
            if capacity > length
            {
                // Java: length = capacity;
                length = capacity;
            }
            // Java: __values: &[i32] = new int[length];
            // Java: System.arraycopy(values, 0, _values, 0, values.length);
            // Java: return _values;
            values.resize(length, 0);
        }
        // Java: else
        // Java: return values;
    }

    // Java: static int binarySearch(_values: &[i32], int size, int data)
    fn binary_search(values: &[i32], size: usize, data: i32) -> i32 {
        // Java: int l = 0;
        let mut l: i32 = 0;
        // Java: int u = size - 1;
        let mut u: i32 = size as i32 - 1;
        // Java: while(l <= u)
        while l <= u
        {
            // Java: int mid = (l + u) / 2;
            let mid = (l + u) / 2;
            // Java: int v = values[mid];
            let v = values[mid as usize];
            // Java: if (data < v)
            if data < v
            {
                // Java: u = mid - 1;
                u = mid - 1;
            }
            // Java: else
            else
            {
                // Java: if (data <= v)
                if data <= v
                {
                    // Java: return mid;
                    return mid;
                }
                // Java: l = mid + 1;
                l = mid + 1;
            }
        }
        // Java: return ~l;
        !l
    }

    // Java: static _insert: &[i32](_values: &[i32], int size, int index, int data)
    fn insert_at(values: &mut Vec<i32>, size: usize, index: usize, data: i32) {
        // Java: values = ensureCapacity(values, size + 1);
        Self::ensure_capacity(values, size + 1);
        // Java: for(int i = size; i > index; --i)
        let mut i = size;
        while i > index
        {
            // Java: values[i] = values[i - 1];
            values[i] = values[i - 1];
            i -= 1;
        }
        // Java: values[index] = data;
        values[index] = data;
        // Java: return values;
    }

    // Java: static _append: &[i32](_values: &[i32], int size, int data)
    fn append_val(values: &mut Vec<i32>, size: usize, data: i32) {
        // Java: values = ensureCapacity(values, size + 1);
        Self::ensure_capacity(values, size + 1);
        // Java: values[size] = data;
        values[size] = data;
        // Java: return values;
    }

    // Java: KSparseIntArray clone()
    pub fn clone(&self) -> KSparseIntArray {
        // Java: KSparseIntArray clone = null;
        // Java: try
        // Java: clone = (KSparseIntArray)super.clone();
        // Java: clone.mKeys = (int[])this.mKeys.clone();
        // Java: clone.mValues = (int[])this.mValues.clone();
        // Java: catch (CloneNotSupportedException var3)
        // Java: return clone;
        KSparseIntArray {
            mKeys: self.mKeys.clone(),
            mValues: self.mValues.clone(),
            mSize: self.mSize,
        }
    }

    // Java: int get(int key)
    pub fn get(&self, key: i32) -> i32 {
        // Java: return this.get(key, 0);
        self.get_or(key, 0)
    }

    // Java: int get(int key, int valueIfKeyNotFound)
    pub fn get_or(&self, key: i32, value_if_not_found: i32) -> i32 {
        // Java: int i = binarySearch(this.mKeys, this.mSize, key);
        let i = Self::binary_search(&self.mKeys, self.mSize, key);
        // Java: return i < 0 ? valueIfKeyNotFound : this.mValues[i];
        if i < 0 { value_if_not_found } else { self.mValues[i as usize] }
    }

    // Java: void delete(int key)
    pub fn delete(&mut self, key: i32) {
        // Java: int i = binarySearch(this.mKeys, this.mSize, key);
        let i = Self::binary_search(&self.mKeys, self.mSize, key);
        // Java: if (i >= 0)
        if i >= 0
        {
            // Java: this.removeAt(i);
            self.removeAt(i as usize);
        }
    }

    // Java: void removeAt(int index)
    pub fn removeAt(&mut self, index: usize) {
        // Java: System.arraycopy(this.mKeys, index + 1, this.mKeys, index, this.mSize - (index + 1));
        self.mKeys.copy_within((index + 1)..self.mSize, index);
        // Java: System.arraycopy(this.mValues, index + 1, this.mValues, index, this.mSize - (index + 1));
        self.mValues.copy_within((index + 1)..self.mSize, index);
        // Java: --this.mSize;
        self.mSize -= 1;
    }

    // Java: void put(int key, int value)
    pub fn put(&mut self, key: i32, value: i32) {
        // Java: int i = binarySearch(this.mKeys, this.mSize, key);
        let i = Self::binary_search(&self.mKeys, self.mSize, key);
        // Java: if (i >= 0)
        if i >= 0
        {
            // Java: this.mValues[i] = value;
            self.mValues[i as usize] = value;
        }
        // Java: else
        else
        {
            // Java: i = ~i;
            let idx = (!i) as usize;
            // Java: this.mKeys = insert(this.mKeys, this.mSize, i, key);
            Self::insert_at(&mut self.mKeys, self.mSize, idx, key);
            // Java: this.mValues = insert(this.mValues, this.mSize, i, value);
            Self::insert_at(&mut self.mValues, self.mSize, idx, value);
            // Java: ++this.mSize;
            self.mSize += 1;
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this.mSize;
        self.mSize as i32
    }

    // Java: int keyAt(int index)
    pub fn keyAt(&self, index: usize) -> i32 {
        // Java: return this.mKeys[index];
        self.mKeys[index]
    }

    // Java: int valueAt(int index)
    pub fn valueAt(&self, index: usize) -> i32 {
        // Java: return this.mValues[index];
        self.mValues[index]
    }

    // Java: void setValueAt(int index, int value)
    pub fn setValueAt(&mut self, index: usize, value: i32) {
        // Java: this.mValues[index] = value;
        self.mValues[index] = value;
    }

    // Java: int indexOfKey(int key)
    pub fn indexOfKey(&self, key: i32) -> i32 {
        // Java: return binarySearch(this.mKeys, this.mSize, key);
        Self::binary_search(&self.mKeys, self.mSize, key)
    }

    // Java: int indexOfValue(int value)
    pub fn indexOfValue(&self, value: i32) -> i32 {
        // Java: for(int i = 0; i < this.mSize; ++i)
        for i in 0..self.mSize
        {
            // Java: if (this.mValues[i] == value)
            if self.mValues[i] == value
            {
                // Java: return i;
                return i as i32;
            }
        }
        // Java: return -1;
        -1
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this.mSize = 0;
        self.mSize = 0;
    }

    // Java: void append(int key, int value)
    pub fn append(&mut self, key: i32, value: i32) {
        // Java: if (this.mSize != 0 && key <= this.mKeys[this.mSize - 1])
        if self.mSize != 0 && key <= self.mKeys[self.mSize - 1]
        {
            // Java: this.put(key, value);
            self.put(key, value);
        }
        // Java: else
        else
        {
            // Java: this.mKeys = append(this.mKeys, this.mSize, key);
            Self::append_val(&mut self.mKeys, self.mSize, key);
            // Java: this.mValues = append(this.mValues, this.mSize, value);
            Self::append_val(&mut self.mValues, self.mSize, value);
            // Java: ++this.mSize;
            self.mSize += 1;
        }
    }

    // Java: _copyKeys: &[i32]()
    pub fn copyKeys(&self) -> Option<Vec<i32>> {
        // Java: return this.size() == 0 ? null : Arrays.copyOf(this.mKeys, this.size());
        if self.mSize == 0 { None } else { Some(self.mKeys[..self.mSize].to_vec()) }
    }

    // Java: String toString()
    pub fn to_string(&self) -> String {
        // Java: if (this.size() <= 0)
        if self.mSize == 0
        {
            // Java: return "{}";
            return "{}".to_string();
        }
        // Java: else
        else
        {
            // Java: StringBuilder buffer = new StringBuilder(this.mSize * 28);
            let mut buffer = String::with_capacity(self.mSize * 28);
            // Java: buffer.append('{');
            buffer.push('{');
            // Java: for(int i = 0; i < this.mSize; ++i)
            for i in 0..self.mSize
            {
                // Java: if (i > 0)
                if i > 0
                {
                    // Java: buffer.append(", ");
                    buffer.push_str(", ");
                }
                // Java: int key = this.keyAt(i);
                let key = self.mKeys[i];
                // Java: buffer.append(key);
                buffer.push_str(&key.to_string());
                // Java: buffer.append('=');
                buffer.push('=');
                // Java: int value = this.valueAt(i);
                let value = self.mValues[i];
                // Java: buffer.append(value);
                buffer.push_str(&value.to_string());
            }
            // Java: buffer.append('}');
            buffer.push('}');
            // Java: return buffer.toString();
            buffer
        }
    }
}
