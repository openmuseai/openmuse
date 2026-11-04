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

// Java: public class XlamChecker extends OOXMLChecker
pub struct XlamChecker {
    // Java field: Boolean mIsXlam
    pub(crate) mIsXlam: Arc<Mutex<Boolean>>,
}

// Java: public class XlamChecker extends OOXMLChecker
impl XlamChecker {
    // Java: XlamChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XlamChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isXlam()
    pub fn isXlam(&self) -> bool {
      // Java: if (null != this.mIsXlam)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsXlam;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsXlam = null != is ? this.hasContentType("application/vnd.ms-excel.addin.macroEnabled.main+xml", is) : false; return this.mIsXlam;
         // TODO_RUST_CODE
      }
    }

}
