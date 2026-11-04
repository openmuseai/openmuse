#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.base.log.Log
// use util::cn::wps::base::log::Log;
// Java: import cn.wps.core.runtime.Platform
// use util_adapter::core::runtime::Platform;
// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::File;
// Java: import cn.wps.moffice.kfs.FileOutputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::FileOutputStream;
// Java: import cn.wps.moffice.online.security.IWpsSecurityFile
// use util_adapter::cn::wps::moffice::online::security::IWpsSecurityFile;
// Java: import cn.wps.moffice.online.security.exception.PermissionDeniedException
// use util_adapter::cn::wps::moffice::online::security::exception::PermissionDeniedException;
// Java: import cn.wps.moffice.util.FileUtil
// use util::cn::wps::moffice::util::FileUtil;
// Java: import cn.wps.securedoc.DocEncrypt
// use thirdparty_doc_encrypt::cn::wps::securedoc::DocEncrypt;
// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::Read;
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
// use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

use super::WpsSecurityTicket::WpsSecurityTicket;
use super::WpsTransform::WpsTransform;
use super::TransformMapEntry::TransformMapEntry;
use super::Tool::Tool;

// Java field: String TAG
const TAG: Option<&str> = None;
// Java field: String WPSCONTENT
const WPSCONTENT: &str = "WpsContent";
// Java field: String WPSTRANSFORM
const WPSTRANSFORM: &str = "WpsTransform";
// Java field: int DECRYPT_BUFFER_SIZE
const DECRYPT_BUFFER_SIZE: i32 = 16384;

// Java: public class WpsSecurityFile implements IWpsSecurityFile
pub struct WpsSecurityFile {
    // Java field: POIFSFileSystem mDocument
    mDocument: Option<Box<dyn std::any::Any>>,
    // Java field: DirectoryNode mRootStorage
    mRootStorage: Option<Box<dyn std::any::Any>>,
    // Java field: WpsSecurityTicket mSecurityTicket
    mSecurityTicket: Option<WpsSecurityTicket>,
    // Java field: WpsTransform mWpsTransform
    mWpsTransform: Option<WpsTransform>,
}

// Java: public class WpsSecurityFile implements IWpsSecurityFile
impl WpsSecurityFile {
    // Java: WpsSecurityFile(POIFSFileSystem document)
    pub fn new(_document: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mDocument = document;
      // Java: this.mRootStorage = this.mDocument.getRoot();
      todo!()
    }

    // Java: boolean decryptFile(String dest, String passwd)
    pub fn decryptFile(&mut self, dest: &str, passwd: &str) -> bool {
      // Java: boolean isSuccess = false;
      let mut isSuccess = false;
      // Java: File fileInfo = null;
      let mut fileInfo: Option<String> = None;
      // Java: File fileContent = null;
      let mut fileContent: Option<String> = None;
      // Java: FileOutputStream out = null;
      // Java: InputStream in = null;
      // Java: byte[] buffer = new byte[16384];
      let mut buffer = vec![0u8; DECRYPT_BUFFER_SIZE as usize];

      // Java: try
      {
         // Java: fileInfo = Platform.createTempFile("doc_info", ".temp");
         // Java: out = new FileOutputStream(fileInfo);
         // Java: in = this.getEncryptionInfo();
         // Java: int read = 0;
         // Java: while((read = in.read(buffer)) > 0)
         {
            // Java: out.write(buffer, 0, read);
         }
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(out);
         // Java: FileUtil.closeQuietly(in);
      }

      // Java: try
      {
         // Java: fileContent = Platform.createTempFile("doc_content", ".temp");
         // Java: out = new FileOutputStream(fileContent);
         // Java: in = this.getWpsContent();
         // Java: int read = 0;
         // Java: while((read = in.read(buffer)) > 0)
         {
            // Java: out.write(buffer, 0, read);
         }
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(out);
         // Java: FileUtil.closeQuietly(in);
      }

      // Java: isSuccess = DocEncrypt.decrypt(fileContent.getAbsolutePath(), fileInfo.getAbsolutePath(), dest, passwd);
      // Java: return isSuccess;
      todo!()
    }

    // Java: int getVersionMajor()
    pub fn getVersionMajor(&mut self) -> i32 {
      // Java: WpsTransform wpsTransform = this.getWpsTransform();
      // Java: WpsVersion wpsVersion = wpsTransform.getWpsVersion();
      // Java: return wpsVersion.getVersionMajor();
      todo!()
    }

    // Java: int getVersionMinor()
    pub fn getVersionMinor(&mut self) -> i32 {
      // Java: WpsTransform wpsTransform = this.getWpsTransform();
      // Java: WpsVersion wpsVersion = wpsTransform.getWpsVersion();
      // Java: return wpsVersion.getVersionMinor();
      todo!()
    }

