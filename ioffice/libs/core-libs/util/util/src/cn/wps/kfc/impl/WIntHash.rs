// Java: public abstract class WIntHash extends WPrimitiveHash
use crate::kfc::Constants;
use crate::kfc::HashCode::HashCode;
use crate::kfc::api::procedure::IIntProcedure::IIntProcedure;

// Java field: long serialVersionUID
const SERIAL_VERSION_UID: i64 = 1;

// Java: public class WIntHash extends WPrimitiveHash
pub struct WIntHash {
    // Java field: __set: &[i32]
    pub _set: Vec<i32>,
    // Java field: int no_entry_value
    pub(crate) no_entry_value: i32,
    // Java field: boolean consumeFreeSlot
    pub(crate) consumeFreeSlot: bool,
    pub(crate) _states: Vec<u8>,
    pub(crate) _size: i32,
    pub(crate) _free: i32,
    pub(crate) _loadFactor: f32,
    pub(crate) _maxSize: i32,
    pub(crate) _autoCompactRemovesRemaining: i32,
    pub(crate) _autoCompactionFactor: f32,
    pub(crate) _autoCompactTemporaryDisable: bool,
}

// Java: public class WIntHash extends WPrimitiveHash
impl WIntHash {
    // Java: WIntHash()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.no_entry_value = Constants.DEFAULT_INT_NO_ENTRY_VALUE;
        let no_entry_value = Constants::DEFAULT_INT_NO_ENTRY_VALUE;
        let mut h = WIntHash {
            _set: Vec::new(),
            no_entry_value,
            consumeFreeSlot: false,
            _states: Vec::new(),
            _size: 0,
            _free: 0,
            _loadFactor: 0.5,
            _maxSize: 0,
            _autoCompactRemovesRemaining: 0,
            _autoCompactionFactor: 0.5,
            _autoCompactTemporaryDisable: false,
        };
        h.setUp(HashCode::fastCeil(10.0 / 0.5) as i32);
        // Java: if (this.no_entry_value != 0)
        if h.no_entry_value != 0 {
            // Java: Arrays.fill(this._set, this.no_entry_value);
            h._set.fill(h.no_entry_value);
        }
        h
    }

    // Java: WIntHash(int initialCapacity)
    pub fn new_with_capacity(initialCapacity: i32) -> Self {
        // Java: super(initialCapacity);
        // Java: this.no_entry_value = Constants.DEFAULT_INT_NO_ENTRY_VALUE;
        let no_entry_value = Constants::DEFAULT_INT_NO_ENTRY_VALUE;
        let cap = std::cmp::max(1, initialCapacity);
        let load_factor = 0.5f32;
        let mut h = WIntHash {
            _set: Vec::new(),
            no_entry_value,
            consumeFreeSlot: false,
            _states: Vec::new(),
            _size: 0,
            _free: 0,
            _loadFactor: load_factor,
            _maxSize: 0,
            _autoCompactRemovesRemaining: 0,
            _autoCompactionFactor: load_factor,
            _autoCompactTemporaryDisable: false,
        };
        h.setUp(HashCode::fastCeil(cap as f32 / load_factor) as i32);
        // Java: if (this.no_entry_value != 0)
        if h.no_entry_value != 0 {
            // Java: Arrays.fill(this._set, this.no_entry_value);
            h._set.fill(h.no_entry_value);
        }
        h
    }

    // Java: WIntHash(int initialCapacity, float loadFactor)
    pub fn new_with_capacity_and_load(initialCapacity: i32, loadFactor: f32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.no_entry_value = Constants.DEFAULT_INT_NO_ENTRY_VALUE;
        let no_entry_value = Constants::DEFAULT_INT_NO_ENTRY_VALUE;
        let cap = std::cmp::max(1, initialCapacity);
        let mut h = WIntHash {
            _set: Vec::new(),
            no_entry_value,
            consumeFreeSlot: false,
            _states: Vec::new(),
            _size: 0,
            _free: 0,
            _loadFactor: loadFactor,
            _maxSize: 0,
            _autoCompactRemovesRemaining: 0,
            _autoCompactionFactor: loadFactor,
            _autoCompactTemporaryDisable: false,
        };
        h.setUp(HashCode::fastCeil(cap as f32 / loadFactor) as i32);
        // Java: if (this.no_entry_value != 0)
        if h.no_entry_value != 0 {
            // Java: Arrays.fill(this._set, this.no_entry_value);
            h._set.fill(h.no_entry_value);
        }
        h
    }

    // Java: WIntHash(int initialCapacity, float loadFactor, int no_entry_value)
    pub fn new_with_all(initialCapacity: i32, loadFactor: f32, no_entry_value: i32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.no_entry_value = no_entry_value;
        let cap = std::cmp::max(1, initialCapacity);
        let mut h = WIntHash {
            _set: Vec::new(),
            no_entry_value,
            consumeFreeSlot: false,
            _states: Vec::new(),
            _size: 0,
            _free: 0,
            _loadFactor: loadFactor,
            _maxSize: 0,
            _autoCompactRemovesRemaining: 0,
            _autoCompactionFactor: loadFactor,
            _autoCompactTemporaryDisable: false,
        };
        h.setUp(HashCode::fastCeil(cap as f32 / loadFactor) as i32);
        // Java: if (no_entry_value != 0)
        if no_entry_value != 0 {
            // Java: Arrays.fill(this._set, no_entry_value);
            h._set.fill(no_entry_value);
        }
        h
    }

    // Java: int getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i32 {
        // Java: return this.no_entry_value;
        self.no_entry_value
    }

    // Java: int setUp(int initialCapacity)
    pub fn setUp(&mut self, initialCapacity: i32) -> i32 {
        // Java: int capacity = super.setUp(initialCapacity);
        let capacity = self.setUp_base(initialCapacity);
        // Java: this._set = new int[capacity];
        self._set = vec![0i32; capacity as usize];
        // Java: return capacity;
        capacity
    }

    fn setUp_base(&mut self, initialCapacity: i32) -> i32 {
        let capacity = prime_finder::next_prime(initialCapacity);
        self.computeMaxSize(capacity);
        self.computeNextAutoCompactionAmount(initialCapacity);
        self._states = vec![0u8; capacity as usize];
        capacity
    }

    // Java: boolean contains(int val)
    pub fn contains(&self, val: i32) -> bool {
        // Java: return this.index(val) >= 0;
        self.index(val) >= 0
    }

    // Java: boolean forEach(IIntProcedure procedure)
    pub fn forEach(&self, procedure: &mut dyn IIntProcedure) -> bool {
        // Java: _states: &[u8] = this._states;
        let states = &self._states;
        // Java: _set: &[i32] = this._set;
        let set = &self._set;
        // Java: int i = set.length;
        let mut i = set.len() as i32;
        // Java: while(i-- > 0)
        while { i -= 1; i >= 0 } {
            // Java: if (states[i] == 1 && !procedure.execute(set[i]))
            if states[i as usize] == 1 && !procedure.execute(set[i as usize]) {
                // Java: return false;
                return false;
            }
        }
        // Java: return true;
        true
    }

    // Java: void removeAt(int index)
    pub fn removeAt(&mut self, index: i32) {
        // Java: this._set[index] = this.no_entry_value;
        self._set[index as usize] = self.no_entry_value;
        // Java: super.removeAt(index);
        self._states[index as usize] = 2;
        self._size -= 1;
    }

    // Java: int index(int val)
    pub fn index(&self, val: i32) -> i32 {
        // Java: _states: &[u8] = this._states;
        let states = &self._states;
        // Java: _set: &[i32] = this._set;
        let set = &self._set;
        // Java: int length = states.length;
        let length = states.len() as i32;
        // Java: int hash = HashCode.hash(val) & Integer.MAX_VALUE;
        let hash = HashCode::hash_int(val) & i32::MAX;
        // Java: int index = hash % length;
        let index = hash % length;
        // Java: byte state = states[index];
        let state = states[index as usize];
        // Java: if (state == 0)
        if state == 0 {
            // Java: return -1;
            return -1;
        }
        // Java: else
        else {
            // Java: return state == 1 && set[index] == val ? index : this.indexRehashed(val, index, hash, state);
            if state == 1 && set[index as usize] == val { index } else { self.indexRehashed(val, index, hash, state) }
        }
    }

    // Java: int indexRehashed(int key, int index, int hash, byte state)
    fn indexRehashed(&self, key: i32, mut index: i32, hash: i32, mut state: u8) -> i32 {
        // Java: int length = this._set.length;
        let length = self._set.len() as i32;
        // Java: int probe = 1 + hash % (length - 2);
        let probe = 1 + hash % (length - 2);
        // Java: int loopIndex = index;
        let loopIndex = index;
        // Java: do
        loop {
            // Java: index -= probe;
            index -= probe;
            // Java: if (index < 0)
            if index < 0 {
                // Java: index += length;
                index += length;
            }
            // Java: state = this._states[index];
            state = self._states[index as usize];
            // Java: if (state == 0)
            if state == 0 {
                // Java: return -1;
                return -1;
            }
            // Java: if (key == this._set[index] && state != 2)
            if key == self._set[index as usize] && state != 2 {
                // Java: return index;
                return index;
            }
            if index == loopIndex { break; }
        }
        // Java: return -1;
        -1
    }

    // Java: int insertKey(int val)
    pub fn insertKey(&mut self, val: i32) -> i32 {
        // Java: int hash = HashCode.hash(val) & Integer.MAX_VALUE;
        let hash = HashCode::hash_int(val) & i32::MAX;
        // Java: int index = hash % this._states.length;
        let index = hash % self._states.len() as i32;
        // Java: byte state = this._states[index];
        let state = self._states[index as usize];
        // Java: this.consumeFreeSlot = false;
        self.consumeFreeSlot = false;
        // Java: if (state == 0)
        if state == 0 {
            // Java: this.consumeFreeSlot = true;
            self.consumeFreeSlot = true;
            // Java: this.insertKeyAt(index, val);
            self.insertKeyAt(index, val);
            // Java: return index;
            return index;
        }
        // Java: else
        else {
            // Java: return state == 1 && this._set[index] == val ? -index - 1 : this.insertKeyRehash(val, index, hash, state);
            if state == 1 && self._set[index as usize] == val { -index - 1 } else { self.insertKeyRehash(val, index, hash, state) }
        }
    }

    // Java: int insertKeyRehash(int val, int index, int hash, byte state)
    fn insertKeyRehash(&mut self, val: i32, mut index: i32, hash: i32, mut state: u8) -> i32 {
        // Java: int length = this._set.length;
        let length = self._set.len() as i32;
        // Java: int probe = 1 + hash % (length - 2);
        let probe = 1 + hash % (length - 2);
        // Java: int loopIndex = index;
        let loopIndex = index;
        // Java: int firstRemoved = -1;
        let mut firstRemoved: i32 = -1;
        // Java: do
        loop {
            // Java: if (state == 2 && firstRemoved == -1)
            if state == 2 && firstRemoved == -1 {
                // Java: firstRemoved = index;
                firstRemoved = index;
            }
            // Java: index -= probe;
            index -= probe;
            // Java: if (index < 0)
            if index < 0 {
                // Java: index += length;
                index += length;
            }
            // Java: state = this._states[index];
            state = self._states[index as usize];
            // Java: if (state == 0)
            if state == 0 {
                // Java: if (firstRemoved != -1)
                if firstRemoved != -1 {
                    // Java: this.insertKeyAt(firstRemoved, val);
                    self.insertKeyAt(firstRemoved, val);
                    // Java: return firstRemoved;
                    return firstRemoved;
                }
                // Java: this.consumeFreeSlot = true;
                self.consumeFreeSlot = true;
                // Java: this.insertKeyAt(index, val);
                self.insertKeyAt(index, val);
                // Java: return index;
                return index;
            }
            // Java: if (state == 1 && this._set[index] == val)
            if state == 1 && self._set[index as usize] == val {
                // Java: return -index - 1;
                return -index - 1;
            }
            if index == loopIndex { break; }
        }
        // Java: if (firstRemoved != -1)
        if firstRemoved != -1 {
            // Java: this.insertKeyAt(firstRemoved, val);
            self.insertKeyAt(firstRemoved, val);
            // Java: return firstRemoved;
            return firstRemoved;
        }
        // Java: else
        else {
            // Java: throw new IllegalStateException("No free or removed slots available. Key set full?!!");
            panic!("No free or removed slots available. Key set full?!!");
        }
    }

    // Java: void insertKeyAt(int index, int val)
    pub fn insertKeyAt(&mut self, index: i32, val: i32) {
        // Java: this._set[index] = val;
        self._set[index as usize] = val;
        // Java: this._states[index] = 1;
        self._states[index as usize] = 1;
    }

    fn computeMaxSize(&mut self, capacity: i32) {
        self._maxSize = std::cmp::min(capacity - 1, (capacity as f32 * self._loadFactor) as i32);
        self._free = capacity - self._size;
    }

    fn computeNextAutoCompactionAmount(&mut self, size: i32) {
        if self._autoCompactionFactor != 0.0 {
            self._autoCompactRemovesRemaining = (size as f32 * self._autoCompactionFactor + 0.5) as i32;
        }
    }

    pub fn postInsertHook(&mut self, usedFreeSlot: bool) {
        if usedFreeSlot {
            self._free -= 1;
        }
        self._size += 1;
        if self._size > self._maxSize || self._free == 0 {
            let new_capacity = if self._size > self._maxSize {
                prime_finder::next_prime(self.capacity() << 1)
            } else {
                self.capacity()
            };
            self.rehash(new_capacity);
            self.computeMaxSize(self.capacity());
        }
    }

    pub fn capacity(&self) -> i32 {
        self._states.len() as i32
    }

    pub fn size(&self) -> i32 {
        self._size
    }

    pub fn rehash(&mut self, _new_capacity: i32) {
        // subclass override
    }
}

mod prime_finder {
    pub fn next_prime(n: i32) -> i32 {
        let mut candidate = if n < 2 { 2 } else { n };
        if candidate % 2 == 0 { candidate += 1; }
        while !is_prime(candidate) {
            candidate += 2;
        }
        candidate
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 { return false; }
        if n == 2 || n == 3 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 { return false; }
            i += 6;
        }
        true
    }
}
