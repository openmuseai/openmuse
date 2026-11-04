fn le_get_int(data: &[u8], offset: i32) -> i32 {
    let o = offset as usize;
    let b0 = data[o] as i32 & 0xFF;
    let b1 = data[o + 1] as i32 & 0xFF;
    let b2 = data[o + 2] as i32 & 0xFF;
    let b3 = data[o + 3] as i32 & 0xFF;
    (b3 << 24) | (b2 << 16) | (b1 << 8) | b0
}

fn le_get_short(data: &[u8], offset: i32) -> i16 {
    let o = offset as usize;
    let lo = data[o] as i16;
    let hi = data[o + 1] as i16;
    (hi << 8) | (lo & 0xFF)
}

fn le_put_int(data: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    data[o] = value as u8;
    data[o + 1] = (value >> 8) as u8;
    data[o + 2] = (value >> 16) as u8;
    data[o + 3] = (value >> 24) as u8;
}

fn le_put_short(data: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    data[o] = (v & 0xFF) as u8;
    data[o + 1] = (v >> 8) as u8;
}

pub const FSPA_SIZE: i32 = 26;

pub struct FSPA {
    pub spid: i32,
    pub xaLeft: i32,
    pub yaTop: i32,
    pub xaRight: i32,
    pub yaBottom: i32,
    pub options: i16,
    pub cTxbx: i32,
}

impl FSPA {
    pub fn new() -> Self {
        FSPA { spid: 0, xaLeft: 0, yaTop: 0, xaRight: 0, yaBottom: 0, options: 0, cTxbx: 0 }
    }

    pub fn new_from_bytes(bytes: &[u8], offset: i32) -> Self {
        let mut off = offset;
        let spid = le_get_int(bytes, off); off += 4;
        let xaLeft = le_get_int(bytes, off); off += 4;
        let yaTop = le_get_int(bytes, off); off += 4;
        let xaRight = le_get_int(bytes, off); off += 4;
        let yaBottom = le_get_int(bytes, off); off += 4;
        let options = le_get_short(bytes, off); off += 2;
        let cTxbx = le_get_int(bytes, off);
        FSPA { spid, xaLeft, yaTop, xaRight, yaBottom, options, cTxbx }
    }

    pub fn getSpid(&self) -> i32 { self.spid }
    pub fn setSpid(&mut self, v: i32) { self.spid = v; }
    pub fn getXaLeft(&self) -> i32 { self.xaLeft }
    pub fn setXaLeft(&mut self, v: i32) { self.xaLeft = v; }
    pub fn getYaTop(&self) -> i32 { self.yaTop }
    pub fn setYaTop(&mut self, v: i32) { self.yaTop = v; }
    pub fn getXaRight(&self) -> i32 { self.xaRight }
    pub fn setXaRight(&mut self, v: i32) { self.xaRight = v; }
    pub fn getYaBottom(&self) -> i32 { self.yaBottom }
    pub fn setYaBottom(&mut self, v: i32) { self.yaBottom = v; }
    pub fn getCTxbx(&self) -> i32 { self.cTxbx }
    pub fn setCTxbx(&mut self, v: i32) { self.cTxbx = v; }

    pub fn toByteArray(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 26];
        let mut offset: i32 = 0;
        le_put_int(&mut buf, offset, self.spid); offset += 4;
        le_put_int(&mut buf, offset, self.xaLeft); offset += 4;
        le_put_int(&mut buf, offset, self.yaTop); offset += 4;
        le_put_int(&mut buf, offset, self.xaRight); offset += 4;
        le_put_int(&mut buf, offset, self.yaBottom); offset += 4;
        le_put_short(&mut buf, offset, self.options); offset += 2;
        le_put_int(&mut buf, offset, self.cTxbx);
        buf
    }

    pub fn toString(&self) -> String {
        format!("spid: {}, xaLeft: {}, yaTop: {}, xaRight: {}, yaBottom: {}, options: {}, cTxbx: {}",
            self.spid, self.xaLeft, self.yaTop, self.xaRight, self.yaBottom, self.options, self.cTxbx)
    }
}
