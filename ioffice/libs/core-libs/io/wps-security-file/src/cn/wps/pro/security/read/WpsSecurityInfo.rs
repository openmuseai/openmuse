#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.Entry
// use io_poi_compact::org::apache::poi::poifs::filesystem::Entry;

use super::WpsSecurityTicket::WpsSecurityTicket;

// Java field: String WpsSecurityTicket
const WPSSECURITYTICKET: &str = "WpsSecurityTicket";
// Java field: String WpsEncryptionInfo
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

    // Java: WpsSecurityTicket getWpsSecurityTicket()
    pub fn getWpsSecurityTicket(&self) -> WpsSecurityTicket {
      // Java: Entry securityTicket = this.mTransformInfo.getEntry("WpsSecurityTicket");
      // Java: DocumentInputStream mEncryptionInfo = this.mTransformInfo.createDocumentInputStream(securityTicket);
      // Java: return new WpsSecurityTicket(mEncryptionInfo);
      todo!()
    }

    // Java: DocumentInputStream getWpsEncryptionInfo()
    pub fn getWpsEncryptionInfo(&self) -> Box<dyn std::any::Any> {
      // Java: return this.mTransformInfo.createDocumentInputStream(this.mTransformInfo.getEntry("WpsEncryptionInfo"));
      todo!()
    }

}
