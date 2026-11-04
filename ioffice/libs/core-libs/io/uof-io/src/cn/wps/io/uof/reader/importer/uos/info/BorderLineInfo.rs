// Java: import cn.wps.io.uof.util.UofUnit
use crate::cn::wps::io::uof::util::UofUnit::UofUnit;

use super::ColorInfo::ColorInfo;

// Java: public class BorderLineInfo
pub struct BorderLineInfo {
    // Java field: String lineType
    pub lineType: Option<String>,
    // Java field: String lineDotted
    pub lineDotted: Option<String>,
    // Java field: UofUnit lineWidth
    pub lineWidth: Option<UofUnit>,
    // Java field: UofUnit lineMargin
    pub lineMargin: Option<UofUnit>,
    // Java field: ColorInfo lineColor
    pub lineColor: Option<ColorInfo>,
    // Java field: Integer lineRef
    pub lineRef: Option<i32>,
    // Java field: ColorInfo lineRefColor
    pub lineRefColor: Option<ColorInfo>,
}

// Java: public class BorderLineInfo
impl BorderLineInfo {
    // Java: BorderLineInfo()
    pub fn BorderLineInfo() -> Self {
      // Java: super();
      BorderLineInfo {
          lineType: None,
          lineDotted: None,
          lineWidth: None,
          lineMargin: None,
          lineColor: None,
          lineRef: None,
          lineRefColor: None,
      }
    }

}
