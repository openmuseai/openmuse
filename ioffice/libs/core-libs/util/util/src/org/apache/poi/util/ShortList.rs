// Java: public class ShortList
pub struct ShortList {
    // Java field: short[] _array
    pub(crate) array: Vec<i16>,
    // Java field: int _limit
    pub(crate) limit: usize,
    // Java field: int _default_size
}

pub(crate) const DEFAULT_SIZE: usize = 128;

// Java: public class ShortList
impl ShortList {
    // Java: ShortList()
    pub fn new() -> Self {
        // Java: this(128);
        Self::new_with_capacity(128)
    }

    // Java: ShortList(ShortList list)
    pub fn new_from(list: &ShortList) -> Self {
        // Java: this(list._array.length);
        let mut s = Self::new_with_capacity(list.array.len());
        // Java: System.arraycopy(list._array, 0, this._array, 0, this._array.length);
        s.array[..list.array.len()].copy_from_slice(&list.array);
        // Java: this._limit = list._limit;
        s.limit = list.limit;
        s
    }

    // Java: ShortList(int initialCapacity)
    pub fn new_with_capacity(initial_capacity: usize) -> Self {
        // Java: super();
        // Java: this._array = new short[initialCapacity];
        // Java: this._limit = 0;
        ShortList {
            array: vec![0i16; initial_capacity],
            limit: 0,
        }
    }

    // Java: void add(int index, short value)
    pub fn add_at(&mut self, index: usize, value: i16) {
        // Java: if (index > this._limit)
        if index > self.limit {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
        }
        // Java: else
        else {
            // Java: if (index == this._limit)
            if index == self.limit {
                // Java: this.add(value);
                self.add(value);
            }
            // Java: else
            else {
                // Java: if (this._limit == this._array.length)
                if self.limit == self.array.len() {
                    // Java: this.growArray(this._limit * 2);
                    self.grow_array(self.limit * 2);
                }
                // Java: System.arraycopy(this._array, index, this._array, index + 1, this._limit - index);
                let count = self.limit - index;
                for i in (0..count).rev() {
                    self.array[index + 1 + i] = self.array[index + i];
                }
                // Java: this._array[index] = value;
                self.array[index] = value;
                // Java: ++this._limit;
                self.limit += 1;
            }
        }
    }

    // Java: boolean add(short value)
    pub fn add(&mut self, value: i16) -> bool {
        // Java: if (this._limit == this._array.length)
        if self.limit == self.array.len() {
            // Java: this.growArray(this._limit * 2);
            self.grow_array(self.limit * 2);
        }
        // Java: this._array[this._limit++] = value;
        self.array[self.limit] = value;
        self.limit += 1;
        // Java: return true;
        true
    }

    // Java: boolean addAll(ShortList c)
    pub fn add_all(&mut self, c: &ShortList) -> bool {
        // Java: if (c._limit != 0)
        if c.limit != 0 {
            // Java: if (this._limit + c._limit > this._array.length)
            if self.limit + c.limit > self.array.len() {
                // Java: this.growArray(this._limit + c._limit);
                self.grow_array(self.limit + c.limit);
            }
            // Java: System.arraycopy(c._array, 0, this._array, this._limit, c._limit);
            self.array[self.limit..self.limit + c.limit].copy_from_slice(&c.array[..c.limit]);
            // Java: this._limit += c._limit;
            self.limit += c.limit;
        }
        // Java: return true;
        true
    }

