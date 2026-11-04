#![allow(non_snake_case, dead_code)]

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    (data[offset] as i16) | ((data[offset + 1] as i16) << 8)
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

pub const FTBKD_SIZE: i32 = 6;

pub struct FTBKD {
    pub itxbxs: i16,
    pub dcpDepend: i16,
    pub options: i16,
}

impl FTBKD {
    pub fn new() -> Self {
        FTBKD { itxbxs: 0, dcpDepend: 0, options: 0 }
    }

    pub fn new_from_bytes(bytes: &[u8], offset: i32) -> Self {
        let o = offset as usize;
        let itxbxs = le_get_short(bytes, o);
        let dcp_depend = le_get_short(bytes, o + 2);
        let options = le_get_short(bytes, o + 4);
        FTBKD { itxbxs, dcpDepend: dcp_depend, options }
    }

    pub fn toByteArray(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 6];
        le_put_short(&mut buf, 0, self.itxbxs);
        le_put_short(&mut buf, 2, self.dcpDepend);
        le_put_short(&mut buf, 4, self.options);
        buf
    }
}
