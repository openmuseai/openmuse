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

// Java: public class Uot3Checker extends Uof3Checker
pub struct Uot3Checker {
    // Java field: Boolean mIsUot3
    pub(crate) mIsUot3: Arc<Mutex<Boolean>>,
}

// Java: public class Uot3Checker extends Uof3Checker
impl Uot3Checker {
    // Java: Uot3Checker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn Uot3Checker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isUot3()
    pub fn isUot3(&self) -> bool {
      // Java: if (null != this.mIsUot3)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsUot3;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getUofInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsUot3 = null != is ? this.hasUof("vnd.uof.text", is) : false; return this.mIsUot3;
         // TODO_RUST_CODE
      }
    }

}
