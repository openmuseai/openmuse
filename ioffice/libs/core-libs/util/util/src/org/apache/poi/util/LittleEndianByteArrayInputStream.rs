use super::LittleEndianInput::LittleEndianInput;
use super::LittleEndianRandomAccessInput::LittleEndianRandomAccessInput;

// Java: public class LittleEndianByteArrayInputStream implements LittleEndianRandomAccessInput
pub struct LittleEndianByteArrayInputStream {
    // Java field: byte[] _buf
    pub(crate) buf: Vec<u8>,
    // Java field: int _endIndex
    pub(crate) end_index: i32,
    // Java field: int _readIndex
    pub(crate) read_index: i32,
}

// Java: public class LittleEndianByteArrayInputStream implements LittleEndianRandomAccessInput
impl LittleEndianByteArrayInputStream {
    // Java: LittleEndianByteArrayInputStream(byte[] buf, int startOffset, int maxReadLen)
    pub fn new(buf: Vec<u8>, start_offset: i32, max_read_len: i32) -> Self {
        // Java: super();
        // Java: this._buf = buf;
        // Java: this._readIndex = startOffset;
        // Java: this._endIndex = startOffset + maxReadLen;
        LittleEndianByteArrayInputStream {
            buf,
            read_index: start_offset,
            end_index: start_offset + max_read_len,
        }
    }

    // Java: LittleEndianByteArrayInputStream(byte[] buf, int startOffset)
    pub fn new_with_offset(buf: Vec<u8>, start_offset: i32) -> Self {
        // Java: this(buf, startOffset, buf.length - startOffset);
        let len = buf.len() as i32;
        Self::new(buf, start_offset, len - start_offset)
    }

    // Java: LittleEndianByteArrayInputStream(byte[] buf)
    pub fn new_from_buf(buf: Vec<u8>) -> Self {
        // Java: this(buf, 0, buf.length);
        let len = buf.len() as i32;
        Self::new(buf, 0, len)
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: return this._endIndex - this._readIndex;
        self.end_index - self.read_index
    }

    // Java: void checkPosition(int i)
    fn check_position(&self, i: i32) {
        // Java: if (i > this._endIndex - this._readIndex)
        if i > self.end_index - self.read_index {
            // Java: throw new RuntimeException("Buffer overrun");
            panic!("Buffer overrun");
        }
    }

    // Java: int getReadIndex()
    pub fn get_read_index(&self) -> i32 {
        // Java: return this._readIndex;
        self.read_index
    }

    // Java: byte readByte()
    pub fn read_byte(&mut self) -> i8 {
        // Java: return 1 > this.available() ? 0 : this._buf[this._readIndex++];
        if 1 > self.available() {
            0
        } else {
            let v = self.buf[self.read_index as usize];
            self.read_index += 1;
            v as i8
        }
    }

