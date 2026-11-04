use std::io::{self, Read, Seek, SeekFrom};
use std::fs::File;

// Java: public class FilePartInputStream extends InputStream
pub struct FilePartInputStream {
    // Java field: FileInputStream stream
    pub(crate) stream: File,
    // Java field: int endPos
    pub(crate) endPos: i32,
    // Java field: int currentPos
    currentPos: i32,
}

// Java: public class FilePartInputStream extends InputStream
impl FilePartInputStream {
    // Java: FilePartInputStream(String src, int beginPos, int endPos)
    pub fn new(src: &str, begin_pos: i32, end_pos: i32) -> io::Result<Self> {
        // Java: super();
        // Java: this.stream = new FileInputStream(src);
        let mut stream = File::open(src)?;
        // Java: int size = this.stream.available();
        let size = stream.metadata()?.len() as i32;
        // Java: if (beginPos >= 0 && beginPos < endPos && endPos <= size)
        if begin_pos >= 0 && begin_pos < end_pos && end_pos <= size {
            // Java: this.endPos = endPos;
            // Java: this.currentPos = beginPos;
            let current_pos = begin_pos;
            // Java: if (beginPos > 0)
            if begin_pos > 0 {
                // Java: this.stream.skip((long)beginPos);
                stream.seek(SeekFrom::Start(begin_pos as u64))?;
            }
            Ok(FilePartInputStream {
                stream,
                endPos: end_pos,
                currentPos: current_pos,
            })
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException();
            Err(io::Error::new(io::ErrorKind::InvalidInput, "IllegalArgumentException"))
        }
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: return this.endPos - this.currentPos;
        self.endPos - self.currentPos
    }

    // Java: void close()
    pub fn close(self) {
        // Java: this.stream.close();
        drop(self.stream);
    }

    // Java: void mark(int readlimit)
    pub fn mark(&mut self, _readlimit: i32) {
        // Java: this.stream.mark(readlimit);
    }

    // Java: boolean markSupported()
    pub fn mark_supported(&self) -> bool {
        // Java: return this.stream.markSupported();
        false
    }

    // Java: int read()
    pub fn read_byte(&mut self) -> io::Result<i32> {
        // Java: if (this.endPos - this.currentPos >= 4)
        if self.endPos - self.currentPos >= 4 {
            // Java: this.currentPos += 4;
            self.currentPos += 4;
            // Java: return this.stream.read();
            let mut buf = [0u8; 1];
            match self.stream.read(&mut buf)? {
                0 => Ok(-1),
                _ => Ok(buf[0] as i32),
            }
        }
        // Java: else
        else {
            // Java: return -1;
            Ok(-1)
        }
    }

    // Java: int read(byte[] b)
    pub fn read_buf(&mut self, b: &mut [u8]) -> io::Result<i32> {
        // Java: return this.read(b, 0, b.length);
        let len = b.len();
        self.read_buf_offset(b, 0, len as i32)
    }

    // Java: int read(byte[] b, int off, int len)
    pub fn read_buf_offset(&mut self, b: &mut [u8], off: i32, len: i32) -> io::Result<i32> {
        // Java: int len2 = 0;
        let len2;
        // Java: if (this.endPos - this.currentPos >= len)
        if self.endPos - self.currentPos >= len {
            // Java: len2 = this.stream.read(b, off, len);
            len2 = self.stream.read(&mut b[off as usize..(off + len) as usize])? as i32;
            // Java: this.currentPos += len2;
            self.currentPos += len2;
            // Java: return len2;
            Ok(len2)
        }
        // Java: else if (this.currentPos < this.endPos)
        else if self.currentPos < self.endPos {
            // Java: len2 = this.stream.read(b, off, this.endPos - this.currentPos);
            let remaining = (self.endPos - self.currentPos) as usize;
            len2 = self.stream.read(&mut b[off as usize..off as usize + remaining])? as i32;
            // Java: this.currentPos += len2;
            self.currentPos += len2;
            // Java: return len2;
            Ok(len2)
        }
        // Java: else
        else {
            // Java: return -1;
            Ok(-1)
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) -> io::Result<()> {
        // Java: this.stream.reset();
        Ok(())
    }

    // Java: long skip(long n)
    pub fn skip(&mut self, n: i64) -> io::Result<i64> {
        // Java: if (this.endPos == this.currentPos)
        if self.endPos == self.currentPos {
            // Java: return 0L;
            return Ok(0);
        }
        // Java: else
        // Java: long n2 = n;
        let mut n2 = n;
        // Java: if ((long)(this.endPos - this.currentPos) < n)
        if ((self.endPos - self.currentPos) as i64) < n {
            // Java: n2 = (long)(this.endPos - this.currentPos);
            n2 = (self.endPos - self.currentPos) as i64;
        }
        // Java: n2 = this.stream.skip(n2);
        n2 = self.stream.seek(SeekFrom::Current(n2))? as i64 - (self.currentPos as i64);
        // Java: this.currentPos = (int)((long)this.currentPos + n2);
        self.currentPos = (self.currentPos as i64 + n2) as i32;
        // Java: return n2;
        Ok(n2)
    }
}

impl Read for FilePartInputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = self.read_buf_offset(buf, 0, buf.len() as i32)?;
        if result < 0 {
            Ok(0)
        } else {
            Ok(result as usize)
        }
    }
}
