#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
// use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// use io_poi_compact::org::apache::poi::poifs::filesystem::DocumentInputStream;

use super::TransformMap::TransformMap;
use super::TransformInfo::TransformInfo;
use super::WpsVersion::WpsVersion;

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
    // Java field: DocumentInputStream mVersion
    mVersion: Option<Box<dyn std::any::Any>>,
    // Java field: TransformMap mTransformMap
    mTransformMap: Option<TransformMap>,
    // Java field: DirectoryNode mTransformInfo
    mTransformInfo: Option<Box<dyn std::any::Any>>,
    // Java field: WpsVersion mWpsVersion
    pub(crate) mWpsVersion: Option<WpsVersion>,
}

// Java: public class WpsTransform
impl WpsTransform {
    // Java: WpsTransform(DirectoryNode wpsTransform)
    pub fn new(_wpsTransform: Box<dyn std::any::Any>) -> Self {
      // Java: super();
      // Java: this.mWpsTransform = wpsTransform;
      // Java: this.mVersion = this.readVersion();
      // Java: this.mTransformMap = this.readTransformMap();
      // Java: this.mWpsVersion = new WpsVersion(this.mVersion);
      todo!()
    }

    // Java: DocumentInputStream readVersion()
    fn readVersion(&self) -> Box<dyn std::any::Any> {
      // Java: return this.mWpsTransform.createDocumentInputStream(VERSION);
      todo!()
    }

    // Java: TransformMap readTransformMap()
    fn readTransformMap(&self) -> TransformMap {
      // Java: return new TransformMap(this.mWpsTransform.createDocumentInputStream(TRANSFORMMAP));
      todo!()
    }

    // Java: TransformInfo getTransformInfo()
    pub fn getTransformInfo(&self) -> TransformInfo {
      // Java: return new TransformInfo((DirectoryNode)this.mWpsTransform.getEntry(TRANSFORMINFO));
      todo!()
    }

    // Java: WpsVersion getWpsVersion()
    pub fn getWpsVersion(&self) -> &Option<WpsVersion> {
      // Java: return this.mWpsVersion;
      &self.mWpsVersion
    }

    // Java: TransformMap getTransformMap()
    pub fn getTransformMap(&self) -> &Option<TransformMap> {
      // Java: return this.mTransformMap;
      &self.mTransformMap
    }

}
