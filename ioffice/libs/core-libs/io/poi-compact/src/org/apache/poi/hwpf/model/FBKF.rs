fn le_get_short(data: &[u8], offset: i32) -> i16 {
    let o = offset as usize;
    let lo = data[o] as i16;
    let hi = data[o + 1] as i16;
    (hi << 8) | (lo & 0xFF)
}

fn le_put_short(data: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    data[o] = (v & 0xFF) as u8;
    data[o + 1] = (v >> 8) as u8;
}

pub struct BitField {
    mask: i32,
}

impl BitField {
    pub fn new(mask: i32) -> Self { BitField { mask } }

    pub fn is_set(&self, value: i16) -> bool {
        (value as i32 & self.mask) != 0
    }

    pub fn set_boolean(&self, holder: i16, flag: bool) -> i32 {
        if flag {
            (holder as i32) | self.mask
        } else {
            (holder as i32) & !self.mask
        }
    }

    pub fn get_short_value(&self, holder: i16) -> i32 {
        let val = (holder as i32) & self.mask;
        let mut shift = 0;
        let mut m = self.mask;
        while m != 0 && (m & 1) == 0 {
            shift += 1;
            m >>= 1;
        }
        val >> shift
    }

    pub fn set_short_value(&self, holder: i16, value: i16) -> i16 {
        let mut shift = 0;
        let mut m = self.mask;
        while m != 0 && (m & 1) == 0 {
            shift += 1;
            m >>= 1;
        }
        let cleared = (holder as i32) & !self.mask;
        let set = cleared | (((value as i32) << shift) & self.mask);
        set as i16
    }
}

pub const SIZE: i32 = 4;

static ITC_FIRST: std::sync::LazyLock<BitField> = std::sync::LazyLock::new(|| BitField::new(127));
static ITC_LIM: std::sync::LazyLock<BitField> = std::sync::LazyLock::new(|| BitField::new(16128));
static F_NATIVE: std::sync::LazyLock<BitField> = std::sync::LazyLock::new(|| BitField::new(16384));
static F_COL: std::sync::LazyLock<BitField> = std::sync::LazyLock::new(|| BitField::new(32768));

pub struct FBKF {
    pub _index: i16,
    pub _info: i16,
}

impl FBKF {
    pub fn new() -> Self {
        FBKF { _index: 0, _info: 0 }
    }

    pub fn new_from_index(index: i32) -> Self {
        FBKF { _index: index as i16, _info: 0 }
    }

    pub fn new_from_buf(buf: &[u8], offset: i32) -> Self {
        let _index = le_get_short(buf, offset);
        let _info = le_get_short(buf, offset + 2);
        FBKF { _index, _info }
    }

    pub fn getIndex(&self) -> i32 { self._index as i32 }
    pub fn setIndex(&mut self, index: i32) { self._index = index as i16; }

    pub fn getItcFirst(&self) -> i32 { ITC_FIRST.get_short_value(self._info) }
    pub fn setItcFirst(&mut self, itc_first: i32) { self._info = ITC_FIRST.set_short_value(self._info, itc_first as i16); }

    pub fn getItcLim(&self) -> i32 { ITC_LIM.get_short_value(self._info) }
    pub fn setItcLim(&mut self, itc_lim: i32) { self._info = ITC_LIM.set_short_value(self._info, itc_lim as i16); }

    pub fn isNative(&self) -> bool { F_NATIVE.is_set(self._info) }
    pub fn setNative(&mut self, f_native: bool) { self._info = F_NATIVE.set_boolean(self._info, f_native) as i16; }

    pub fn isCol(&self) -> bool { F_COL.is_set(self._info) }
    pub fn setCol(&mut self, f_col: bool) { self._info = F_COL.set_boolean(self._info, f_col) as i16; }

    pub fn toByteArray(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 4];
        le_put_short(&mut buf, 0, self._index);
        le_put_short(&mut buf, 2, self._info);
        buf
    }
}
