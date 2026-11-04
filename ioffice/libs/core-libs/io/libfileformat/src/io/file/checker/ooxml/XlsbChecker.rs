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

// Java: public class XlsbChecker extends OOXMLChecker
pub struct XlsbChecker {
    // Java field: Boolean mIsXlsb
    pub(crate) mIsXlsb: Arc<Mutex<Boolean>>,
}

// Java: public class XlsbChecker extends OOXMLChecker
impl XlsbChecker {
    // Java: XlsbChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XlsbChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isXlsb()
    pub fn isXlsb(&self) -> bool {
      // Java: if (null != this.mIsXlsb)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsXlsb;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsXlsb = null != is ? this.hasContentType("application/vnd.ms-excel.sheet.binary.macroEnabled.main", is) : false; return this.mIsXlsb;
         // TODO_RUST_CODE
      }
    }

}
