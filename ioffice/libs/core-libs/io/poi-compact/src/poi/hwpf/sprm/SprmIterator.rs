// Java field: SprmOperation op
static op: std::sync::OnceLock<std::sync::Mutex<SprmOperation>> = std::sync::OnceLock::new();

// Java: public class SprmIterator
pub struct SprmIterator {
    // Java field: byte[] _grpprl
    _grpprl: Option<Vec<u8>>,
    // Java field: int _offset
    pub(crate) _offset: i32,
}

// Java: public class SprmIterator
impl SprmIterator {
    // Java: SprmIterator()
    pub fn SprmIterator(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: SprmIterator(byte[] grpprl, int offset)
    pub fn SprmIterator(&self, byte[] grpprl, int offset) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this._grpprl = grpprl;
      // TODO_RUST_CODE
      // Java: this._offset = offset;
      // TODO_RUST_CODE
    }

    // Java: void init(byte[] grpprl, int offset)
    pub fn init(&self, byte[] grpprl, int offset) {
      // Java: this._grpprl = grpprl;
      // TODO_RUST_CODE
      // Java: this._offset = offset;
      // TODO_RUST_CODE
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
      // Java: return this._offset < this._grpprl.length - 2;
      // TODO_RUST_CODE
    }

    // Java: SprmOperation next()
    pub fn next(&self) -> SprmOperation {
      // Java: op.init(this._grpprl, this._offset);
      // TODO_RUST_CODE
      // Java: this._offset += op.size();
      // TODO_RUST_CODE
      // Java: return op;
      // TODO_RUST_CODE
    }

}
