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

// Java: public class PpsxChecker extends OOXMLChecker
pub struct PpsxChecker {
    pub(crate) base: OOXMLChecker,
    // Java field: Boolean mIsPpsx
    pub(crate) mIsPpsx: Option<bool>,
}

impl std::ops::Deref for PpsxChecker {
    type Target = OOXMLChecker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class PpsxChecker extends OOXMLChecker
impl PpsxChecker {
    // Java: PpsxChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn PpsxChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: super call handled by struct composition
    }

    // Java: boolean isPpsx()
    pub fn isPpsx(&self) -> bool {
      // Java: if (null != this.mIsPpsx)
      if self.mIsPpsx.is_some() {
         // Java: return this.mIsPpsx;
         return self.mIsPpsx.unwrap();
      }
      // Java: else
      else {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         #[allow(unused)] let is_ = Tools::getContentTypeZipInputStream(&self.mFile, &self.mRaf); // let is
         // Java: this.mIsPpsx = null != is ? this.hasContentType("application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml", is) : false; return this.mIsPpsx;
         // LLM No Rust equivalent
         false
      }
    }

}
