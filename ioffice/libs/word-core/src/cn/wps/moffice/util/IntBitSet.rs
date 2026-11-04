#![allow(dead_code, non_snake_case)]

// java public class IntBitSet
pub struct IntBitSet {
    // java private final int shift;
    shift: i32,
    // java private final int mask;
    mask: i32,
}

impl IntBitSet {
    // java public IntBitSet(int low, int high)
    pub fn new(low: i32, high: i32) -> Self {
        Self {
            shift: low,
            mask: ((1 << (high - low + 1)) - 1) << low,
        }
    }

    // java public final int get(int flags)
    pub fn get(&self, flags: i32) -> i32 {
        (flags & self.mask) >> self.shift
    }

    // java public final int set(int flags, int v)
    pub fn set(&self, flags: i32, v: i32) -> i32 {
        (flags & !self.mask) | (v << self.shift)
    }
}
