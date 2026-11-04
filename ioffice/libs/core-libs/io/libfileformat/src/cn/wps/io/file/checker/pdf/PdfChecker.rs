#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_imports)]

// Java: import cn.wps.base.log.Log
// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.kfs.ifs.RafStream
// Java: import cn.wps.util.IOUtils
// Java: import java.io.IOException
// Java: import java.io.PushbackInputStream

// Java: public class PdfChecker
pub struct PdfChecker {
    // Java field: PushbackInputStream _pushbackInputStream
    _pushbackInputStream: Option<Vec<u8>>,
    // Java field: RandomAccessFile mRaf
    mRaf: Option<Vec<u8>>,
    // Java field: Boolean _isPdf
    pub(crate) _isPdf: Option<bool>,
}

// Java field: String _header_signature
pub(crate) const _header_signature: &str = "%PDF";
// Java field: int PDF_CHECK_SIZE
pub(crate) const PDF_CHECK_SIZE: i32 = 512;
// Java field: String TAG
pub(crate) const TAG: &str = "PdfChecker";

// Java: public class PdfChecker
impl PdfChecker {
    // Java: PdfChecker(PushbackInputStream pushbackInputStream, RandomAccessFile raf)
    pub fn new(data: Option<Vec<u8>>) -> Self {
      // Java: super();
      // Java: this._pushbackInputStream = pushbackInputStream;
      // Java: this.mRaf = raf;
      Self {
          _pushbackInputStream: data.clone(),
          mRaf: data,
          _isPdf: None,
      }
    }

    // Java: boolean isPdf()
    pub fn isPdf(&mut self) -> bool {
      // Java: if (null != this._isPdf)
      if let Some(cached) = self._isPdf
      {
         // Java: return this._isPdf;
         return cached;
      }
      // Java: else
      else
      {
         // Java: byte[] checkBlock = new byte[512];
         let mut checkBlock = vec![0u8; 512];
         // Java: int searchRet = -1;
         let mut searchRet: i32 = -1;
         // Java: try
         {
            // Java: if (null != this.mRaf)
            if let Some(ref data) = self.mRaf
            {
               // Java: (new RafStream(this.mRaf)).read(checkBlock);
               let len = data.len().min(512);
               checkBlock[..len].copy_from_slice(&data[..len]);
            }
            // Java: else
            else
            {
               // Java: IOUtils.readFully(this._pushbackInputStream, checkBlock);
               if let Some(ref data) = self._pushbackInputStream {
                   let len = data.len().min(512);
                   checkBlock[..len].copy_from_slice(&data[..len]);
               }
            }
            // Java: String checkBlockStr = new String(checkBlock);
            let checkBlockStr = String::from_utf8_lossy(&checkBlock);
            // Java: searchRet = checkBlockStr.indexOf("%PDF");
            searchRet = checkBlockStr.find("%PDF").map_or(-1, |i| i as i32);
         }
         // Java: catch (IOException e)
         {
            // Java: Log.e(TAG, "IOException: ", e);
         }
         // Java: return this._isPdf = -1 != searchRet;
         let result = searchRet != -1;
         self._isPdf = Some(result);
         return result;
      }
    }

}
