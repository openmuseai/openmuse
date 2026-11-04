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

// Java: public class DocmChecker extends OOXMLChecker
pub struct DocmChecker {
    // Java field: Boolean mIsDocm
    pub(crate) mIsDocm: Arc<Mutex<Boolean>>,
}

// Java: public class DocmChecker extends OOXMLChecker
impl DocmChecker {
    // Java: DocmChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DocmChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isDocm()
    pub fn isDocm(&self) -> bool {
      // Java: if (null != this.mIsDocm)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsDocm;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
         // TODO_RUST_CODE
         // Java: this.mIsDocm = null != is ? this.hasContentType("application/vnd.ms-word.document.macroEnabled.main+xml", is) : false; return this.mIsDocm;
         // TODO_RUST_CODE
      }
    }

}
