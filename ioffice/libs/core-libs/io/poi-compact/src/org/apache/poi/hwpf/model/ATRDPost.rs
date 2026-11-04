use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
    data[offset + 2] = bytes[2];
    data[offset + 3] = bytes[3];
}

fn le_get_short(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

pub const SIZE: i32 = 18;

pub struct ATRDPost {
    dttm_bytes: [u8; 4],
    padding1: i16,
    cDepth: i32,
    diatrdParent: i32,
    _info: i32,
}

impl ATRDPost {
    pub fn new() -> Self {
        ATRDPost {
            dttm_bytes: [0u8; 4],
            padding1: 0,
            cDepth: 0,
            diatrdParent: 0,
            _info: 0,
        }
    }

    pub fn new_from_stream(buf: &mut DocumentInputStream, _offset: i32) -> Self {
        let mut dttm_bytes = [0u8; 4];
        buf.readFully(&mut dttm_bytes);
        let padding1 = buf.readShort();
        let cDepth = buf.readInt();
        let diatrdParent = buf.readInt();
        let _info = buf.readInt();
        ATRDPost { dttm_bytes, padding1, cDepth, diatrdParent, _info }
    }

    pub fn getDepth(&self) -> i32 {
        self.cDepth
    }

    pub fn setDepth(&mut self, depth: i32) {
        self.cDepth = depth;
    }

    pub fn getOffset(&self) -> i32 {
        self.diatrdParent
    }

    pub fn setOffset(&mut self, offset: i32) {
        self.diatrdParent = offset;
    }

    pub fn serialize(&self, buf: &mut [u8], offset: usize) {
        buf[offset..offset + 4].copy_from_slice(&self.dttm_bytes);
        le_put_short(buf, offset + 4, self.padding1);
        le_put_int(buf, offset + 6, self.cDepth);
        le_put_int(buf, offset + 10, self.diatrdParent);
        le_put_int(buf, offset + 14, self._info);
    }
}
