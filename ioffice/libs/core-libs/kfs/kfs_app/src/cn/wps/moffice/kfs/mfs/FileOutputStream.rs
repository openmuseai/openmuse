#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsFile
use crate::kfs::mfs::core::iface::IMfsFile::*;
// Java: import cn.wps.moffice.kfs.mfs.File
use crate::kfs::mfs::File::File;

// Java: public class FileOutputStream extends OutputStream
pub struct FileOutputStream {
    // Java field:    private long mFilePointer;
    mFilePointer: i64,
    // Java field:    private IMfsFile mMfsFile;
    mMfsFile: Option<Box<dyn IMfsFile>>,
    // Java field:    private final byte[] scratch;
    scratch: Vec<u8>,
    // Java field:    private java.io.FileOutputStream mJdkOutputStream;
    mJdkOutputStream: Option<std::fs::File>,
}

impl Default for FileOutputStream {
    fn default() -> Self {
        Self { mFilePointer: 0, mMfsFile: None, scratch: vec![0u8; 8], mJdkOutputStream: None }
    }
}

// Java: public class FileOutputStream extends OutputStream
impl FileOutputStream {
    // Java: public FileOutputStream(String name) throws FileNotFoundException
    pub fn new(name: String) -> FileOutputStream {
        // Java: this(name != null ? new File(name) : null, false);
        let mut file = File::new__string(name);
        Self::new__file(&mut file, false)
    }

    // Java: public FileOutputStream(String name, boolean append) throws FileNotFoundException
    pub fn new__string_boolean(name: String, append: bool) -> FileOutputStream {
        let mut file = File::new__string(name);
        Self::new__file(&mut file, append)
    }

    // Java: public FileOutputStream(File file) throws FileNotFoundException
    pub fn new__file_noappend(file: &mut File) -> FileOutputStream {
        Self::new__file(file, false)
    }

    // Java: public FileOutputStream(File file, boolean append) throws FileNotFoundException
    pub fn new__file(file: &mut File, append: bool) -> FileOutputStream {
        // Java: if (file.exists0())
        if file.exists0() {
            use std::io::Write;
            let f = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(append)
                .truncate(!append)
                .open(file.getPath())
                .ok();
            FileOutputStream { mFilePointer: 0, mMfsFile: None, scratch: vec![0u8; 8], mJdkOutputStream: f }
        } else {
            // Java: this.mMfsFile = newMfsFile(file);
            let mfs = file.getMfsFile();
            if mfs.is_none() {
                file.createNewFile();
            }
            let mfs = file.getMfsFile();
            FileOutputStream { mFilePointer: 0, mMfsFile: mfs, scratch: vec![0u8; 8], mJdkOutputStream: None }
        }
    }

    // Java: public void write(int oneByte) throws IOException
    pub fn write(&mut self, oneByte: i32) {
        // Java: this.scratch[0] = (byte)(oneByte & 255);
        self.scratch[0] = (oneByte & 255) as u8;
        // Java: this.write(this.scratch, 0, 1);
        let buf = self.scratch.clone();
        self.write__bytearr_int_int(&buf, 0, 1);
    }

    // Java: public void write(byte[] b) throws IOException
    pub fn write__bytearr(&mut self, b: &[u8]) {
        // Java: this.write(b, 0, b.length);
        let len = b.len() as i32;
        self.write__bytearr_int_int(b, 0, len);
    }

    // Java: public void write(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn write__bytearr_int_int(&mut self, buffer: &[u8], byteOffset: i32, byteCount: i32) {
        // Java: if (this.mMfsFile != null)
        if let Some(ref mut mfs) = self.mMfsFile {
            // Java: this.mMfsFile.write(this.mFilePointer, buffer, byteOffset, byteCount);
            mfs.write(self.mFilePointer, buffer.to_vec(), byteOffset, byteCount);
            // Java: this.mFilePointer += (long)byteCount;
            self.mFilePointer += byteCount as i64;
        }
        // Java: else
        else if let Some(ref mut jdk) = self.mJdkOutputStream {
            use std::io::Write;
            let off = byteOffset as usize;
            let cnt = byteCount as usize;
            let _ = jdk.write_all(&buffer[off..off + cnt]);
        }
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        self.mJdkOutputStream = None;
        self.mMfsFile = None;
    }
}
