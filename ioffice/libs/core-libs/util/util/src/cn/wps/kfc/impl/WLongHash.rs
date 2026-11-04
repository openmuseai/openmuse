// Java: import cn.wps.kfc.Constants
// Java: import cn.wps.kfc.HashCode
// Java: import cn.wps.kfc.api.procedure.ILongProcedure
// Java: import java.util.Arrays

// Java: public class WLongHash extends WPrimitiveHash
pub struct WLongHash {
    // Java field: long serialVersionUID
    // Java field: long[] _set
    pub(crate) _set: Vec<i64>,
    // Java field: long no_entry_value
    pub(crate) no_entry_value: i64,
    // Java field: boolean consumeFreeSlot
    pub(crate) consumeFreeSlot: bool,
}

impl WLongHash {
    // Java: WLongHash()
    pub fn WLongHash_new() -> Self {
        // Java: super();
        // Java: this.no_entry_value = Constants.DEFAULT_LONG_NO_ENTRY_VALUE;
        // Java: if (this.no_entry_value != 0L)
        // Java: Arrays.fill(this._set, this.no_entry_value);
        WLongHash {
            _set: Vec::new(),
            no_entry_value: 0,
            consumeFreeSlot: false,
        }
    }

    // Java: WLongHash(int initialCapacity)
    pub fn WLongHash_with_capacity(initial_capacity: usize) -> Self {
        // Java: super(initialCapacity);
        // Java: this.no_entry_value = Constants.DEFAULT_LONG_NO_ENTRY_VALUE;
        // Java: if (this.no_entry_value != 0L)
        // Java: Arrays.fill(this._set, this.no_entry_value);
        WLongHash {
            _set: Vec::with_capacity(initial_capacity),
            no_entry_value: 0,
            consumeFreeSlot: false,
        }
    }

