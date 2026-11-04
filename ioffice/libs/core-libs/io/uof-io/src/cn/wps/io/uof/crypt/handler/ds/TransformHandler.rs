// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::XPathHandler::XPathHandler;

// Java: public class TransformHandler extends SecurityHandlerBase
pub struct TransformHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: XPathHandler mXPathHandler
    mXPathHandler: Option<XPathHandler>,
}

// Java: public class TransformHandler extends SecurityHandlerBase
impl TransformHandler {
    // Java: TransformHandler(UofEncryptInfo uofEncryptInfo)
    pub fn TransformHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      TransformHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mXPathHandler: None,
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
         // Java: case 83750045:
         83750045i32 =>
            // Java: return this.getXPathHandler();
            Some(Box::new(XPathHandler::XPathHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: XPathHandler getXPathHandler()
    fn getXPathHandler(&mut self) -> &XPathHandler {
      // Java: if (null == this.mXPathHandler)
      if self.mXPathHandler.is_none() {
         // Java: this.mXPathHandler = new XPathHandler(this.mUofEncryptInfo);
         self.mXPathHandler = Some(XPathHandler::XPathHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mXPathHandler;
      self.mXPathHandler.as_ref().unwrap()
    }

}

impl ElementHandler for TransformHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) {
        self.startElement(var1, var2);
    }
    fn endElement(&self, var1: i32) {
        self.endElement(var1);
    }
    fn characters(&self, var1: &str) {
        self.characters(var1);
    }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> {
        self.enterSubElement(var1)
    }
}
