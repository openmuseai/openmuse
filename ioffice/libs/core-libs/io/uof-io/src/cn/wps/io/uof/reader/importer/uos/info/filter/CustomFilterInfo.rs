// Java: public class CustomFilterInfo
pub struct CustomFilterInfo {
    // Java field: String type
    pub r#type: Option<String>,
    // Java field: String op1
    pub op1: Option<String>,
    // Java field: String value1
    pub value1: Option<String>,
    // Java field: String op2
    pub op2: Option<String>,
    // Java field: String value2
    pub value2: Option<String>,
}

// Java: public class CustomFilterInfo
impl CustomFilterInfo {
    // Java: CustomFilterInfo()
    pub fn CustomFilterInfo() -> Self {
      // Java: super();
      CustomFilterInfo {
          r#type: None,
          op1: None,
          value1: None,
          op2: None,
          value2: None,
      }
    }

}
