#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;

use super::WpsSecurityInfo::WpsSecurityInfo;

// Java field: String WpsSecurityInfo
const WPSSECURITYINFO_NAME: &str = "WpsSecurityInfo";

// Java: public class TransformInfo
pub struct TransformInfo {
    // Java field: DirectoryNode mWpsTransform
    mWpsTransform: Option<Box<dyn std::any::Any>>,
    // Java field: DirectoryNode mTransformInfo
    mTransformInfo: Option<Box<dyn std::any::Any>>,
    // Java field: WpsSecurityInfo wpsSecurityInfo
    wpsSecurityInfo: Option<WpsSecurityInfo>,
}

// Java: public class TransformInfo
impl TransformInfo {
    // Java: TransformInfo(DirectoryNode wpsTransform)
    pub fn new(_wpsTransform: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mWpsTransform = wpsTransform;
      // Java: this.mTransformInfo = this.mWpsTransform.createDirectory(WpsSecurityInfo);
      // Java: this.wpsSecurityInfo = new WpsSecurityInfo(this.mTransformInfo);
      todo!()
    }

    // Java: WpsSecurityInfo getWpsSecurityInfo()
    pub fn getWpsSecurityInfo(&self) -> &Option<WpsSecurityInfo> {
      // Java: return this.wpsSecurityInfo;
      &self.wpsSecurityInfo
    }

}
