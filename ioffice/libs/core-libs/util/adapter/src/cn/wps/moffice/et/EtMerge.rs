// Java: public class EtMerge
pub struct EtMerge {
    // Java field: int tRow
    pub(crate) tRow: i32,
    // Java field: int tCol
    pub(crate) tCol: i32,
    // Java field: int bRow
    pub(crate) bRow: i32,
    // Java field: int bCol
    pub(crate) bCol: i32,
}

// Java: public class EtMerge
impl EtMerge {
    // Java: EtMerge()
    pub fn new() -> Self {
      // Java: super();
      EtMerge {
          tRow: 0,
          tCol: 0,
          bRow: 0,
          bCol: 0,
      }
    }

}
