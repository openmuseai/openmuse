// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::CipherValueHandler::CipherValueHandler;
use super::CipherReferenceHandler::CipherReferenceHandler;

// Java: public class CipherDataHandler extends SecurityHandlerBase
pub struct CipherDataHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: CipherValueHandler mCipherValueHandler
    mCipherValueHandler: Option<CipherValueHandler>,
    // Java field: CipherReferenceHandler mCipherReferenceHandler
    mCipherReferenceHandler: Option<CipherReferenceHandler>,
}

// Java: public class CipherDataHandler extends SecurityHandlerBase
impl CipherDataHandler {
    // Java: CipherDataHandler(UofEncryptInfo uofEncryptInfo)
    pub fn CipherDataHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      CipherDataHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mCipherValueHandler: None,
          mCipherReferenceHandler: None,
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
         // Java: case -508792314:
         -508792314i32 =>
            // Java: return this.getCipherValueHandler();
            Some(Box::new(CipherValueHandler::CipherValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1763626336:
         1763626336i32 =>
            // Java: return this.getCipherReferenceHandler();
            Some(Box::new(CipherReferenceHandler::CipherReferenceHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: CipherValueHandler getCipherValueHandler()
    fn getCipherValueHandler(&mut self) -> &CipherValueHandler {
      // Java: if (null == this.mCipherValueHandler)
      if self.mCipherValueHandler.is_none() {
         // Java: this.mCipherValueHandler = new CipherValueHandler(this.mUofEncryptInfo);
         self.mCipherValueHandler = Some(CipherValueHandler::CipherValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mCipherValueHandler;
      self.mCipherValueHandler.as_ref().unwrap()
    }

    // Java: CipherReferenceHandler getCipherReferenceHandler()
    fn getCipherReferenceHandler(&mut self) -> &CipherReferenceHandler {
      // Java: if (null == this.mCipherReferenceHandler)
      if self.mCipherReferenceHandler.is_none() {
         // Java: this.mCipherReferenceHandler = new CipherReferenceHandler(this.mUofEncryptInfo);
         self.mCipherReferenceHandler = Some(CipherReferenceHandler::CipherReferenceHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mCipherReferenceHandler;
      self.mCipherReferenceHandler.as_ref().unwrap()
    }

}

impl ElementHandler for CipherDataHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
