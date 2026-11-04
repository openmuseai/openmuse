// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::PrintNum_F807Handler::PrintNum_F807Handler;
use super::PermissionSetup_F802Handler::PermissionSetup_F802Handler;

// Java: public class Rules_F801Handler extends SecurityHandlerBase
pub struct Rules_F801Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: PrintNum_F807Handler mPrintNum_F807Handler
    mPrintNum_F807Handler: Option<PrintNum_F807Handler>,
    // Java field: PermissionSetup_F802Handler mPermissionSetup_F802Handler
    mPermissionSetup_F802Handler: Option<PermissionSetup_F802Handler>,
}

// Java: public class Rules_F801Handler extends SecurityHandlerBase
impl Rules_F801Handler {
    // Java: Rules_F801Handler(UofEncryptInfo uofEncryptInfo)
    pub fn Rules_F801Handler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      Rules_F801Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mPrintNum_F807Handler: None,
          mPermissionSetup_F802Handler: None,
      }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&self, _element: i32, _attrsNotCopy: &dyn XmlRoAttr) {
    }

    // Java: void endElement(int elementId)
    pub fn endElement(&self, _elementId: i32) {
    }

    // Java: void characters(String chars)
    pub fn characters(&self, _chars: &str) {
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // Java: switch (subElement)
      match subElement
      {
         // Java: case 63490:
         63490i32 =>
            // Java: return this.getPermissionSetup_F802Handler();
            Some(Box::new(PermissionSetup_F802Handler::PermissionSetup_F802Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63495:
         63495i32 =>
            // Java: return this.getPrintNum_F807Handler();
            Some(Box::new(PrintNum_F807Handler::PrintNum_F807Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: PrintNum_F807Handler getPrintNum_F807Handler()
    fn getPrintNum_F807Handler(&mut self) -> &PrintNum_F807Handler {
      // Java: if (null == this.mPrintNum_F807Handler)
      if self.mPrintNum_F807Handler.is_none() {
         // Java: this.mPrintNum_F807Handler = new PrintNum_F807Handler(this.mUofEncryptInfo);
         self.mPrintNum_F807Handler = Some(PrintNum_F807Handler::PrintNum_F807Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPrintNum_F807Handler;
      self.mPrintNum_F807Handler.as_ref().unwrap()
    }

    // Java: PermissionSetup_F802Handler getPermissionSetup_F802Handler()
    fn getPermissionSetup_F802Handler(&mut self) -> &PermissionSetup_F802Handler {
      // Java: if (null == this.mPermissionSetup_F802Handler)
      if self.mPermissionSetup_F802Handler.is_none() {
         // Java: this.mPermissionSetup_F802Handler = new PermissionSetup_F802Handler(this.mUofEncryptInfo);
         self.mPermissionSetup_F802Handler = Some(PermissionSetup_F802Handler::PermissionSetup_F802Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPermissionSetup_F802Handler;
      self.mPermissionSetup_F802Handler.as_ref().unwrap()
    }

}

impl ElementHandler for Rules_F801Handler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