    // Java: int readInt()
    pub fn read_int(&mut self) -> i32 {
        // Java: this.checkPosition(4);
        self.check_position(4);
        // Java: int i = this._readIndex;
        let mut i = self.read_index as usize;
        // Java: int b0 = this._buf[i++] & 255;
        let b0 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b1 = this._buf[i++] & 255;
        let b1 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b2 = this._buf[i++] & 255;
        let b2 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b3 = this._buf[i++] & 255;
        let b3 = self.buf[i] as i32 & 255; i += 1;
        // Java: this._readIndex = i;
        self.read_index = i as i32;
        // Java: return (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0);
        (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
    }

    // Java: long readLong()
    pub fn read_long(&mut self) -> i64 {
        // Java: this.checkPosition(8);
        self.check_position(8);
        // Java: int i = this._readIndex;
        let mut i = self.read_index as usize;
        // Java: int b0 = this._buf[i++] & 255;
        let b0 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b1 = this._buf[i++] & 255;
        let b1 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b2 = this._buf[i++] & 255;
        let b2 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b3 = this._buf[i++] & 255;
        let b3 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b4 = this._buf[i++] & 255;
        let b4 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b5 = this._buf[i++] & 255;
        let b5 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b6 = this._buf[i++] & 255;
        let b6 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b7 = this._buf[i++] & 255;
        let b7 = self.buf[i] as i32 & 255; i += 1;
        // Java: this._readIndex = i;
        self.read_index = i as i32;
        // Java: return ((long)b7 << 56) + ((long)b6 << 48) + ((long)b5 << 40) + ((long)b4 << 32) + ((long)b3 << 24) + (long)(b2 << 16) + (long)(b1 << 8) + (long)(b0 << 0);
        ((b7 as i64) << 56) + ((b6 as i64) << 48) + ((b5 as i64) << 40) + ((b4 as i64) << 32)
            + ((b3 as i64) << 24) + ((b2 << 16) as i64) + ((b1 << 8) as i64) + (b0 as i64)
    }

    // Java: short readShort()
    pub fn read_short(&mut self) -> i16 {
        // Java: return (short)this.readUShort();
        self.read_u_short() as i16
    }

    // Java: int readUByte()
    pub fn read_u_byte(&mut self) -> i32 {
        // Java: this.checkPosition(1);
        self.check_position(1);
        // Java: return this._buf[this._readIndex++] & 255;
        let v = self.buf[self.read_index as usize] as i32 & 255;
        self.read_index += 1;
        v
    }

    // Java: int readUShort()
    pub fn read_u_short(&mut self) -> i32 {
        // Java: this.checkPosition(2);
        self.check_position(2);
        // Java: int i = this._readIndex;
        let mut i = self.read_index as usize;
        // Java: int b0 = this._buf[i++] & 255;
        let b0 = self.buf[i] as i32 & 255; i += 1;
        // Java: int b1 = this._buf[i++] & 255;
        let b1 = self.buf[i] as i32 & 255; i += 1;
        // Java: this._readIndex = i;
        self.read_index = i as i32;
        // Java: return (b1 << 8) + (b0 << 0);
        (b1 << 8) + b0
    }

    // Java: void readFully(byte[] buf, int off, int len)
    pub fn read_fully_offset(&mut self, buf: &mut [u8], off: usize, len: usize) {
        // Java: this.checkPosition(len);
        self.check_position(len as i32);
        // Java: System.arraycopy(this._buf, this._readIndex, buf, off, len);
        let ri = self.read_index as usize;
        buf[off..off + len].copy_from_slice(&self.buf[ri..ri + len]);
        // Java: this._readIndex += len;
        self.read_index += len as i32;
    }

    // Java: void readFully(byte[] buf)
    pub fn read_fully(&mut self, buf: &mut [u8]) {
        // Java: this.readFully(buf, 0, buf.length);
        let len = buf.len();
        self.read_fully_offset(buf, 0, len);
    }

    // Java: long skip(long len)
    pub fn skip_bytes(&mut self, len: i64) -> i64 {
        // Java: this.checkPosition((int)len);
        self.check_position(len as i32);
        // Java: this._readIndex = (int)((long)this._readIndex + len);
        self.read_index = (self.read_index as i64 + len) as i32;
        // Java: return len;
        len
    }

    // Java: long tell()
    pub fn tell(&self) -> i64 {
        // Java: return (long)this._readIndex;
        self.read_index as i64
    }

    // Java: long seek(long pos)
    pub fn seek(&mut self, pos: i64) -> i64 {
        // Java: this._readIndex = (int)pos;
        self.read_index = pos as i32;
        // Java: return (long)this._readIndex;
        self.read_index as i64
    }

    // Java: double readDouble()
    pub fn read_double(&mut self) -> f64 {
        // Java: return Double.longBitsToDouble(this.readLong());
        f64::from_bits(self.read_long() as u64)
    }
}

impl LittleEndianInput for LittleEndianByteArrayInputStream {
    fn available(&self) -> i32 { self.available() }
    fn readByte(&mut self) -> i8 { self.read_byte() }
    fn readUByte(&mut self) -> i32 { self.read_u_byte() }
    fn readShort(&mut self) -> i16 { self.read_short() }
    fn readUShort(&mut self) -> i32 { self.read_u_short() }
    fn readInt(&mut self) -> i32 { self.read_int() }
    fn readLong(&mut self) -> i64 { self.read_long() }
    fn readDouble(&mut self) -> f64 { self.read_double() }
    fn readFully(&mut self, var1: &mut [u8]) { self.read_fully(var1) }
    fn readFully_offset(&mut self, var1: &mut [u8], var2: usize, var3: usize) { self.read_fully_offset(var1, var2, var3) }
    fn skip(&mut self, var1: i64) -> i64 { self.skip_bytes(var1) }
}

impl LittleEndianRandomAccessInput for LittleEndianByteArrayInputStream {
    fn tell(&self) -> i64 { self.tell() }
    fn seek(&mut self, var1: i64) -> i64 { self.seek(var1) }
}
