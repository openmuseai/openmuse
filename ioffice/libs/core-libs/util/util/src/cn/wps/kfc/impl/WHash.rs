#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.kfc.HashCode
use super::super::HashCode::HashCode;
// Java: import java.io.Serializable

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -1792948471915530295;
// Java field: float DEFAULT_LOAD_FACTOR
pub(crate) const DEFAULT_LOAD_FACTOR: f32 = 0.5;
// Java field: int DEFAULT_CAPACITY
pub(crate) const DEFAULT_CAPACITY: i32 = 10;

// Java: public class WHash implements Serializable
pub struct WHash {
    // Java field: int _size
    pub _size: i32,
    // Java field: int _free
    pub _free: i32,
    // Java field: float _loadFactor
    pub _loadFactor: f32,
    // Java field: int _maxSize
    pub _maxSize: i32,
    // Java field: int _autoCompactRemovesRemaining
    pub _autoCompactRemovesRemaining: i32,
    // Java field: float _autoCompactionFactor
    pub _autoCompactionFactor: f32,
    // Java field: boolean _autoCompactTemporaryDisable
    pub _autoCompactTemporaryDisable: bool,
}

// Java: public class WHash implements Serializable
impl WHash {
    // Java: WHash()
    pub fn new() -> Self {
        // Java: this(10, 0.5F);
        Self::with_capacity_and_load(10, 0.5)
    }

    // Java: WHash(int initialCapacity)
    pub fn with_capacity(initialCapacity: i32) -> Self {
        // Java: this(initialCapacity, 0.5F);
        Self::with_capacity_and_load(initialCapacity, 0.5)
    }

    // Java: WHash(int initialCapacity, float loadFactor)
    pub fn with_capacity_and_load(initialCapacity: i32, loadFactor: f32) -> Self {
        // Java: super();
        // Java: this._autoCompactTemporaryDisable = false;
        // Java: this._loadFactor = loadFactor;
        // Java: this._autoCompactionFactor = loadFactor;
        let mut wh = WHash {
            _size: 0,
            _free: 0,
            _loadFactor: loadFactor,
            _maxSize: 0,
            _autoCompactRemovesRemaining: 0,
            _autoCompactionFactor: loadFactor,
            _autoCompactTemporaryDisable: false,
        };
        // Java: this.setUp(HashCode.fastCeil((float)initialCapacity / loadFactor));
        wh.setUp(HashCode::fastCeil(initialCapacity as f32 / loadFactor));
        wh
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return 0 == this._size;
        0 == self._size
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._size;
        self._size
    }

    // Java: abstract int capacity()
    pub fn capacity(&self) -> i32 {
        0
    }

    // Java: void ensureCapacity(int desiredCapacity)
    pub fn ensureCapacity(&mut self, desiredCapacity: i32) {
        // Java: if (desiredCapacity > this._maxSize - this.size())
        if desiredCapacity > self._maxSize - self.size() {
            // Java: this.rehash(PrimeFinder.nextPrime(Math.max(this.size() + 1, HashCode.fastCeil((float)(desiredCapacity + this.size()) / this._loadFactor) + 1)));
            let size = self.size();
            self.rehash(super::PrimeFinder::PrimeFinder::nextPrime(
                (size + 1).max(HashCode::fastCeil((desiredCapacity + size) as f32 / self._loadFactor) + 1),
            ));
            // Java: this.computeMaxSize(this.capacity());
            let cap = self.capacity();
            self.computeMaxSize(cap);
        }
    }

    // Java: void compact()
    pub fn compact(&mut self) {
        // Java: this.rehash(PrimeFinder.nextPrime(Math.max(this._size + 1, HashCode.fastCeil((float)this.size() / this._loadFactor) + 1)));
        let size = self.size();
        self.rehash(super::PrimeFinder::PrimeFinder::nextPrime(
            (self._size + 1).max(HashCode::fastCeil(size as f32 / self._loadFactor) + 1),
        ));
        // Java: this.computeMaxSize(this.capacity());
        let cap = self.capacity();
        self.computeMaxSize(cap);
        // Java: if (this._autoCompactionFactor != 0.0F)
        if self._autoCompactionFactor != 0.0 {
            // Java: this.computeNextAutoCompactionAmount(this.size());
            let size = self.size();
            self.computeNextAutoCompactionAmount(size);
        }
    }

    // Java: void setAutoCompactionFactor(float factor)
    pub fn setAutoCompactionFactor(&mut self, factor: f32) {
        // Java: if (factor < 0.0F)
        if factor < 0.0 {
            // Java: throw new IllegalArgumentException("Factor must be >= 0: " + factor);
            panic!("Factor must be >= 0: {}", factor);
        }
        // Java: else
        else {
            // Java: this._autoCompactionFactor = factor;
            self._autoCompactionFactor = factor;
        }
    }

