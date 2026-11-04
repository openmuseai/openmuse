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

// Java: public class Uop3Checker extends Uof3Checker
pub struct Uop3Checker {
    // Java field: Boolean mIsUop3
    pub(crate) mIsUop3: Arc<Mutex<Boolean>>,
}

// Java: public class Uop3Checker extends Uof3Checker
impl Uop3Checker {
    // Java: Uop3Checker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn Uop3Checker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isUop3()
    pub fn isUop3(&self) -> bool {
      // Java: if (null != this.mIsUop3)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsUop3;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getUofInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsUop3 = null != is ? this.hasUof("vnd.uof.spreadsheet", is) : false; return this.mIsUop3;
         // TODO_RUST_CODE
      }
    }

}
