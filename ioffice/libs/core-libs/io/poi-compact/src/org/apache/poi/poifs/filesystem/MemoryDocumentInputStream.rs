// Java: import java.io.IOException

use std::io;

// Java field: int EOF
const EOF: i32 = -1;
// Java field: int SIZE_SHORT
const SIZE_SHORT: i32 = 2;
// Java field: int SIZE_INT
const SIZE_INT: i32 = 4;
// Java field: int SIZE_LONG
const SIZE_LONG: i32 = 8;

// Java: public class MemoryDocumentInputStream extends DocumentInputStream
pub struct MemoryDocumentInputStream {
    // Java field: int mOffset
    pub(crate) mOffset: i32,
    // Java field: int mMarkedOffset
    pub(crate) mMarkedOffset: i32,
    // Java field: byte[] mDocument
    pub(crate) mDocument: Vec<u8>,
    // Java field: boolean mClosed
    pub(crate) mClosed: bool,
}

// Java: public class MemoryDocumentInputStream extends DocumentInputStream
impl MemoryDocumentInputStream {
    // Java: MemoryDocumentInputStream(byte[] buffer, int offset)
    pub fn new(buffer: Vec<u8>, offset: i32) -> Self {
        // Java: super();
        // Java: this.mDocument = buffer;
        // Java: this.mOffset = offset;
        MemoryDocumentInputStream {
            mOffset: offset,
            mMarkedOffset: 0,
            mDocument: buffer,
            mClosed: false,
        }
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: if (this.mClosed)
        if self.mClosed {
            // Java: throw new IllegalStateException("cannot perform requested operation on a closed stream");
            panic!("cannot perform requested operation on a closed stream");
        }
        // Java: else
        else {
            // Java: return this.mDocument.length - this.mOffset;
            self.mDocument.len() as i32 - self.mOffset
        }
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.mClosed = true;
        self.mClosed = true;
    }

    // Java: void mark(int ignoredReadlimit)
    pub fn mark(&mut self, _ignored_readlimit: i32) {
        // Java: this.mMarkedOffset = this.mOffset;
        self.mMarkedOffset = self.mOffset;
    }

