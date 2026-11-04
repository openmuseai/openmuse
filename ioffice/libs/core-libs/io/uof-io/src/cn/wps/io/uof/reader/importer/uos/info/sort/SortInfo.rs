// Java: import java.util.ArrayList
// Java: import java.util.List

use super::SortConditionInfo::SortConditionInfo;

// Java: public class SortInfo
pub struct SortInfo {
    // Java field: boolean isSortByRow
    pub isSortByRow: bool,
    // Java field: boolean isCaseSensitive
    pub isCaseSensitive: bool,
    // Java field: String sortMethod
    pub sortMethod: Option<String>,
    // Java field: String sortRange
    pub sortRange: Option<String>,
    // Java field: List<SortConditionInfo> conditionInfos
    pub conditionInfos: Vec<SortConditionInfo>,
}

// Java: public class SortInfo
impl SortInfo {
    // Java: SortInfo()
    pub fn SortInfo() -> Self {
      // Java: super();
      SortInfo {
          isSortByRow: false,
          isCaseSensitive: false,
          sortMethod: None,
          sortRange: None,
          conditionInfos: Vec::new(),
      }
    }

}
