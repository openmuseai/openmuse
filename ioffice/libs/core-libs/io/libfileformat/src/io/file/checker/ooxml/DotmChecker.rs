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

// Java: public class DotmChecker extends OOXMLChecker
pub struct DotmChecker {
    // Java field: Boolean mIsDotm
    pub(crate) mIsDotm: Arc<Mutex<Boolean>>,
}

// Java: public class DotmChecker extends OOXMLChecker
impl DotmChecker {
    // Java: DotmChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DotmChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isDotm()
    pub fn isDotm(&self) -> bool {
      // Java: if (null != this.mIsDotm)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsDotm;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsDotm = null != is ? this.hasContentType("application/vnd.ms-word.template.macroEnabledTemplate.main+xml", is) : false; return this.mIsDotm;
         // TODO_RUST_CODE
      }
    }

}
