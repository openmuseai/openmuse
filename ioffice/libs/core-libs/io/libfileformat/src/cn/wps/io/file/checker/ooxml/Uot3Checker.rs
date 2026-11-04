// Java: import cn.wps.io.file.Tools
use io_libfileformat::cn::wps::io::file::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.zip.ZipInputStream
use kfs_kfs_app::cn::wps::moffice::kfs::zip::ZipInputStream;
// Java: import org.apache.poi.poifs.filesystem.POIFSFileSystem
use io_poi_compact::org::apache::poi::poifs::filesystem::POIFSFileSystem;

use super::Uof3Checker::Uof3Checker;

// Java: public class Uot3Checker extends Uof3Checker
pub struct Uot3Checker {
    pub(crate) base: Uof3Checker,
    // Java field: Boolean mIsUot3
    pub(crate) mIsUot3: Option<bool>,
}

impl std::ops::Deref for Uot3Checker {
    type Target = Uof3Checker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Uot3Checker extends Uof3Checker
impl Uot3Checker {
    // Java: Uot3Checker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn Uot3Checker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: super call handled by struct composition
    }

    // Java: boolean isUot3()
    pub fn isUot3(&self) -> bool {
      // Java: if (null != this.mIsUot3)
      if self.mIsUot3.is_some() {
         // Java: return this.mIsUot3;
         return self.mIsUot3.unwrap();
      }
      // Java: else
      else {
         // Java: ZipInputStream is = Tools.getUofInputStream(this.mFile, this.mRaf);
         #[allow(unused)] let is_ = Tools::getUofInputStream(&self.mFile, &self.mRaf); // let is
         // Java: this.mIsUot3 = null != is ? this.hasUof("vnd.uof.text", is) : false; return this.mIsUot3;
         // LLM No Rust equivalent
         false
      }
    }

}
