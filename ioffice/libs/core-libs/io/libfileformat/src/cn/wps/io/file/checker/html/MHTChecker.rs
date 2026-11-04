// Java: import cn.wps.moffice.kfs.RandomAccessFile
// use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile; // not yet available
// Java: import cn.wps.moffice.kfs.ifs.RafStream
// use kfs_kfs_app::cn::wps::moffice::kfs::ifs::RafStream; // not yet available
// Java: import cn.wps.moffice.util.FileUtil
// use util::cn::wps::moffice::util::FileUtil; // not yet available
// Java: import cn.wps.moffice.util.KSLog
// use util::cn::wps::moffice::util::KSLog; // not yet available
// Java: import java.io.BufferedInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStreamReader

use std::io::Read;

// Java field: String TAG
pub(crate) const TAG: &str = "MHTChecker";

// Java: public class MHTChecker
pub struct MHTChecker {
    // Java field: RandomAccessFile mRaf
    // mRaf: Option<RandomAccessFile>, // TODO: RandomAccessFile not yet available
    // Java field: BufferedInputStream mBufferedInputStream
    // mBufferedInputStream: Option<BufferedInputStream>, // TODO: BufferedInputStream not yet available
    // Java field: Boolean _isMHT
    pub(crate) _isMHT: Option<bool>,
    data: Option<Vec<u8>>,
}

// Java: public class MHTChecker
impl MHTChecker {
    // Java: MHTChecker(BufferedInputStream bufferedInputStream, RandomAccessFile raf)
    pub fn new(data: Option<Vec<u8>>) -> Self {
      // Java: super();
      // Java: this.mBufferedInputStream = bufferedInputStream;
      // Java: this.mRaf = raf;
      Self {
          _isMHT: None,
          data,
      }
    }

    // Java: boolean isMHT()
    pub fn isMHT(&mut self) -> bool {
      // Java: if (null != this._isMHT)
      if let Some(cached) = self._isMHT
      {
         // Java: return this._isMHT;
         return cached;
      }
      // Java: else if (this.mBufferedInputStream == null && this.mRaf == null)
      else if self.data.is_none()
      {
         // Java: return this._isMHT = false;
         self._isMHT = Some(false);
         return false;
      }
      // Java: else
      else
      {
         // Java: boolean hasMimeVersion = false;
         let mut hasMimeVersion = false;
         // Java: boolean hasContentType = false;
         let mut hasContentType = false;
         // Java: boolean hasBoundary = false;
         let mut hasBoundary = false;
         // Java: String line = this.read();
         let line = self.read_content();
         // Java: if (null == line)
         if line.is_none()
         {
            // Java: return this._isMHT = false;
            self._isMHT = Some(false);
            return false;
         }
         // Java: else
         else
         {
            // Java: line = line.toUpperCase();
            let line = line.unwrap().to_uppercase();
            // Java: if (line.contains("MIME-VERSION:"))
            if line.contains("MIME-VERSION:")
            {
               // Java: hasMimeVersion = true;
               hasMimeVersion = true;
            }
            // Java: if (line.contains("CONTENT-TYPE:"))
            if line.contains("CONTENT-TYPE:")
            {
               // Java: hasContentType = true;
               hasContentType = true;
            }
            // Java: if (line.contains("BOUNDARY="))
            if line.contains("BOUNDARY=")
            {
               // Java: hasBoundary = true;
               hasBoundary = true;
            }
            // Java: this._isMHT = (hasContentType || hasBoundary) && hasMimeVersion; return this._isMHT;
            let result = (hasContentType || hasBoundary) && hasMimeVersion;
            self._isMHT = Some(result);
            return result;
         }
      }
    }

    // Java: String read()
    fn read_content(&self) -> Option<String> {
      // Java: InputStreamReader reader = null;
      // Java: char[] buff = new char[1024];
      // Java: String var10;
      // Java: try
      if let Some(ref data) = self.data {
         // Java: reader = null != this.mRaf ? new InputStreamReader(new RafStream(this.mRaf)) : new InputStreamReader(this.mBufferedInputStream);
         // Java: int n = reader.read(buff);
         let n = data.len().min(1024);
         // Java: if (-1 != n)
         if n > 0
         {
            // Java: var10 = new String(buff, 0, n);
            // Java: return var10;
            return String::from_utf8(data[..n].to_vec()).ok();
         }
         // Java: var10 = null;
         None
      } else {
         // Java: catch (IOException e)
         // Java: KSLog.e(TAG, "IOException", e);
         // Java: var10 = null;
         // Java: return var10;
         None
      }
      // Java: finally
      // Java: FileUtil.closeQuietly(reader);
      // Java: return var10;
    }

}
