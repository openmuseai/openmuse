#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::File;
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::FileInputStream;
// Java: import cn.wps.moffice.util.FileUtil
// use util::cn::wps::moffice::util::FileUtil;
// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;

// Java field: String WPSSECURITYTICKET
const WPSSECURITYTICKET: &str = "WpsSecurityTicket";
// Java field: String WPSENCRYPTIONINFO
const WPSENCRYPTIONINFO: &str = "WpsEncryptionInfo";

// Java: public class WpsSecurityInfo
pub struct WpsSecurityInfo {
    // Java field: DirectoryNode mTransformInfo
    mTransformInfo: Option<Box<dyn std::any::Any>>,
}

// Java: public class WpsSecurityInfo
impl WpsSecurityInfo {
    // Java: WpsSecurityInfo(DirectoryNode transformInfo)
    pub fn new(_transformInfo: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mTransformInfo = transformInfo;
      WpsSecurityInfo {
          mTransformInfo: Some(_transformInfo),
      }
    }

    // Java: void createWpsEncryptionInfo(String infoFilePath)
    pub fn createWpsEncryptionInfo(&mut self, infoFilePath: &str) {
      // Java: File infoFile = new File(infoFilePath);
      let infoFile = std::path::Path::new(infoFilePath);
      // Java: if (infoFile.exists())
      if infoFile.exists() {
         // Java: FileInputStream is = new FileInputStream(infoFile);
         // Java: try
         {
            // Java: this.mTransformInfo.createDocument("WpsEncryptionInfo", is);
            todo!()
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(is);
         }
      }
    }

    // Java: void createWpsSecurityTicket(String guid, String endata)
    pub fn createWpsSecurityTicket(&mut self, guid: &str, endata: &str) {
      // Java: WpsSecurityTicket wpsSecurityTicket = new WpsSecurityTicket(guid, endata);
      // Java: this.mTransformInfo.createDocument("WpsSecurityTicket", wpsSecurityTicket.getInputStream());
      todo!()
    }

}
