// Java: public class EtRowInfo
pub struct EtRowInfo {
    // Java field: short height
    pub(crate) height: i16,
    // Java field: int index
    pub(crate) index: i32,
}

// Java: public class EtRowInfo
impl EtRowInfo {
    // Java: EtRowInfo()
    pub fn new() -> Self {
      // Java: super();
      EtRowInfo {
          height: 0,
          index: 0,
      }
    }

}
