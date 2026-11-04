// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.reader.handler.UofHandlerBase
use crate::cn::wps::io::uof::reader::handler::UofHandlerBase::UofHandlerBase;

use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public class SecurityHandlerBase extends UofHandlerBase
pub struct SecurityHandlerBase {
    pub(crate) base: UofHandlerBase,
    // Java field: UofEncryptInfo mUofEncryptInfo
    pub(crate) mUofEncryptInfo: UofEncryptInfo,
}

// Java: public class SecurityHandlerBase extends UofHandlerBase
impl SecurityHandlerBase {
    // Java: SecurityHandlerBase(UofEncryptInfo uofEncryptInfo)
    pub fn SecurityHandlerBase(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super();
      let base = UofHandlerBase::UofHandlerBase();
      let mut s = SecurityHandlerBase {
          base,
          mUofEncryptInfo: UofEncryptInfo::new(),
      };
      // Java: this.mUofEncryptInfo = uofEncryptInfo;
      s.mUofEncryptInfo = uofEncryptInfo;
      s
    }

}

impl ElementHandler for SecurityHandlerBase {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) {
        self.base.startElement(var1, var2);
    }
    fn endElement(&self, var1: i32) {
        self.base.endElement(var1);
    }
    fn characters(&self, var1: &str) {
        self.base.characters(var1);
    }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> {
        self.base.enterSubElement(var1)
    }
}
