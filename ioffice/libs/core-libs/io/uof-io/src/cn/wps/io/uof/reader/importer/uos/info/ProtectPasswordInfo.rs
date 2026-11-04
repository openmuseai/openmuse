// Java: public class ProtectPasswordInfo
pub struct ProtectPasswordInfo {
    // Java field: String algorithmName
    pub algorithmName: Option<String>,
    // Java field: String passWordHashCode
    pub passWordHashCode: Option<String>,
    // Java field: String saltValue
    pub saltValue: Option<String>,
    // Java field: Integer spinCount
    pub spinCount: Option<i32>,
}

// Java: public class ProtectPasswordInfo
impl ProtectPasswordInfo {
    // Java: ProtectPasswordInfo()
    pub fn ProtectPasswordInfo() -> Self {
      // Java: super();
      ProtectPasswordInfo {
          algorithmName: None,
          passWordHashCode: None,
          saltValue: None,
          spinCount: None,
      }
    }

}
