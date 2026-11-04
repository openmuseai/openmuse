// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.handler.SecurityHandlerBase
use crate::cn::wps::io::uof::crypt::handler::SecurityHandlerBase::SecurityHandlerBase;
// Java: import cn.wps.io.uof.crypt.handler.ds.EncryptionKeyHandler
use crate::cn::wps::io::uof::crypt::handler::ds::EncryptionKeyHandler::EncryptionKeyHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.KeyNameHandler
use crate::cn::wps::io::uof::crypt::handler::ds::KeyNameHandler::KeyNameHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.KeyValueHandler
use crate::cn::wps::io::uof::crypt::handler::ds::KeyValueHandler::KeyValueHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.MgmtDataHandler
use crate::cn::wps::io::uof::crypt::handler::ds::MgmtDataHandler::MgmtDataHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.PGPDataHandler
use crate::cn::wps::io::uof::crypt::handler::ds::PGPDataHandler::PGPDataHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.RetrievalMethodHandler
use crate::cn::wps::io::uof::crypt::handler::ds::RetrievalMethodHandler::RetrievalMethodHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.SPKIDataHandler
use crate::cn::wps::io::uof::crypt::handler::ds::SPKIDataHandler::SPKIDataHandler;
// Java: import cn.wps.io.uof.crypt.handler.ds.X509DataHandler
use crate::cn::wps::io::uof::crypt::handler::ds::X509DataHandler::X509DataHandler;
// Java: import cn.wps.kfc.xpxml.reader.ElementHandler
use core_libs_kfc::kfc::xpxml::reader::ElementHandler::ElementHandler;
// Java: import cn.wps.kfc.xpxml.reader.XmlRoAttr
use core_libs_kfc::kfc::xpxml::reader::XmlRoAttr::XmlRoAttr;

// Java: public class RecipientKeyInfoHandler extends SecurityHandlerBase
pub struct RecipientKeyInfoHandler {
    pub(crate) base: SecurityHandlerBase,
    // Java field: MgmtDataHandler mMgmtDataHandler
    mMgmtDataHandler: Option<MgmtDataHandler>,
    // Java field: KeyNameHandler mKeyNameHandler
    mKeyNameHandler: Option<KeyNameHandler>,
    // Java field: PGPDataHandler mPGPDataHandler
    mPGPDataHandler: Option<PGPDataHandler>,
    // Java field: RetrievalMethodHandler mRetrievalMethodHandler
    mRetrievalMethodHandler: Option<RetrievalMethodHandler>,
    // Java field: KeyValueHandler mKeyValueHandler
    mKeyValueHandler: Option<KeyValueHandler>,
    // Java field: SPKIDataHandler mSPKIDataHandler
    mSPKIDataHandler: Option<SPKIDataHandler>,
    // Java field: EncryptionKeyHandler mEncryptionKeyHandler
    mEncryptionKeyHandler: Option<EncryptionKeyHandler>,
    // Java field: X509DataHandler mX509DataHandler
    mX509DataHandler: Option<X509DataHandler>,
}

