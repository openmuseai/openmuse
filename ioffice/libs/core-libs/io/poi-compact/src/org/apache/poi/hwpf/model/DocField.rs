use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    let bytes = value.to_le_bytes();
    buf[o] = bytes[0];
    buf[o + 1] = bytes[1];
    buf[o + 2] = bytes[2];
    buf[o + 3] = bytes[3];
}

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

fn get_from_unicode_le(buf: &[u8], offset: i32, len: i32) -> String {
    let o = offset as usize;
    let l = len as usize;
    let chars: Vec<u16> = buf[o..o + l * 2]
        .chunks(2)
        .map(|c| u16::from_le_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
        .collect();
    String::from_utf16_lossy(&chars)
}

fn put_unicode_le(s: &str, buf: &mut [u8], offset: i32) {
    let o = offset as usize;
    for (i, ch) in s.encode_utf16().enumerate() {
        let bytes = ch.to_le_bytes();
        let pos = o + i * 2;
        if pos + 1 < buf.len() {
            buf[pos] = bytes[0];
            buf[pos + 1] = bytes[1];
        }
    }
}

pub struct DocField {
    _nGcpStart: i32,
    _nGcpEnd: i32,
    _nBoolAttr: i8,
    _nStrLen: i16,
    _strName: Vec<u8>,
    _reserved: i32,
}

impl DocField {
    pub fn new() -> Self {
        DocField {
            _nGcpStart: 0,
            _nGcpEnd: 0,
            _nBoolAttr: 0,
            _nStrLen: 0,
            _strName: Vec::new(),
            _reserved: 0,
        }
    }

    pub fn new_from_stream(stream: &mut DocumentInputStream) -> Self {
        let _nGcpStart = stream.readInt();
        let _nGcpEnd = stream.readInt();
        let _nBoolAttr = stream.readByte();
        let _nStrLen = stream.readShort();
        let mut _strName = vec![0u8; (_nStrLen as usize) * 2];
        stream.read(&mut _strName);
        let _reserved = stream.readInt();
        DocField {
            _nGcpStart,
            _nGcpEnd,
            _nBoolAttr,
            _nStrLen,
            _strName,
            _reserved,
        }
    }

    pub fn getName(&self) -> String {
        get_from_unicode_le(&self._strName, 0, self._nStrLen as i32)
    }

    pub fn getGcpStart(&self) -> i32 {
        self._nGcpStart
    }

    pub fn getGcpEnd(&self) -> i32 {
        self._nGcpEnd
    }

    pub fn getBooleanAttr(&self) -> i8 {
        self._nBoolAttr
    }

    pub fn getReserved(&self) -> i32 {
        self._reserved
    }

    pub fn setGcpStart(&mut self, val: i32) {
        self._nGcpStart = val;
    }

    pub fn setGcpEnd(&mut self, val: i32) {
        self._nGcpEnd = val;
    }

    pub fn setBoolAttr(&mut self, val: i8) {
        self._nBoolAttr = val;
    }

    pub fn setName(&mut self, str_name: &str) {
        self._nStrLen = (0xFFFF & str_name.len()) as i16;
        self._strName = vec![0u8; 2 + (self._nStrLen as usize) * 2];
        put_unicode_le(str_name, &mut self._strName, 0);
    }

    pub fn setReserved(&mut self, val: i32) {
        self._reserved = val;
    }

    pub fn writeTo(&self, stream: &mut HWPFOutputStream) {
        let buf_len = 15 + (self._nStrLen as usize) * 2;
        let mut buf = vec![0u8; buf_len];
        let mut offset: i32 = 0;
        le_put_int(&mut buf, offset, self._nGcpStart);
        offset += 4;
        le_put_int(&mut buf, offset, self._nGcpEnd);
        offset += 4;
        buf[offset as usize] = self._nBoolAttr as u8;
        offset += 1;
        le_put_short(&mut buf, offset, self._nStrLen);
        offset += 2;
        let src_len = (self._nStrLen as usize) * 2;
        buf[offset as usize..offset as usize + src_len]
            .copy_from_slice(&self._strName[..src_len]);
        offset += src_len as i32;
        le_put_int(&mut buf, offset, self._reserved);
        stream.write_all(&buf).ok();
    }
}
