#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use super::File::File;
use super::FileOutputStream::FileOutputStream;

// Java: public class FileWriter extends OutputStreamWriter
#[derive(Default)]
pub struct FileWriter {
    output_stream: FileOutputStream,
}

// Java: public class FileWriter extends OutputStreamWriter
impl FileWriter {
    // Java: public FileWriter(File file) throws IOException
    pub fn new(file: &File) -> FileWriter {
        Self { output_stream: FileOutputStream::new(file) }
    }

    // Java: public FileWriter(File file, boolean append) throws IOException
    pub fn new__file_boolean(file: &File, append: bool) -> FileWriter {
        Self { output_stream: FileOutputStream::new__file_boolean(file, append) }
    }

    // Java: public FileWriter(String filename) throws IOException
    pub fn new__string(filename: String) -> FileWriter {
        Self { output_stream: FileOutputStream::new__string(filename) }
    }

    pub fn write(&mut self, b: i32) { self.output_stream.write(b); }
    pub fn write_buf(&mut self, buf: &[u8]) { self.output_stream.write__bytearr(buf); }
    pub fn close(&mut self) { self.output_stream.close(); }
}
