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

// Java: public class XltmChecker extends OOXMLChecker
pub struct XltmChecker {
    // Java field: Boolean mIsXltm
    pub(crate) mIsXltm: Arc<Mutex<Boolean>>,
}

// Java: public class XltmChecker extends OOXMLChecker
impl XltmChecker {
    // Java: XltmChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XltmChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isXltm()
    pub fn isXltm(&self) -> bool {
      // Java: if (null != this.mIsXltm)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsXltm;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsXltm = null != is ? this.hasContentType("application/vnd.ms-excel.template.macroEnabled.main+xml", is) : false; return this.mIsXltm;
         // TODO_RUST_CODE
      }
    }

}
