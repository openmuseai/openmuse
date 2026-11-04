#![allow(dead_code, non_snake_case)]

// java public class BoolBitSet
pub struct BoolBitSet {
    // java private final int mask;
    mask: i32,
}

impl BoolBitSet {
    // java public BoolBitSet(int pos)
    pub fn new(pos: i32) -> Self {
        Self { mask: 1 << pos }
    }

    // java public final boolean get(int flags)
    pub fn get(&self, flags: i32) -> bool {
        (flags & self.mask) != 0
    }

    // java public final int set(int flags, boolean v)
    pub fn set(&self, flags: i32, v: bool) -> i32 {
        if v {
            flags | self.mask
        } else {
            flags & !self.mask
        }
    }
}
