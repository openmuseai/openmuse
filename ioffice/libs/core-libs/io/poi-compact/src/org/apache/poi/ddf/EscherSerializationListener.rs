pub use super::EscherRecord::EscherRecord;

// Java: public interface EscherSerializationListener
pub trait EscherSerializationListener {
    // Java: abstract void beforeRecordSerialize(int var1, short var2, EscherRecord var3)
    fn beforeRecordSerialize(&mut self, offset: i32, record_id: i16, record: &EscherRecord);

    // Java: abstract void afterRecordSerialize(int var1, short var2, int var3, EscherRecord var4)
    fn afterRecordSerialize(&mut self, offset: i32, record_id: i16, size: i32, record: &EscherRecord);
}
