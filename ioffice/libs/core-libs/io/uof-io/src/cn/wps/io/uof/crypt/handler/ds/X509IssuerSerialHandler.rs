// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::X509IssuerNameHandler::X509IssuerNameHandler;
use super::X509SerialNumberHandler::X509SerialNumberHandler;

// Java: public class X509IssuerSerialHandler extends SecurityHandlerBase
pub struct X509IssuerSerialHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: X509IssuerNameHandler mX509IssuerNameHandler
    mX509IssuerNameHandler: Option<X509IssuerNameHandler>,
    // Java field: X509SerialNumberHandler mX509SerialNumberHandler
    mX509SerialNumberHandler: Option<X509SerialNumberHandler>,
}

// Java: public class X509IssuerSerialHandler extends SecurityHandlerBase
impl X509IssuerSerialHandler {
    // Java: X509IssuerSerialHandler(UofEncryptInfo uofEncryptInfo)
    pub fn X509IssuerSerialHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      X509IssuerSerialHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mX509IssuerNameHandler: None,
          mX509SerialNumberHandler: None,
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
         // Java: case -1474472157:
         -1474472157i32 =>
            // Java: return this.getX509SerialNumberHandler();
            Some(Box::new(X509SerialNumberHandler::X509SerialNumberHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1802284842:
         1802284842i32 =>
            // Java: return this.getX509IssuerNameHandler();
            Some(Box::new(X509IssuerNameHandler::X509IssuerNameHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: X509IssuerNameHandler getX509IssuerNameHandler()
    fn getX509IssuerNameHandler(&mut self) -> &X509IssuerNameHandler {
      // Java: if (null == this.mX509IssuerNameHandler)
      if self.mX509IssuerNameHandler.is_none() {
         // Java: this.mX509IssuerNameHandler = new X509IssuerNameHandler(this.mUofEncryptInfo);
         self.mX509IssuerNameHandler = Some(X509IssuerNameHandler::X509IssuerNameHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509IssuerNameHandler;
      self.mX509IssuerNameHandler.as_ref().unwrap()
    }

    // Java: X509SerialNumberHandler getX509SerialNumberHandler()
    fn getX509SerialNumberHandler(&mut self) -> &X509SerialNumberHandler {
      // Java: if (null == this.mX509SerialNumberHandler)
      if self.mX509SerialNumberHandler.is_none() {
         // Java: this.mX509SerialNumberHandler = new X509SerialNumberHandler(this.mUofEncryptInfo);
         self.mX509SerialNumberHandler = Some(X509SerialNumberHandler::X509SerialNumberHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509SerialNumberHandler;
      self.mX509SerialNumberHandler.as_ref().unwrap()
    }

}

impl ElementHandler for X509IssuerSerialHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
