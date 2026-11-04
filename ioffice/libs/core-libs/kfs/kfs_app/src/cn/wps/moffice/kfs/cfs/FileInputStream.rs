#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut)]

use crate::kfs::tools::Tools::Tools;
use super::CfsCore::*;
use super::File::File;

// Java: public class FileInputStream extends InputStream implements Closeable
#[derive(Default)]
pub struct FileInputStream {
    // Java field:    private int fd;
    fd: i32,
    // Java field:    private final boolean shouldCloseFd;
    shouldCloseFd: bool,
}

// Java: public class FileInputStream extends InputStream implements Closeable
impl FileInputStream {
    // Java: public FileInputStream(File file) throws FileNotFoundException
    pub fn new(file: &File) -> FileInputStream {
        // Java: this.fd = CfsCore.nativeCfsOpen(Tools.string2bytes(file.getPath()), CfsCore.O_RDONLY);
        let fd = CfsCore::nativeCfsOpen(&Tools::string2bytes(file.getPath()), O_RDONLY);
        if fd < 0 { panic!("FileNotFoundException"); }
        FileInputStream { fd, shouldCloseFd: true }
    }

    // Java: public FileInputStream(String path) throws FileNotFoundException
    pub fn new__string(path: String) -> FileInputStream {
        let file = File::new__string(path);
        Self::new(&file)
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        let mut buf = [0u8; 1];
        let n = self.read__bytearr_int_int(&mut buf, 0, 1);
        if n == -1 { -1 } else { (buf[0] as i32) & 255 }
    }

    // Java: public int read(byte[] buffer) throws IOException
    pub fn read__bytearr(&mut self, buffer: &mut [u8]) -> i32 {
        let len = buffer.len() as i32;
        self.read__bytearr_int_int(buffer, 0, len)
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read__bytearr_int_int(&mut self, buffer: &mut [u8], byteOffset: i32, byteCount: i32) -> i32 {
        // Java: return CfsCore.read(this.fd, buffer, byteOffset, byteCount);
        CfsCore::read(self.fd, buffer, byteOffset, byteCount)
    }

    // Java: public long skip(long count) throws IOException
    pub fn skip(&mut self, count: i64) -> i64 {
        if count > 0 {
            // Java: CfsCore.nativeCfsSeek(this.fd, (int)count, CfsCore.SEEK_CUR);
            CfsCore::nativeCfsSeek(self.fd, count, SEEK_CUR);
            count
        } else {
            0
        }
    }

    // Java: public int available() throws IOException
    pub fn available(&mut self) -> i32 {
        // Java: return CfsCore.nativeCfsAvaliable(this.fd);
        CfsCore::nativeCfsAvaliable(self.fd)
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        if self.shouldCloseFd && self.fd >= 0 {
            // Java: CfsCore.nativeCfsClose(this.fd);
            CfsCore::nativeCfsClose(self.fd);
            self.fd = -1;
        }
    }
}
