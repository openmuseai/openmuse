// java public class KmoBitField
pub struct KmoBitField {
    // java private final int _mask;
    _mask: i32,
    // java private final int _shift_count;
    _shift_count: i32,
}

// java public class KmoBitField
impl KmoBitField {
    // java public  KmoBitField(int mask)
    pub fn KmoBitField(mask: i32) -> Self {
        // java this._mask = mask;
        let _mask = mask;
        // java int count = 0;
        let mut count: i32 = 0;
        // java int bit_pattern = mask;
        let mut bit_pattern: i32 = mask;
        // java if (mask != 0)
        if mask != 0 {
            // java while((bit_pattern & 1) == 0)
            while (bit_pattern & 1) == 0 {
                // java ++count;
                count += 1;
                // java bit_pattern >>= 1;
                bit_pattern >>= 1;
            }
        }
        // java this._shift_count = count;
        let _shift_count = count;
        Self {
            _mask,
            _shift_count,
        }
    }

    // java public int getValue(int holder)
    pub fn getValue(&self, holder: i32) -> i32 {
        // java return this.getRawValue(holder) >>> this._shift_count;
        ((self.getRawValue(holder) as u32) >> (self._shift_count as u32)) as i32
    }

    // java public short getShortValue(short holder)
    pub fn getShortValue(&self, holder: i16) -> i16 {
        // java return (short)this.getValue(holder);
        self.getValue(holder as i32) as i16
    }

    // java public int getRawValue(int holder)
    pub fn getRawValue(&self, holder: i32) -> i32 {
        // java return holder & this._mask;
        holder & self._mask
    }

    // java public short getShortRawValue(short holder)
    pub fn getShortRawValue(&self, holder: i16) -> i16 {
        // java return (short)this.getRawValue(holder);
        self.getRawValue(holder as i32) as i16
    }

    // java public boolean isSet(int holder)
    pub fn isSet(&self, holder: i32) -> bool {
        // java return (holder & this._mask) != 0;
        (holder & self._mask) != 0
    }

    // java public boolean isAllSet(int holder)
    pub fn isAllSet(&self, holder: i32) -> bool {
        // java return (holder & this._mask) == this._mask;
        (holder & self._mask) == self._mask
    }

    // java public int setValue(int holder, int value)
    pub fn setValue(&self, holder: i32, value: i32) -> i32 {
        // java return holder & ~this._mask | value << this._shift_count & this._mask;
        holder & !self._mask | (value.wrapping_shl(self._shift_count as u32) & self._mask)
    }

    // java public short setShortValue(short holder, short value)
    pub fn setShortValue(&self, holder: i16, value: i16) -> i16 {
        // java return (short)this.setValue(holder, value);
        self.setValue(holder as i32, value as i32) as i16
    }

    // java public int clear(int holder)
    pub fn clear(&self, holder: i32) -> i32 {
        // java return holder & ~this._mask;
        holder & !self._mask
    }

    // java public short clearShort(short holder)
    pub fn clearShort(&self, holder: i16) -> i16 {
        // java return (short)this.clear(holder);
        self.clear(holder as i32) as i16
    }

    // java public byte clearByte(byte holder)
    pub fn clearByte(&self, holder: i8) -> i8 {
        // java return (byte)this.clear(holder);
        self.clear(holder as i32) as i8
    }

    // java public int set(int holder)
    pub fn set(&self, holder: i32) -> i32 {
        // java return holder | this._mask;
        holder | self._mask
    }

    // java public short setShort(short holder)
    pub fn setShort(&self, holder: i16) -> i16 {
        // java return (short)this.set(holder);
        self.set(holder as i32) as i16
    }

    // java public byte setByte(byte holder)
    pub fn setByte(&self, holder: i8) -> i8 {
        // java return (byte)this.set(holder);
        self.set(holder as i32) as i8
    }

    // java public int setBoolean(int holder, boolean flag)
    pub fn setBoolean(&self, holder: i32, flag: bool) -> i32 {
        // java return flag ? this.set(holder) : this.clear(holder);
        if flag { self.set(holder) } else { self.clear(holder) }
    }

    // java public short setShortBoolean(short holder, boolean flag)
    pub fn setShortBoolean(&self, holder: i16, flag: bool) -> i16 {
        // java return flag ? this.setShort(holder) : this.clearShort(holder);
        if flag {
            self.setShort(holder)
        } else {
            self.clearShort(holder)
        }
    }

    // java public byte setByteBoolean(byte holder, boolean flag)
    pub fn setByteBoolean(&self, holder: i8, flag: bool) -> i8 {
        // java return flag ? this.setByte(holder) : this.clearByte(holder);
        if flag {
            self.setByte(holder)
        } else {
            self.clearByte(holder)
        }
    }
}
