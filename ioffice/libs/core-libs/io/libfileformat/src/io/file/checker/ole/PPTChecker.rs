// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_PowerPoint_Document
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_PowerPoint_Document: &str = "";

// Java: public class PPTChecker extends OleFileChecker
pub struct PPTChecker {
    // Java field: Boolean _isPPT
    pub(crate) _isPPT: Arc<Mutex<Boolean>>,
}

// Java: public class PPTChecker extends OleFileChecker
impl PPTChecker {
    // Java: PPTChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn PPTChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isPPT()
    pub fn isPPT(&self) -> bool {
      // Java: if (null != this._isPPT)
        // TODO_RUST_CODE
      {
         // Java: return this._isPPT;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this._isPPT = this.hasEntry("PowerPoint Document");
         // TODO_RUST_CODE
         // Java: return this._isPPT;
         // TODO_RUST_CODE
      }
    }

}
