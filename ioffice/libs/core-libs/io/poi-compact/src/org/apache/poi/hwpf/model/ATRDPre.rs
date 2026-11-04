use super::Xst::Xst;

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

fn le_get_int(data: &[u8], offset: i32) -> i32 {
    let o = offset as usize;
    (data[o] as i32) | ((data[o+1] as i32) << 8) | ((data[o+2] as i32) << 16) | ((data[o+3] as i32) << 24)
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

pub struct ATRDPre {
    pub xstUsrInitl: Xst,
    pub ibst: i16,
    pub bitsNotUsed: i16,
    pub grfNotUsed: i16,
    pub lTagBkmk: i32,
}

pub const SIZE: i32 = 30;
pub const XSTUSRINITL_SIZE: i32 = 20;

impl ATRDPre {
    pub fn new() -> Self {
        ATRDPre {
            xstUsrInitl: Xst::new(),
            ibst: 0,
            bitsNotUsed: 0,
            grfNotUsed: 0,
            lTagBkmk: 0,
        }
    }

    pub fn new_from_buf(buf: &[u8], offset: i32) -> Self {
        let mut off = offset;
        let xstUsrInitl = Xst::new_from_bytes(buf, off);
        off += 20;
        let ibst = le_get_short(buf, off);
        off += 2;
        let bitsNotUsed = le_get_short(buf, off);
        off += 2;
        let grfNotUsed = le_get_short(buf, off);
        off += 2;
        let lTagBkmk = le_get_int(buf, off);
        ATRDPre { xstUsrInitl, ibst, bitsNotUsed, grfNotUsed, lTagBkmk }
    }

    pub fn getXstUsrInitl(&self) -> &Xst { &self.xstUsrInitl }
    pub fn setXstUsrInitl(&mut self, user: Xst) { self.xstUsrInitl = user; }
    pub fn getIbst(&self) -> i32 { self.ibst as i32 }
    pub fn setIbst(&mut self, index: i32) { self.ibst = index as i16; }
    pub fn getLTagBkmk(&self) -> i32 { self.lTagBkmk }
    pub fn setLTagBkmk(&mut self, tag: i32) { self.lTagBkmk = tag; }

    pub fn toByteArray(&self) -> Vec<u8> {
        let mut offset: i32 = 0;
        let mut buf = vec![0u8; 30];
        self.xstUsrInitl.serialize(&mut buf, offset);
        offset += 20;
        le_put_short(&mut buf, offset, self.ibst);
        offset += 2;
        le_put_short(&mut buf, offset, self.bitsNotUsed);
        offset += 2;
        le_put_short(&mut buf, offset, self.grfNotUsed);
        offset += 2;
        le_put_int(&mut buf, offset, self.lTagBkmk);
        buf
    }
}
