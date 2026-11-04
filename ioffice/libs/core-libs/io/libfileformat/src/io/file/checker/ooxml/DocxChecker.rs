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

// Java: public class DocxChecker extends OOXMLChecker
pub struct DocxChecker {
    // Java field: String BACKUP_DOCX_FEATURE
    BACKUP_DOCX_FEATURE: String,
    // Java field: Boolean mIsDocx
    pub(crate) mIsDocx: Arc<Mutex<Boolean>>,
    // Java field: DocxNewChecker mDocxNewChecker
    mDocxNewChecker: Option<DocxNewChecker>,
}

// Java: public class DocxChecker extends OOXMLChecker
impl DocxChecker {
    // Java: DocxChecker(File file, RandomAccessFile raf, POIFSFileSystem poifs)
    pub fn DocxChecker(&self, File file, RandomAccessFile raf, POIFSFileSystem poifs) {
      // Java: super(file, raf, poifs);
      // TODO_RUST_CODE
      // Java: this.mDocxNewChecker = new DocxNewChecker(file, raf, poifs);
      // TODO_RUST_CODE
    }

    // Java: boolean isDocx()
    pub fn isDocx(&self) -> bool {
      // Java: if (null != this.mIsDocx)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsDocx;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: boolean docx = this.mDocxNewChecker.isDocx();
         // TODO_RUST_CODE
         // Java: if (docx)
            // TODO_RUST_CODE
         {
            // Java: return true;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: ZipInputStream is = Tools.getContentTypeZipInputStream(this.mFile, this.mRaf);
            // TODO_RUST_CODE
            // Java: return null != is ? this.mIsDocx = this.hasContentType("application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml", is) : this.hasFolder("word");
            // TODO_RUST_CODE
         }
      }
    }

}
