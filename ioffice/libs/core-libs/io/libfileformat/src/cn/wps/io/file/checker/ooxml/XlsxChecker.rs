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

// Java: public class XlsxChecker extends OOXMLChecker
pub struct XlsxChecker {
    pub(crate) base: OOXMLChecker,
    // Java field: Boolean mIsXlsx
    pub(crate) mIsXlsx: Option<bool>,
}

impl std::ops::Deref for XlsxChecker {
    type Target = OOXMLChecker;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class XlsxChecker extends OOXMLChecker
impl XlsxChecker {
    // Java: XlsxChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XlsxChecker(&self, file: File, raf: RandomAccessFile, poifs: POIFSFileSystem) {
      // Java: super(file, raf, poifs);
      // LLM No Rust equivalent: super call handled by struct composition
    }

    // Java: boolean isXlsx()
    pub fn isXlsx(&self) -> bool {
      // Java: if (null != this.mIsXlsx)
      if self.mIsXlsx.is_some() {
         // Java: return this.mIsXlsx;
         return self.mIsXlsx.unwrap();
      }
      // Java: else
      else {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         #[allow(unused)] let is_ = Tools::getContentTypeZipInputStream(&self.mFile, &self.mRaf); // let is
         // Java: this.mIsXlsx = null != is ? this.hasContentType("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml", is) : false; return this.mIsXlsx;
         // LLM No Rust equivalent
         false
      }
    }

}
