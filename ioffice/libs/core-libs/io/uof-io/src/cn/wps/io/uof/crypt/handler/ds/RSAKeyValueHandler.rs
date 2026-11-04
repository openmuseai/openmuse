// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::ExponentHandler::ExponentHandler;
use super::ModulusHandler::ModulusHandler;

// Java: public class RSAKeyValueHandler extends SecurityHandlerBase
pub struct RSAKeyValueHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: ExponentHandler mExponentHandler
    mExponentHandler: Option<ExponentHandler>,
    // Java field: ModulusHandler mModulusHandler
    mModulusHandler: Option<ModulusHandler>,
}

// Java: public class RSAKeyValueHandler extends SecurityHandlerBase
impl RSAKeyValueHandler {
    // Java: RSAKeyValueHandler(UofEncryptInfo uofEncryptInfo)
    pub fn RSAKeyValueHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      RSAKeyValueHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mExponentHandler: None,
          mModulusHandler: None,
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
         // Java: case -1861525809:
         -1861525809i32 =>
            // Java: return this.getExponentHandler();
            Some(Box::new(ExponentHandler::ExponentHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -1402879657:
         -1402879657i32 =>
            // Java: return this.getModulusHandler();
            Some(Box::new(ModulusHandler::ModulusHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: ExponentHandler getExponentHandler()
    fn getExponentHandler(&mut self) -> &ExponentHandler {
      // Java: if (null == this.mExponentHandler)
      if self.mExponentHandler.is_none() {
         // Java: this.mExponentHandler = new ExponentHandler(this.mUofEncryptInfo);
         self.mExponentHandler = Some(ExponentHandler::ExponentHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mExponentHandler;
      self.mExponentHandler.as_ref().unwrap()
    }

    // Java: ModulusHandler getModulusHandler()
    fn getModulusHandler(&mut self) -> &ModulusHandler {
      // Java: if (null == this.mModulusHandler)
      if self.mModulusHandler.is_none() {
         // Java: this.mModulusHandler = new ModulusHandler(this.mUofEncryptInfo);
         self.mModulusHandler = Some(ModulusHandler::ModulusHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mModulusHandler;
      self.mModulusHandler.as_ref().unwrap()
    }

}

impl ElementHandler for RSAKeyValueHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
