// Java: public class HyperLinkInfo
pub struct HyperLinkInfo {
    // Java field: Integer identifier
    pub identifier: Option<i32>,
    // Java field: String source
    pub source: Option<String>,
    // Java field: String target
    pub target: Option<String>,
}

// Java: public class HyperLinkInfo
impl HyperLinkInfo {
    // Java: HyperLinkInfo()
    pub fn HyperLinkInfo() -> Self {
      // Java: super();
      HyperLinkInfo {
          identifier: None,
          source: None,
          target: None,
      }
    }

}
