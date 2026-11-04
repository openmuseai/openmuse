// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.io.uof.crypt.handler.ds.DigestMethodHandler
use crate::cn::wps::io::uof::crypt::handler::ds::DigestMethodHandler::DigestMethodHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.DigestValueHandler
use crate::cn::wps::io::uof::crypt::handler::ds::DigestValueHandler::DigestValueHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use crate::cn::wps::io::uof::crypt::handler::ds::TransformsHandler::TransformsHandler;

// Java: public class PasswordCheckValueHandler extends SecurityHandlerBase
pub struct PasswordCheckValueHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: DigestValueHandler mDigestValueHandler
    mDigestValueHandler: Option<DigestValueHandler>,
    // Java field: DigestMethodHandler mDigestMethodHandler
    mDigestMethodHandler: Option<DigestMethodHandler>,
    // Java field: cn.wps.io.uof.crypt.handler.ds.TransformsHandler mTransformsHandler
    mTransformsHandler: Option<TransformsHandler>,
}

// Java: public class PasswordCheckValueHandler extends SecurityHandlerBase
impl PasswordCheckValueHandler {
    // Java: PasswordCheckValueHandler(UofEncryptInfo uofEncryptInfo)
    pub fn PasswordCheckValueHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      PasswordCheckValueHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mDigestValueHandler: None,
          mDigestMethodHandler: None,
          mTransformsHandler: None,
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
         // Java: case -259611155:
         -259611155i32 =>
            // Java: return this.getDigestValueHandler();
            Some(Box::new(DigestValueHandler::DigestValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 266395303:
         266395303i32 =>
            // Java: return this.getTransformsHandler();
            Some(Box::new(TransformsHandler::TransformsHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 288246757:
         288246757i32 =>
            // Java: return this.getDigestMethodHandler();
            Some(Box::new(DigestMethodHandler::DigestMethodHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: DigestValueHandler getDigestValueHandler()
    fn getDigestValueHandler(&mut self) -> &DigestValueHandler {
      // Java: if (null == this.mDigestValueHandler)
      if self.mDigestValueHandler.is_none() {
         // Java: this.mDigestValueHandler = new DigestValueHandler(this.mUofEncryptInfo);
         self.mDigestValueHandler = Some(DigestValueHandler::DigestValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mDigestValueHandler;
      self.mDigestValueHandler.as_ref().unwrap()
    }

    // Java: DigestMethodHandler getDigestMethodHandler()
    fn getDigestMethodHandler(&mut self) -> &DigestMethodHandler {
      // Java: if (null == this.mDigestMethodHandler)
      if self.mDigestMethodHandler.is_none() {
         // Java: this.mDigestMethodHandler = new DigestMethodHandler(this.mUofEncryptInfo);
         self.mDigestMethodHandler = Some(DigestMethodHandler::DigestMethodHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mDigestMethodHandler;
      self.mDigestMethodHandler.as_ref().unwrap()
    }

    // Java: cn.wps.io.uof.crypt.handler.ds.TransformsHandler getTransformsHandler()
    fn getTransformsHandler(&mut self) -> &TransformsHandler {
      // Java: if (null == this.mTransformsHandler)
      if self.mTransformsHandler.is_none() {
         // Java: this.mTransformsHandler = new cn.wps.io.uof.crypt.handler.ds.TransformsHandler(this.mUofEncryptInfo);
         self.mTransformsHandler = Some(TransformsHandler::TransformsHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mTransformsHandler;
      self.mTransformsHandler.as_ref().unwrap()
    }

}

impl ElementHandler for PasswordCheckValueHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
