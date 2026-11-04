// Java: import cn.wps.moffice.kfs.File
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::File; // not available in this crate
// Java: import cn.wps.moffice.kfs.FileInputStream
// use kfs_kfs_app::cn::wps::moffice::kfs::mfs::FileInputStream; // not available in this crate
// Java: import java.io.BufferedReader
// Java: import java.io.FileNotFoundException
// Java: import java.io.IOException
use std::io::BufRead;
use std::io::BufReader;
// Java: import java.io.InputStreamReader
// Java: import java.nio.charset.Charset
use std::path::PathBuf;

type BufferedReader = BufReader<std::fs::File>;

// Java: public class FileMgr
pub struct FileMgr {
    // Java field: BufferedReader mReader
    pub(crate) mReader: Option<BufferedReader>,
}

// Java: public class FileMgr
#[allow(non_snake_case)]
impl FileMgr {
    // Java: FileMgr(String path)
    pub fn new(path: &str) -> Self {
      // Java: super();
      let mut mgr = Self { mReader: None };
      // Java: this.init(path, "ISO8859-1");
      mgr.init(path, "ISO8859-1");
      mgr
    }

    // Java: FileMgr(String path, String encoding)
    pub fn new_with_encoding(path: &str, encoding: &str) -> Self {
      // Java: super();
      let mut mgr = Self { mReader: None };
      // Java: this.init(path, encoding);
      mgr.init(path, encoding);
      mgr
    }

    // Java: void init(String path, String encoding)
    pub fn init(&mut self, path: &str, _encoding: &str) {
      // Java: try
      {
         // Java: Charset charset = Charset.forName(encoding);
         // Java: File file = new File(path);
         let file_path = PathBuf::from(path);
         // Java: InputStreamReader fileReader = new InputStreamReader(new FileInputStream(file), charset);
         // Java: this.mReader = new BufferedReader(fileReader);
         match std::fs::File::open(&file_path) {
             Ok(f) => {
                 self.mReader = Some(BufReader::new(f));
             }
             // Java: catch (FileNotFoundException e)
             Err(e) => {
                 // Java: e.printStackTrace();
                 eprintln!("{}", e);
             }
         }
      }
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: if (this.mReader != null)
      if self.mReader.is_some() {
         // Java: try
         {
            // Java: this.mReader.close();
            // Java: catch (IOException e)
            // Java: e.printStackTrace();
            drop(self.mReader.take());
         }
         // Java: this.mReader = null;
         self.mReader = None;
      }
    }

    // Java: String getLine()
    pub fn getLine(&mut self) -> Option<String> {
      // Java: String line = null;
      let mut line: Option<String> = None;
      // Java: try
      {
         // Java: if (this.mReader != null)
         if let Some(reader) = &mut self.mReader {
            // Java: line = this.mReader.readLine();
            let mut buf = String::new();
            match reader.read_line(&mut buf) {
                Ok(0) => {}
                Ok(_) => {
                    if buf.ends_with('\n') {
                        buf.pop();
                        if buf.ends_with('\r') {
                            buf.pop();
                        }
                    }
                    line = Some(buf);
                }
                // Java: catch (IOException e)
                Err(e) => {
                    // Java: e.printStackTrace();
                    eprintln!("{}", e);
                }
            }
         }
      }
      // Java: return line;
      line
    }

}
