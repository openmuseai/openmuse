#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_assignments)]

// Java: import cn.wps.base.BufferBytes
use crate::cn::wps::base::BufferBytes::BufferBytes;
// Java: import cn.wps.base.IBufferReader
use crate::cn::wps::base::IBufferReader::IBufferReader;
// Java: import java.io.IOException
// Java: import java.io.InputStream

use std::io::Read;

// Java field: String TAG
pub(crate) const TAG: &str = "BufferReader";

// Java: public class BufferReader implements IBufferReader
pub struct BufferReader {
    // Java field: BufferBytes buffer
    buffer: Option<BufferBytes>,
    // Java field: int pos
    pub(crate) pos: i32,
    // Java field: byte[] bytes
    pub(crate) bytes: Option<Vec<u8>>,
    // Java field: int length
    pub(crate) length: i32,
}

// Java: public class BufferReader implements IBufferReader
impl BufferReader {
    // Java: BufferReader(InputStream is)
    pub fn from_stream(is: &mut impl Read) -> Self {
        // Java: super();
        let mut br = BufferReader { buffer: None, pos: -1, bytes: None, length: 0 };
        // Java: try
        {
            let available = 16384;
            // Java: this.init(is, is.available());
            let _ = br.init(is, available);
        }
        // Java: catch (IOException var3)
        {
        }
        br
    }

    // Java: BufferReader(InputStream is, int count)
    pub fn from_stream_count(is: &mut impl Read, count: i32) -> Self {
        // Java: super();
        let mut br = BufferReader { buffer: None, pos: -1, bytes: None, length: 0 };
        // Java: try
        {
            // Java: this.init(is, count);
            let _ = br.init(is, count);
        }
        // Java: catch (IOException var4)
        {
        }
        br
    }

