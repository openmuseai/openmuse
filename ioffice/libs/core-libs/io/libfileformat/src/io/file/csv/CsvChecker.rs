// Java: import cn.wps.io.file.FileFormatEnum
use util::cn::wps::io::file::FileFormatEnum;
// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;

// Java: public class CsvChecker
pub struct CsvChecker {
    // Java field: Boolean mIsCsv
    pub(crate) mIsCsv: Arc<Mutex<Boolean>>,
    // Java field: File mFile
    mFile: Option<File>,
    // Java field: RandomAccessFile mRaf
    mRaf: RandomAccessFile,
}

// Java: public class CsvChecker
impl CsvChecker {
    // Java: CsvChecker(File file, RandomAccessFile raf)
    pub fn CsvChecker(&self, File file, RandomAccessFile raf) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFile = file;
      // TODO_RUST_CODE
      // Java: this.mRaf = raf;
      // TODO_RUST_CODE
    }

    // Java: boolean isCsv()
    pub fn isCsv(&self) -> bool {
      // Java: if (null != this.mIsCsv)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsCsv;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: String ext = null != this.mRaf ? Tools.getLowerCaseExtension(this.mRaf.getFileName()) : Tools.getLowerCaseExtension(this.mFile.getAbsolutePath());
         // TODO_RUST_CODE
         // Java: if (null == ext)
            // TODO_RUST_CODE
         {
            // Java: return false;
            // TODO_RUST_CODE
         }
         // Java: else
            // TODO_RUST_CODE
         {
            // Java: this.mIsCsv = ext.equals(FileFormatEnum.CSV.getExt());
            // TODO_RUST_CODE
            // Java: return this.mIsCsv;
            // TODO_RUST_CODE
         }
      }
    }

}
