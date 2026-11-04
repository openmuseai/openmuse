#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.mfs.File
use crate::kfs::mfs::File::File;
use crate::kfs::mfs::FileInputStream::FileInputStream;

// Java: public class FileReader extends InputStreamReader
#[derive(Default)]
pub struct FileReader {
    input_stream: FileInputStream,
}

// Java: public class FileReader extends InputStreamReader
impl FileReader {
    // Java: public FileReader(File file) throws FileNotFoundException
    pub fn new(file: &mut File) -> FileReader {
        // Java: super(new FileInputStream(file));
        Self { input_stream: FileInputStream::new__file(file) }
    }

    // Java: public FileReader(String filename) throws FileNotFoundException
    pub fn new__string(filename: String) -> FileReader {
        // Java: super(new FileInputStream(filename));
        Self { input_stream: FileInputStream::new(filename) }
    }

    pub fn read(&mut self) -> i32 { self.input_stream.read() }
    pub fn read_buf(&mut self, buf: &mut [u8]) -> i32 { self.input_stream.read__bytearr(buf) }
    pub fn close(&mut self) { self.input_stream.close(); }
}
