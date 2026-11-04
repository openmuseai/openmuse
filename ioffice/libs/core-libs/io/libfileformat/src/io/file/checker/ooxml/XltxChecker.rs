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

// Java: public class XltxChecker extends OOXMLChecker
pub struct XltxChecker {
    // Java field: Boolean mIsXltx
    pub(crate) mIsXltx: Arc<Mutex<Boolean>>,
}

// Java: public class XltxChecker extends OOXMLChecker
impl XltxChecker {
    // Java: XltxChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn XltxChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isXltx()
    pub fn isXltx(&self) -> bool {
      // Java: if (null != this.mIsXltx)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsXltx;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsXltx = null != is ? this.hasContentType("application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml", is) : false; return this.mIsXltx;
         // TODO_RUST_CODE
      }
    }

}
