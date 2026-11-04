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

// Java: public class DotxChecker extends OOXMLChecker
pub struct DotxChecker {
    // Java field: Boolean mIsDotx
    pub(crate) mIsDotx: Arc<Mutex<Boolean>>,
}

// Java: public class DotxChecker extends OOXMLChecker
impl DotxChecker {
    // Java: DotxChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DotxChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isDotx()
    pub fn isDotx(&self) -> bool {
      // Java: if (null != this.mIsDotx)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsDotx;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsDotx = null != is ? this.hasContentType("application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml", is) : false; return this.mIsDotx;
         // TODO_RUST_CODE
      }
    }

}
