// Java: public class SortConditionInfo
pub struct SortConditionInfo {
    // Java field: boolean isDesOrder
    pub isDesOrder: bool,
    // Java field: String sortBy
    pub sortBy: Option<String>,
    // Java field: String sortRange
    pub sortRange: Option<String>,
    // Java field: String customSequence
    pub customSequence: Option<String>,
    // Java field: Integer cellXFRef
    pub cellXFRef: Option<i32>,
    // Java field: String icon
    pub icon: Option<String>,
}

// Java: public class SortConditionInfo
impl SortConditionInfo {
    // Java: SortConditionInfo()
    pub fn SortConditionInfo() -> Self {
      // Java: super();
      SortConditionInfo {
          isDesOrder: false,
          sortBy: None,
          sortRange: None,
          customSequence: None,
          cellXFRef: None,
          icon: None,
      }
    }

}
