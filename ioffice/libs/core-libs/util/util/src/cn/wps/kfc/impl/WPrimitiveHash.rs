#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.kfc.HashCode
use super::super::HashCode::HashCode;
use super::WHash::WHash;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 1;

pub(crate) const FREE: u8 = 0;
pub(crate) const FULL: u8 = 1;
pub(crate) const REMOVED: u8 = 2;

// Java: public class WPrimitiveHash extends WHash
pub struct WPrimitiveHash {
    pub base: WHash,
    // Java field: __states: &[u8]
    pub _states: Vec<u8>,
    // Java field: byte FREE
    // Java field: byte FULL
    // Java field: byte REMOVED
}

// Java: public class WPrimitiveHash extends WHash
impl WPrimitiveHash {
    // Java: WPrimitiveHash()
    pub fn new() -> Self {
        // Java: super();
        WPrimitiveHash {
            base: WHash::new(),
            _states: Vec::new(),
        }
    }

    // Java: WPrimitiveHash(int initialCapacity)
    pub fn with_capacity(initialCapacity: i32) -> Self {
        // Java: this(initialCapacity, 0.5F);
        Self::with_capacity_and_load(initialCapacity, 0.5)
    }

    // Java: WPrimitiveHash(int initialCapacity, float loadFactor)
    pub fn with_capacity_and_load(initialCapacity: i32, loadFactor: f32) -> Self {
        // Java: super();
        // Java: initialCapacity = Math.max(1, initialCapacity);
        let initialCapacity = initialCapacity.max(1);
        // Java: this._loadFactor = loadFactor;
        let mut wph = WPrimitiveHash {
            base: WHash::with_capacity_and_load(1, loadFactor),
            _states: Vec::new(),
        };
        wph.base._loadFactor = loadFactor;
        // Java: this.setUp(HashCode.fastCeil((float)initialCapacity / loadFactor));
        wph.setUp(HashCode::fastCeil(initialCapacity as f32 / loadFactor));
        wph
    }

    // Java: int capacity()
    pub fn capacity(&self) -> i32 {
        // Java: return this._states.length;
        self._states.len() as i32
    }

    // Java: void removeAt(int index)
    pub fn removeAt(&mut self, index: i32) {
        // Java: this._states[index] = 2;
        self._states[index as usize] = 2;
        // Java: super.removeAt(index);
        self.base.removeAt(index);
    }

    // Java: int setUp(int initialCapacity)
    pub fn setUp(&mut self, initialCapacity: i32) -> i32 {
        // Java: int capacity = super.setUp(initialCapacity);
        let capacity = self.base.setUp(initialCapacity);
        // Java: this._states = new byte[capacity];
        self._states = vec![0u8; capacity as usize];
        // Java: return capacity;
        capacity
    }
}