// Java: public class RecipientKeyInfoHandler extends SecurityHandlerBase
impl RecipientKeyInfoHandler {
    // Java: RecipientKeyInfoHandler(UofEncryptInfo uofEncryptInfo)
    pub fn RecipientKeyInfoHandler(uofEncryptInfo: UofEncryptInfo) -> Self {
      // Java: super(uofEncryptInfo);
      RecipientKeyInfoHandler {
          base: SecurityHandlerBase::SecurityHandlerBase(uofEncryptInfo),
          mMgmtDataHandler: None, mKeyNameHandler: None, mPGPDataHandler: None,
          mRetrievalMethodHandler: None, mKeyValueHandler: None, mSPKIDataHandler: None,
          mEncryptionKeyHandler: None, mX509DataHandler: None,
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
         // Java: case -1173053755:
         -1173053755i32 =>
            // Java: return this.getSPKIDataHandler();
            Some(Box::new(SPKIDataHandler::SPKIDataHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case -29495696:
         -29495696i32 =>
            // Java: return this.getX509DataHandler();
            Some(Box::new(X509DataHandler::X509DataHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 94524547:
         94524547i32 =>
            // Java: return this.getPGPDataHandler();
            Some(Box::new(PGPDataHandler::PGPDataHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 556894834:
         556894834i32 =>
            // Java: return this.getKeyValueHandler();
            Some(Box::new(KeyValueHandler::KeyValueHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 653483333:
         653483333i32 =>
            // Java: return this.getEncryptionKeyHandler();
            Some(Box::new(EncryptionKeyHandler::EncryptionKeyHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 849010026:
         849010026i32 =>
            // Java: return this.getKeyNameHandler();
            Some(Box::new(KeyNameHandler::KeyNameHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1205836843:
         1205836843i32 =>
            // Java: return this.getMgmtDataHandler();
            Some(Box::new(MgmtDataHandler::MgmtDataHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: case 1714619669:
         1714619669i32 =>
            // Java: return this.getRetrievalMethodHandler();
            Some(Box::new(RetrievalMethodHandler::RetrievalMethodHandler(self.base.mUofEncryptInfo.clone()))),
         // Java: default:
         _ =>
            // Java: return null;
            None,
      }
    }

    // Java: MgmtDataHandler getMgmtDataHandler()
    fn getMgmtDataHandler(&mut self) -> &MgmtDataHandler {
      // Java: if (null == this.mMgmtDataHandler)
      if self.mMgmtDataHandler.is_none() {
         // Java: this.mMgmtDataHandler = new MgmtDataHandler(this.mUofEncryptInfo);
         self.mMgmtDataHandler = Some(MgmtDataHandler::MgmtDataHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mMgmtDataHandler;
      self.mMgmtDataHandler.as_ref().unwrap()
    }

    // Java: KeyNameHandler getKeyNameHandler()
    fn getKeyNameHandler(&mut self) -> &KeyNameHandler {
      // Java: if (null == this.mKeyNameHandler)
      if self.mKeyNameHandler.is_none() {
         // Java: this.mKeyNameHandler = new KeyNameHandler(this.mUofEncryptInfo);
         self.mKeyNameHandler = Some(KeyNameHandler::KeyNameHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mKeyNameHandler;
      self.mKeyNameHandler.as_ref().unwrap()
    }

    // Java: PGPDataHandler getPGPDataHandler()
    fn getPGPDataHandler(&mut self) -> &PGPDataHandler {
      // Java: if (null == this.mPGPDataHandler)
      if self.mPGPDataHandler.is_none() {
         // Java: this.mPGPDataHandler = new PGPDataHandler(this.mUofEncryptInfo);
         self.mPGPDataHandler = Some(PGPDataHandler::PGPDataHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mPGPDataHandler;
      self.mPGPDataHandler.as_ref().unwrap()
    }

    // Java: RetrievalMethodHandler getRetrievalMethodHandler()
    fn getRetrievalMethodHandler(&mut self) -> &RetrievalMethodHandler {
      // Java: if (null == this.mRetrievalMethodHandler)
      if self.mRetrievalMethodHandler.is_none() {
         // Java: this.mRetrievalMethodHandler = new RetrievalMethodHandler(this.mUofEncryptInfo);
         self.mRetrievalMethodHandler = Some(RetrievalMethodHandler::RetrievalMethodHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mRetrievalMethodHandler;
      self.mRetrievalMethodHandler.as_ref().unwrap()
    }

    // Java: KeyValueHandler getKeyValueHandler()
    fn getKeyValueHandler(&mut self) -> &KeyValueHandler {
      // Java: if (null == this.mKeyValueHandler)
      if self.mKeyValueHandler.is_none() {
         // Java: this.mKeyValueHandler = new KeyValueHandler(this.mUofEncryptInfo);
         self.mKeyValueHandler = Some(KeyValueHandler::KeyValueHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mKeyValueHandler;
      self.mKeyValueHandler.as_ref().unwrap()
    }

    // Java: SPKIDataHandler getSPKIDataHandler()
    fn getSPKIDataHandler(&mut self) -> &SPKIDataHandler {
      // Java: if (null == this.mSPKIDataHandler)
      if self.mSPKIDataHandler.is_none() {
         // Java: this.mSPKIDataHandler = new SPKIDataHandler(this.mUofEncryptInfo);
         self.mSPKIDataHandler = Some(SPKIDataHandler::SPKIDataHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mSPKIDataHandler;
      self.mSPKIDataHandler.as_ref().unwrap()
    }

    // Java: EncryptionKeyHandler getEncryptionKeyHandler()
    fn getEncryptionKeyHandler(&mut self) -> &EncryptionKeyHandler {
      // Java: if (null == this.mEncryptionKeyHandler)
      if self.mEncryptionKeyHandler.is_none() {
         // Java: this.mEncryptionKeyHandler = new EncryptionKeyHandler(this.mUofEncryptInfo);
         self.mEncryptionKeyHandler = Some(EncryptionKeyHandler::EncryptionKeyHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mEncryptionKeyHandler;
      self.mEncryptionKeyHandler.as_ref().unwrap()
    }

    // Java: X509DataHandler getX509DataHandler()
    fn getX509DataHandler(&mut self) -> &X509DataHandler {
      // Java: if (null == this.mX509DataHandler)
      if self.mX509DataHandler.is_none() {
         // Java: this.mX509DataHandler = new X509DataHandler(this.mUofEncryptInfo);
         self.mX509DataHandler = Some(X509DataHandler::X509DataHandler(self.base.mUofEncryptInfo.clone()));
      }
      // Java: return this.mX509DataHandler;
      self.mX509DataHandler.as_ref().unwrap()
    }

}

impl ElementHandler for RecipientKeyInfoHandler {
    fn startElement(&self, var1: i32, var2: &dyn XmlRoAttr) { self.startElement(var1, var2); }
    fn endElement(&self, var1: i32) { self.endElement(var1); }
    fn characters(&self, var1: &str) { self.characters(var1); }
    fn enterSubElement(&self, var1: i32) -> Option<Box<dyn ElementHandler>> { self.enterSubElement(var1) }
}
