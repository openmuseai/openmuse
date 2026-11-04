// Java: public class EtColInfo
pub struct EtColInfo {
    // Java field: int width
    pub(crate) width: i32,
    // Java field: short min
    pub(crate) min: i16,
    // Java field: short max
    pub(crate) max: i16,
}

// Java: public class EtColInfo
impl EtColInfo {
    // Java: EtColInfo()
    pub fn new() -> Self {
      // Java: super();
      EtColInfo {
          width: 0,
          min: 0,
          max: 0,
      }
    }

}
