// Java: import java.util.ArrayList
// Java: import java.util.List

use super::DateFilterInfo::DateFilterInfo;
use super::CustomFilterInfo::CustomFilterInfo;
use super::DynamicFilterInfo::DynamicFilterInfo;
use super::ColorFilter::ColorFilter;

// Java: public class FilterConditionInfo
pub struct FilterConditionInfo {
    // Java field: List<String> textFilters
    pub textFilters: Vec<String>,
    // Java field: List<DateFilterInfo> dateFilters
    pub dateFilters: Vec<DateFilterInfo>,
    // Java field: boolean whetherFilterEmptyItem
    pub whetherFilterEmptyItem: bool,
    // Java field: String normalType
    pub normalType: Option<String>,
    // Java field: String normalValue
    pub normalValue: Option<String>,
    // Java field: CustomFilterInfo customFilterInfo
    pub customFilterInfo: CustomFilterInfo,
    // Java field: DynamicFilterInfo dynamicFilterInfo
    pub dynamicFilterInfo: Option<DynamicFilterInfo>,
    // Java field: ColorFilter colorFilter
    pub colorFilter: ColorFilter,
}

// Java: public class FilterConditionInfo
impl FilterConditionInfo {
    // Java: FilterConditionInfo()
    pub fn FilterConditionInfo() -> Self {
      // Java: super();
      FilterConditionInfo {
          textFilters: Vec::new(),
          dateFilters: Vec::new(),
          whetherFilterEmptyItem: false,
          normalType: None,
          normalValue: None,
          customFilterInfo: CustomFilterInfo::CustomFilterInfo(),
          dynamicFilterInfo: None,
          colorFilter: ColorFilter::ColorFilter(),
      }
    }

}
