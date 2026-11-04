// Java: public class IntList
pub struct IntList {
    // Java field: __array: &[i32]
    _array: Vec<i32>,
    // Java field: int _limit
    _limit: usize,
    // Java field: int fillval
    fillval: i32,
    // Java field: int DEFAULT_CAPACITY
}

pub const DEFAULT_CAPACITY: i32 = 128;

// Java: public class IntList
impl IntList {
    // Java: IntList()
    pub fn new() -> Self {
        // Java: this(128);
        Self::with_capacity(128)
    }

    // Java: IntList(int initialCapacity)
    pub fn with_capacity(initial_capacity: usize) -> Self {
        // Java: this(initialCapacity, 0);
        Self::with_capacity_and_fill(initial_capacity, 0)
    }

    // Java: IntList(IntList list)
    pub fn from_list(list: &IntList) -> Self {
        // Java: this(list._array.length);
        let mut new_list = Self::with_capacity(list._array.len());
        // Java: System.arraycopy(list._array, 0, this._array, 0, this._array.length);
        new_list._array[..list._array.len()].copy_from_slice(&list._array);
        // Java: this._limit = list._limit;
        new_list._limit = list._limit;
        new_list
    }

    // Java: IntList(int initialCapacity, int fillvalue)
    pub fn with_capacity_and_fill(initial_capacity: usize, fillvalue: i32) -> Self {
        // Java: super();
        // Java: this.fillval = 0;
        let mut s = Self {
            _array: Vec::new(),
            _limit: 0,
            fillval: 0,
        };
        // Java: if (initialCapacity != 0)
        if initial_capacity != 0 {
            // Java: this._array = new int[initialCapacity];
            s._array = vec![0; initial_capacity];
            // Java: if (this.fillval != 0)
            if fillvalue != 0 {
                // Java: this.fillval = fillvalue;
                s.fillval = fillvalue;
                // Java: this.fillArray(this.fillval, this._array, 0);
                s.fillArray(s.fillval, 0);
            }
            // Java: this._limit = 0;
            s._limit = 0;
        }
        s
    }

    // Java: void fillArray(int val, _array: &[i32], int index)
    fn fillArray(&mut self, val: i32, index: usize) {
        // Java: for(int k = index; k < array.length; ++k)
        for k in index..self._array.len() {
            // Java: array[k] = val;
            self._array[k] = val;
        }
    }

    // Java: void add(int index, int value)
    pub fn add_at(&mut self, index: usize, value: i32) {
        // Java: if (index > this._limit)
        if index > self._limit {
            // Java: throw new IndexOutOfBoundsException("Index " + index + " out of bounds for length" + this._limit);
            panic!("Index {} out of bounds for length {}", index, self._limit);
        // Java: else
        } else {
            // Java: if (index == this._limit)
            if index == self._limit {
                // Java: this.add(value);
                self.add(value);
            // Java: else
            } else {
                // Java: if (this._limit == this._array.length)
                if self._limit == self._array.len() {
                    // Java: this.growArray(this._limit * 2);
                    self.growArray(self._limit * 2);
                }
                // Java: System.arraycopy(this._array, index, this._array, index + 1, this._limit - index);
                self._array.copy_within(index..self._limit, index + 1);
                // Java: this._array[index] = value;
                self._array[index] = value;
                // Java: ++this._limit;
                self._limit += 1;
            }
        }
    }

    // Java: boolean add(int value)
    pub fn add(&mut self, value: i32) -> bool {
        // Java: if (this._limit == this._array.length)
        if self._limit == self._array.len() {
            // Java: this.growArray(this._limit * 2);
            self.growArray(self._limit * 2);
        }
        // Java: this._array[this._limit++] = value;
        self._array[self._limit] = value;
        self._limit += 1;
        // Java: return true;
        true
    }

    // Java: boolean addAll(IntList c)
    pub fn addAll(&mut self, c: &IntList) -> bool {
        // Java: if (c._limit != 0)
        if c._limit != 0 {
            // Java: if (this._limit + c._limit > this._array.length)
            if self._limit + c._limit > self._array.len() {
                // Java: this.growArray(this._limit + c._limit);
                self.growArray(self._limit + c._limit);
            }
            // Java: System.arraycopy(c._array, 0, this._array, this._limit, c._limit);
            self._array[self._limit..self._limit + c._limit].copy_from_slice(&c._array[..c._limit]);
            // Java: this._limit += c._limit;
            self._limit += c._limit;
        }
        // Java: return true;
        true
    }

