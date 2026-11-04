#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.encrypt.EncryptionInfoInit
// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream

use super::EncryptionHeader::EncryptionHeader;
use super::EncryptionVerifier::EncryptionVerifier;
use super::EncryptionInfo::EncryptionInfo;

// Java: public class EncryptionInfoPro extends EncryptionInfo
pub struct EncryptionInfoPro {
    // Java field: EncryptionInfoInit encryptionInfoInit
    encryptionInfoInit: Option<Box<dyn std::any::Any>>,
    pub base: EncryptionInfo,
}

// Java: public class EncryptionInfoPro extends EncryptionInfo
impl EncryptionInfoPro {
    // Java: EncryptionInfoPro(DocumentInputStream dis)
    pub fn new(_dis: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: this.versionMajor = dis.readShort();
      // Java: this.versionMinor = dis.readShort();
      // Java: this.encryptionFlags = dis.readInt();
      // Java: this.encryptionInfoInit = new EncryptionInfoInit(dis);
      // Java: if (this.versionMajor == 4 && this.versionMinor == 4 && this.encryptionFlags == 64)
      {
         // Java: this.header = new EncryptionHeader(this.encryptionInfoInit);
         // Java: this.verifier = new EncryptionVerifier(this.encryptionInfoInit);
      }
      // Java: else
      {
         // Java: this.header = new EncryptionHeader(this.encryptionInfoInit);
         // Java: this.verifier = new EncryptionVerifier(this.encryptionInfoInit);
      }
      todo!()
    }

    // Java: int getVersionMajor()
    pub fn getVersionMajor(&self) -> i32 {
      // Java: return this.versionMajor;
      self.base.versionMajor
    }

    // Java: int getVersionMinor()
    pub fn getVersionMinor(&self) -> i32 {
      // Java: return this.versionMinor;
      self.base.versionMinor
    }

    // Java: int getEncryptionFlags()
    pub fn getEncryptionFlags(&self) -> i32 {
      // Java: return this.encryptionFlags;
      self.base.encryptionFlags
    }

    // Java: EncryptionHeader getHeader()
    pub fn getHeader(&self) -> &Option<EncryptionHeader> {
      // Java: return this.header;
      &self.base.header
    }

    // Java: EncryptionVerifier getVerifier()
    pub fn getVerifier(&self) -> &Option<EncryptionVerifier> {
      // Java: return this.verifier;
      &self.base.verifier
    }

}
