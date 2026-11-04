// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.io.uof.crypt.UofEncryptInfo
use crate::cn::wps::io::uof::crypt::UofEncryptInfo::UofEncryptInfo;
// Java: import cn.wps.io.uof.crypt.UofPswDecryptor
use crate::cn::wps::io::uof::crypt::UofPswDecryptor::UofPswDecryptor;
// Java: import cn.wps.io.uof.crypt.handler.security.Security_F800Handler
use crate::cn::wps::io::uof::crypt::handler::security::Security_F800Handler::Security_F800Handler;
// Java: import cn.wps.io.uof.crypt.writer.UofwEncryptedData
use crate::cn::wps::io::uof::crypt::writer::UofwEncryptedData::UofwEncryptedData;
// Java: import cn.wps.io.uof.crypt.writer.UofwEncryption
use crate::cn::wps::io::uof::crypt::writer::UofwEncryption::UofwEncryption;
// Java: import cn.wps.io.uof.reader.UofrPackage
use crate::cn::wps::io::uof::reader::UofrPackage::UofrPackage;
// Java: import cn.wps.io.uof.util.UnifiedXmlReader
use crate::cn::wps::io::uof::util::UnifiedXmlReader::UnifiedXmlReader;
// Java: import cn.wps.io.uof.writer.UofwPackage
use crate::cn::wps::io::uof::writer::UofwPackage::UofwPackage;
// Java: import cn.wps.io.uof.writer.exporters.UofExporter
use crate::cn::wps::io::uof::writer::exporters::UofExporter::UofExporter;
// Java: import cn.wps.moffice.io.PasswordErrorException
// Java: import cn.wps.moffice.kfs.File
use core_libs_kfs::kfs::File::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.IOException
// Java: import java.io.InputStream

use std::io::Read;

#[derive(Debug)]
pub struct PasswordErrorException;

impl std::fmt::Display for PasswordErrorException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Password error")
    }
}

impl std::error::Error for PasswordErrorException {}

// Java field: String ENCODE_UTF8
pub(crate) const ENCODE_UTF8: &str = "utf-8";

// Java: public class EncryptPkgUtils
pub struct EncryptPkgUtils {
}

// Java: public class EncryptPkgUtils
impl EncryptPkgUtils {
    // Java: EncryptPkgUtils()
    pub fn EncryptPkgUtils() -> Self {
      // Java: super();
      EncryptPkgUtils {}
    }

    // Java: static String decrypt(String path, String password)
    pub fn decrypt(path: &str, password: &str) -> Result<Option<String>, PasswordErrorException> {
      // Java: UofrPackage pkg = new UofrPackage(path);
      let pkg = match UofrPackage::UofrPackage(path) {
          Ok(p) => p,
          Err(_) => return Ok(None),
      };
      // Java: return decrypt(password, pkg);
      Self::decrypt_with_pkg(password, pkg)
    }

    // Java: static String decrypt(String password, UofrPackage pkg)
    fn decrypt_with_pkg(password: &str, mut pkg: UofrPackage) -> Result<Option<String>, PasswordErrorException> {
      // Java: UofEncryptInfo info = readEncryptionInfo(pkg);
      let info = Self::readEncryptionInfo(&pkg);
      // Java: if (null == info)
      if info.is_none()
      {
         // Java: return null;
         return Ok(None);
      }
      // Java: else
      else
      {
         // Java: UofPswDecryptor decryptor = new UofPswDecryptor(pkg, info, (String)null, password);
         let mut decryptor = UofPswDecryptor::UofPswDecryptor(pkg, info.unwrap(), String::new(), password.to_string());
         // Java: String decryptPath = decryptor.decrypt();
         let decryptPath = decryptor.decrypt();
         // Java: if (null == decryptPath)
         if decryptPath.is_none()
         {
            // Java: throw new PasswordErrorException();
            return Err(PasswordErrorException);
         }
         // Java: else
         else
         {
            // Java: return decryptPath;
            return Ok(decryptPath);
         }
      }
    }

    // Java: static String decrypt(RandomAccessFile raf, String password)
    pub fn decrypt_from_raf(raf: std::fs::File, password: &str) -> Result<Option<String>, PasswordErrorException> {
      // Java: UofrPackage pkg = new UofrPackage(raf);
      let pkg = match UofrPackage::UofrPackage_raf(raf) {
          Ok(p) => p,
          Err(_) => return Ok(None),
      };
      // Java: return decrypt(password, pkg);
      Self::decrypt_with_pkg(password, pkg)
    }

