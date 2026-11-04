fn le_get_int(data: &[u8], offset: i32) -> i32 {
    let o = offset as usize;
    (data[o] as i32) | ((data[o+1] as i32) << 8) | ((data[o+2] as i32) << 16) | ((data[o+3] as i32) << 24)
}

fn le_get_short(data: &[u8], offset: i32) -> i16 {
    let o = offset as usize;
    (data[o] as i16) | ((data[o+1] as i16) << 8)
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o+1] = (v >> 8) as u8;
}

pub const FTXBXS_SIZE: i32 = 22;

pub struct FTXBXS {
    pub ci: i32,
    pub cReusable: i32,
    pub fReusable: i16,
    pub itxbxsDest: i32,
    pub lid: i32,
    pub txidUndo: i32,
}

impl FTXBXS {
    pub fn new() -> Self {
        FTXBXS { ci: 0, cReusable: 0, fReusable: 0, itxbxsDest: 0, lid: 0, txidUndo: 0 }
    }

    pub fn new_from_buf(bytes: &[u8], offset: i32) -> Self {
        let mut off = offset;
        let ci = le_get_int(bytes, off); off += 4;
        let cReusable = le_get_int(bytes, off); off += 4;
        let fReusable = le_get_short(bytes, off); off += 2;
        let itxbxsDest = le_get_int(bytes, off); off += 4;
        let lid = le_get_int(bytes, off); off += 4;
        let txidUndo = le_get_int(bytes, off);
        FTXBXS { ci, cReusable, fReusable, itxbxsDest, lid, txidUndo }
    }

    pub fn getCi(&self) -> i32 { self.ci }
    pub fn setCi(&mut self, ci: i32) { self.ci = ci; }
    pub fn getCReusable(&self) -> i32 { self.cReusable }
    pub fn setCReusable(&mut self, c: i32) { self.cReusable = c; }
    pub fn getFReusable(&self) -> i16 { self.fReusable }
    pub fn setFReusable(&mut self, f: i16) { self.fReusable = f; }
    pub fn getItxbxsDest(&self) -> i32 { self.itxbxsDest }
    pub fn setItxbxsDest(&mut self, i: i32) { self.itxbxsDest = i; }
    pub fn getLid(&self) -> i32 { self.lid }
    pub fn setLid(&mut self, lid: i32) { self.lid = lid; }

    pub fn toByteArray(&self) -> Vec<u8> {
        let mut offset: i32 = 0;
        let mut buf = vec![0u8; 22];
        le_put_int(&mut buf, offset, self.ci); offset += 4;
        le_put_int(&mut buf, offset, self.cReusable); offset += 4;
        le_put_short(&mut buf, offset, self.fReusable); offset += 2;
        le_put_int(&mut buf, offset, self.itxbxsDest); offset += 4;
        le_put_int(&mut buf, offset, self.lid); offset += 4;
        le_put_int(&mut buf, offset, self.txidUndo);
        buf
    }
}