    // Java: boolean addAll(int index, ShortList c)
    pub fn add_all_at(&mut self, index: usize, c: &ShortList) -> bool {
        // Java: if (index > this._limit)
        if index > self.limit {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
        }
        // Java: else
        else {
            // Java: if (c._limit != 0)
            if c.limit != 0 {
                // Java: if (this._limit + c._limit > this._array.length)
                if self.limit + c.limit > self.array.len() {
                    // Java: this.growArray(this._limit + c._limit);
                    self.grow_array(self.limit + c.limit);
                }
                // Java: System.arraycopy(this._array, index, this._array, index + c._limit, this._limit - index);
                let count = self.limit - index;
                for i in (0..count).rev() {
                    self.array[index + c.limit + i] = self.array[index + i];
                }
                // Java: System.arraycopy(c._array, 0, this._array, index, c._limit);
                self.array[index..index + c.limit].copy_from_slice(&c.array[..c.limit]);
                // Java: this._limit += c._limit;
                self.limit += c.limit;
            }
            // Java: return true;
            true
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this._limit = 0;
        self.limit = 0;
    }

    // Java: boolean contains(short o)
    pub fn contains(&self, o: i16) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: for(int j = 0; !rval && j < this._limit; ++j)
        let mut j = 0;
        while !rval && j < self.limit {
            // Java: if (this._array[j] == o)
            if self.array[j] == o {
                // Java: rval = true;
                rval = true;
            }
            j += 1;
        }
        // Java: return rval;
        rval
    }

