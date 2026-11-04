// Java: public class NameInfo
pub struct NameInfo {
    // Java field: String name
    pub name: Option<String>,
    // Java field: String formula
    pub formula: Option<String>,
}

// Java: public class NameInfo
impl NameInfo {
    // Java: NameInfo()
    pub fn NameInfo() -> Self {
      // Java: super();
      NameInfo {
          name: None,
          formula: None,
      }
    }

}
