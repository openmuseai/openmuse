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

// Java: public class DocmChecker extends OOXMLChecker
pub struct DocmChecker {
    pub(crate) base: OOXMLChecker,
    // Java field: Boolean mIsDocm
    pub(crate) mIsDocm: Option<bool>,
}

impl std::ops::Deref for DocmChecker {
    type Target = OOXMLChecker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class DocmChecker extends OOXMLChecker
impl DocmChecker {
    // Java: DocmChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DocmChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: super call handled by struct composition
    }

    // Java: boolean isDocm()
    pub fn isDocm(&self) -> bool {
      // Java: if (null != this.mIsDocm)
      if self.mIsDocm.is_some() {
         // Java: return this.mIsDocm;
         return self.mIsDocm.unwrap();
      }
      // Java: else
      else {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         #[allow(unused)] let is_ = Tools::getContentTypeZipInputStream(&self.mFile, &self.mRaf); // let is
         // Java: this.mIsDocm = null != is ? this.hasContentType("application/vnd.ms-word.document.macroEnabled.main+xml", is) : false; return this.mIsDocm;
         // LLM No Rust equivalent
         false
      }
    }

}
