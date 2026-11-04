#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.RandomAccessFile
// Java: import cn.wps.moffice.kfs.ifs.RafStream
// Java: import cn.wps.moffice.util.FileUtil
// Java: import java.io.BufferedInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStreamReader

use std::any::Any;
use std::io::{self, Read, BufReader};

// Java field: int BUFFERSIZE
pub(crate) const BUFFERSIZE: usize = 2048;

// Java: public class XmlChecker
pub struct XmlChecker {
    // Java field: BufferedInputStream _bufferedInputStream
    _bufferedInputStream: Option<Box<dyn Any>>,
    // Java field: RandomAccessFile mRaf
    mRaf: Option<Box<dyn Any>>,
    // Java field: Boolean _isXml
    pub(crate) _isXml: Option<bool>,
    // Java field: String content
    content: Option<String>,
}

// Java: public class XmlChecker
impl XmlChecker {
    // Java: XmlChecker(BufferedInputStream bufferedInputStream, RandomAccessFile raf)
    pub fn new(bufferedInputStream: Option<Box<dyn Any>>, raf: Option<Box<dyn Any>>) -> Self {
      // Java: super();
      // Java: this._bufferedInputStream = bufferedInputStream;
      // Java: this.mRaf = raf;
      XmlChecker {
          _bufferedInputStream: bufferedInputStream,
          mRaf: raf,
          _isXml: None,
          content: None,
      }
    }

    // Java: boolean isXml03()
    pub fn isXml03(&mut self) -> bool {
      // Java: return !this.isXml() ? false : this.content.contains("<w:wordDocument");
      if !self.isXml() {
          false
      } else {
          self.content.as_ref().map_or(false, |c| c.contains("<w:wordDocument"))
      }
    }

    // Java: boolean isXml()
    pub fn isXml(&mut self) -> bool {
      // Java: if (this._isXml != null)
      if let Some(v) = self._isXml {
         // Java: return this._isXml;
         return v;
      }
      // Java: else if (this._bufferedInputStream == null && this.mRaf == null)
      if self._bufferedInputStream.is_none() && self.mRaf.is_none() {
         // Java: return this._isXml = false;
         self._isXml = Some(false);
         return false;
      }
      // Java: else
      {
         // Java: char[] data = new char[BUFFERSIZE];
         // Java: String strData = null;
         // Java: InputStreamReader reader = null;
         // Java: try
         // Java: reader = null != this.mRaf ? new InputStreamReader(new RafStream(this.mRaf)) : new InputStreamReader(this._bufferedInputStream);
         // Java: int nRead = reader.read(data);
         // Java: if (nRead <= 0)
         // Java: boolean var12 = this._isXml = false;
         // Java: return var12;
         // Java: this.content = new String(data, 0, nRead);
         // Java: catch (IOException e)
         // Java: String msg = e.getMessage();
         // Java: if (null != msg)
         // Java: System.out.println(msg);
         // Java: boolean var6 = this._isXml = false;
         // Java: return var6;
         // Java: finally
         // Java: FileUtil.closeQuietly(reader);

         self.content = None;

         // Java: if (null != this.content && 0 != this.content.length())
         if let Some(ref content) = self.content {
            if !content.is_empty() {
               // Java: if (this.content.contains("<?xml"))
               if content.contains("<?xml") {
                  // Java: return this._isXml = true;
                  self._isXml = Some(true);
                  return true;
               }
               // Java: else
               else {
                  // Java: return this._isXml = false;
                  self._isXml = Some(false);
                  return false;
               }
            }
         }
         // Java: else
         // Java: return this._isXml = false;
         self._isXml = Some(false);
         return false;
      }
    }

}
