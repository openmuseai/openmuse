// Java: import cn.wps.base.assertion.Assert

use crate::cn::wps::encrypt::EncryptionInfo::EncryptionInfo;
use crate::cn::wps::encrypt::EncryptionInfoInit::EncryptionInfoInit;
use crate::cn::wps::encrypt::EncryptionHeader::EncryptionHeader;
use crate::cn::wps::encrypt::EncryptionHeader::EncryptionInfoInitTrait;
use crate::cn::wps::encrypt::EncryptionVerifier::EncryptionVerifier;

// Java: public class EncryptionInfoPro extends EncryptionInfo
pub struct EncryptionInfoPro {
    pub(crate) base: EncryptionInfo,
    // Java field: EncryptionInfoInit encryptionInfoInit
    pub(crate) encryptionInfoInit: EncryptionInfoInit,
}

// Java: public class EncryptionInfoPro extends EncryptionInfo
impl EncryptionInfoPro {
    // Java: EncryptionInfoPro(String password)
    pub fn new(password: &str) -> Self {
        // Java: super();
        // Java: this.versionMajor = 3;
        let versionMajor = 3;
        // Java: this.versionMinor = 2;
        let versionMinor = 2;
        // Java: this.encryptionFlags = 36;
        let encryptionFlags = 36;
        let mut encryptionInfoInit = EncryptionInfoInit::new();
        // Java: this.encryptionInfoInit.putNamedItem("versionMajor", this.versionMajor);
        encryptionInfoInit.putNamedItem("versionMajor", Box::new(versionMajor));
        // Java: this.encryptionInfoInit.putNamedItem("versionMinor", this.versionMinor);
        encryptionInfoInit.putNamedItem("versionMinor", Box::new(versionMinor));
        // Java: this.encryptionInfoInit.putNamedItem("encryptionFlags", this.encryptionFlags);
        encryptionInfoInit.putNamedItem("encryptionFlags", Box::new(encryptionFlags));
        // Java: this.header = new EncryptionHeader(this.encryptionInfoInit);
        let header = EncryptionHeader::new_from_info_init(&mut encryptionInfoInit as &mut dyn EncryptionInfoInitTrait);
        // Java: this.verifier = new EncryptionVerifier(password, this.encryptionInfoInit);
        let verifier = EncryptionVerifier::new_from_info_init(password, &mut encryptionInfoInit as &mut dyn EncryptionInfoInitTrait);
        let base = EncryptionInfo {
            versionMajor,
            versionMinor,
            encryptionFlags,
            header: Some(header),
            verifier: Some(verifier),
            allInfo: None,
            version: None,
        };
        EncryptionInfoPro {
            base,
            encryptionInfoInit,
        }
    }

    // Java: EncryptionInfoInit getInfo()
    pub fn getInfo(&self) -> &EncryptionInfoInit {
        // Java: return this.encryptionInfoInit;
        &self.encryptionInfoInit
    }

    // Java: byte[] getAllInfo()
    pub fn getAllInfo(&self) -> Option<&[u8]> {
        // Java: Assert.fail("Professional should getInfo");
        panic!("Professional should getInfo");
        // Java: return this.allInfo;
        self.base.allInfo.as_deref()
    }
}
