// Java: public class SprmBuffer implements Cloneable
pub struct SprmBuffer {
    // Java field: byte[] _buf
    pub(crate) _buf: Vec<u8>,
}

// Java: public class SprmBuffer implements Cloneable
impl SprmBuffer {
    pub fn new() -> Self {
        SprmBuffer { _buf: Vec::new() }
    }
}

impl Clone for SprmBuffer {
    fn clone(&self) -> Self {
        SprmBuffer {
            _buf: self._buf.clone(),
        }
    }
}
