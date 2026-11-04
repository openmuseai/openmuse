#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.base.log.Log
// Java: import cn.wps.core.runtime.Platform
// Java: import cn.wps.encrypt.EncryptOOXmlFilePro
// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileInputStream
// Java: import cn.wps.moffice.online.security.IWpsSecurityFileWrite
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.pro.security.read.Tool
// Java: import cn.wps.securedoc.DocEncrypt
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.util.Random
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem

// Java field: String TAG
pub(crate) const TAG: Option<&str> = None;
// Java field: String WPSTRANSFORM
pub(crate) const WPSTRANSFORM: &str = "WpsTransform";

// Java: public class WpsSecurityFileWrite implements IWpsSecurityFileWrite
pub struct WpsSecurityFileWrite {
    // Java field: String mPath
    pub(crate) mPath: String,
    // Java field: String mSrcPath
    pub(crate) mSrcPath: String,
    // Java field: String mTempPath
    mTempPath: Option<String>,
    // Java field: String mInfoPath
    mInfoPath: Option<String>,
    // Java field: POIFSFileSystem mDocument
    mDocument: Option<Box<dyn std::any::Any>>,
    // Java field: DirectoryNode mRootStorage
    mRootStorage: Option<Box<dyn std::any::Any>>,
    // Java field: WpsTransform mWpsTransform
    mWpsTransform: Option<Box<dyn std::any::Any>>,
    // Java field: EncryptOOXmlFilePro mEncryptOOXmlFilePro
    mEncryptOOXmlFilePro: Option<Box<dyn std::any::Any>>,
    // Java field: String mMd5Str
    pub(crate) mMd5Str: Option<String>,
}

// Java: public class WpsSecurityFileWrite implements IWpsSecurityFileWrite
impl WpsSecurityFileWrite {
    // Java: WpsSecurityFileWrite(String path, String srcPath, InputStream template)
    pub fn new(_path: &str, _srcPath: &str, _template: &mut dyn std::any::Any) -> Self {
      // Java: super();
      // Java: this.mPath = path;
      // Java: this.mSrcPath = srcPath;
      // Java: this.mDocument = this.createBlank(template);
      // Java: this.mRootStorage = this.mDocument.getRoot();
      todo!()
    }

    // Java: POIFSFileSystem createBlank(InputStream template)
    fn createBlank(&mut self, _template: &mut dyn std::any::Any) {
      // Java: this.mDocument = new POIFSFileSystem();
      // Java: if (null != template)
      {
         // Java: try
         {
            // Java: POIFSFileSystem tempFileSystem = new POIFSFileSystem(template);
            // Java: POIFSFileSystem target = this.mDocument;
            // Java: Tool.copyNodes(tempFileSystem, target);
         }
         // Java: catch (IOException e)
         {
            // Java: e.printStackTrace();
         }
      }
      // Java: return this.mDocument;
      todo!()
    }

    // Java: static String getRandomPassword()
    pub fn getRandomPassword() -> String {
      // Java: return genRandomNum(32);
      Self::genRandomNum(32)
    }

    // Java: void save(String passwd, String guid, String endata)
    pub fn save(&mut self, _passwd: &str, _guid: &str, _endata: &str) {
      // Java: Assert.assertNotNull(this.mMd5Str);
      // Java: this.setWpsSecurityTicket(guid, endata);
      // Java: Assert.assertNotNull("mTempPath should not be null.", this.mTempPath);
      // Java: this.mEncryptOOXmlFilePro.write(this.mPath, this.mTempPath);
      todo!()
    }

