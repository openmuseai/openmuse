// Java: public class DynamicFilterInfo
pub struct DynamicFilterInfo {
    // Java field: String filterType
    pub filterType: Option<String>,
    // Java field: String filterValue
    pub filterValue: Option<String>,
}

// Java: public class DynamicFilterInfo
impl DynamicFilterInfo {
    // Java: DynamicFilterInfo()
    pub fn DynamicFilterInfo() -> Self {
      // Java: super();
      DynamicFilterInfo {
          filterType: None,
          filterValue: None,
      }
    }

}
