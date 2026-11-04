// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

use super::Encryption_F80AHandler::Encryption_F80AHandler;
use super::OrderlySignature_F80CHandler::OrderlySignature_F80CHandler;
use super::Signature_F80FHandler::Signature_F80FHandler;
use super::Rules_F801Handler::Rules_F801Handler;

// Java: public class Security_F800Handler extends SecurityHandlerBase
pub struct Security_F800Handler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: Encryption_F80AHandler mEncryption_F80AHandler
    mEncryption_F80AHandler: Option<Encryption_F80AHandler>,
    // Java field: OrderlySignature_F80CHandler mOrderlySignature_F80CHandler
    mOrderlySignature_F80CHandler: Option<OrderlySignature_F80CHandler>,
    // Java field: Signature_F80FHandler mSignature_F80FHandler
    mSignature_F80FHandler: Option<Signature_F80FHandler>,
    // Java field: Rules_F801Handler mRules_F801Handler
    mRules_F801Handler: Option<Rules_F801Handler>,
}

// Java: public class Security_F800Handler extends SecurityHandlerBase
impl Security_F800Handler {
    // Java: Security_F800Handler(UofEncryptInfo uofEncryptInfo)
    pub fn Security_F800Handler(mut uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      // Java: this.mUofEncryptInfo.setHashValSize(32);
      uofEncryptInfo.setHashValSize(32);
      // Java: this.mUofEncryptInfo.setChainMode("OFB");
      uofEncryptInfo.setChainMode("OFB".to_string());
      Security_F800Handler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mEncryption_F80AHandler: None,
          mOrderlySignature_F80CHandler: None,
          mSignature_F80FHandler: None,
          mRules_F801Handler: None,
      }
    }

    // Java: ElementHandler enterSubElement(int subElement)
    pub fn enterSubElement(&self, subElement: i32) -> Option<Box<dyn ElementHandler>> {
      // Java: switch (subElement)
      match subElement
      {
         // Java: case 63489:
         63489i32 =>
            // Java: return this.getRules_F801Handler();
            Some(Box::new(Rules_F801Handler::Rules_F801Handler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63498:
         63498i32 =>
            // Java: return this.getEncryption_F80AHandler();
            Some(Box::new(Encryption_F80AHandler::Encryption_F80AHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63500:
         63500i32 =>
            // Java: return this.getOrderlySignature_F80CHandler();
            Some(Box::new(OrderlySignature_F80CHandler::OrderlySignature_F80CHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 63503:
         63503i32 =>
            // Java: return this.getSignature_F80FHandler();
            Some(Box::new(Signature_F80FHandler::Signature_F80FHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: Encryption_F80AHandler getEncryption_F80AHandler()
    fn getEncryption_F80AHandler(&mut self) -> &Encryption_F80AHandler {
      // Java: if (null == this.mEncryption_F80AHandler)
      if self.mEncryption_F80AHandler.is_none() {
         // Java: this.mEncryption_F80AHandler = new Encryption_F80AHandler(this.mUofEncryptInfo);
         self.mEncryption_F80AHandler = Some(Encryption_F80AHandler::Encryption_F80AHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mEncryption_F80AHandler;
      self.mEncryption_F80AHandler.as_ref().unwrap()
    }

    // Java: OrderlySignature_F80CHandler getOrderlySignature_F80CHandler()
    fn getOrderlySignature_F80CHandler(&mut self) -> &OrderlySignature_F80CHandler {
      // Java: if (null == this.mOrderlySignature_F80CHandler)
      if self.mOrderlySignature_F80CHandler.is_none() {
         // Java: this.mOrderlySignature_F80CHandler = new OrderlySignature_F80CHandler(this.mUofEncryptInfo);
         self.mOrderlySignature_F80CHandler = Some(OrderlySignature_F80CHandler::OrderlySignature_F80CHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mOrderlySignature_F80CHandler;
      self.mOrderlySignature_F80CHandler.as_ref().unwrap()
    }

    // Java: Signature_F80FHandler getSignature_F80FHandler()
    fn getSignature_F80FHandler(&mut self) -> &Signature_F80FHandler {
      // Java: if (null == this.mSignature_F80FHandler)
      if self.mSignature_F80FHandler.is_none() {
         // Java: this.mSignature_F80FHandler = new Signature_F80FHandler(this.mUofEncryptInfo);
         self.mSignature_F80FHandler = Some(Signature_F80FHandler::Signature_F80FHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSignature_F80FHandler;
      self.mSignature_F80FHandler.as_ref().unwrap()
    }

    // Java: Rules_F801Handler getRules_F801Handler()
    fn getRules_F801Handler(&mut self) -> &Rules_F801Handler {
      // Java: if (null == this.mRules_F801Handler)
      if self.mRules_F801Handler.is_none() {
         // Java: this.mRules_F801Handler = new Rules_F801Handler(this.mUofEncryptInfo);
         self.mRules_F801Handler = Some(Rules_F801Handler::Rules_F801Handler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mRules_F801Handler;
      self.mRules_F801Handler.as_ref().unwrap()
    }

}

impl ElementHandler for Security_F800Handler {
    fn startElement(&self, _var1: i32, _var2: &dyn XmlRoAttr) { }
    fn endElement(&self, _var1: i32) { }
    fn characters(&self, _var1: &str) { }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
