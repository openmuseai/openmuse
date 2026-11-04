pub struct FLD {
    pub _fieldChar: u8,
    pub _info: u8,
}

impl FLD {
    pub fn new() -> Self {
        FLD { _fieldChar: 0, _info: 0 }
    }

    pub fn new_from_buf(data: &[u8], offset: i32) -> Self {
        let o = offset as usize;
        FLD {
            _fieldChar: data[o],
            _info: data[o + 1],
        }
    }

    pub fn new_from_bytes(field_char: u8, data: u8) -> Self {
        FLD { _fieldChar: field_char, _info: data }
    }

    pub fn getFiledCH(&self) -> u8 { self._fieldChar & 0x1F }
    pub fn setFiledCH(&mut self, val: u8) { self._fieldChar = (self._fieldChar & !0x1F) | (val & 0x1F); }
    pub fn getFiledChar(&self) -> u8 { self._fieldChar }
    pub fn setFiledChar(&mut self, field_char: u8) { self._fieldChar = field_char; }
    pub fn getData(&self) -> u8 { self._info }
    pub fn setData(&mut self, info: u8) { self._info = info; }

    pub fn toByteArray(&self) -> Vec<u8> {
        vec![self._fieldChar, self._info]
    }
}