    // Java: String generateDataSign(String passwd)
    pub fn generateDataSign(&mut self, _passwd: &str) -> Option<String> {
      // Java: Assert.assertNull(this.mMd5Str);
      // Java: this.mEncryptOOXmlFilePro = new EncryptOOXmlFilePro(this.mDocument);
      // Java: String encryptedContentPath = this.getEncryptTempPath();
      // Java: String infoPath = this.getInfoPath();
      // Java: if (DocEncrypt.encrypt(this.mSrcPath, infoPath, encryptedContentPath, passwd))
      {
         // Java: this.getWpsSecurityInfo().createWpsEncryptionInfo(infoPath);
         // Java: FileInputStream in = null;
         // Java: try
         {
            // Java: in = new FileInputStream(encryptedContentPath);
            // Java: this.mMd5Str = Tool.getMD5(in);
         }
         // Java: catch (IOException e)
         {
            // Java: Log.e(TAG, "IOException", e);
            // Java: throw e;
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(in);
         }
      }
      // Java: return this.mMd5Str;
      todo!()
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: this.mDocument.dispose();
      self.mDocument = None;
    }

    // Java: static String genRandomNum(int pwd_len)
    fn genRandomNum(pwd_len: i32) -> String {
      // Java: int maxNum = 16;
      let _maxNum = 16;
      // Java: int count = 0;
      let mut count = 0;
      // Java: char[] str = new char[]{'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};
      let str_chars: [char; 16] = ['a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
      // Java: StringBuilder pwd = new StringBuilder(pwd_len);
      let mut pwd = String::with_capacity(pwd_len as usize);
      // Java: Random r = new Random();
      // Java: while(count < pwd_len)
      while count < pwd_len {
         // Java: int i = Math.abs(r.nextInt(16));
         let i = ((std::time::SystemTime::now()
             .duration_since(std::time::UNIX_EPOCH)
             .unwrap_or_default()
             .subsec_nanos() as usize + count as usize) % 16) as usize;
         // Java: if (i >= 0 && i < str.length)
         if i < str_chars.len() {
            // Java: pwd.append(str[i]);
            pwd.push(str_chars[i]);
            // Java: ++count;
            count += 1;
         }
      }
      // Java: return pwd.toString();
      pwd
    }

    // Java: void setWpsSecurityTicket(String guid, String endata)
    fn setWpsSecurityTicket(&mut self, _guid: &str, _endata: &str) {
      // Java: Assert.assertNotNull(guid);
      // Java: Assert.assertNotNull(endata);
      // Java: this.getWpsSecurityInfo().createWpsSecurityTicket(guid, endata);
      todo!()
    }

    // Java: String getEncryptTempPath()
    fn getEncryptTempPath(&mut self) -> &str {
      // Java: if (this.mTempPath == null)
      if self.mTempPath.is_none() {
         // Java: this.mTempPath = Platform.createTempFile("wscont", ".tmp").getAbsolutePath();
         todo!()
      }
      // Java: return this.mTempPath;
      self.mTempPath.as_deref().unwrap()
    }

    // Java: String getInfoPath()
    fn getInfoPath(&mut self) -> &str {
      // Java: if (null == this.mInfoPath)
      if self.mInfoPath.is_none() {
         // Java: this.mInfoPath = Platform.createTempFile("wsinfo", ".tmp").getAbsolutePath();
         todo!()
      }
      // Java: File file = new File(this.mInfoPath);
      // Java: if (file.exists())
      {
         // Java: file.delete();
      }
      // Java: return this.mInfoPath;
      self.mInfoPath.as_deref().unwrap()
    }

    // Java: WpsTransform getWpsTransform()
    fn getWpsTransform(&mut self) -> &Option<Box<dyn std::any::Any>> {
      // Java: if (null == this.mWpsTransform)
      if self.mWpsTransform.is_none() {
         // Java: this.mWpsTransform = new WpsTransform(this.mRootStorage.createDirectory("WpsTransform"));
         todo!()
      }
      // Java: return this.mWpsTransform;
      &self.mWpsTransform
    }

    // Java: WpsSecurityInfo getWpsSecurityInfo()
    fn getWpsSecurityInfo(&mut self) -> Option<Box<dyn std::any::Any>> {
      // Java: return this.getWpsTransform().getTransformInfo().getWpsSecurityInfo();
      todo!()
    }

    // Java: String getDestPath()
    pub fn getDestPath(&self) -> &str {
      // Java: return this.mPath;
      &self.mPath
    }

    // Java: String getSrcPath()
    pub fn getSrcPath(&self) -> &str {
      // Java: return this.mSrcPath;
      &self.mSrcPath
    }

}