    // Java: float getAutoCompactionFactor()
    pub fn getAutoCompactionFactor(&self) -> f32 {
        // Java: return this._autoCompactionFactor;
        self._autoCompactionFactor
    }

    // Java: void trimToSize()
    pub fn trimToSize(&mut self) {
        // Java: this.compact();
        self.compact();
    }

    // Java: void removeAt(int index)
    pub fn removeAt(&mut self, _index: i32) {
        // Java: --this._size;
        self._size -= 1;
        // Java: if (this._autoCompactionFactor != 0.0F)
        if self._autoCompactionFactor != 0.0 {
            // Java: --this._autoCompactRemovesRemaining;
            self._autoCompactRemovesRemaining -= 1;
            // Java: if (!this._autoCompactTemporaryDisable && this._autoCompactRemovesRemaining <= 0)
            if !self._autoCompactTemporaryDisable && self._autoCompactRemovesRemaining <= 0 {
                // Java: this.compact();
                self.compact();
            }
        }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
        // Java: this._size = 0;
        self._size = 0;
        // Java: this._free = this.capacity();
        self._free = self.capacity();
    }

    // Java: int setUp(int initialCapacity)
    pub fn setUp(&mut self, initialCapacity: i32) -> i32 {
        // Java: int capacity = PrimeFinder.nextPrime(initialCapacity);
        let capacity = super::PrimeFinder::PrimeFinder::nextPrime(initialCapacity);
        // Java: this.computeMaxSize(capacity);
        self.computeMaxSize(capacity);
        // Java: this.computeNextAutoCompactionAmount(initialCapacity);
        self.computeNextAutoCompactionAmount(initialCapacity);
        // Java: return capacity;
        capacity
    }

    // Java: abstract void rehash(int var1)
    pub fn rehash(&mut self, _var1: i32) {
    }

    // Java: void tempDisableAutoCompaction()
    pub fn tempDisableAutoCompaction(&mut self) {
        // Java: this._autoCompactTemporaryDisable = true;
        self._autoCompactTemporaryDisable = true;
    }

    // Java: void reenableAutoCompaction(boolean check_for_compaction)
    pub fn reenableAutoCompaction(&mut self, check_for_compaction: bool) {
        // Java: this._autoCompactTemporaryDisable = false;
        self._autoCompactTemporaryDisable = false;
        // Java: if (check_for_compaction && this._autoCompactRemovesRemaining <= 0 && this._autoCompactionFactor != 0.0F)
        if check_for_compaction && self._autoCompactRemovesRemaining <= 0 && self._autoCompactionFactor != 0.0 {
            // Java: this.compact();
            self.compact();
        }
    }

    // Java: void computeMaxSize(int capacity)
    pub fn computeMaxSize(&mut self, capacity: i32) {
        // Java: this._maxSize = Math.min(capacity - 1, (int)((float)capacity * this._loadFactor));
        self._maxSize = (capacity - 1).min((capacity as f32 * self._loadFactor) as i32);
        // Java: this._free = capacity - this._size;
        self._free = capacity - self._size;
    }

    // Java: void computeNextAutoCompactionAmount(int size)
    pub fn computeNextAutoCompactionAmount(&mut self, size: i32) {
        // Java: if (this._autoCompactionFactor != 0.0F)
        if self._autoCompactionFactor != 0.0 {
            // Java: this._autoCompactRemovesRemaining = (int)((float)size * this._autoCompactionFactor + 0.5F);
            self._autoCompactRemovesRemaining = (size as f32 * self._autoCompactionFactor + 0.5) as i32;
        }
    }

    // Java: void postInsertHook(boolean usedFreeSlot)
    pub fn postInsertHook(&mut self, usedFreeSlot: bool) {
        // Java: if (usedFreeSlot)
        if usedFreeSlot {
            // Java: --this._free;
            self._free -= 1;
        }
        // Java: if (++this._size > this._maxSize || this._free == 0)
        self._size += 1;
        if self._size > self._maxSize || self._free == 0 {
            // Java: int newCapacity = this._size > this._maxSize ? PrimeFinder.nextPrime(this.capacity() << 1) : this.capacity();
            let newCapacity = if self._size > self._maxSize {
                super::PrimeFinder::PrimeFinder::nextPrime(self.capacity() << 1)
            } else {
                self.capacity()
            };
            // Java: this.rehash(newCapacity);
            self.rehash(newCapacity);
            // Java: this.computeMaxSize(this.capacity());
            let cap = self.capacity();
            self.computeMaxSize(cap);
        }
    }

    // Java: int calculateGrownCapacity()
    pub fn calculateGrownCapacity(&self) -> i32 {
        // Java: return this.capacity() << 1;
        self.capacity() << 1
    }
}
