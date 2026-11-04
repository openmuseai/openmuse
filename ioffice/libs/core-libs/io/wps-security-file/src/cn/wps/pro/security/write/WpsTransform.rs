#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unreachable_code)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;

use super::TransformInfo::TransformInfo;

// Java field: String VERSION
const VERSION: &str = "Version";
// Java field: String TRANSFORMMAP
const TRANSFORMMAP: &str = "TransformMap";
// Java field: String TRANSFORMINFO
const TRANSFORMINFO: &str = "TransformInfo";

// Java: public class WpsTransform
pub struct WpsTransform {
    // Java field: DirectoryNode mWpsTransform
    mWpsTransform: Option<Box<dyn std::any::Any>>,
    // Java field: TransformInfo mTransformInfo
    mTransformInfo: Option<TransformInfo>,
}

// Java: public class WpsTransform
impl WpsTransform {
    // Java: WpsTransform(DirectoryNode wpsTransform)
    pub fn new(_wpsTransform: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mWpsTransform = wpsTransform;
      // Java: wpsTransform.createDocument("Version", (new WpsVersion()).getInputStream());
      // Java: wpsTransform.createDocument("TransformMap", (new TransformMap()).getInputStream());
      // Java: this.mTransformInfo = new TransformInfo(this.mWpsTransform.createDirectory("TransformInfo"));
      todo!()
    }

    // Java: TransformInfo getTransformInfo()
    pub fn getTransformInfo(&self) -> &Option<TransformInfo> {
      // Java: return this.mTransformInfo;
      &self.mTransformInfo
    }

}