    // Java: BufferReader(byte[] bytes)
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        // Java: super();
        let length = bytes.len() as i32;
        // Java: this.bytes = bytes;
        // Java: this.length = this.bytes.length;
        BufferReader { buffer: None, pos: -1, bytes: Some(bytes), length }
    }

    // Java: void init(InputStream is, int count)
    pub fn init(&mut self, is: &mut impl Read, count: i32) -> std::io::Result<()> {
        // Java: if (count <= 16384)
        if count <= 16384 {
            // Java: this.buffer = BufferBytes.obtain();
            self.buffer = Some(BufferBytes::obtain());
            // Java: this.bytes = this.buffer.bytes;
            let b = self.buffer.as_ref().unwrap();
            self.bytes = Some(b.bytes.clone());
        }
        // Java: else
        else {
            // Java: this.bytes = new byte[count];
            self.bytes = Some(vec![0u8; count as usize]);
        }
        // Java: this.length = is.read(this.bytes, 0, count);
        if let Some(ref mut buf) = self.bytes {
            let max = count as usize;
            let buf_len = buf.len();
            let end = max.min(buf_len);
            let slice = &mut buf[..end];
            self.length = is.read(slice)? as i32;
        }
        Ok(())
    }

    fn bytes_ref(&self) -> &[u8] {
        self.bytes.as_deref().unwrap_or(&[])
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this.pos = -1;
        self.pos = -1;
    }

    // Java: void skip(int n)
    pub fn skip(&mut self, n: i32) {
        // Java: this.pos += n;
        self.pos += n;
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: return this.length - this.pos - 1;
        self.length - self.pos - 1
    }

    // Java: int getCurrentPos()
    pub fn getCurrentPos(&self) -> i32 {
        // Java: return this.pos;
        self.pos
    }

    // Java: void setCurrentPos(int pos)
    pub fn setCurrentPos(&mut self, pos: i32) {
        // Java: this.pos = pos;
        self.pos = pos;
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.bytes = null;
        self.bytes = None;
        // Java: if (null != this.buffer)
        if self.buffer.is_some() {
            // Java: this.buffer.recycle();
            if let Some(buf) = self.buffer.take() {
                buf.recycle();
            }
            // Java: this.buffer = null;
        }
    }

    // Java: byte[] getByte()
    pub fn getByte(&self) -> &Option<Vec<u8>> {
        // Java: return this.bytes;
        &self.bytes
    }

    // Java: boolean readBoolean()
    pub fn readBoolean(&mut self) -> bool {
        // Java: return (this.bytes[++this.pos] & 255) != 0;
        self.pos += 1;
        (self.bytes_ref()[self.pos as usize] as i32 & 255) != 0
    }

    // Java: byte readByte()
    pub fn readByte_one(&mut self) -> i8 {
        // Java: return this.bytes[++this.pos];
        self.pos += 1;
        self.bytes_ref()[self.pos as usize] as i8
    }

    // Java: int readUnsignedByte()
    pub fn readUnsignedByte_one(&mut self) -> i32 {
        // Java: return this.bytes[++this.pos] & 255;
        self.pos += 1;
        self.bytes_ref()[self.pos as usize] as i32 & 255
    }

    // Java: byte[] readByte(int n)
    pub fn readByte_n(&mut self, n: i32) -> Vec<u8> {
        // Java: byte[] result = new byte[n];
        let mut result = vec![0u8; n as usize];
        // Java: System.arraycopy(this.bytes, ++this.pos, result, 0, n);
        self.pos += 1;
        let src = self.pos as usize;
        result.copy_from_slice(&self.bytes_ref()[src..src + n as usize]);
        // Java: this.pos += n - 1;
        self.pos += n - 1;
        // Java: return result;
        result
    }

    // Java: int[] readUnsignedByte(int n)
    pub fn readUnsignedByte_n(&mut self, n: i32) -> Vec<i32> {
        // Java: int[] resultBytes = new int[n];
        let mut resultBytes = vec![0i32; n as usize];
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n as usize {
            // Java: resultBytes[i] = this.bytes[++this.pos] & 255;
            self.pos += 1;
            resultBytes[i] = self.bytes_ref()[self.pos as usize] as i32 & 255;
        }
        // Java: return resultBytes;
        resultBytes
    }

    // Java: short readShort()
    pub fn readShort_one(&mut self) -> i16 {
        // Java: return (short)(this.bytes[++this.pos] & 255 | (this.bytes[++this.pos] & 255) << 8);
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        (b0 | (b1 << 8)) as i16
    }

    // Java: short readShortBE()
    pub fn readShortBE(&mut self) -> i16 {
        // Java: return (short)((this.bytes[++this.pos] & 255) << 8 | this.bytes[++this.pos] & 255);
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        ((b0 << 8) | b1) as i16
    }

    // Java: int readUnsignedShort()
    pub fn readUnsignedShort(&mut self) -> i32 {
        // Java: return this.bytes[++this.pos] & 255 | (this.bytes[++this.pos] & 255) << 8;
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        b0 | (b1 << 8)
    }

    // Java: int readUnsignedShortBE()
    pub fn readUnsignedShortBE(&mut self) -> i32 {
        // Java: return (this.bytes[++this.pos] & 255) << 8 | this.bytes[++this.pos] & 255;
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        (b0 << 8) | b1
    }

    // Java: int[] readShort(int n)
    pub fn readShort_n(&mut self, n: i32) -> Vec<i32> {
        // Java: int[] result = new int[n];
        let mut result = vec![0i32; n as usize];
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n as usize {
            // Java: result[i] = (short)(this.bytes[++this.pos] & 255 | (this.bytes[++this.pos] & 255) << 8);
            self.pos += 1;
            let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            self.pos += 1;
            let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            result[i] = (b0 | (b1 << 8)) as i16 as i32;
        }
        // Java: return result;
        result
    }

    // Java: int readInt()
    pub fn readInt_one(&mut self) -> i32 {
        // Java: return this.bytes[++this.pos] & 255 | (this.bytes[++this.pos] & 255) << 8 | (this.bytes[++this.pos] & 255) << 16 | (this.bytes[++this.pos] & 255) << 24;
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b2 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b3 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    // Java: int readIntBE()
    pub fn readIntBE(&mut self) -> i32 {
        // Java: return (this.bytes[++this.pos] & 255) << 24 | (this.bytes[++this.pos] & 255) << 16 | (this.bytes[++this.pos] & 255) << 8 | this.bytes[++this.pos] & 255;
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b2 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b3 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
    }

    // Java: int[] readInt(int n)
    pub fn readInt_n(&mut self, n: i32) -> Vec<i32> {
        // Java: int[] result = new int[n];
        let mut result = vec![0i32; n as usize];
        // Java: for(int i = 0; i < n; ++i)
        for i in 0..n as usize {
            // Java: result[i] = this.bytes[++this.pos] & 255 | (this.bytes[++this.pos] & 255) << 8 | (this.bytes[++this.pos] & 255) << 16 | (this.bytes[++this.pos] & 255) << 24;
            self.pos += 1;
            let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            self.pos += 1;
            let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            self.pos += 1;
            let b2 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            self.pos += 1;
            let b3 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            result[i] = b0 | (b1 << 8) | (b2 << 16) | (b3 << 24);
        }
        // Java: return result;
        result
    }

    // Java: int readUnsignedInt()
    pub fn readUnsignedInt_one(&mut self) -> i32 {
        // Java: return this.readInt();
        self.readInt_one()
    }

    // Java: int readUnsignedIntBE()
    pub fn readUnsignedIntBE(&mut self) -> i32 {
        // Java: return this.readIntBE();
        self.readIntBE()
    }

    // Java: int[] readUnsignedInt(int n)
    pub fn readUnsignedInt_n(&mut self, n: i32) -> Vec<i32> {
        // Java: return this.readInt(n);
        self.readInt_n(n)
    }

    // Java: long readLong()
    pub fn readLong(&mut self) -> i64 {
        // Java: return (long)(this.bytes[++this.pos] & 255 | (this.bytes[++this.pos] & 255) << 8 | (this.bytes[++this.pos] & 255) << 16) | (long)(this.bytes[++this.pos] & 255) << 24;
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b2 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b3 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        (b0 | (b1 << 8) | (b2 << 16)) as i64 | ((b3 as i64 & 255) << 24)
    }

    // Java: long readLongBE()
    pub fn readLongBE(&mut self) -> i64 {
        // Java: return (long)(this.bytes[++this.pos] & 255) << 24 | (long)((this.bytes[++this.pos] & 255) << 16) | (long)((this.bytes[++this.pos] & 255) << 8) | (long)(this.bytes[++this.pos] & 255);
        self.pos += 1;
        let b0 = self.bytes_ref()[self.pos as usize] as i64 & 255;
        self.pos += 1;
        let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b2 = self.bytes_ref()[self.pos as usize] as i32 & 255;
        self.pos += 1;
        let b3 = self.bytes_ref()[self.pos as usize] as i64 & 255;
        (b0 << 24) | ((b1 << 16) as i64) | ((b2 << 8) as i64) | b3
    }

    // Java: float readFloat()
    pub fn readFloat(&mut self) -> f32 {
        // Java: return Float.intBitsToFloat(this.readInt());
        f32::from_bits(self.readInt_one() as u32)
    }

    // Java: float readFloatBE()
    pub fn readFloatBE(&mut self) -> f32 {
        // Java: return Float.intBitsToFloat(this.readIntBE());
        f32::from_bits(self.readIntBE() as u32)
    }

    // Java: double readDouble()
    pub fn readDouble(&mut self) -> f64 {
        // Java: return Double.longBitsToDouble(this.readLong());
        f64::from_bits(self.readLong() as u64)
    }

    // Java: double readDoubleBE()
    pub fn readDoubleBE(&mut self) -> f64 {
        // Java: return Double.longBitsToDouble(this.readLongBE());
        f64::from_bits(self.readLongBE() as u64)
    }

    // Java: void read(byte[] dst)
    pub fn read_all(&mut self, dst: &mut [u8]) {
        // Java: if (dst != null)
        {
            // Java: int len = dst.length;
            let len = dst.len() as i32;
            // Java: if (len != 0 && this.pos + len < this.length)
            if len != 0 && self.pos + len < self.length {
                // Java: System.arraycopy(this.bytes, ++this.pos, dst, 0, len);
                self.pos += 1;
                let src = self.pos as usize;
                dst.copy_from_slice(&self.bytes_ref()[src..src + len as usize]);
                // Java: this.pos += len - 1;
                self.pos += len - 1;
            }
        }
    }

    // Java: void read(byte[] dst, int n)
    pub fn read_n(&mut self, dst: &mut [u8], n: i32) {
        // Java: if (dst != null)
        {
            // Java: int len = dst.length;
            let len = dst.len() as i32;
            // Java: if (len != 0 && this.pos + n < this.length)
            if len != 0 && self.pos + n < self.length {
                // Java: System.arraycopy(this.bytes, ++this.pos, dst, 0, len < n ? len : n);
                self.pos += 1;
                let src = self.pos as usize;
                let copy_len = if len < n { len } else { n } as usize;
                dst[..copy_len].copy_from_slice(&self.bytes_ref()[src..src + copy_len]);
                // Java: this.pos += n - 1;
                self.pos += n - 1;
            }
        }
    }

    // Java: String readString(int n)
    pub fn readString(&mut self, n: i32) -> String {
        // Java: StringBuffer result = new StringBuffer();
        let mut result = String::new();
        // Java: for(int i = 0; i < n; ++i)
        for _i in 0..n {
            // Java: int b1 = this.bytes[++this.pos] & 255;
            self.pos += 1;
            let b1 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            // Java: int b2 = this.bytes[++this.pos] & 255;
            self.pos += 1;
            let b2 = self.bytes_ref()[self.pos as usize] as i32 & 255;
            // Java: if (b1 == 0 && b2 == 0)
            if b1 == 0 && b2 == 0 {
                // Java: break;
                break;
            }
            // Java: if (b2 == 240)
            if b2 == 240 {
                // Java: result.append((char)b1);
                if let Some(c) = char::from_u32(b1 as u32) {
                    result.push(c);
                }
            }
            // Java: else
            else {
                // Java: result.append((char)(b1 | b2 << 8));
                if let Some(c) = char::from_u32((b1 | (b2 << 8)) as u32) {
                    result.push(c);
                }
            }
        }
        // Java: return result.toString();
        result
    }

    // Java: int readColor()
    pub fn readColor(&mut self) -> i32 {
        // Java: int red = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let red = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int green = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let green = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int blue = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let blue = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: ++this.pos;
        self.pos += 1;
        // Java: return -16777216 | red << 16 | green << 8 | blue;
        -16777216i32 | (red << 16) | (green << 8) | blue
    }

    // Java: int readColorRe()
    pub fn readColorRe(&mut self) -> i32 {
        // Java: int blue = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let blue = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int green = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let green = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int red = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let red = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: ++this.pos;
        self.pos += 1;
        // Java: return -16777216 | red << 16 | green << 8 | blue;
        -16777216i32 | (red << 16) | (green << 8) | blue
    }

    // Java: int readRGB()
    pub fn readRGB(&mut self) -> i32 {
        // Java: int red = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let red = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int green = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let green = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int blue = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let blue = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: return -16777216 | red << 16 | green << 8 | blue;
        -16777216i32 | (red << 16) | (green << 8) | blue
    }

    // Java: int readBGR()
    pub fn readBGR(&mut self) -> i32 {
        // Java: int blue = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let blue = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int green = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let green = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int red = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let red = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: return -16777216 | red << 16 | green << 8 | blue;
        -16777216i32 | (red << 16) | (green << 8) | blue
    }

    // Java: int readARGB()
    pub fn readARGB(&mut self) -> i32 {
        // Java: int alpha = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let alpha = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int red = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let red = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int green = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let green = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int blue = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let blue = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: return alpha << 24 | red << 16 | green << 8 | blue;
        (alpha << 24) | (red << 16) | (green << 8) | blue
    }

    // Java: int readBGRA()
    pub fn readBGRA(&mut self) -> i32 {
        // Java: int blue = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let blue = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int green = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let green = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int red = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let red = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: int alpha = this.bytes[++this.pos] & 255;
        self.pos += 1;
        let alpha = self.bytes_ref()[self.pos as usize] as i32 & 255;
        // Java: return alpha << 24 | red << 16 | green << 8 | blue;
        (alpha << 24) | (red << 16) | (green << 8) | blue
    }
}

