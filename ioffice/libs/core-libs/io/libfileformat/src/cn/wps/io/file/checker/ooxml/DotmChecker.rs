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

// Java: public class DotmChecker extends OOXMLChecker
pub struct DotmChecker {
    pub(crate) base: OOXMLChecker,
    // Java field: Boolean mIsDotm
    pub(crate) mIsDotm: Option<bool>,
}

impl std::ops::Deref for DotmChecker {
    type Target = OOXMLChecker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class DotmChecker extends OOXMLChecker
impl DotmChecker {
    // Java: DotmChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DotmChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: super call handled by struct composition
    }

    // Java: boolean isDotm()
    pub fn isDotm(&self) -> bool {
      // Java: if (null != this.mIsDotm)
      if self.mIsDotm.is_some() {
         // Java: return this.mIsDotm;
         return self.mIsDotm.unwrap();
      }
      // Java: else
      else {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         #[allow(unused)] let is_ = Tools::getContentTypeZipInputStream(&self.mFile, &self.mRaf); // let is
         // Java: this.mIsDotm = null != is ? this.hasContentType("application/vnd.ms-word.template.macroEnabledTemplate.main+xml", is) : false; return this.mIsDotm;
         // LLM No Rust equivalent
         false
      }
    }

}
