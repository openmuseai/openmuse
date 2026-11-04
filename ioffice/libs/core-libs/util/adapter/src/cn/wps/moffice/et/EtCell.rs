use super::EtXF::EtXF;

// Java: public class EtCell
pub struct EtCell {
    // Java field: EtXF etXF
    pub(crate) etXF: EtXF,
    // Java field: int row
    pub(crate) row: i32,
    // Java field: int col
    pub(crate) col: i32,
    // Java field: String data
    pub(crate) data: String,
}

// Java: public class EtCell
impl EtCell {
    // Java: EtCell()
    pub fn new() -> Self {
      // Java: super();
      EtCell {
          etXF: EtXF::new(),
          row: 0,
          col: 0,
          data: String::new(),
      }
    }

}
