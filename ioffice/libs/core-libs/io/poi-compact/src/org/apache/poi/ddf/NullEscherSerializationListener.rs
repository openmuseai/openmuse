use super::EscherSerializationListener::{EscherSerializationListener, EscherRecord};

// Java: public class NullEscherSerializationListener implements EscherSerializationListener
pub struct NullEscherSerializationListener;

// Java: public class NullEscherSerializationListener implements EscherSerializationListener
impl NullEscherSerializationListener {
    // Java: NullEscherSerializationListener()
    pub fn new() -> Self {
        // Java: super();
        NullEscherSerializationListener
    }
}

impl EscherSerializationListener for NullEscherSerializationListener {
    // Java: void beforeRecordSerialize(int offset, short recordId, EscherRecord record)
    fn beforeRecordSerialize(&mut self, _offset: i32, _record_id: i16, _record: &EscherRecord) {
    }

    // Java: void afterRecordSerialize(int offset, short recordId, int size, EscherRecord record)
    fn afterRecordSerialize(&mut self, _offset: i32, _record_id: i16, _size: i32, _record: &EscherRecord) {
    }
}
