#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.moffice.kfs.mfs.File
use crate::kfs::mfs::File::File;
use crate::kfs::mfs::FileOutputStream::FileOutputStream;

// Java: public class FileWriter extends OutputStreamWriter
#[derive(Default)]
pub struct FileWriter {
    output_stream: FileOutputStream,
}

// Java: public class FileWriter extends OutputStreamWriter
impl FileWriter {
    // Java: public FileWriter(File file) throws IOException
    pub fn new(file: &mut File) -> FileWriter {
        // Java: super(new FileOutputStream(file));
        Self { output_stream: FileOutputStream::new__file_noappend(file) }
    }

    // Java: public FileWriter(File file, boolean append) throws IOException
    pub fn new__file_boolean(file: &mut File, append: bool) -> FileWriter {
        // Java: super(new FileOutputStream(file, append));
        Self { output_stream: FileOutputStream::new__file(file, append) }
    }

    // Java: public FileWriter(String filename) throws IOException
    pub fn new__string(filename: String) -> FileWriter {
        // Java: super(new FileOutputStream(new File(filename)));
        let mut file = File::new__string(filename);
        Self { output_stream: FileOutputStream::new__file_noappend(&mut file) }
    }

    // Java: public FileWriter(String filename, boolean append) throws IOException
    pub fn new__string_boolean(filename: String, append: bool) -> FileWriter {
        // Java: super(new FileOutputStream(filename, append));
        Self { output_stream: FileOutputStream::new__string_boolean(filename, append) }
    }

    pub fn write(&mut self, b: i32) { self.output_stream.write(b); }
    pub fn write_buf(&mut self, buf: &[u8]) { self.output_stream.write__bytearr(buf); }
    pub fn close(&mut self) { self.output_stream.close(); }
}
