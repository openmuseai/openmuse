#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;

// Java: public class TransformInfo
pub struct TransformInfo {
    // Java field: DirectoryNode mWpsTransform
    mWpsTransform: Option<Box<dyn std::any::Any>>,
    // Java field: DirectoryNode mTransformInfo
    pub(crate) mTransformInfo: Option<Box<dyn std::any::Any>>,
}

// Java: public class TransformInfo
impl TransformInfo {
    // Java: TransformInfo(DirectoryNode wpsTransform)
    pub fn new(wpsTransform: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mWpsTransform = wpsTransform;
      TransformInfo {
          mWpsTransform: Some(wpsTransform),
          mTransformInfo: None,
      }
    }

    // Java: WpsSecurityInfo getWpsSecurityInfo(String key)
    pub fn getWpsSecurityInfo(&mut self, key: &str) -> Option<Box<dyn std::any::Any>> {
      // Java: this.mTransformInfo = (DirectoryNode)this.mWpsTransform.getEntry(key);
      // Java: return this.mTransformInfo != null ? new WpsSecurityInfo(this.mTransformInfo) : null;
      todo!()
    }

}
