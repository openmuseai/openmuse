// Java: import cn.wps.io.uof.util.UofUnit
use crate::cn::wps::io::uof::util::UofUnit::UofUnit;

// Java: public class HeaderFooterInfo
pub struct HeaderFooterInfo {
    // Java field: UofUnit margin
    pub margin: Option<UofUnit>,
    // Java field: String first
    pub first: Option<String>,
    // Java field: String odd
    pub odd: Option<String>,
    // Java field: String even
    pub even: Option<String>,
}

// Java: public class HeaderFooterInfo
impl HeaderFooterInfo {
    // Java: HeaderFooterInfo()
    pub fn HeaderFooterInfo() -> Self {
      // Java: super();
      HeaderFooterInfo {
          margin: None,
          first: None,
          odd: None,
          even: None,
      }
    }

}
