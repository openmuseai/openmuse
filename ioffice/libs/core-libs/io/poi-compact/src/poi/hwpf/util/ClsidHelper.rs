// Java: import cn.wps.base.log.IOLog
use util::cn::wps::base::log::IOLog;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import java.io.FileNotFoundException
// TODO_RUST_CODE use ???; // java.io.FileNotFoundException
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import org.apache.poi.hpsf.ClassID
use io_poi_compact::org::apache::poi::hpsf::ClassID;
// Java: import org.apache.poi.hwpf.HWPFDocumentCore
use io_poi_compact::org::apache::poi::hwpf::HWPFDocumentCore;
// Java: import org.apache.poi.poifs.filesystem.DirectoryNode
use io_poi_compact::org::apache::poi::poifs::filesystem::DirectoryNode;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String TAG
pub(crate) const TAG: String = /* TODO */;
// Java field: String DEFAULT_CLSID
pub(crate) const DEFAULT_CLSID: String = /* TODO */;

// Java: public class ClsidHelper
pub struct ClsidHelper {
}

// Java: public class ClsidHelper
impl ClsidHelper {
    // Java: ClsidHelper()
    pub fn ClsidHelper(&self) {
      // Java: super();
      // TODO_RUST_CODE
    }

    // Java: static String getClsid(String path)
    pub fn getClsid(&self, String path) -> String {
      // Java: try
        // TODO_RUST_CODE
      {
         // Java: POIFSFileSystem poifsFileSystem = HWPFDocumentCore.verifyAndBuildPOIFS(new RandomAccessFile(path, "r"));
         // TODO_RUST_CODE
         // Java: DirectoryNode node = poifsFileSystem.getRoot();
         // TODO_RUST_CODE
         // Java: if (null == node)
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
         // Java: ClassID clsid = node.getStorageClsid();
         // TODO_RUST_CODE
         // Java: if (null == clsid)
            // TODO_RUST_CODE
         {
            // Java: return null;
            // TODO_RUST_CODE
         }
         // Java: return clsid.toString();
         // TODO_RUST_CODE
      }
      // Java: catch (FileNotFoundException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.d(TAG, "FileNotFoundException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IOException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.d(TAG, "IOException", e);
         // TODO_RUST_CODE
      }
      // Java: catch (IllegalArgumentException e)
        // TODO_RUST_CODE
      {
         // Java: IOLog.d(TAG, "IOException", e);
         // TODO_RUST_CODE
         // Java: return "00020830-0000-0000-C000-000000000000";
         // TODO_RUST_CODE
      }
      // Java: return null;
      // TODO_RUST_CODE
    }

}