    // Java: boolean markSupported()
    pub fn markSupported(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: int read()
    pub fn read_byte(&mut self) -> io::Result<i32> {
        // Java: this.dieIfClosed();
        self.dieIfClosed()?;
        // Java: return this.atEOD() ? -1 : this.mDocument[this.mOffset++];
        if self.atEOD() {
            Ok(-1)
        } else {
            let b = self.mDocument[self.mOffset as usize] as i32;
            self.mOffset += 1;
            Ok(b)
        }
    }

    // Java: int read(byte[] b)
    pub fn read_buf(&mut self, b: &mut [u8]) -> io::Result<i32> {
        // Java: return this.read(b, 0, b.length);
        let len = b.len() as i32;
        self.read_buf_offset(b, 0, len)
    }

    // Java: int read(byte[] b, int off, int len)
    pub fn read_buf_offset(&mut self, b: &mut [u8], off: i32, len: i32) -> io::Result<i32> {
        // Java: this.dieIfClosed();
        self.dieIfClosed()?;
        // Java: if (b == null)
        // (b is always non-null in Rust)
        {
            // Java: throw new IllegalArgumentException("buffer must not be null");
        }
        // Java: else if (off >= 0 && len >= 0 && b.length >= off + len)
        if off >= 0 && len >= 0 && b.len() as i32 >= off + len {
            // Java: if (len == 0)
            if len == 0 {
                // Java: return 0;
                return Ok(0);
            }
            // Java: else if (this.atEOD())
            else if self.atEOD() {
                // Java: return -1;
                return Ok(-1);
            }
            // Java: else
            else {
                // Java: int limit = Math.min(this.available(), len);
                let limit = std::cmp::min(self.available(), len);
                // Java: this.readFully(b, off, limit);
                self.readFully_offset(b, off, limit);
                // Java: return limit;
                return Ok(limit);
            }
        }
        // Java: else
        else {
            // Java: throw new IndexOutOfBoundsException("can't read past buffer boundaries");
            panic!("can't read past buffer boundaries");
        }
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.mOffset = this.mMarkedOffset;
        self.mOffset = self.mMarkedOffset;
    }

    // Java: long seek(long pos)
    pub fn seek(&mut self, pos: i64) -> i64 {
        // Java: if (pos >= 0L && pos <= (long)this.mDocument.length)
        if pos >= 0 && pos <= self.mDocument.len() as i64 {
            // Java: if (this.mClosed)
            if self.mClosed {
                // Java: throw new IllegalStateException("Operand stream alread closed");
                panic!("Operand stream alread closed");
            }
            // Java: else
            else {
                // Java: this.mOffset = (int)pos;
                self.mOffset = pos as i32;
                // Java: return (long)this.mOffset;
                return self.mOffset as i64;
            }
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException("Position for seeking out of range");
            panic!("Position for seeking out of range");
        }
    }

    // Java: long skip(long n)
    pub fn skip(&mut self, n: i64) -> i64 {
        // Java: try
        {
            // Java: this.dieIfClosed();
            if let Err(_) = self.dieIfClosed() {
                // Java: catch (IOException e)
                {
                    // Java: throw new RuntimeException(e);
                    panic!("stream is closed");
                }
            }
        }
        // Java: if (n < 0L)
        if n < 0 {
            // Java: return 0L;
            return 0;
        }
        // Java: else
        else {
            // Java: this.mOffset = (int)((long)this.mOffset + n);
            self.mOffset = (self.mOffset as i64 + n) as i32;
            // Java: return n;
            return n;
        }
    }

    // Java: long tell()
    pub fn tell(&self) -> i64 {
        // Java: return (long)this.mOffset;
        self.mOffset as i64
    }

    // Java: void dieIfClosed()
    fn dieIfClosed(&self) -> io::Result<()> {
        // Java: if (this.mClosed)
        if self.mClosed {
            // Java: throw new IOException("cannot perform requested operation on a closed stream");
            return Err(io::Error::new(io::ErrorKind::Other, "cannot perform requested operation on a closed stream"));
        }
        Ok(())
    }

    // Java: boolean atEOD()
    fn atEOD(&self) -> bool {
        // Java: return this.mOffset == this.mDocument.length;
        self.mOffset == self.mDocument.len() as i32
    }

    // Java: void checkAvaliable(int requestedSize)
    fn checkAvaliable(&self, requested_size: i32) {
        // Java: if (this.mClosed)
        if self.mClosed {
            // Java: throw new IllegalStateException("cannot perform requested operation on a closed stream");
            panic!("cannot perform requested operation on a closed stream");
        }
        // Java: else if (requestedSize > this.mDocument.length - this.mOffset)
        else if requested_size > self.mDocument.len() as i32 - self.mOffset {
            // Java: throw new RuntimeException("Buffer underrun - requested " + requestedSize + " bytes but " + (this.mDocument.length - this.mOffset) + " was available");
            panic!("Buffer underrun - requested {} bytes but {} was available", requested_size, self.mDocument.len() as i32 - self.mOffset);
        }
    }

    // Java: byte readByte()
    pub fn readByte(&mut self) -> i8 {
        // Java: return (byte)this.readUByte();
        self.readUByte() as i8
    }

    // Java: double readDouble()
    pub fn readDouble(&mut self) -> f64 {
        // Java: return Double.longBitsToDouble(this.readLong());
        f64::from_bits(self.readLong() as u64)
    }

    // Java: void readFully(byte[] buf)
    pub fn readFully(&mut self, buf: &mut [u8]) {
        // Java: this.readFully(buf, 0, buf.length);
        let len = buf.len() as i32;
        self.readFully_offset(buf, 0, len);
    }

    // Java: short readShort()
    pub fn readShort(&mut self) -> i16 {
        // Java: return (short)this.readUShort();
        self.readUShort() as i16
    }

    // Java: void readFully(byte[] buf, int off, int len)
    pub fn readFully_offset(&mut self, buf: &mut [u8], off: i32, len: i32) {
        // Java: this.checkAvaliable(len);
        self.checkAvaliable(len);
        // Java: System.arraycopy(this.mDocument, this.mOffset, buf, off, len);
        let src_start = self.mOffset as usize;
        let dst_start = off as usize;
        let count = len as usize;
        buf[dst_start..dst_start + count].copy_from_slice(&self.mDocument[src_start..src_start + count]);
        // Java: this.mOffset += len;
        self.mOffset += len;
    }

    // Java: long readLong()
    pub fn readLong(&mut self) -> i64 {
        // Java: this.checkAvaliable(8);
        self.checkAvaliable(8);
        // Java: int i = this.mOffset;
        let mut i = self.mOffset as usize;
        // Java: byte[] _buf = this.mDocument;
        let _buf = &self.mDocument;
        // Java: int b0 = _buf[i++] & 255;
        let b0 = _buf[i] as i64 & 255; i += 1;
        // Java: int b1 = _buf[i++] & 255;
        let b1 = _buf[i] as i64 & 255; i += 1;
        // Java: int b2 = _buf[i++] & 255;
        let b2 = _buf[i] as i64 & 255; i += 1;
        // Java: int b3 = _buf[i++] & 255;
        let b3 = _buf[i] as i64 & 255; i += 1;
        // Java: int b4 = _buf[i++] & 255;
        let b4 = _buf[i] as i64 & 255; i += 1;
        // Java: int b5 = _buf[i++] & 255;
        let b5 = _buf[i] as i64 & 255; i += 1;
        // Java: int b6 = _buf[i++] & 255;
        let b6 = _buf[i] as i64 & 255; i += 1;
        // Java: int b7 = _buf[i++] & 255;
        let b7 = _buf[i] as i64 & 255; i += 1;
        // Java: this.mOffset = i;
        self.mOffset = i as i32;
        // Java: return ((long)b7 << 56) + ((long)b6 << 48) + ((long)b5 << 40) + ((long)b4 << 32) + ((long)b3 << 24) + (long)(b2 << 16) + (long)(b1 << 8) + (long)(b0 << 0);
        (b7 << 56) + (b6 << 48) + (b5 << 40) + (b4 << 32) + (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
    }

    // Java: int readInt()
    pub fn readInt(&mut self) -> i32 {
        // Java: this.checkAvaliable(4);
        self.checkAvaliable(4);
        // Java: int i = this.mOffset;
        let mut i = self.mOffset as usize;
        // Java: byte[] _buf = this.mDocument;
        let _buf = &self.mDocument;
        // Java: int b0 = _buf[i++] & 255;
        let b0 = _buf[i] as i32 & 255; i += 1;
        // Java: int b1 = _buf[i++] & 255;
        let b1 = _buf[i] as i32 & 255; i += 1;
        // Java: int b2 = _buf[i++] & 255;
        let b2 = _buf[i] as i32 & 255; i += 1;
        // Java: int b3 = _buf[i++] & 255;
        let b3 = _buf[i] as i32 & 255; i += 1;
        // Java: this.mOffset = i;
        self.mOffset = i as i32;
        // Java: return (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0);
        (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
    }

    // Java: int readUShort()
    pub fn readUShort(&mut self) -> i32 {
        // Java: this.checkAvaliable(2);
        self.checkAvaliable(2);
        // Java: int i = this.mOffset;
        let mut i = self.mOffset as usize;
        // Java: byte[] _buf = this.mDocument;
        let _buf = &self.mDocument;
        // Java: int b0 = _buf[i++] & 255;
        let b0 = _buf[i] as i32 & 255; i += 1;
        // Java: int b1 = _buf[i++] & 255;
        let b1 = _buf[i] as i32 & 255; i += 1;
        // Java: this.mOffset = i;
        self.mOffset = i as i32;
        // Java: return (b1 << 8) + (b0 << 0);
        (b1 << 8) + b0
    }

    // Java: int readUByte()
    pub fn readUByte(&mut self) -> i32 {
        // Java: this.checkAvaliable(1);
        self.checkAvaliable(1);
        // Java: return this.mDocument[this.mOffset++] & 255;
        let val = self.mDocument[self.mOffset as usize] as i32 & 255;
        self.mOffset += 1;
        val
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "MemoryDocumentInputStream: position = " + (int)this.tell();
        format!("MemoryDocumentInputStream: position = {}", self.tell() as i32)
    }
}
