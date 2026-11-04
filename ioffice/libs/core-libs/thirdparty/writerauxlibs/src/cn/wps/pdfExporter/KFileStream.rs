// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File; // not available in this crate
// Java: import cn.wps.moffice.kfs.FileOutputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileOutputStream; // not available in this crate
// Java: import java.io.IOException
// (mapped to std::io::Error)
use std::fs::OpenOptions;
use std::io::Write;

// Java: public class KFileStream extends KStream
pub struct KFileStream {
    // Java field: FileOutputStream mWriter
    mWriter: Option<std::fs::File>,
    pub(crate) length: i32,
}

// Java: public class KFileStream extends KStream
#[allow(non_snake_case)]
impl KFileStream {
    // Java: KFileStream(String path)
    pub fn new(path: &str) -> std::io::Result<Self> {
      // Java: super();
      // Java: File file = new File(path);
      let file_path = std::path::Path::new(path);
      // Java: if (!file.exists())
        if !file_path.exists()
      {
         // Java: file.createNewFile();
         let _ = std::fs::File::create(path);
      }
      // Java: this.mWriter = new FileOutputStream(file);
      let writer = OpenOptions::new().write(true).truncate(true).open(path)?;
      Ok(Self { mWriter: Some(writer), length: 0 })
    }

    // Java: void write(byte[] data, int start, int count)
    pub fn write_bytes(&mut self, data: &[u8], start: i32, count: i32) {
      // Java: try
      {
         // Java: this.mWriter.write(data, start, count);
         if let Some(ref mut w) = self.mWriter {
             let s = start as usize;
             let c = count as usize;
             let _ = w.write_all(&data[s..s + c]);
         }
         // Java: this.length += count;
         self.length += count;
      }
      // Java: catch (IOException e)
      {
         // Java: e.printStackTrace();
      }
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: if (this.mWriter != null)
        if self.mWriter.is_some()
      {
         // Java: this.mWriter.close();
         drop(self.mWriter.take());
         // Java: this.mWriter = null;
      }
    }

}
