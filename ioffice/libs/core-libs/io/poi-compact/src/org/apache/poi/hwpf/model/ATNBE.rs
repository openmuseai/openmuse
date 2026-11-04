use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

pub const SIZE: i32 = 10;

pub struct ATNBE {
    pub _bmc: i16,
    pub _lTag: i32,
    pub _lTagOld: i32,
}

impl ATNBE {
    pub fn new() -> Self {
        ATNBE { _bmc: 256, _lTag: 0, _lTagOld: -1 }
    }

    pub fn new_from_stream(buf: &mut DocumentInputStream, offset: i32) -> Self {
        buf.seek(offset as i64);
        let _bmc = buf.readShort();
        let _lTag = buf.readInt();
        let _lTagOld = buf.readInt();
        ATNBE { _bmc, _lTag, _lTagOld }
    }

    pub fn getLTag(&self) -> i32 { self._lTag }
    pub fn setLTag(&mut self, l_tag: i32) { self._lTag = l_tag; }

    pub fn serialize(&self, buf: &mut [u8], offset: i32) {
        let mut off = offset;
        le_put_short(buf, off, self._bmc);
        off += 2;
        le_put_int(buf, off, self._lTag);
        off += 4;
        le_put_int(buf, off, self._lTagOld);
    }
}
