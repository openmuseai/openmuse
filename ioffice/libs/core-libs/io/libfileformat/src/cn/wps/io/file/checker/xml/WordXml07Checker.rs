#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals, unused_imports)]

// Java: import cn.wps.moffice.kfs.RandomAccessFile
use kfs_kfs_app::cn::wps::moffice::kfs::RandomAccessFile;
// Java: import cn.wps.moffice.kfs.ifs.RafStream
use kfs_kfs_app::cn::wps::moffice::kfs::ifs::RafStream;
// Java: import cn.wps.moffice.util.FileUtil
use util::cn::wps::moffice::util::FileUtil;
// Java: import java.io.BufferedInputStream
use std::io::{BufReader, Read};
// Java: import java.io.IOException
// Java: import java.io.InputStreamReader

// Java: public class WordXml07Checker
pub struct WordXml07Checker {
    // Java field: BufferedInputStream _bufferedInputStream
    _bufferedInputStream: Option<Box<dyn Read>>,
    // Java field: RandomAccessFile mRaf
    mRaf: Option<Box<dyn Read>>,
    // Java field: Boolean _isWordXml07
    pub(crate) _isWordXml07: Option<bool>,
}

// Java field: int BUFFERSIZE
static BUFFERSIZE: std::sync::OnceLock<i32> = std::sync::OnceLock::new();
// Java field: String KEY_WORDS
pub(crate) const KEY_WORDS: &str = "Word.Document";
// Java field: String KEY_XML_07
pub(crate) const KEY_XML_07: &str = "<pkg:package xmlns:pkg=\"http://schemas.microsoft.com/office/2006/xmlPackage\">";

fn get_buffersize() -> i32 {
    *BUFFERSIZE.get_or_init(|| 2048)
}

// Java: public class WordXml07Checker
impl WordXml07Checker {
    // Java: WordXml07Checker(BufferedInputStream bufferedInputStream, RandomAccessFile raf)
    pub fn new(buffered_input_stream: Option<Box<dyn Read>>, raf: Option<Box<dyn Read>>) -> Self {
      // Java: super();
      Self {
          // Java: this._bufferedInputStream = bufferedInputStream;
          _bufferedInputStream: buffered_input_stream,
          // Java: this.mRaf = raf;
          mRaf: raf,
          _isWordXml07: None,
      }
    }

    // Java: boolean isWordXml07()
    pub fn isWordXml07(&mut self) -> bool {
      // Java: if (this._isWordXml07 != null)
      if let Some(cached) = self._isWordXml07
      {
         // Java: return this._isWordXml07;
         return cached;
      }
      // Java: else if (this._bufferedInputStream == null && this.mRaf == null)
      else if self._bufferedInputStream.is_none() && self.mRaf.is_none()
      {
         // Java: return this._isWordXml07 = false;
         self._isWordXml07 = Some(false);
         return false;
      }
      // Java: else
      else
      {
         // Java: char[] data = new char[BUFFERSIZE];
         let buf_size = get_buffersize() as usize;
         let mut data = vec![0u8; buf_size];
         // Java: String strData = null;
         let mut str_data: Option<String> = None;
         // Java: InputStreamReader reader = null;
         // Java: try
         {
            // Java: reader = null != this.mRaf ? new InputStreamReader(new RafStream(this.mRaf)) : new InputStreamReader(this._bufferedInputStream);
            let reader: &mut dyn Read = if let Some(ref mut raf) = self.mRaf {
                raf.as_mut()
            } else if let Some(ref mut bis) = self._bufferedInputStream {
                bis.as_mut()
            } else {
                self._isWordXml07 = Some(false);
                return false;
            };
            // Java: int nRead = reader.read(data);
            match reader.read(&mut data) {
                Ok(n_read) => {
                    // Java: if (nRead <= 0)
                    if n_read == 0
                    {
                       // Java: boolean var13 = this._isWordXml07 = false;
                       self._isWordXml07 = Some(false);
                       // Java: return var13;
                       return false;
                    }
                    // Java: strData = new String(data, 0, nRead);
                    str_data = Some(String::from_utf8_lossy(&data[..n_read]).to_string());
                },
                Err(e) => {
                    // Java: catch (IOException e)
                    {
                        // Java: String msg = e.getMessage();
                        let msg = e.to_string();
                        // Java: if (null != msg)
                        if !msg.is_empty()
                        {
                            // Java: System.out.println(msg);
                            println!("{}", msg);
                        }
                        // Java: boolean var6 = this._isWordXml07 = false;
                        self._isWordXml07 = Some(false);
                        // Java: return var6;
                        return false;
                    }
                },
            };
         }
         // Java: finally
         {
            // Java: FileUtil.closeQuietly(reader);
         }
         // Java: if (null != strData && 0 != strData.length())
         if let Some(ref s) = str_data { if !s.is_empty()
         {
            // Java: if (strData.contains("Word.Document") && strData.contains("<pkg:package xmlns:pkg=\"http://schemas.microsoft.com/office/2006/xmlPackage\">"))
            if s.contains(KEY_WORDS) && s.contains(KEY_XML_07)
            {
               // Java: return this._isWordXml07 = true;
               self._isWordXml07 = Some(true);
               return true;
            }
            // Java: else
            else
            {
               // Java: return this._isWordXml07 = false;
               self._isWordXml07 = Some(false);
               return false;
            }
         }}
         // Java: else
         {
            // Java: return this._isWordXml07 = false;
            self._isWordXml07 = Some(false);
            return false;
         }
      }
    }

}