    // Java: boolean addAll(int index, IntList c)
    pub fn addAll_at(&mut self, index: usize, c: &IntList) -> bool {
        // Java: if (index > this._limit)
        if index > self._limit {
            // Java: throw new IndexOutOfBoundsException("Index " + index + " out of bounds for length" + this._limit);
            panic!("Index {} out of bounds for length {}", index, self._limit);
        // Java: else
        } else {
            // Java: if (c._limit != 0)
            if c._limit != 0 {
                // Java: if (this._limit + c._limit > this._array.length)
                if self._limit + c._limit > self._array.len() {
                    // Java: this.growArray(this._limit + c._limit);
                    self.growArray(self._limit + c._limit);
                }
                // Java: System.arraycopy(this._array, index, this._array, index + c._limit, this._limit - index);
                self._array.copy_within(index..self._limit, index + c._limit);
                // Java: System.arraycopy(c._array, 0, this._array, index, c._limit);
                self._array[index..index + c._limit].copy_from_slice(&c._array[..c._limit]);
                // Java: this._limit += c._limit;
                self._limit += c._limit;
            }
            // Java: return true;
            true
        }
    }

    // Java: void addAll(IntBuffer buf)
    pub fn addAll_buf(&mut self, buf: &[i32]) {
        // Java: int n = buf.limit();
        let n = buf.len();
        // Java: if (n != 0)
        if n != 0 {
            // Java: if (this._limit + n > this._array.length)
            if self._limit + n > self._array.len() {
                // Java: this.growArray(this._limit + n);
                self.growArray(self._limit + n);
            }
            // Java: buf.get(this._array, this._limit, n);
            self._array[self._limit..self._limit + n].copy_from_slice(buf);
            // Java: this._limit += n;
            self._limit += n;
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this._limit = 0;
        self._limit = 0;
    }

    // Java: boolean contains(int o)
    pub fn contains(&self, o: i32) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: for(int j = 0; !rval && j < this._limit; ++j)
        for j in 0..self._limit {
            if rval { break; }
            // Java: if (this._array[j] == o)
            if self._array[j] == o {
                // Java: rval = true;
                rval = true;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: boolean containsAll(IntList c)
    pub fn containsAll(&self, c: &IntList) -> bool {
        // Java: boolean rval = true;
        let mut rval = true;
        // Java: if (this != c)
        // Java: for(int j = 0; rval && j < c._limit; ++j)
        for j in 0..c._limit {
            if !rval { break; }
            // Java: if (!this.contains(c._array[j]))
            if !self.contains(c._array[j]) {
                // Java: rval = false;
                rval = false;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &IntList) -> bool {
        // Java: boolean rval = this == o;
        // Java: if (!rval && o != null && o.getClass() == this.getClass())
        // Java: IntList other = (IntList)o;
        // Java: if (other._limit == this._limit)
        if other._limit == self._limit {
            // Java: rval = true;
            let mut rval = true;
            // Java: for(int j = 0; rval && j < this._limit; ++j)
            for j in 0..self._limit {
                if !rval { break; }
                // Java: rval = this._array[j] == other._array[j];
                rval = self._array[j] == other._array[j];
            }
            return rval;
        }
        // Java: return rval;
        false
    }

    // Java: int get(int index)
    pub fn get(&self, index: usize) -> i32 {
        // Java: if (index >= this._limit)
        if index >= self._limit {
            // Java: throw new IndexOutOfBoundsException("Index " + index + " out of bounds for length" + this._limit);
            panic!("Index {} out of bounds for length {}", index, self._limit);
        // Java: else
        }
        // Java: return this._array[index];
        self._array[index]
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int hash = 0;
        let mut hash: i32 = 0;
        // Java: for(int j = 0; j < this._limit; ++j)
        for j in 0..self._limit {
            // Java: hash = 31 * hash + this._array[j];
            hash = hash.wrapping_mul(31).wrapping_add(self._array[j]);
        }
        // Java: return hash;
        hash
    }

    // Java: int indexOf(int o)
    pub fn indexOf(&self, o: i32) -> i32 {
        // Java: int rval;
        // Java: for(rval = 0; rval < this._limit && o != this._array[rval]; ++rval)
        let mut rval: usize = 0;
        while rval < self._limit && o != self._array[rval] {
            rval += 1;
        }
        // Java: if (rval == this._limit)
        if rval == self._limit {
            // Java: rval = -1;
            return -1;
        }
        // Java: return rval;
        rval as i32
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return this._limit == 0;
        self._limit == 0
    }

    // Java: int lastIndexOf(int o)
    pub fn lastIndexOf(&self, o: i32) -> i32 {
        // Java: int rval;
        // Java: for(rval = this._limit - 1; rval >= 0 && o != this._array[rval]; --rval)
        if self._limit == 0 { return -1; }
        let mut rval = self._limit as i32 - 1;
        while rval >= 0 && o != self._array[rval as usize] {
            rval -= 1;
        }
        // Java: return rval;
        rval
    }

    // Java: int remove(int index)
    pub fn remove(&mut self, index: usize) -> i32 {
        // Java: if (index >= this._limit)
        if index >= self._limit {
            // Java: throw new IndexOutOfBoundsException("Index " + index + " out of bounds for length" + this._limit);
            panic!("Index {} out of bounds for length {}", index, self._limit);
        // Java: else
        }
        // Java: int rval = this._array[index];
        let rval = self._array[index];
        // Java: System.arraycopy(this._array, index + 1, this._array, index, this._limit - index - 1);
        self._array.copy_within(index + 1..self._limit, index);
        // Java: --this._limit;
        self._limit -= 1;
        // Java: return rval;
        rval
    }

    // Java: boolean removeValue(int o)
    pub fn removeValue(&mut self, o: i32) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: for(int j = 0; !rval && j < this._limit; ++j)
        let mut j = 0;
        while !rval && j < self._limit {
            // Java: if (o == this._array[j])
            if o == self._array[j] {
                // Java: if (j + 1 < this._limit)
                if j + 1 < self._limit {
                    // Java: System.arraycopy(this._array, j + 1, this._array, j, this._limit - j - 1);
                    self._array.copy_within(j + 1..self._limit, j);
                }
                // Java: --this._limit;
                self._limit -= 1;
                // Java: rval = true;
                rval = true;
            }
            j += 1;
        }
        // Java: return rval;
        rval
    }

    // Java: boolean removeAll(IntList c)
    pub fn removeAll(&mut self, c: &IntList) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: for(int j = 0; j < c._limit; ++j)
        for j in 0..c._limit {
            // Java: if (this.removeValue(c._array[j]))
            if self.removeValue(c._array[j]) {
                // Java: rval = true;
                rval = true;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: boolean retainAll(IntList c)
    pub fn retainAll(&mut self, c: &IntList) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: int j = 0;
        let mut j = 0;
        // Java: while(j < this._limit)
        while j < self._limit {
            // Java: if (!c.contains(this._array[j]))
            if !c.contains(self._array[j]) {
                // Java: this.remove(j);
                self.remove(j);
                // Java: rval = true;
                rval = true;
            // Java: else
            } else {
                // Java: ++j;
                j += 1;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: int set(int index, int element)
    pub fn set(&mut self, index: usize, element: i32) -> i32 {
        // Java: if (index >= this._limit)
        if index >= self._limit {
            // Java: throw new IndexOutOfBoundsException("Index " + index + " out of bounds for length" + this._limit);
            panic!("Index {} out of bounds for length {}", index, self._limit);
        // Java: else
        }
        // Java: int rval = this._array[index];
        let rval = self._array[index];
        // Java: this._array[index] = element;
        self._array[index] = element;
        // Java: return rval;
        rval
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._limit;
        self._limit as i32
    }

    // Java: _toArray: &[i32]()
    pub fn toArray(&self) -> Vec<i32> {
        // Java: _rval: &[i32] = new int[this._limit];
        // Java: System.arraycopy(this._array, 0, rval, 0, this._limit);
        // Java: return rval;
        self._array[..self._limit].to_vec()
    }

    // Java: _toArray: &[i32](_a: &[i32])
    pub fn toArray_into(&self, a: &mut [i32]) -> Vec<i32> {
        // Java: _rval: &[i32];
        // Java: if (a.length == this._limit)
        if a.len() == self._limit {
            // Java: System.arraycopy(this._array, 0, a, 0, this._limit);
            a.copy_from_slice(&self._array[..self._limit]);
            // Java: rval = a;
            return a.to_vec();
        // Java: else
        }
        // Java: rval = this.toArray();
        // Java: return rval;
        self.toArray()
    }

    // Java: void growArray(int new_size)
    fn growArray(&mut self, new_size: usize) {
        // Java: int size = new_size == this._array.length ? new_size + 1 : new_size;
        let size = if new_size == self._array.len() { new_size + 1 } else { new_size };
        // Java: _new_array: &[i32] = new int[size];
        let mut new_array = vec![0i32; size];
        // Java: if (this.fillval != 0)
        if self.fillval != 0 {
            // Java: this.fillArray(this.fillval, new_array, this._array.length);
            let old_len = self._array.len();
            for k in old_len..new_array.len() {
                new_array[k] = self.fillval;
            }
        }
        // Java: System.arraycopy(this._array, 0, new_array, 0, this._limit);
        new_array[..self._limit].copy_from_slice(&self._array[..self._limit]);
        // Java: this._array = new_array;
        self._array = new_array;
    }
}

impl Clone for IntList {
    fn clone(&self) -> Self {
        IntList::from_list(self)
    }
}