    // Java: WLongHash(int initialCapacity, float loadFactor)
    pub fn WLongHash_with_capacity_and_load(initial_capacity: usize, _load_factor: f32) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.no_entry_value = Constants.DEFAULT_LONG_NO_ENTRY_VALUE;
        // Java: if (this.no_entry_value != 0L)
        // Java: Arrays.fill(this._set, this.no_entry_value);
        WLongHash {
            _set: Vec::with_capacity(initial_capacity),
            no_entry_value: 0,
            consumeFreeSlot: false,
        }
    }

    // Java: WLongHash(int initialCapacity, float loadFactor, long no_entry_value)
    pub fn WLongHash_with_no_entry(initial_capacity: usize, _load_factor: f32, no_entry_value: i64) -> Self {
        // Java: super(initialCapacity, loadFactor);
        // Java: this.no_entry_value = no_entry_value;
        // Java: if (no_entry_value != 0L)
        // Java: Arrays.fill(this._set, no_entry_value);
        let set = if no_entry_value != 0 {
            vec![no_entry_value; initial_capacity]
        } else {
            Vec::with_capacity(initial_capacity)
        };
        WLongHash {
            _set: set,
            no_entry_value,
            consumeFreeSlot: false,
        }
    }

    // Java: long getNoEntryValue()
    pub fn getNoEntryValue(&self) -> i64 {
        // Java: return this.no_entry_value;
        self.no_entry_value
    }

    // Java: int setUp(int initialCapacity)
    pub fn setUp(&mut self, initial_capacity: usize) -> usize {
        // Java: int capacity = super.setUp(initialCapacity);
        // Java: this._set = new long[capacity];
        // Java: return capacity;
        self._set = vec![0i64; initial_capacity];
        initial_capacity
    }

    // Java: boolean contains(long val)
    pub fn contains(&self, val: i64) -> bool {
        // Java: return this.index(val) >= 0;
        self.index(val) >= 0
    }

    // Java: boolean forEach(ILongProcedure procedure)
    pub fn forEach(&self, procedure: &mut dyn FnMut(i64) -> bool) -> bool {
        // Java: _states: &[u8] = this._states;
        // Java: long[] set = this._set;
        // Java: int i = set.length;
        // Java: while(i-- > 0)
        // Java: if (states[i] == 1 && !procedure.execute(set[i]))
        // Java: return false;
        // Java: return true;
        for &val in &self._set {
            if !procedure(val) {
                return false;
            }
        }
        true
    }

    // Java: void removeAt(int index)
    pub fn removeAt(&mut self, index: usize) {
        // Java: this._set[index] = this.no_entry_value;
        // Java: super.removeAt(index);
        if index < self._set.len() {
            self._set[index] = self.no_entry_value;
        }
    }

    // Java: int index(long val)
    pub fn index(&self, val: i64) -> i32 {
        // Java: _states: &[u8] = this._states;
        // Java: long[] set = this._set;
        // Java: int length = states.length;
        // Java: int hash = HashCode.hash(val) & Integer.MAX_VALUE;
        // Java: int index = hash % length;
        // Java: byte state = states[index];
        // Java: if (state == 0)
        // Java: return -1;
        // Java: else
        // Java: return state == 1 && set[index] == val ? index : this.indexRehashed(val, index, hash, state);
        match self._set.iter().position(|&x| x == val) {
            Some(i) => i as i32,
            None => -1,
        }
    }

    // Java: int indexRehashed(long key, int index, int hash, byte state)
    pub fn indexRehashed(&self, _key: i64, _index: i32, _hash: i32, _state: i8) -> i32 {
        // Java: int length = this._set.length;
        // Java: int probe = 1 + hash % (length - 2);
        // Java: int loopIndex = index;
        // Java: do
        // Java: index -= probe;
        // Java: if (index < 0)
        // Java: index += length;
        // Java: state = this._states[index];
        // Java: if (state == 0)
        // Java: return -1;
        // Java: if (key == this._set[index] && state != 2)
        // Java: return index;
        // Java: while(index != loopIndex);
        // Java: return -1;
        -1
    }

    // Java: int insertKey(long val)
    pub fn insertKey(&mut self, val: i64) -> i32 {
        // Java: int hash = HashCode.hash(val) & Integer.MAX_VALUE;
        // Java: int index = hash % this._states.length;
        // Java: byte state = this._states[index];
        // Java: this.consumeFreeSlot = false;
        self.consumeFreeSlot = false;
        // Java: if (state == 0)
        // Java: this.consumeFreeSlot = true;
        // Java: this.insertKeyAt(index, val);
        // Java: return index;
        // Java: else
        // Java: return state == 1 && this._set[index] == val ? -index - 1 : this.insertKeyRehash(val, index, hash, state);
        self.consumeFreeSlot = true;
        let index = self._set.len();
        self._set.push(val);
        index as i32
    }

    // Java: int insertKeyRehash(long val, int index, int hash, byte state)
    pub fn insertKeyRehash(&mut self, val: i64, _index: i32, _hash: i32, _state: i8) -> i32 {
        // Java: int length = this._set.length;
        // Java: int probe = 1 + hash % (length - 2);
        // Java: int loopIndex = index;
        // Java: int firstRemoved = -1;
        // Java: do
        // Java: if (state == 2 && firstRemoved == -1)
        // Java: firstRemoved = index;
        // Java: index -= probe;
        // Java: if (index < 0)
        // Java: index += length;
        // Java: state = this._states[index];
        // Java: if (state == 0)
        let first_removed: i32 = -1;
        // Java: if (firstRemoved != -1)
        if first_removed != -1 {
            // Java: this.insertKeyAt(firstRemoved, val);
            self.insertKeyAt(first_removed as usize, val);
            // Java: return firstRemoved;
            return first_removed;
        }
        // Java: this.consumeFreeSlot = true;
        self.consumeFreeSlot = true;
        // Java: this.insertKeyAt(index, val);
        let index = self._set.len();
        self._set.push(val);
        // Java: return index;
        // Java: if (state == 1 && this._set[index] == val)
        // Java: return -index - 1;
        // Java: while(index != loopIndex);
        // Java: if (firstRemoved != -1)
        // Java: this.insertKeyAt(firstRemoved, val);
        // Java: return firstRemoved;
        // Java: else
        // Java: throw new IllegalStateException("No free or removed slots available. Key set full?!!");
        index as i32
    }

    // Java: void insertKeyAt(int index, long val)
    pub fn insertKeyAt(&mut self, index: usize, val: i64) {
        // Java: this._set[index] = val;
        // Java: this._states[index] = 1;
        if index < self._set.len() {
            self._set[index] = val;
        } else {
            self._set.push(val);
        }
    }
}
