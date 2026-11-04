#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

use crate::kfs::tools::Tools::Tools;
use super::CfsCore::*;
use super::File::File;

// Java: public class FileOutputStream extends OutputStream implements Closeable
pub struct FileOutputStream {
    // Java field:    private final int fd;
    fd: i32,
    // Java field:    private final boolean shouldCloseFd;
    shouldCloseFd: bool,
    // Java field:    private final int mode;
    mode: i32,
}

impl Default for FileOutputStream {
    fn default() -> Self { Self { fd: -1, shouldCloseFd: true, mode: 0 } }
}

// Java: public class FileOutputStream extends OutputStream implements Closeable
impl FileOutputStream {
    // Java: public FileOutputStream(java.io.File file) throws FileNotFoundException
    pub fn new(file: &File) -> FileOutputStream {
        Self::new__file_boolean(file, false)
    }

    // Java: public FileOutputStream(java.io.File file, boolean append) throws FileNotFoundException
    pub fn new__file_boolean(file: &File, append: bool) -> FileOutputStream {
        // Java: int flags = CfsCore.O_WRONLY | CfsCore.O_CREAT;
        let mut flags = O_WRONLY | O_CREAT;
        // Java: if (append) flags |= CfsCore.O_APPEND; else flags |= CfsCore.O_TRUNC;
        if append { flags |= O_APPEND; } else { flags |= O_TRUNC; }
        // Java: this.fd = CfsCore.nativeCfsOpen(Tools.string2bytes(file.getPath()), flags);
        let fd = CfsCore::nativeCfsOpen(&Tools::string2bytes(file.getPath()), flags);
        if fd < 0 { panic!("FileNotFoundException"); }
        FileOutputStream { fd, shouldCloseFd: true, mode: flags }
    }

    // Java: public FileOutputStream(String name) throws FileNotFoundException
    pub fn new__string(name: String) -> FileOutputStream {
        let file = File::new__string(name);
        Self::new(&file)
    }

    // Java: public FileOutputStream(String name, boolean append) throws FileNotFoundException
    pub fn new__string_boolean(name: String, append: bool) -> FileOutputStream {
        let file = File::new__string(name);
        Self::new__file_boolean(&file, append)
    }

    // Java: public void write(int oneByte) throws IOException
    pub fn write(&mut self, oneByte: i32) {
        let buf = [(oneByte & 255) as u8];
        self.write__bytearr_int_int(&buf, 0, 1);
    }

    // Java: public void write(byte[] b) throws IOException
    pub fn write__bytearr(&mut self, b: &[u8]) {
        let len = b.len() as i32;
        self.write__bytearr_int_int(b, 0, len);
    }

    // Java: public void write(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn write__bytearr_int_int(&mut self, buffer: &[u8], byteOffset: i32, byteCount: i32) {
        // Java: CfsCore.write(this.fd, buffer, byteOffset, byteCount);
        CfsCore::write(self.fd, buffer, byteOffset, byteCount);
    }

    // Java: public void flush() throws IOException
    pub fn flush(&mut self) {
        // Java: CfsCore.nativeCfsSync(this.fd);
        CfsCore::nativeCfsSync(self.fd);
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        if self.shouldCloseFd && self.fd >= 0 {
            // Java: CfsCore.nativeCfsClose(this.fd);
            CfsCore::nativeCfsClose(self.fd);
        }
    }
}
