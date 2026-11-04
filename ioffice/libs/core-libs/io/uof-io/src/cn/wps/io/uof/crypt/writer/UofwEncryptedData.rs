// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.UofPswEncryptor
use crate::cn::wps::io::uof::crypt::UofPswEncryptor::UofPswEncryptor;
// Java: import cn.wps.io.uof.writer.UofwPackage
use crate::cn::wps::io::uof::writer::UofwPackage::UofwPackage;
// Java: import java.io.OutputStream
use std::io::Write;

// Java: public class UofwEncryptedData
pub struct UofwEncryptedData {
}

// Java: public class UofwEncryptedData
impl UofwEncryptedData {
    // Java: UofwEncryptedData()
    pub fn UofwEncryptedData() -> Self {
      // Java: super();
      UofwEncryptedData {}
    }

    // Java: static UofEncryptInfo exportEncryptData(UofwPackage pkg, String srcPath, String password)
    pub fn exportEncryptData(pkg: &mut UofwPackage, srcPath: &str, password: &str) -> Option<UofEncryptInfo> {
      // Java: OutputStream os = pkg.createOutputStream("security/encryptedData.bin");
      let os = pkg.createOutputStream("security/encryptedData.bin".to_string());
      // Java: if (null == os)
      if os.is_none()
      {
         // Java: return null;
         None
      }
      // Java: else
      else
      {
         // Java: UofPswEncryptor encryptor = new UofPswEncryptor(srcPath, os, password, "");
         let buf: Vec<u8> = Vec::new();
         let mut encryptor = UofPswEncryptor::UofPswEncryptor(srcPath.to_string(), Box::new(buf), password.to_string(), String::new());
         // Java: return encryptor.encrypt();
         let result = encryptor.encrypt();
         result
      }
    }

}