    // Java: String getDoc_guid()
    pub fn getDoc_guid(&mut self) -> Option<String> {
      // Java: try
      {
         // Java: return this.geWpsSecurityTicket().getDoc_guid();
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
         // Java: return null;
      }
      todo!()
    }

    // Java: String getEncdata()
    pub fn getEncdata(&mut self) -> Option<String> {
      // Java: try
      {
         // Java: return this.geWpsSecurityTicket().getEncdata();
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
         // Java: return null;
      }
      todo!()
    }

    // Java: String getDataSign()
    pub fn getDataSign(&self) -> Option<String> {
      // Java: InputStream is = null;
      // Java: try
      {
         // Java: is = this.getWpsContent();
         // Java: String e = Tool.getMD5(is);
         // Java: return e;
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
      }
      // Java: finally
      {
         // Java: FileUtil.closeQuietly(is);
      }
      // Java: return null;
      todo!()
    }

    // Java: boolean isWpsSecurityFile()
    pub fn isWpsSecurityFile(&self) -> bool {
      // Java: try
      {
         // Java: return this.hasWpsContent();
      }
      // Java: catch (IOException e)
      {
         // Java: Log.e(TAG, "IOException", e);
         // Java: return false;
      }
      todo!()
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: if (this.mDocument != null)
      if self.mDocument.is_some() {
         // Java: this.mDocument.dispose();
         // Java: this.mDocument = null;
         self.mDocument = None;
      }
    }

    // Java: WpsSecurityTicket geWpsSecurityTicket()
    fn geWpsSecurityTicket(&mut self) -> &WpsSecurityTicket {
      // Java: if (null == this.mSecurityTicket)
      if self.mSecurityTicket.is_none() {
         // Java: this.mSecurityTicket = this.getWpsSecurityInfo().getWpsSecurityTicket();
         todo!()
      }
      // Java: return this.mSecurityTicket;
      self.mSecurityTicket.as_ref().unwrap()
    }

    // Java: boolean hasWpsContent()
    fn hasWpsContent(&self) -> bool {
      // Java: return null != this.mRootStorage ? this.mRootStorage.hasEntry("WpsContent") : false;
      todo!()
    }

    // Java: DocumentInputStream getWpsContent()
    fn getWpsContent(&self) -> Box<dyn std::any::Any> {
      // Java: return null != this.mRootStorage ? this.mRootStorage.createDocumentInputStream("WpsContent") : null;
      todo!()
    }

    // Java: WpsTransform getWpsTransform()
    pub fn getWpsTransform(&mut self) -> &WpsTransform {
      // Java: if (null != this.mWpsTransform)
      if self.mWpsTransform.is_some() {
         // Java: return this.mWpsTransform;
         return self.mWpsTransform.as_ref().unwrap();
      }
      // Java: else
      {
         // Java: this.mWpsTransform = new WpsTransform((DirectoryNode)this.mRootStorage.getEntry("WpsTransform"));
         // Java: return this.mWpsTransform;
         todo!()
      }
    }

    // Java: WpsSecurityInfo getWpsSecurityInfo()
    fn getWpsSecurityInfo(&mut self) -> Box<dyn std::any::Any> {
      // Java: WpsTransform trans = this.getWpsTransform();
      // Java: ArrayList<TransformMapEntry> entrys = trans.getTransformMap().getTransformMapEntry();
      // Java: int size = entrys.size();
      // Java: String name = null;
      let mut name: Option<String> = None;
      // Java: for(int i = 0; i < size; ++i)
      {
         // Java: TransformMapEntry entry = (TransformMapEntry)entrys.get(i);
         // Java: String transformID = entry.getTransformID();
         // Java: if (transformID.equals("{64419DEF-FF3A-40CB-A659-B9CD7E922CBD}") || transformID.equals("{ED1FA402-65C7-4B5E-99E4-41C5BC3FD077}"))
         {
         }
            // Java: name = entry.getTransformItemName();
            // Java: break;
      }
      // Java: if (null == name)
      if name.is_none() {
         // Java: throw new PermissionDeniedException();
         panic!("PermissionDeniedException");
      }
      // Java: else
      {
         // Java: return trans.getTransformInfo().getWpsSecurityInfo(name);
         todo!()
      }
    }

    // Java: DocumentInputStream getEncryptionInfo()
    fn getEncryptionInfo(&mut self) -> Box<dyn std::any::Any> {
      // Java: return this.getWpsSecurityInfo().getWpsEncryptionInfo();
      todo!()
    }

}
