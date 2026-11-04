// Java: import cn.wps.io.file.Tools
use kfs_kfs_app::cn::wps::moffice::kfs::tools::Tools;
// Java: import cn.wps.moffice.kfs.File
use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File;
// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.ifs.RafStream
use kfs_kfs_app::cn::wps::moffice::kfs::ifs::RafStream;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import cn.wps.moffice.util.KSLog
use util::cn::wps::moffice::util::KSLog;
// Java: import java.io.IOException
// TODO_RUST_CODE use ???; // java.io.IOException
// Java: import java.io.InputStream
// TODO_RUST_CODE use ???; // java.io.InputStream

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: int BUFFER_SIZE
pub(crate) const BUFFER_SIZE: i32 = 0;

// Java: public class RtfChecker
pub struct RtfChecker {
    // Java field: char[] RTF_FEATURE
    RTF_FEATURE: Vec<char>,
    // Java field: Boolean mIsRtf
    pub(crate) mIsRtf: Arc<Mutex<Boolean>>,
    // Java field: File mFile
    pub(crate) mFile: Arc<Mutex<File>>,
    // Java field: RandomAccessFile mRaf
    mRaf: RandomAccessFile,
}

// Java: public class RtfChecker
impl RtfChecker {
    // Java: RtfChecker(File file, RandomAccessFile raf)
    pub fn RtfChecker(&self, File file, RandomAccessFile raf) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.mFile = file;
      // TODO_RUST_CODE
      // Java: this.mRaf = raf;
      // TODO_RUST_CODE
    }

    // Java: boolean isRtf()
    pub fn isRtf(&self) -> bool {
      // Java: if (null != this.mIsRtf)
        // TODO_RUST_CODE
      {
         // Java: return this.mIsRtf;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: InputStream fis = null;
         // TODO_RUST_CODE
         // Java: byte[] buff = new byte[20];
         // TODO_RUST_CODE
         // Java: int pos = 0;
         // TODO_RUST_CODE
         // Java: int n;
         // TODO_RUST_CODE
         // Java: try
            // TODO_RUST_CODE
         {
            // Java: fis = (InputStream)(null != this.mRaf ? new RafStream(this.mRaf) : Tools.createFileInputStream(this.mFile));
            // TODO_RUST_CODE
            // Java: if (null != fis)
                // TODO_RUST_CODE
            {
               // Java: n = fis.read(buff);
               // TODO_RUST_CODE
               // Java: for(int i = 0; i < n; ++i)
                    // TODO_RUST_CODE
               {
                  // Java: if (buff[i] == this.RTF_FEATURE[pos])
                        // TODO_RUST_CODE
                  {
                     // Java: ++pos;
                     // TODO_RUST_CODE
                  }
                  // Java: if (pos >= 5)
                        // TODO_RUST_CODE
                  {
                     // Java: break;
                     // TODO_RUST_CODE
                  }
               }
               // Java: if (5 == pos)
                    // TODO_RUST_CODE
               {
                  // Java: boolean var14 = this.mIsRtf = true;
                  // TODO_RUST_CODE
                  // Java: return var14;
                  // TODO_RUST_CODE
               }
               // Java: boolean var13 = this.mIsRtf = false;
               // TODO_RUST_CODE
               // Java: return var13;
               // TODO_RUST_CODE
            }
            // Java: this.mIsRtf = false; n = 0;
            // TODO_RUST_CODE
         }
         // Java: catch (IOException e)
            // TODO_RUST_CODE
         {
            // Java: KSLog.e(TAG, "IOExecption", e);
            // TODO_RUST_CODE
            // Java: boolean i = this.mIsRtf = false;
            // TODO_RUST_CODE
            // Java: return i;
            // TODO_RUST_CODE
         }
         // Java: finally
            // TODO_RUST_CODE
         {
            // Java: FileUtil.closeQuietly(fis);
            // TODO_RUST_CODE
         }
         // Java: return this.mIsRtf;
         // TODO_RUST_CODE
      }
    }

}
