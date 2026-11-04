#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.log.Log
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.security.GeneralSecurityException
// Java: import javax.crypto.Cipher
// Java: import javax.crypto.CipherInputStream
// Java: import javax.crypto.SecretKey
// Java: import javax.crypto.spec.SecretKeySpec
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// Java: import org.apache.poi.util.LittleEndian

use std::sync::{OnceLock, Mutex};

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;

static S_EDIT_PASS: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_editPass() -> &'static Mutex<Option<String>> {
    S_EDIT_PASS.get_or_init(|| Mutex::new(None))
}

// Java: public class EncryptOOXmlFilePro
pub struct EncryptOOXmlFilePro {
    // Java field: POIFSFileSystem mFilesystem
    mFilesystem: Option<Box<dyn std::any::Any>>,
    // Java field: String mOpenPassword
    pub(crate) mOpenPassword: Option<String>,
    // Java field: Decryptor decryptor
    decryptor: Option<Box<dyn std::any::Any>>,
    // Java field: boolean isVerified
    pub(crate) isVerified: bool,
    // Java field: ModifyVerifier modifyVerifier
    pub(crate) modifyVerifier: Option<Box<dyn std::any::Any>>,
    // Java field: String editPass
    pub(crate) editPass: Option<String>,
    // Java field: DirectoryNode mRootStorage
    mRootStorage: Option<Box<dyn std::any::Any>>,
    // Java field: EncryptionInfoPro mInfo
    pub(crate) mInfo: Option<Box<dyn std::any::Any>>,
}

// Java: public class EncryptOOXmlFilePro
impl EncryptOOXmlFilePro {
    // Java: EncryptOOXmlFilePro(POIFSFileSystem filesystem)
    pub fn new(_filesystem: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: this.mFilesystem = filesystem;
      // Java: this.mRootStorage = filesystem.getRoot();
      todo!()
    }

    // Java: void setOpenPassword(String openPwd)
    pub fn setOpenPassword(&mut self, openPwd: &str) {
      // Java: this.mOpenPassword = openPwd;
      self.mOpenPassword = Some(openPwd.to_string());
      // Java: this.mInfo = new EncryptionInfoPro(this.mOpenPassword);
      // Java: this.decryptor = Decryptor.getInstance(this.mInfo);
      todo!()
    }

    // Java: String getOpenPassword()
    pub fn getOpenPassword(&self) -> &Option<String> {
      // Java: return this.mOpenPassword;
      &self.mOpenPassword
    }

    // Java: byte[] getAllbyte()
    pub fn getAllbyte(&self) -> Vec<u8> {
      // Java: return this.mInfo.getAllInfo();
      todo!()
    }

    // Java: static void setEditPassword(String editPwd)
    pub fn setEditPassword(editPwd: &str) {
      // Java: editPass = editPwd;
      let mut ep = get_editPass().lock().unwrap();
      *ep = Some(editPwd.to_string());
    }

    // Java: static String getEditPassword()
    pub fn getEditPassword() -> Option<String> {
      // Java: return editPass;
      let ep = get_editPass().lock().unwrap();
      ep.clone()
    }

    // Java: ModifyVerifier getModifyVerifier()
    pub fn getModifyVerifier(&self) -> &Option<Box<dyn std::any::Any>> {
      // Java: return this.modifyVerifier;
      &self.modifyVerifier
    }

    // Java: static boolean verifyEditPassword(ModifyVerifier srcModifyVer, String pwd)
    pub fn verifyEditPassword(_srcModifyVer: &dyn std::any::Any, pwd: &str) -> bool {
      // Java: editPass = pwd;
      let mut ep = get_editPass().lock().unwrap();
      *ep = Some(pwd.to_string());
      drop(ep);
      // Java: boolean val = CryptTool.checkModifyPassword(srcModifyVer, pwd);
      // Java: return val;
      todo!()
    }

    // Java: ModifyVerifier createSaltHashData(String editPwd)
    pub fn createSaltHashData(&mut self, _editPwd: &str) -> &Option<Box<dyn std::any::Any>> {
      // Java: ModifyVerifier modver = CryptTool.createModifyVerifier(editPwd);
      // Java: this.modifyVerifier = modver;
      // Java: return this.modifyVerifier;
      todo!()
    }

    // Java: boolean verifyOpenPassword(String pwd)
    pub fn verifyOpenPassword(&mut self, _pwd: &str) -> bool {
      // Java: try
      {
         // Java: this.isVerified = this.decryptor.verifyPassword(pwd);
         // Java: if (this.isVerified)
         {
            // Java: this.mOpenPassword = pwd;
         }
         // Java: return this.isVerified;
      }
      // Java: catch (GeneralSecurityException e)
      {
         // Java: e.printStackTrace();
         // Java: return false;
      }
      todo!()
    }

