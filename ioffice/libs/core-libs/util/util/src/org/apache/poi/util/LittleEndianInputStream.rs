// Java: import java.io.FilterInputStream
// Java: import java.io.IOException
// Java: import java.io.InputStream
use std::io::Read;
use super::LittleEndianInput::LittleEndianInput;

// Java: public class LittleEndianInputStream extends FilterInputStream implements LittleEndianInput
pub struct LittleEndianInputStream {
    inner: Box<dyn Read>,
}

fn read_one(r: &mut dyn Read) -> i32 {
    let mut buf = [0u8; 1];
    match r.read(&mut buf) {
        Ok(0) | Err(_) => -1,
        Ok(_) => buf[0] as i32,
    }
}

// Java: public class LittleEndianInputStream extends FilterInputStream implements LittleEndianInput
impl LittleEndianInputStream {
    // Java: LittleEndianInputStream(InputStream is)
    pub fn new(is: Box<dyn Read>) -> Self {
        // Java: super(is);
        LittleEndianInputStream { inner: is }
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: try
        // Java: return super.available();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        0
    }

    // Java: byte readByte()
    pub fn read_byte(&mut self) -> i8 {
        // Java: return (byte)this.readUByte();
        self.read_u_byte() as i8
    }

    // Java: int readUByte()
    pub fn read_u_byte(&mut self) -> i32 {
        // Java: int ch;
        // Java: try
        // Java: ch = this.in.read();
        let ch = read_one(&mut *self.inner);
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        // Java: checkEOF(ch);
        Self::check_eof(ch);
        // Java: return ch;
        ch
    }

    // Java: double readDouble()
    pub fn read_double(&mut self) -> f64 {
        // Java: return Double.longBitsToDouble(this.readLong());
        f64::from_bits(self.read_long() as u64)
    }

    // Java: int readInt()
    pub fn read_int(&mut self) -> i32 {
        // Java: int ch1;
        // Java: int ch2;
        // Java: int ch3;
        // Java: int ch4;
        // Java: try
        // Java: ch1 = this.in.read();
        let ch1 = read_one(&mut *self.inner);
        // Java: ch2 = this.in.read();
        let ch2 = read_one(&mut *self.inner);
        // Java: ch3 = this.in.read();
        let ch3 = read_one(&mut *self.inner);
        // Java: ch4 = this.in.read();
        let ch4 = read_one(&mut *self.inner);
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        // Java: checkEOF(ch1 | ch2 | ch3 | ch4);
        Self::check_eof(ch1 | ch2 | ch3 | ch4);
        // Java: return (ch4 << 24) + (ch3 << 16) + (ch2 << 8) + (ch1 << 0);
        (ch4 << 24) + (ch3 << 16) + (ch2 << 8) + ch1
    }

    // Java: long readLong()
    pub fn read_long(&mut self) -> i64 {
        // Java: int b0;
        // Java: int b1;
        // Java: int b2;
        // Java: int b3;
        // Java: int b4;
        // Java: int b5;
        // Java: int b6;
        // Java: int b7;
        // Java: try
        // Java: b0 = this.in.read();
        let b0 = read_one(&mut *self.inner);
        // Java: b1 = this.in.read();
        let b1 = read_one(&mut *self.inner);
        // Java: b2 = this.in.read();
        let b2 = read_one(&mut *self.inner);
        // Java: b3 = this.in.read();
        let b3 = read_one(&mut *self.inner);
        // Java: b4 = this.in.read();
        let b4 = read_one(&mut *self.inner);
        // Java: b5 = this.in.read();
        let b5 = read_one(&mut *self.inner);
        // Java: b6 = this.in.read();
        let b6 = read_one(&mut *self.inner);
        // Java: b7 = this.in.read();
        let b7 = read_one(&mut *self.inner);
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        // Java: checkEOF(b0 | b1 | b2 | b3 | b4 | b5 | b6 | b7);
        Self::check_eof(b0 | b1 | b2 | b3 | b4 | b5 | b6 | b7);
        // Java: return ((long)b7 << 56) + ((long)b6 << 48) + ((long)b5 << 40) + ((long)b4 << 32) + ((long)b3 << 24) + (long)(b2 << 16) + (long)(b1 << 8) + (long)(b0 << 0);
        ((b7 as i64) << 56) + ((b6 as i64) << 48) + ((b5 as i64) << 40) + ((b4 as i64) << 32)
            + ((b3 as i64) << 24) + ((b2 << 16) as i64) + ((b1 << 8) as i64) + (b0 as i64)
    }

    // Java: short readShort()
    pub fn read_short(&mut self) -> i16 {
        // Java: return (short)this.readUShort();
        self.read_u_short() as i16
    }

    // Java: int readUShort()
    pub fn read_u_short(&mut self) -> i32 {
        // Java: int ch1;
        // Java: int ch2;
        // Java: try
        // Java: ch1 = this.in.read();
        let ch1 = read_one(&mut *self.inner);
        // Java: ch2 = this.in.read();
        let ch2 = read_one(&mut *self.inner);
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
        // Java: checkEOF(ch1 | ch2);
        Self::check_eof(ch1 | ch2);
        // Java: return (ch2 << 8) + (ch1 << 0);
        (ch2 << 8) + ch1
    }

    // Java: static void checkEOF(int value)
    fn check_eof(value: i32) {
        // Java: if (value < 0)
        if value < 0 {
            // Java: throw new RuntimeException("Unexpected end-of-file");
            panic!("Unexpected end-of-file");
        }
    }

    // Java: void readFully(byte[] buf)
    pub fn read_fully(&mut self, buf: &mut [u8]) {
        // Java: this.readFully(buf, 0, buf.length);
        let len = buf.len();
        self.read_fully_offset(buf, 0, len);
    }

    // Java: void readFully(byte[] buf, int off, int len)
    pub fn read_fully_offset(&mut self, buf: &mut [u8], off: usize, len: usize) {
        // Java: int max = off + len;
        let max = off + len;
        // Java: for(int i = off; i < max; ++i)
        for i in off..max {
            // Java: int ch;
            // Java: try
            // Java: ch = this.in.read();
            let ch = read_one(&mut *self.inner);
            // Java: catch (IOException e)
            // Java: throw new RuntimeException(e);
            // Java: checkEOF(ch);
            Self::check_eof(ch);
            // Java: buf[i] = (byte)ch;
            buf[i] = ch as u8;
        }
    }

    // Java: long skip(long len)
    pub fn skip_bytes(&mut self, len: i64) -> i64 {
        // Java: try
        // Java: return this.in.skip(len);
        let mut skipped = 0i64;
        let mut buf = [0u8; 1024];
        let mut remaining = len;
        while remaining > 0 {
            let to_read = std::cmp::min(remaining as usize, buf.len());
            match self.inner.read(&mut buf[..to_read]) {
                Ok(0) => break,
                Ok(n) => { skipped += n as i64; remaining -= n as i64; }
                Err(_) => break,
            }
        }
        skipped
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }
}

impl LittleEndianInput for LittleEndianInputStream {
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
