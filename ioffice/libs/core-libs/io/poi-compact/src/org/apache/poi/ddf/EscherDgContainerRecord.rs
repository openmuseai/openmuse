// Java field: int MAIN_SEPERATOR
pub(crate) const MAIN_SEPERATOR: i32 = 0;
// Java field: int HEADER_SEPERATOR
pub(crate) const HEADER_SEPERATOR: i32 = 1;

// Java: public class EscherDgContainerRecord extends EscherContainerRecord
pub struct EscherDgContainerRecord {
    // Java field: byte mSeperatorType
    pub(crate) mSeperatorType: i8,
}

// Java: public class EscherDgContainerRecord extends EscherContainerRecord
impl EscherDgContainerRecord {
    // Java: EscherDgContainerRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherDgContainerRecord { mSeperatorType: 0 }
    }

    // Java: void setSeperatorType(byte seperatorType)
    pub fn setSeperatorType(&mut self, seperator_type: i8) {
      // Java: this.mSeperatorType = seperatorType;
      self.mSeperatorType = seperator_type;
    }

    // Java: byte getSeperatorType()
    pub fn getSeperatorType(&self) -> i8 {
      // Java: return this.mSeperatorType;
      self.mSeperatorType
    }

}
