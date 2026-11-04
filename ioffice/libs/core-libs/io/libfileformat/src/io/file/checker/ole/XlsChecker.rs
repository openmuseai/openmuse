// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java field: String OLE_DIRECTORY_ENTRY_ELENAME_WORKBOOK
pub(crate) const OLE_DIRECTORY_ENTRY_ELENAME_WORKBOOK: &str = "";

// Java: public class XlsChecker extends OleFileChecker
pub struct XlsChecker {
    // Java field: Boolean _isXls
    pub(crate) _isXls: Arc<Mutex<Boolean>>,
}

// Java: public class XlsChecker extends OleFileChecker
impl XlsChecker {
    // Java: XlsChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XlsChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isXls()
    pub fn isXls(&self) -> bool {
      // Java: if (null != this._isXls)
        // TODO_RUST_CODE
      {
         // Java: return this._isXls;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: this._isXls = this.hasEntry("Workbook");
         // TODO_RUST_CODE
         // Java: return this._isXls;
         // TODO_RUST_CODE
      }
    }

}