    // Java: boolean containsAll(ShortList c)
    pub fn contains_all(&self, c: &ShortList) -> bool {
        // Java: boolean rval = true;
        let mut rval = true;
        // Java: if (this != c)
        {
            // Java: for(int j = 0; rval && j < c._limit; ++j)
            let mut j = 0;
            while rval && j < c.limit {
                // Java: if (!this.contains(c._array[j]))
                if !self.contains(c.array[j]) {
                    // Java: rval = false;
                    rval = false;
                }
                j += 1;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, other: &ShortList) -> bool {
        // Java: boolean rval = this == o;
        // Java: if (!rval && o != null && o.getClass() == this.getClass())
        // Java: ShortList other = (ShortList)o;
        // Java: if (other._limit == this._limit)
        if other.limit == self.limit {
            // Java: rval = true;
            let mut rval = true;
            // Java: for(int j = 0; rval && j < this._limit; ++j)
            let mut j = 0;
            while rval && j < self.limit {
                // Java: rval = this._array[j] == other._array[j];
                rval = self.array[j] == other.array[j];
                j += 1;
            }
            rval
        } else {
            false
        }
        // Java: return rval;
    }

    // Java: short get(int index)
    pub fn get(&self, index: usize) -> i16 {
        // Java: if (index >= this._limit)
        if index >= self.limit {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
        }
        // Java: else
        else {
            // Java: return this._array[index];
            self.array[index]
        }
    }

    // Java: int hashCode()
    pub fn hash_code(&self) -> i32 {
        // Java: int hash = 0;
        let mut hash: i32 = 0;
        // Java: for(int j = 0; j < this._limit; ++j)
        for j in 0..self.limit {
            // Java: hash = 31 * hash + this._array[j];
            hash = hash.wrapping_mul(31).wrapping_add(self.array[j] as i32);
        }
        // Java: return hash;
        hash
    }

    // Java: int indexOf(short o)
    pub fn index_of(&self, o: i16) -> i32 {
        // Java: int rval;
        // Java: for(rval = 0; rval < this._limit && o != this._array[rval]; ++rval)
        let mut rval = 0usize;
        while rval < self.limit && o != self.array[rval] {
            rval += 1;
        }
        // Java: if (rval == this._limit)
        if rval == self.limit {
            // Java: rval = -1;
            return -1;
        }
        // Java: return rval;
        rval as i32
    }

    // Java: boolean isEmpty()
    pub fn is_empty(&self) -> bool {
        // Java: return this._limit == 0;
        self.limit == 0
    }

    // Java: int lastIndexOf(short o)
    pub fn last_index_of(&self, o: i16) -> i32 {
        // Java: int rval;
        // Java: for(rval = this._limit - 1; rval >= 0 && o != this._array[rval]; --rval)
        if self.limit == 0 { return -1; }
        let mut rval = self.limit as i32 - 1;
        while rval >= 0 && o != self.array[rval as usize] {
            rval -= 1;
        }
        // Java: return rval;
        rval
    }

    // Java: short remove(int index)
    pub fn remove(&mut self, index: usize) -> i16 {
        // Java: if (index >= this._limit)
        if index >= self.limit {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
        }
        // Java: else
        else {
            // Java: short rval = this._array[index];
            let rval = self.array[index];
            // Java: System.arraycopy(this._array, index + 1, this._array, index, this._limit - index - 1);
            let count = self.limit - index - 1;
            for i in 0..count {
                self.array[index + i] = self.array[index + 1 + i];
            }
            // Java: --this._limit;
            self.limit -= 1;
            // Java: return rval;
            rval
        }
    }

    // Java: boolean removeValue(short o)
    pub fn remove_value(&mut self, o: i16) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: for(int j = 0; !rval && j < this._limit; ++j)
        let mut j = 0;
        while !rval && j < self.limit {
            // Java: if (o == this._array[j])
            if o == self.array[j] {
                // Java: System.arraycopy(this._array, j + 1, this._array, j, this._limit - j - 1);
                let count = self.limit - j - 1;
                for i in 0..count {
                    self.array[j + i] = self.array[j + 1 + i];
                }
                // Java: --this._limit;
                self.limit -= 1;
                // Java: rval = true;
                rval = true;
            }
            j += 1;
        }
        // Java: return rval;
        rval
    }

    // Java: boolean removeAll(ShortList c)
    pub fn remove_all(&mut self, c: &ShortList) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: for(int j = 0; j < c._limit; ++j)
        for j in 0..c.limit {
            // Java: if (this.removeValue(c._array[j]))
            if self.remove_value(c.array[j]) {
                // Java: rval = true;
                rval = true;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: boolean retainAll(ShortList c)
    pub fn retain_all(&mut self, c: &ShortList) -> bool {
        // Java: boolean rval = false;
        let mut rval = false;
        // Java: int j = 0;
        let mut j = 0usize;
        // Java: while(j < this._limit)
        while j < self.limit {
            // Java: if (!c.contains(this._array[j]))
            if !c.contains(self.array[j]) {
                // Java: this.remove(j);
                self.remove(j);
                // Java: rval = true;
                rval = true;
            }
            // Java: else
            else {
                // Java: ++j;
                j += 1;
            }
        }
        // Java: return rval;
        rval
    }

    // Java: short set(int index, short element)
    pub fn set(&mut self, index: usize, element: i16) -> i16 {
        // Java: if (index >= this._limit)
        if index >= self.limit {
            // Java: throw new IndexOutOfBoundsException();
            panic!("IndexOutOfBoundsException");
        }
        // Java: else
        else {
            // Java: short rval = this._array[index];
            let rval = self.array[index];
            // Java: this._array[index] = element;
            self.array[index] = element;
            // Java: return rval;
            rval
        }
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._limit;
        self.limit as i32
    }

    // Java: short[] toArray()
    pub fn to_array(&self) -> Vec<i16> {
        // Java: short[] rval = new short[this._limit];
        // Java: System.arraycopy(this._array, 0, rval, 0, this._limit);
        // Java: return rval;
        self.array[..self.limit].to_vec()
    }

    // Java: short[] toArray(short[] a)
    pub fn to_array_into(&self, a: &mut Vec<i16>) -> Vec<i16> {
        // Java: short[] rval;
        // Java: if (a.length == this._limit)
        if a.len() == self.limit {
            // Java: System.arraycopy(this._array, 0, a, 0, this._limit);
            a[..self.limit].copy_from_slice(&self.array[..self.limit]);
            // Java: rval = a;
            a.clone()
        }
        // Java: else
        else {
            // Java: rval = this.toArray();
            self.to_array()
        }
        // Java: return rval;
    }

    // Java: void growArray(int new_size)
    fn grow_array(&mut self, new_size: usize) {
        // Java: int size = new_size == this._array.length ? new_size + 1 : new_size;
        let size = if new_size == self.array.len() { new_size + 1 } else { new_size };
        // Java: short[] new_array = new short[size];
        let mut new_array = vec![0i16; size];
        // Java: System.arraycopy(this._array, 0, new_array, 0, this._limit);
        new_array[..self.limit].copy_from_slice(&self.array[..self.limit]);
        // Java: this._array = new_array;
        self.array = new_array;
    }
}