impl IBufferReader for BufferReader {
    fn close(&mut self) { self.close(); }
    fn reset(&mut self) { self.reset(); }
    fn skip(&mut self, var1: i32) { self.skip(var1); }
    fn available(&self) -> i32 { self.available() }
    fn getCurrentPos(&self) -> i32 { self.getCurrentPos() }
    fn setCurrentPos(&mut self, var1: i32) { self.setCurrentPos(var1); }
    fn readBoolean(&mut self) -> bool { self.readBoolean() }
    fn readByte_one(&mut self) -> i8 { self.readByte_one() }
    fn readUnsignedByte_one(&mut self) -> i32 { self.readUnsignedByte_one() }
    fn readByte_n(&mut self, var1: i32) -> Vec<u8> { self.readByte_n(var1) }
    fn readUnsignedByte_n(&mut self, var1: i32) -> Vec<i32> { self.readUnsignedByte_n(var1) }
    fn readShort_one(&mut self) -> i16 { self.readShort_one() }
    fn readShortBE(&mut self) -> i16 { self.readShortBE() }
    fn readUnsignedShort(&mut self) -> i32 { self.readUnsignedShort() }
    fn readUnsignedShortBE(&mut self) -> i32 { self.readUnsignedShortBE() }
    fn readShort_n(&mut self, var1: i32) -> Vec<i32> { self.readShort_n(var1) }
    fn readInt_one(&mut self) -> i32 { self.readInt_one() }
    fn readIntBE(&mut self) -> i32 { self.readIntBE() }
    fn readInt_n(&mut self, var1: i32) -> Vec<i32> { self.readInt_n(var1) }
    fn readUnsignedInt_one(&mut self) -> i32 { self.readUnsignedInt_one() }
    fn readUnsignedIntBE(&mut self) -> i32 { self.readUnsignedIntBE() }
    fn readUnsignedInt_n(&mut self, var1: i32) -> Vec<i32> { self.readUnsignedInt_n(var1) }
    fn readLong(&mut self) -> i64 { self.readLong() }
    fn readLongBE(&mut self) -> i64 { self.readLongBE() }
    fn readFloat(&mut self) -> f32 { self.readFloat() }
    fn readFloatBE(&mut self) -> f32 { self.readFloatBE() }
    fn readDouble(&mut self) -> f64 { self.readDouble() }
    fn readDoubleBE(&mut self) -> f64 { self.readDoubleBE() }
    fn read_all(&mut self, var1: &mut [u8]) { self.read_all(var1); }
    fn read_n(&mut self, var1: &mut [u8], var2: i32) { self.read_n(var1, var2); }
    fn readString(&mut self, var1: i32) -> String { self.readString(var1) }
    fn readColor(&mut self) -> i32 { self.readColor() }
    fn readColorRe(&mut self) -> i32 { self.readColorRe() }
    fn readRGB(&mut self) -> i32 { self.readRGB() }
    fn readBGR(&mut self) -> i32 { self.readBGR() }
    fn readARGB(&mut self) -> i32 { self.readARGB() }
    fn readBGRA(&mut self) -> i32 { self.readBGRA() }
}
