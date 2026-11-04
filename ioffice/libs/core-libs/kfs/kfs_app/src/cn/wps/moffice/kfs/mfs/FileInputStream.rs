#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java: import cn.wps.moffice.kfs.mfs.core.iface.IMfsFile
use crate::kfs::mfs::core::iface::IMfsFile::*;
// Java: import cn.wps.moffice.kfs.mfs.File
use crate::kfs::mfs::File::File;

// Java: public class FileInputStream extends InputStream
#[derive(Default)]
pub struct FileInputStream {
    // Java field:    private IMfsFile mMfsFile;
    mMfsFile: Option<Box<dyn IMfsFile>>,
    // Java field:    private long mFilePointer;
    mFilePointer: i64,
    // Java field:    private java.io.FileInputStream mSdkInputStream;
    mSdkInputStream: Option<std::fs::File>,
}

// Java: public class FileInputStream extends InputStream
impl FileInputStream {
    // Java: public FileInputStream(String name) throws FileNotFoundException
    pub fn new(name: String) -> FileInputStream {
        // Java: this(name != null ? new File(name) : null);
        let mut file = File::new__string(name);
        Self::new__file(&mut file)
    }

    /// Compatibility alias for generated call sites.
    pub fn from_name(name: impl Into<String>) -> Result<FileInputStream, String> {
        Ok(Self::new(name.into()))
    }

    // Java: public FileInputStream(File file) throws FileNotFoundException
    pub fn new__file(file: &mut File) -> FileInputStream {
        // Java: if (file.exists0())
        if file.exists0() {
            // Java: this.mSdkInputStream = new java.io.FileInputStream(file);
            let sdk = std::fs::File::open(file.getPath()).ok();
            FileInputStream { mMfsFile: None, mFilePointer: 0, mSdkInputStream: sdk }
        } else {
            // Java: this.mFilePointer = 0L;
            // Java: this.mMfsFile = file.getMfsFile();
            let mfs = file.getMfsFile();
            // Java: if (null == this.mMfsFile) throw new FileNotFoundException();
            if mfs.is_none() {
                panic!("FileNotFoundException");
            }
            FileInputStream { mMfsFile: mfs, mFilePointer: 0, mSdkInputStream: None }
        }
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        // Java: byte[] buffer = new byte[1];
        let mut buffer = vec![0u8; 1];
        // Java: int result = this.read(buffer, 0, 1);
        let result = self.read__bytearr_int_int(&mut buffer, 0, 1);
        // Java: return result == -1 ? -1 : buffer[0] & 255;
        if result == -1 { -1 } else { (buffer[0] as i32) & 255 }
    }

    // Java: public int read(byte[] buffer) throws IOException
    pub fn read__bytearr(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: return this.read(buffer, 0, buffer.length);
        let len = buffer.len() as i32;
        self.read__bytearr_int_int(buffer, 0, len)
    }

    // Java: public int read(byte[] buffer, int byteOffset, int byteCount) throws IOException
    pub fn read__bytearr_int_int(&mut self, buffer: &mut [u8], byteOffset: i32, byteCount: i32) -> i32 {
        // Java: if (this.mSdkInputStream != null)
        if let Some(ref mut sdk) = self.mSdkInputStream {
            use std::io::Read;
            let off = byteOffset as usize;
            let cnt = byteCount as usize;
            match sdk.read(&mut buffer[off..off + cnt]) {
                Ok(0) => -1,
                Ok(n) => n as i32,
                Err(_) => -1,
            }
        }
        // Java: else if (null != this.mMfsFile)
        else if let Some(ref mut mfs) = self.mMfsFile {
            // Java: int count = this.mMfsFile.read(this.mFilePointer, buffer, byteOffset, byteCount);
            let count = mfs.read(self.mFilePointer, buffer.to_vec(), byteOffset, byteCount);
            // Java: this.mFilePointer += (long)(count > 0 ? count : 0);
            self.mFilePointer += (if count > 0 { count } else { 0 }) as i64;
            // Java: return count;
            count
        }
        // Java: else
        else {
            -1
        }
    }

    // Java: public long skip(long count) throws IOException
    pub fn skip(&mut self, count: i64) -> i64 {
        // Java: if (count > 0L)
        if count > 0 {
            // Java: if (this.mSdkInputStream != null)
            if self.mSdkInputStream.is_some() {
                use std::io::Seek;
                if let Some(ref mut sdk) = self.mSdkInputStream {
                    let _ = sdk.seek(std::io::SeekFrom::Current(count));
                }
                count
            }
            // Java: else
            else {
                // Java: long currentPos = this.getFilePointer();
                let currentPos = self.mFilePointer;
                // Java: long eof = this.length();
                let eof = self.length();
                // Java: int newCount = (int)(currentPos + count > eof ? eof - currentPos : count);
                let newCount = if currentPos + count > eof { eof - currentPos } else { count };
                // Java: this.mFilePointer += (long)newCount;
                self.mFilePointer += newCount;
                newCount
            }
        }
        // Java: else
        else {
            0
        }
    }

    // Java: private long getFilePointer() throws IOException
    fn getFilePointer(&self) -> i64 {
        // Java: return this.mFilePointer;
        self.mFilePointer
    }

    // Java: private long length() throws IOException
    fn length(&mut self) -> i64 {
        // Java: return this.mMfsFile.length();
        if let Some(ref mut mfs) = self.mMfsFile {
            mfs.length()
        } else {
            0
        }
    }

    // Java: public int available() throws IOException
    pub fn available(&mut self) -> i32 {
        // Java: if (this.mSdkInputStream != null)
        if self.mSdkInputStream.is_some() {
            0 // simplified
        }
        // Java: else
        else {
            // Java: long currentPos = this.getFilePointer();
            let currentPos = self.mFilePointer;
            // Java: long eof = this.length();
            let eof = self.length();
            // Java: int available = (int)(eof - currentPos);
            (eof - currentPos) as i32
        }
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        self.mSdkInputStream = None;
        self.mMfsFile = None;
    }
}

impl std::io::Read for FileInputStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.read__bytearr(buf);
        if n < 0 {
            Ok(0)
        } else {
            Ok(n as usize)
        }
    }
}
