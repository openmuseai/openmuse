// Java: import java.io.FilterOutputStream
// Java: import java.io.IOException
// Java: import java.io.OutputStream
use std::io::Write;
use super::LittleEndianOutput::LittleEndianOutput;

// Java: public class LittleEndianOutputStream extends FilterOutputStream implements LittleEndianOutput
pub struct LittleEndianOutputStream {
    inner: Box<dyn Write>,
}

// Java: public class LittleEndianOutputStream extends FilterOutputStream implements LittleEndianOutput
impl LittleEndianOutputStream {
    // Java: LittleEndianOutputStream(OutputStream out)
    pub fn new(out: Box<dyn Write>) -> Self {
        // Java: super(out);
        LittleEndianOutputStream { inner: out }
    }

    // Java: void writeByte(int v)
    pub fn write_byte(&mut self, v: i32) {
        // Java: try
        // Java: this.out.write(v);
        self.inner.write_all(&[v as u8]).unwrap();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }

    // Java: void writeDouble(double v)
    pub fn write_double(&mut self, v: f64) {
        // Java: this.writeLong(Double.doubleToLongBits(v));
        self.write_long(v.to_bits() as i64);
    }

    // Java: void writeInt(int v)
    pub fn write_int(&mut self, v: i32) {
        // Java: int b3 = v >>> 24 & 255;
        let b3 = ((v as u32) >> 24) & 255;
        // Java: int b2 = v >>> 16 & 255;
        let b2 = ((v as u32) >> 16) & 255;
        // Java: int b1 = v >>> 8 & 255;
        let b1 = ((v as u32) >> 8) & 255;
        // Java: int b0 = v >>> 0 & 255;
        let b0 = (v as u32) & 255;
        // Java: try
        // Java: this.out.write(b0);
        // Java: this.out.write(b1);
        // Java: this.out.write(b2);
        // Java: this.out.write(b3);
        self.inner.write_all(&[b0 as u8, b1 as u8, b2 as u8, b3 as u8]).unwrap();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }

    // Java: void writeLong(long v)
    pub fn write_long(&mut self, v: i64) {
        // Java: this.writeInt((int)(v >> 0));
        self.write_int(v as i32);
        // Java: this.writeInt((int)(v >> 32));
        self.write_int((v >> 32) as i32);
    }

    // Java: void writeShort(int v)
    pub fn write_short(&mut self, v: i32) {
        // Java: int b1 = v >>> 8 & 255;
        let b1 = ((v as u32) >> 8) & 255;
        // Java: int b0 = v >>> 0 & 255;
        let b0 = (v as u32) & 255;
        // Java: try
        // Java: this.out.write(b0);
        // Java: this.out.write(b1);
        self.inner.write_all(&[b0 as u8, b1 as u8]).unwrap();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }

    // Java: void write(byte[] b)
    pub fn write_bytes(&mut self, b: &[u8]) {
        // Java: try
        // Java: super.write(b);
        self.inner.write_all(b).unwrap();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }

    // Java: void write(byte[] b, int off, int len)
    pub fn write_bytes_offset(&mut self, b: &[u8], off: i32, len: i32) {
        // Java: try
        // Java: super.write(b, off, len);
        let o = off as usize;
        let l = len as usize;
        self.inner.write_all(&b[o..o + l]).unwrap();
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e);
    }
}

// impl LittleEndianOutput for LittleEndianOutputStream {
//     fn writeByte(&mut self, var1: i32) { self.write_byte(var1) }
//     fn writeShort(&mut self, var1: i32) { self.write_short(var1) }
//     fn writeInt(&mut self, var1: i32) { self.write_int(var1) }
//     fn writeLong(&mut self, var1: i64) { self.write_long(var1) }
//     fn writeDouble(&mut self, var1: f64) { self.write_double(var1) }
//     fn write(&mut self, var1: &[u8]) { self.write_bytes(var1) }
//     fn write_with_offset(&mut self, var1: &[u8], var2: i32, var3: i32) { self.write_bytes_offset(var1, var2, var3) }
// }
