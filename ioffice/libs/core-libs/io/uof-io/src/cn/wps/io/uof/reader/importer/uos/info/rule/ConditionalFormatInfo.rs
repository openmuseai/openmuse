// Java: import cn.wps.io.uof.reader.importer.uos.info.AreasInfo
use crate::cn::wps::io::uof::reader::importer::uos::info::AreasInfo::AreasInfo;

// Java: public class RulePriority
pub struct RulePriority {
    // Java field: Integer priority
    pub priority: Option<i32>,
}

// Java: public class RulePriority
impl RulePriority {
    // Java: RulePriority()
    pub fn RulePriority() -> Self {
      // Java: super();
      RulePriority {
          priority: None,
      }
    }
}

// Java: public class ConditionalFormatInfo
pub struct ConditionalFormatInfo {
    // Java field: AreasInfo areasInfo
    pub areasInfo: AreasInfo,
    // Java field: String type
    pub r#type: Option<String>,
    // Java field: RulePriority rulePriority
    pub rulePriority: RulePriority,
    // Java field: Integer styleRef
    pub styleRef: Option<i32>,
}

// Java: public class ConditionalFormatInfo
impl ConditionalFormatInfo {
    // Java: ConditionalFormatInfo()
    pub fn ConditionalFormatInfo() -> Self {
      // Java: super();
      ConditionalFormatInfo {
          areasInfo: AreasInfo::AreasInfo(),
          r#type: None,
          rulePriority: RulePriority::RulePriority(),
          styleRef: None,
      }
    }

}
