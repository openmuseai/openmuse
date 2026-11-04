// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

// Java: public class Uos3Checker extends Uof3Checker
pub struct Uos3Checker {
    // Java field: Boolean mIsUos3
    pub(crate) mIsUos3: Arc<Mutex<Boolean>>,
}

// Java: public class Uos3Checker extends Uof3Checker
impl Uos3Checker {
    // Java: Uos3Checker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn Uos3Checker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isUos3()
    pub fn isUos3(&self) -> bool {
      // Java: if (null != this.mIsUos3)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsUos3;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getUofInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsUos3 = null != is ? this.hasUof("vnd.uof.spreadsheet", is) : false; return this.mIsUos3;
         // TODO_RUST_CODE
      }
    }

}
