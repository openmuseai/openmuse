// Java: import cn.wps.base.assertion.KSAssert
// use util::cn::wps::base::assertion::KSAssert; // not available in this crate
// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File; // not available in this crate
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream; // not available in this crate
// Java: import java.io.IOException
// (mapped to std::io::Error)
use std::io::Read;
use std::path::PathBuf;

// Java: public class KFileReaderStream extends KStream
pub struct KFileReaderStream {
    // Java field: File mFile
    pub(crate) mFile: PathBuf,
    // Java field: FileInputStream mReader
    pub(crate) mReader: Option<std::fs::File>,
    pub(crate) length: i32,
}

// Java: public class KFileReaderStream extends KStream
#[allow(non_snake_case)]
impl KFileReaderStream {
    // Java: KFileReaderStream(String path)
    pub fn new(path: &str) -> std::io::Result<Self> {
      // Java: super();
      // Java: this.mFile = new File(path);
      let file_path = PathBuf::from(path);
      // Java: this.length = (int)this.mFile.length();
      let length = std::fs::metadata(&file_path).map(|m| m.len() as i32).unwrap_or(0);
      Ok(Self { mFile: file_path, mReader: None, length })
    }

    // Java: KFileReaderStream(File path)
    pub fn new_from_path(path: PathBuf) -> std::io::Result<Self> {
      // Java: super();
      // Java: this.mFile = path;
      // Java: this.length = (int)this.mFile.length();
      let length = std::fs::metadata(&path).map(|m| m.len() as i32).unwrap_or(0);
      Ok(Self { mFile: path, mReader: None, length })
    }

    // Java: String getFilePath()
    pub fn getFilePath(&self) -> String {
      // Java: return this.mFile.getAbsolutePath();
      self.mFile.to_string_lossy().to_string()
    }

    // Java: int read(byte[] buf, int offset, int start, int count)
    pub fn read_to_buf(&mut self, buf: &mut [u8], offset: i32, _start: i32, count: i32) -> i32 {
      // Java: try
      {
         // Java: if (this.mReader == null)
            if self.mReader.is_none()
         {
            // Java: this.mReader = new FileInputStream(this.mFile);
            self.mReader = std::fs::File::open(&self.mFile).ok();
         }
         // Java: return this.mReader.read(buf, offset, count);
         if let Some(ref mut reader) = self.mReader {
             let o = offset as usize;
             let c = count as usize;
             match reader.read(&mut buf[o..o + c]) {
                 Ok(n) => return n as i32,
                 Err(_) => return 0,
             }
         }
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
         // Java: return 0;
         return 0;
      }
    }

    // Java: int read(byte[] buffer)
    pub fn read_all(&mut self, _buffer: &mut [u8]) -> i32 {
      // Java: KSAssert.assertTrue(false);
      panic!("KFileReaderStream::read_all should not be called");
      // Java: return 0;
    }

    // Java: byte[] copyData()
    pub fn copyData(&mut self) -> Option<Vec<u8>> {
      // Java: try
      {
         // Java: byte[] buffer = new byte[this.length];
         let mut buffer = vec![0u8; self.length as usize];
         // Java: FileInputStream reader = new FileInputStream(this.mFile);
         if let Ok(mut reader) = std::fs::File::open(&self.mFile) {
         // Java: int len = reader.read(buffer);
             let len = reader.read(&mut buffer).unwrap_or(0);
         // Java: reader.close();
             drop(reader);
         // Java: if (len == buffer.length)
            if len == buffer.len()
         {
            // Java: return buffer;
            return Some(buffer);
         }
         }
      }
      // Java: catch (OutOfMemoryError e)
      {
         // Java: e.printStackTrace();
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
      // Java: return null;
      None
    }

    // Java: void write(byte[] data, int start, int count)
    pub fn write(&mut self, _data: &[u8], _start: i32, _count: i32) {
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: if (this.mReader != null)
        if self.mReader.is_some()
      {
         // Java: this.mReader.close();
         drop(self.mReader.take());
         // Java: this.mReader = null;
      }
      // Java: super.close();
    }

}
