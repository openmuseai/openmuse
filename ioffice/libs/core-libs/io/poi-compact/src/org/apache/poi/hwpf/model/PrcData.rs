use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;

pub struct PrcData {
    _tableStream: *mut DocumentInputStream,
    _offset: i32,
    _data: Option<Vec<u8>>,
}

impl PrcData {
    pub fn new(table_stream: &mut DocumentInputStream, offset: i32) -> Self {
        PrcData {
            _tableStream: table_stream as *mut DocumentInputStream,
            _offset: offset,
            _data: None,
        }
    }

    pub fn getData(&mut self) -> &Vec<u8> {
        if self._data.is_none() {
            let ts = unsafe { &mut *self._tableStream };
            ts.seek(self._offset as i64);
            let size = ts.readUShort() as usize;
            let mut buf = vec![0u8; size];
            ts.read(&mut buf);
            self._data = Some(buf);
        }
        self._data.as_ref().unwrap()
    }
}
