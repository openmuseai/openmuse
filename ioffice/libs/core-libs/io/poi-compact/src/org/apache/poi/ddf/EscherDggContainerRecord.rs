use super::EscherContainerRecord::EscherContainerRecord;

// Java: public class EscherDggContainerRecord extends EscherContainerRecord
pub struct EscherDggContainerRecord {
    pub(crate) base: EscherContainerRecord,
}

// Java: public class EscherDggContainerRecord extends EscherContainerRecord
impl EscherDggContainerRecord {
    // Java: EscherDggContainerRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherDggContainerRecord {
          base: EscherContainerRecord::new(),
      }
    }

}
