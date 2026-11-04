// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::OAEPparamsHandler::OAEPparamsHandler;
use super::KeySizeHandler::KeySizeHandler;

// Java: public class EncryptionMethodHandler extends SecurityHandlerBase
pub struct EncryptionMethodHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: OAEPparamsHandler mOAEPparamsHandler
    mOAEPparamsHandler: Option<OAEPparamsHandler>,
    // Java field: KeySizeHandler mKeySizeHandler
    mKeySizeHandler: Option<KeySizeHandler>,
}

// Java: public class EncryptionMethodHandler extends SecurityHandlerBase
impl EncryptionMethodHandler {
    // Java: EncryptionMethodHandler(UofEncryptInfo uofEncryptInfo)
    pub fn EncryptionMethodHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      EncryptionMethodHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mOAEPparamsHandler: None,
          mKeySizeHandler: None,
      }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
      // Java: XmlRoAttr tmp;
      // Java: if ((tmp = attrsNotCopy.getByName(-2065509297)) != null)
      if let Some(tmp) = attrsNotCopy.get_by_name(-2065509297) {
         // Java: String algorithm = tmp.toString();
         let _algorithm = tmp.to_string();
         // Java: this.mUofEncryptInfo.setCipherAlgo("SM4");
         self.base.mUofEncryptInfo.setCipherAlgo("SM4".to_string());
      }
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
         // Java: case 849167072:
         849167072i32 =>
            // Java: return this.getKeySizeHandler();
            Some(Box::new(KeySizeHandler::KeySizeHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1415097795:
         1415097795i32 =>
            // Java: return this.getOAEPparamsHandler();
            Some(Box::new(OAEPparamsHandler::OAEPparamsHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: OAEPparamsHandler getOAEPparamsHandler()
    fn getOAEPparamsHandler(&mut self) -> &OAEPparamsHandler {
      // Java: if (null == this.mOAEPparamsHandler)
      if self.mOAEPparamsHandler.is_none() {
         // Java: this.mOAEPparamsHandler = new OAEPparamsHandler(this.mUofEncryptInfo);
         self.mOAEPparamsHandler = Some(OAEPparamsHandler::OAEPparamsHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mOAEPparamsHandler;
      self.mOAEPparamsHandler.as_ref().unwrap()
    }

    // Java: KeySizeHandler getKeySizeHandler()
    fn getKeySizeHandler(&mut self) -> &KeySizeHandler {
      // Java: if (null == this.mKeySizeHandler)
      if self.mKeySizeHandler.is_none() {
         // Java: this.mKeySizeHandler = new KeySizeHandler(this.mUofEncryptInfo);
         self.mKeySizeHandler = Some(KeySizeHandler::KeySizeHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mKeySizeHandler;
      self.mKeySizeHandler.as_ref().unwrap()
    }

}

impl ElementHandler for EncryptionMethodHandler {
    fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) { }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
