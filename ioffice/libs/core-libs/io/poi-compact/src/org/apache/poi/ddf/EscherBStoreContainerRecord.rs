use super::EscherContainerRecord::EscherContainerRecord;

// Java: public class EscherBStoreContainerRecord extends EscherContainerRecord
pub struct EscherBStoreContainerRecord {
    pub(crate) base: EscherContainerRecord,
}

// Java: public class EscherBStoreContainerRecord extends EscherContainerRecord
impl EscherBStoreContainerRecord {
    // Java: EscherBStoreContainerRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherBStoreContainerRecord {
          base: EscherContainerRecord::new(),
      }
    }

}
