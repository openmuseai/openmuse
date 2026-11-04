use crate::cn::wps::kso::kproto::KoBin::{Decode, Encode, KoBin};
use crate::cn::wps::kso::kproto::KoZone::KoZone;

// Java: public class KpBool
pub struct KpBool {
    pub(crate) value: i32,
    last_value: i32,
}

impl KpBool {
    pub fn new() -> Self {
        Self {
            value: 0,
            last_value: 0,
        }
    }

    pub fn KpBool_from(bin: &dyn KoBin) -> Self {
        let mut map = Self::new();
        map.load(bin);
        map
    }

    pub fn load(&mut self, bin: &dyn KoBin) {
        self.value = Decode::toInt(bin);
        self.last_value = self.value;
    }

    pub fn save(&mut self, zone: &KoZone) -> Box<dyn KoBin> {
        self.last_value = self.value;
        Encode::from_int(self.value, zone)
    }

    pub fn has(&self, index: i32) -> bool {
        index < 16 && is_on(self.value, mask_has(index))
    }

    pub fn clear(&mut self, index: i32) {
        if index < 16 {
            self.value = turn_off(self.value, mask_has(index) | mask_bit(index));
        }
    }

    pub fn get(&self, index: i32) -> bool {
        index < 16 && is_on(self.value, mask_bit(index))
    }

    pub fn set(&mut self, index: i32, on_off: bool) {
        if index < 16 {
            self.value = turn_on(self.value, mask_has(index));
            self.value = turn(self.value, mask_bit(index), on_off);
        }
    }

    pub fn dirty(&self) -> bool {
        self.value != self.last_value
    }

    // Java: public boolean equals(Object o)
    pub fn equals(&self, o: &KpBool) -> bool {
        // Java: if (null != o && o instanceof KpBool)
        // Java: KpBool dst = (KpBool)o;
        // Java: return this.value == dst.value;
        self.value == o.value
        // Java: else return false; —— Rust 形参非 Option，「null」分支由调用点承担
    }

    // Java: public int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.value;
        self.value
    }
}

fn mask_has(mut index: i32) -> i32 {
    index <<= 1;
    1 << (index + 1)
}

fn mask_bit(mut index: i32) -> i32 {
    index <<= 1;
    1 << index
}

fn is_on(rhs: i32, mask: i32) -> bool {
    (rhs & mask) != 0
}

fn turn(rhs: i32, mask: i32, on_off: bool) -> i32 {
    if on_off {
        rhs | mask
    } else {
        rhs & !mask
    }
}

fn turn_on(rhs: i32, mask: i32) -> i32 {
    turn(rhs, mask, true)
}

fn turn_off(rhs: i32, mask: i32) -> i32 {
    turn(rhs, mask, false)
}