    // Java: boolean isEncryptFile()
    pub fn isEncryptFile(&self) -> bool {
      // Java: try
      {
         // Java: if (this.mFilesystem.getRoot().getEntry("EncryptionInfo") != null)
         {
            // Java: return true;
         }
      }
      // Java: catch (Exception e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return false;
      todo!()
    }

    // Java: int readCryptFileStreamToFile(String outfilepath)
    pub fn readCryptFileStreamToFile(&mut self, _outfilepath: &str) -> i32 {
      // Java: POIFSFileSystem filesystem = this.mFilesystem;
      // Java: Decryptor d = Decryptor.getInstance(this.mInfo);
      // Java: try
      {
         // Java: if (!d.verifyPassword(this.mOpenPassword))
         {
            // Java: throw new RuntimeException("Unable to process: document is encrypted");
         }
         // Java: else
         {
            // Java: InputStream dataStream = d.getDataStream(filesystem);
            // Java: File tempfile = new File(outfilepath);
            // Java: FileOutputStream out = new FileOutputStream(tempfile);
            // Java: byte[] b = new byte[10240];
            // Java: int len;
            // Java: while((len = dataStream.read(b)) > 0)
            {
               // Java: out.write(b, 0, len);
            }
            // Java: FileUtil.closeQuietly(out);
            // Java: dataStream.close();
            // Java: return 0;
         }
      }
      // Java: catch (GeneralSecurityException ex)
      {
         // Java: throw new RuntimeException("Unable to process encrypted document", ex);
      }
      todo!()
    }

    // Java: Cipher getCipher()
    pub fn getCipher(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: try
      {
         // Java: CryptTool cryptTool = new CryptTool();
         // Java: Cipher cipher = Cipher.getInstance("AES");
         // Java: byte[] passwordHash = this.mInfo.getVerifier().getPasswordHash();
         // Java: cryptTool.setPasswordHash(passwordHash);
         // Java: byte[] key = cryptTool.generateKey(0, 16);
         // Java: SecretKey sKey = new SecretKeySpec(key, "AES");
         // Java: cipher.init(1, sKey);
         // Java: return cipher;
      }
      // Java: catch (Exception e)
      {
         // Java: e.printStackTrace();
         // Java: return null;
      }
      todo!()
    }

    // Java: void write(String destFile, String wpsContentFilePath)
    pub fn write_str(&mut self, _destFile: &str, _wpsContentFilePath: &str) {
      // Java: this.write(new File(destFile), wpsContentFilePath);
      todo!()
    }

    // Java: boolean encryptContent(String srcFile, String tmpFilePath)
    pub fn encryptContent(&self, _srcFile: &str, _tmpFilePath: &str) -> bool {
      // Java: File sourceFile = new File(srcFile);
      // Java: try
      {
         // Java: FileInputStream inStream = new FileInputStream(sourceFile);
         // Java: CipherInputStream input = new CipherInputStream(inStream, this.getCipher());
         // Java: byte[] encryptData = new byte[10240];
         // Java: File file = new File(tmpFilePath);
         // Java: if (file.exists())
         {
            // Java: file.delete();
         }
         // Java: FileOutputStream output = new FileOutputStream(tmpFilePath);
         // Java: byte[] len = new byte[8];
         // Java: int encryptLen = 0;
         // Java: LittleEndian.putLong(len, 0, sourceFile.length());
         // Java: output.write(len, 0, 8);
         // Java: while((encryptLen = input.read(encryptData)) != -1)
         {
            // Java: output.write(encryptData, 0, encryptLen);
         }
         // Java: FileUtil.closeQuietly(output);
         // Java: input.close();
         // Java: CipherInputStream var13 = null;
         // Java: inStream.close();
         // Java: return true;
      }
      // Java: catch (FileNotFoundException e)
      {
         // Java: e.printStackTrace();
         // Java: return false;
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         // Java: return false;
      }
      todo!()
    }

    // Java: void write(File destFile, String wpsContentFilePath)
    pub fn write_file(&mut self, _destFile: &mut dyn std::any::Any, _wpsContentFilePath: &str) {
      // Java: FileOutputStream fos = null;
      // Java: FileInputStream fis = null;
      // Java: try
      {
         // Java: fis = new FileInputStream(wpsContentFilePath);
         // Java: this.mRootStorage.createDocument("WpsContent", fis, false);
         // Java: fos = new FileOutputStream(destFile);
         // Java: this.mFilesystem.writeFilesystem(fos);
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
         // Java: throw e;
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(fos);
         // Java: FileUtil.closeQuietly(fis);
      }
      todo!()
    }

    // Java: EncryptionInfoInit getInfo()
    pub fn getInfo(&self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this.mInfo.getInfo();
      todo!()
    }

}