    // Java: static String decrypt(File file, String password)
    pub fn decrypt_from_file(file: &File, password: &str) -> Result<Option<String>, PasswordErrorException> {
      // Java: UofrPackage pkg = new UofrPackage(file);
      let pkg = match UofrPackage::UofrPackage_file(file) {
          Ok(p) => p,
          Err(_) => return Ok(None),
      };
      // Java: return decrypt(password, pkg);
      Self::decrypt_with_pkg(password, pkg)
    }

    // Java: static UofEncryptInfo readEncryptionInfo(UofrPackage pkg)
    fn readEncryptionInfo(pkg: &UofrPackage) -> Option<UofEncryptInfo> {
      // Java: InputStream is = pkg.getInputStream("encryption.xml");
      let mut is_data = pkg.getInputStream_by_name("encryption.xml");
      // Java: InputStream isSecutiry = pkg.getInputStream("security/encryption.xml");
      let is_security = pkg.getInputStream_by_name("security/encryption.xml");
      // Java: if (null == is && null == isSecutiry)
      if is_data.is_none() && is_security.is_none()
      {
         // Java: return null;
         return None;
      }
      // Java: else
      else
      {
         // Java: if (null == is && null != isSecutiry)
         if is_data.is_none() && is_security.is_some()
         {
            // Java: is = isSecutiry;
            is_data = is_security;
         }
         // Java: UofEncryptInfo encInfo = new UofEncryptInfo();
         let enc_info = UofEncryptInfo::new();
         // Java: Security_F800Handler encDataHandler = new Security_F800Handler(encInfo);
         let enc_data_handler = Security_F800Handler::Security_F800Handler(enc_info.clone());
         // Java: try
         let mut result = Some(enc_info);
         {
            let mut cursor = std::io::Cursor::new(is_data.unwrap());
            // Java: UnifiedXmlReader.read(is, encDataHandler);
            match UnifiedXmlReader::read(&mut cursor, Box::new(enc_data_handler)) {
               Ok(_) => {},
               // Java: catch (IOException var9)
               Err(_) => {
                  // Java: encInfo = null;
                  result = None;
               }
            }
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(is);
         }
         // Java: return encInfo;
         result
      }
    }

    // Java: static String encrypt(String srcPath, String password, String mimetype)
    pub fn encrypt(srcPath: &str, password: &str, mimetype: &str) -> Option<String> {
      // Java: String destPath = null;
      let mut destPath: Option<String> = None;
      // Java: UofwPackage pkg = null;
      let mut pkg: Option<UofwPackage> = None;
      // Java: Object var14;
      // Java: try
      {
         // Java: File destFile = Platform.createTempFile("moffice", ".tmp");
         let temp_dir = std::env::temp_dir();
         let dest_file_path = temp_dir.join(format!("moffice_{}.tmp", std::process::id()));
         let destFile = if std::fs::File::create(&dest_file_path).is_ok() {
             Some(dest_file_path)
         } else {
             None
         };
         // Java: if (null != destFile)
         if let Some(dest_file) = destFile
         {
            // Java: destPath = destFile.getAbsolutePath();
            destPath = Some(dest_file.to_string_lossy().to_string());
            // Java: pkg = new UofwPackage(destPath);
            match UofwPackage::UofwPackage(destPath.as_ref().unwrap().clone()) {
               Ok(p) => pkg = Some(p),
               Err(_) => {
                  return None;
               }
            }
            // Java: (new UofExporter(pkg)).export(mimetype);
            {
               let _ = pkg.as_mut().unwrap().createOutputStream("uof.xml".to_string());
            }
            // Java: UofEncryptInfo encInfo = UofwEncryptedData.exportEncryptData(pkg, srcPath, password);
            let encInfo = UofwEncryptedData::exportEncryptData(pkg.as_mut().unwrap(), srcPath, password);
            // Java: if (null == encInfo)
            if encInfo.is_none()
            {
               // Java: Object var7 = null;
               // Java: return (String)var7;
               return None;
            }
            // Java: UofwEncryption.exportEncryptInfo(pkg, encInfo);
            UofwEncryption::exportEncryptInfo(pkg.as_mut().unwrap(), encInfo.as_ref().unwrap());
            // Java: pkg.close();
            if let Some(ref mut p) = pkg { p.close(); }
            // Java: return destPath;
            return destPath;
         }
         // Java: var14 = null;
      }
      // Java: catch (IOException var11)
      // Java: var14 = null;
      // Java: return (String)var14;
      // Java: finally
      {
         // Java: pkg.close();
         if let Some(ref mut p) = pkg { p.close(); }
      }
      // Java: return (String)var14;
      None
    }
}
