#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments)]

// Java: import cn.wps.moffice.kfs.RandomAccessFile
use crate::kfs::mfs::RandomAccessFile::RandomAccessFile;

// Java: public class RafPartInputStream extends InputStream
pub struct RafPartInputStream {
    // Java field:    private final RandomAccessFile mRaf;
    mRaf: RandomAccessFile,
    // Java field:    private int mEndPos;
    mEndPos: i32,
    // Java field:    private int mCurPos;
    mCurPos: i32,
}

// Java: public class RafPartInputStream extends InputStream
impl RafPartInputStream {
    // Java: public RafPartInputStream(RandomAccessFile raf, int beginPos, int endPos) throws IOException
    pub fn new(mut raf: RandomAccessFile, beginPos: i32, endPos: i32) -> RafPartInputStream {
        // Java: raf.seek(0L);
        raf.seek(0);
        let size = raf.length();
        // Java: if (beginPos >= 0 && beginPos < endPos && (long)endPos <= size)
        if beginPos >= 0 && beginPos < endPos && (endPos as i64) <= size {
            // Java: if (beginPos > 0) this.mRaf.skipBytes(beginPos);
            if beginPos > 0 {
                raf.skipBytes(beginPos);
            }
            RafPartInputStream { mRaf: raf, mEndPos: endPos, mCurPos: beginPos }
        } else {
            panic!("IllegalArgumentException");
        }
    }

    // Java: public int available() throws IOException
    pub fn available(&self) -> i32 {
        // Java: return this.mEndPos - this.mCurPos;
        self.mEndPos - self.mCurPos
    }

    // Java: public void close() throws IOException
    pub fn close(&mut self) {
        // Java: this.mRaf.close();
        self.mRaf.close();
    }

    // Java: public int read() throws IOException
    pub fn read(&mut self) -> i32 {
        // Java: if (this.mEndPos - this.mCurPos >= 1)
        if self.mEndPos - self.mCurPos >= 1 {
            // Java: ++this.mCurPos;
            self.mCurPos += 1;
            // Java: return this.mRaf.read();
            self.mRaf.read()
        } else {
            -1
        }
    }

    // Java: public int read(byte[] b) throws IOException
    pub fn read__bytearr(&mut self, b: &mut [u8]) -> i32 {
        // Java: return this.read(b, 0, b.length);
        let len = b.len() as i32;
        self.read__bytearr_int_int(b, 0, len)
    }

    // Java: public int read(byte[] b, int off, int len) throws IOException
    pub fn read__bytearr_int_int(&mut self, b: &mut [u8], off: i32, len: i32) -> i32 {
        // Java: if (this.mEndPos - this.mCurPos >= len)
        if self.mEndPos - self.mCurPos >= len {
            let lenRes = self.mRaf.read__bytearr_int_int(b, off, len);
            self.mCurPos += lenRes;
            lenRes
        }
        // Java: else if (this.mCurPos < this.mEndPos)
        else if self.mCurPos < self.mEndPos {
            let remaining = self.mEndPos - self.mCurPos;
            let lenRes = self.mRaf.read__bytearr_int_int(b, off, remaining);
            self.mCurPos += lenRes;
            lenRes
        } else {
            -1
        }
    }

    // Java: public long skip(long n) throws IOException
    pub fn skip(&mut self, n: i64) -> i64 {
        // Java: if (this.mEndPos == this.mCurPos) return 0L;
        if self.mEndPos == self.mCurPos { return 0; }
        // Java: long nRes = n;
        let mut nRes = n;
        // Java: if ((long)(this.mEndPos - this.mCurPos) < n) nRes = (long)(this.mEndPos - this.mCurPos);
        let remaining = (self.mEndPos - self.mCurPos) as i64;
        if remaining < n { nRes = remaining; }
        // Java: nRes = (long)this.mRaf.skipBytes((int)nRes);
        nRes = self.mRaf.skipBytes(nRes as i32) as i64;
        // Java: this.mCurPos = (int)((long)this.mCurPos + nRes);
        self.mCurPos = (self.mCurPos as i64 + nRes) as i32;
        nRes
    }
}
