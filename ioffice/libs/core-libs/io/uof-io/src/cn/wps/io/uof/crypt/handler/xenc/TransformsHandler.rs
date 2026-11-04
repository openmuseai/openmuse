// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.io.uof.crypt.handler.ds.TransformHandler
use crate::cn::wps::io::uof::crypt::handler::ds::TransformHandler::TransformHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public class TransformsHandler extends SecurityHandlerBase
pub struct TransformsHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: TransformHandler mTransformHandler
    mTransformHandler: Option<TransformHandler>,
}

// Java: public class TransformsHandler extends SecurityHandlerBase
impl TransformsHandler {
    // Java: TransformsHandler(UofEncryptInfo uofEncryptInfo)
    pub fn TransformsHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      TransformsHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mTransformHandler: None,
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
         // Java: case -1238332596:
         -1238332596i32 =>
            // Java: return this.getTransformHandler();
            Some(Box::new(TransformHandler::TransformHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: TransformHandler getTransformHandler()
    fn getTransformHandler(&mut self) -> &TransformHandler {
      // Java: if (null == this.mTransformHandler)
      if self.mTransformHandler.is_none() {
         // Java: this.mTransformHandler = new TransformHandler(this.mUofEncryptInfo);
         self.mTransformHandler = Some(TransformHandler::TransformHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mTransformHandler;
      self.mTransformHandler.as_ref().unwrap()
    }

}

impl ElementHandler for TransformsHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
