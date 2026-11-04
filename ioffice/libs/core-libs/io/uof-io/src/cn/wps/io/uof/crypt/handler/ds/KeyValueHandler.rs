// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::DSAKeyValueHandler::DSAKeyValueHandler;
use super::RSAKeyValueHandler::RSAKeyValueHandler;

// Java: public class KeyValueHandler extends SecurityHandlerBase
pub struct KeyValueHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: DSAKeyValueHandler mDSAKeyValueHandler
    mDSAKeyValueHandler: Option<DSAKeyValueHandler>,
    // Java field: RSAKeyValueHandler mRSAKeyValueHandler
    mRSAKeyValueHandler: Option<RSAKeyValueHandler>,
}

// Java: public class KeyValueHandler extends SecurityHandlerBase
impl KeyValueHandler {
    // Java: KeyValueHandler(UofEncryptInfo uofEncryptInfo)
    pub fn KeyValueHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      KeyValueHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mDSAKeyValueHandler: None,
          mRSAKeyValueHandler: None,
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
         // Java: case -2027888124:
         -2027888124i32 =>
            // Java: return this.getDSAKeyValueHandler();
            Some(Box::new(DSAKeyValueHandler::DSAKeyValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1415403374:
         -1415403374i32 =>
            // Java: return this.getRSAKeyValueHandler();
            Some(Box::new(RSAKeyValueHandler::RSAKeyValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: DSAKeyValueHandler getDSAKeyValueHandler()
    fn getDSAKeyValueHandler(&mut self) -> &DSAKeyValueHandler {
      // Java: if (null == this.mDSAKeyValueHandler)
      if self.mDSAKeyValueHandler.is_none() {
         // Java: this.mDSAKeyValueHandler = new DSAKeyValueHandler(this.mUofEncryptInfo);
         self.mDSAKeyValueHandler = Some(DSAKeyValueHandler::DSAKeyValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mDSAKeyValueHandler;
      self.mDSAKeyValueHandler.as_ref().unwrap()
    }

    // Java: RSAKeyValueHandler getRSAKeyValueHandler()
    fn getRSAKeyValueHandler(&mut self) -> &RSAKeyValueHandler {
      // Java: if (null == this.mRSAKeyValueHandler)
      if self.mRSAKeyValueHandler.is_none() {
         // Java: this.mRSAKeyValueHandler = new RSAKeyValueHandler(this.mUofEncryptInfo);
         self.mRSAKeyValueHandler = Some(RSAKeyValueHandler::RSAKeyValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mRSAKeyValueHandler;
      self.mRSAKeyValueHandler.as_ref().unwrap()
    }

}

impl ElementHandler for KeyValueHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
