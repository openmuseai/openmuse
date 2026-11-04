// Java: public class BitField
#[derive(Clone)]
pub struct BitField {
    // Java field: int _mask
    pub(crate) _mask: i32,
    // Java field: int _shift_count
    pub(crate) _shift_count: i32,
}

// Java: public class BitField
impl BitField {
    // Java: BitField(int mask)
    pub fn new(mask: i32) -> Self {
        // Java: super();
        // LLM JavaLine Aggregation: no parent class in Rust
        // Java: this._mask = mask;
        // LLM JavaLine Aggregation: merged into struct constructor at end of fn
        // Java: int count = 0;
        let mut count = 0;
        // Java: int bit_pattern = mask;
        let mut bit_pattern = mask;
        // Java: if (mask != 0)
        if mask != 0 {
            // Java: while((bit_pattern & 1) == 0)
            while (bit_pattern & 1) == 0 {
                // Java: ++count;
                count += 1;
                // Java: bit_pattern >>= 1;
                bit_pattern >>= 1;
            }
        }
        // Java: this._shift_count = count;
        // LLM JavaLine Aggregation: merged into struct constructor below
        BitField { _mask: mask, _shift_count: count }
    }

    // Java: int getValue(int holder)
    pub fn getValue(&self, holder: i32) -> i32 {
        // Java: return this.getRawValue(holder) >>> this._shift_count;
        ((self.getRawValue(holder) as u32) >> self._shift_count) as i32
    }

    // Java: short getShortValue(short holder)
    pub fn getShortValue(&self, holder: i16) -> i16 {
        // Java: return (short)this.getValue(holder);
        self.getValue(holder as i32) as i16
    }

    // Java: int getRawValue(int holder)
    pub fn getRawValue(&self, holder: i32) -> i32 {
        // Java: return holder & this._mask;
        holder & self._mask
    }

    // Java: short getShortRawValue(short holder)
    pub fn getShortRawValue(&self, holder: i16) -> i16 {
        // Java: return (short)this.getRawValue(holder);
        self.getRawValue(holder as i32) as i16
    }

    // Java: boolean isSet(int holder)
    pub fn isSet(&self, holder: i32) -> bool {
        // Java: return (holder & this._mask) != 0;
        (holder & self._mask) != 0
    }

    // Java: boolean isAllSet(int holder)
    pub fn isAllSet(&self, holder: i32) -> bool {
        // Java: return (holder & this._mask) == this._mask;
        (holder & self._mask) == self._mask
    }

    // Java: int setValue(int holder, int value)
    pub fn setValue(&self, holder: i32, value: i32) -> i32 {
        // Java: return holder & ~this._mask | value << this._shift_count & this._mask;
        (holder & !self._mask)
            | (value.wrapping_shl(self._shift_count as u32) & self._mask)
    }

    // Java: short setShortValue(short holder, short value)
    pub fn setShortValue(&self, holder: i16, value: i16) -> i16 {
        // Java: return (short)this.setValue(holder, value);
        self.setValue(holder as i32, value as i32) as i16
    }

    // Java: int clear(int holder)
    pub fn clear(&self, holder: i32) -> i32 {
        // Java: return holder & ~this._mask;
        holder & !self._mask
    }

    // Java: short clearShort(short holder)
    pub fn clearShort(&self, holder: i16) -> i16 {
        // Java: return (short)this.clear(holder);
        self.clear(holder as i32) as i16
    }

    // Java: byte clearByte(byte holder)
    pub fn clearByte(&self, holder: i8) -> i8 {
        // Java: return (byte)this.clear(holder);
        self.clear(holder as i32) as i8
    }

    // Java: int set(int holder)
    pub fn set(&self, holder: i32) -> i32 {
        // Java: return holder | this._mask;
        holder | self._mask
    }

    // Java: short setShort(short holder)
    pub fn setShort(&self, holder: i16) -> i16 {
        // Java: return (short)this.set(holder);
        self.set(holder as i32) as i16
    }

    // Java: byte setByte(byte holder)
    pub fn setByte(&self, holder: i8) -> i8 {
        // Java: return (byte)this.set(holder);
        self.set(holder as i32) as i8
    }

    // Java: int setBoolean(int holder, boolean flag)
    pub fn setBoolean(&self, holder: i32, flag: bool) -> i32 {
        // Java: return flag ? this.set(holder) : this.clear(holder);
        if flag { self.set(holder) } else { self.clear(holder) }
    }

    // Java: short setShortBoolean(short holder, boolean flag)
    pub fn setShortBoolean(&self, holder: i16, flag: bool) -> i16 {
        // Java: return flag ? this.setShort(holder) : this.clearShort(holder);
        if flag { self.setShort(holder) } else { self.clearShort(holder) }
    }

    // Java: byte setByteBoolean(byte holder, boolean flag)
    pub fn setByteBoolean(&self, holder: i8, flag: bool) -> i8 {
        // Java: return flag ? this.setByte(holder) : this.clearByte(holder);
        if flag { self.setByte(holder) } else { self.clearByte(holder) }
    }
}
