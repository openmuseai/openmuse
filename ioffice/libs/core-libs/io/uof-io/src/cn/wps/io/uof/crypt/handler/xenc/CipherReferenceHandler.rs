// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::TransformsHandler::TransformsHandler;

// Java: public class CipherReferenceHandler extends SecurityHandlerBase
pub struct CipherReferenceHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: TransformsHandler mTransformsHandler
    mTransformsHandler: Option<TransformsHandler>,
}

// Java: public class CipherReferenceHandler extends SecurityHandlerBase
impl CipherReferenceHandler {
    // Java: CipherReferenceHandler(UofEncryptInfo uofEncryptInfo)
    pub fn CipherReferenceHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      CipherReferenceHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mTransformsHandler: None,
      }
    }

    // Java: void startElement(int element, XmlRoAttr attrsNotCopy)
    pub fn startElement(&mut self, _element: i32, attrsNotCopy: &dyn XmlRoAttr) {
      // Java: XmlRoAttr val = attrsNotCopy.getByName(84300);
      let val = attrsNotCopy.get_by_name(84300);
      // Java: if (null != val)
      if let Some(v) = val {
         // Java: this.mUofEncryptInfo.setCipherReference(val.toString());
         self.base.mUofEncryptInfo.setCipherReference(v.to_string());
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
         // Java: case 266395303:
         266395303i32 =>
            // Java: return this.getTransformsHandler();
            Some(Box::new(TransformsHandler::TransformsHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: TransformsHandler getTransformsHandler()
    fn getTransformsHandler(&mut self) -> &TransformsHandler {
      // Java: if (null == this.mTransformsHandler)
      if self.mTransformsHandler.is_none() {
         // Java: this.mTransformsHandler = new TransformsHandler(this.mUofEncryptInfo);
         self.mTransformsHandler = Some(TransformsHandler::TransformsHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mTransformsHandler;
      self.mTransformsHandler.as_ref().unwrap()
    }

}

impl ElementHandler for CipherReferenceHandler {
    fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) { }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
