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

// Java: public class PpsxChecker extends OOXMLChecker
pub struct PpsxChecker {
    // Java field: Boolean mIsPpsx
    pub(crate) mIsPpsx: Arc<Mutex<Boolean>>,
}

// Java: public class PpsxChecker extends OOXMLChecker
impl PpsxChecker {
    // Java: PpsxChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn PpsxChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isPpsx()
    pub fn isPpsx(&self) -> bool {
      // Java: if (null != this.mIsPpsx)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsPpsx;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsPpsx = null != is ? this.hasContentType("application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml", is) : false; return this.mIsPpsx;
         // TODO_RUST_CODE
      }
    }

}
