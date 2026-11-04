use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

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

pub struct Xst {
    pub cch: i16,
    pub entry: String,
}

impl Xst {
    pub fn new() -> Self {
        Xst { cch: 0, entry: String::new() }
    }

    pub fn new_from_stream(table_stream: &mut DocumentInputStream, offset: i32) -> Self {
        table_stream.seek(offset as i64);
        let cch = table_stream.readShort();
        let buf_size = (cch as i32 * 2) as usize;
        let mut buf = vec![0u8; buf_size];
        table_stream.read_off(&mut buf, 0, buf_size as i32);
        let entry = String::from_utf16_lossy(
            &buf.chunks(2)
                .map(|c| u16::from_le_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
                .collect::<Vec<u16>>()
        );
        Xst { cch, entry }
    }

    pub fn new_from_bytes(table_stream: &[u8], offset: i32) -> Self {
        let cch = le_get_short(table_stream, offset);
        let off = (offset + 2) as usize;
        let size = std::cmp::min(cch as i32 * 2, 18) as usize;
        let end = std::cmp::min(off + size, table_stream.len());
        let slice = &table_stream[off..end];
        let entry = String::from_utf16_lossy(
            &slice.chunks(2)
                .map(|c| u16::from_le_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
                .collect::<Vec<u16>>()
        );
        Xst { cch, entry }
    }

    pub fn getNumberOfChars(&self) -> i32 {
        self.cch as i32
    }

    pub fn setNumberOfChars(&mut self, count: i32) {
        self.cch = count as i16;
    }

    pub fn getString(&self) -> &str {
        &self.entry
    }

    pub fn setString(&mut self, s: String) {
        self.entry = s;
    }

    pub fn serialize(&self, buf: &mut [u8], offset: i32) {
        le_put_short(buf, offset, self.cch);
        let off = (offset + 2) as usize;
        let utf16: Vec<u16> = self.entry.encode_utf16().collect();
        for (i, &ch) in utf16.iter().enumerate() {
            let bytes = ch.to_le_bytes();
            let pos = off + i * 2;
            if pos + 1 < buf.len() {
                buf[pos] = bytes[0];
                buf[pos + 1] = bytes[1];
            }
        }
    }
}
