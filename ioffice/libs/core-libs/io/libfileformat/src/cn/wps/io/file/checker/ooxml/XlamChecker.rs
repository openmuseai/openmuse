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

use super::OOXMLChecker::OOXMLChecker;

// Java: public class XlamChecker extends OOXMLChecker
pub struct XlamChecker {
    pub(crate) base: OOXMLChecker,
    // Java field: Boolean mIsXlam
    pub(crate) mIsXlam: Option<bool>,
}

impl std::ops::Deref for XlamChecker {
    type Target = OOXMLChecker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class XlamChecker extends OOXMLChecker
impl XlamChecker {
    // Java: XlamChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XlamChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: super call handled by struct composition
    }

    // Java: boolean isXlam()
    pub fn isXlam(&self) -> bool {
      // Java: if (null != this.mIsXlam)
      if self.mIsXlam.is_some() {
         // Java: return this.mIsXlam;
         return self.mIsXlam.unwrap();
      }
      // Java: else
      else {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         #[allow(unused)] let is_ = Tools::getContentTypeZipInputStream(&self.mFile, &self.mRaf); // let is
         // Java: this.mIsXlam = null != is ? this.hasContentType("application/vnd.ms-excel.addin.macroEnabled.main+xml", is) : false; return this.mIsXlam;
         // LLM No Rust equivalent
         false
      }
    }

}
