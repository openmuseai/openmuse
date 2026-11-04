#![allow(non_snake_case, dead_code)]

use crate::cn::wps::kso::kproto::KoBin::{Decode, Encode, KoBin, Null};
use crate::cn::wps::kso::kproto::KoZone::KoZone;

// java public class KpBinBox
//
// Java 侧字段是 `private KoBin.Factory.IntBin bhBin = new KoBin.Factory.IntBin();`
// —— 即「按 bh（bin handle，int）装」的箱子，而非持有 KoBin 对象引用。
// Rust 用 `bh: i32` 表达同一语义（`get()` 时按 bh 还原 KoBin）。
pub struct KpBinBox {
    bh: i32,
}

impl KpBinBox {
    // java public KpBinBox()
    pub fn new() -> Self {
        Self { bh: 0 }
    }

    // java public KpBinBox(KoBin bin)
    pub fn KpBinBox_from(bin: Option<&dyn KoBin>) -> Self {
        let mut inst = Self::new();
        // Java: this.set(bin);
        inst.set(bin);
        inst
    }

    // java public KoBin set(KoBin bin)
    pub fn set(&mut self, bin: Option<&dyn KoBin>) -> Box<dyn KoBin> {
        // Java: if (bin == null) { bin = KoBin.Null.empty(); }
        let empty = Null::empty();
        let bin: &dyn KoBin = bin.unwrap_or(empty.as_ref());
        // Java: return this.bhBin.from(KoBin.Decode.toBH(bin));
        self.bh = Decode::toBH(bin);
        Encode::fromBH(self.bh, KoZone::defaultZone())
    }

    // java public KoBin get()
    pub fn get(&self) -> Box<dyn KoBin> {
        // Java: return this.empty() ? KoBin.Null.empty() : this.bhBin;
        if self.empty() {
            Null::empty()
        } else {
            Encode::fromBH(self.bh, KoZone::defaultZone())
        }
    }

    // java public void clear()
    pub fn clear(&mut self) {
        // Java: this.set((KoBin)null);
        self.set(None);
    }

    // java public boolean empty()
    pub fn empty(&self) -> bool {
        // Java: return this.bhBin.asInt() == KoBin.Null.empty().asInt();
        self.bh == Null::empty().asInt()
    }
}
