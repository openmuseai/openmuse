#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

use super::LittleEndianOutput::LittleEndianOutput;
use super::DelayableLittleEndianOutput::DelayableLittleEndianOutput;

// Java: public class LittleEndianByteArrayOutputStream implements LittleEndianOutput, DelayableLittleEndianOutput
pub struct LittleEndianByteArrayOutputStream {
    // Java field: __buf: &[u8]
    _buf: Vec<u8>,
    // Java field: int _endIndex
    _endIndex: i32,
    // Java field: int _writeIndex
    pub(crate) _writeIndex: i32,
}

// Java: public class LittleEndianByteArrayOutputStream implements LittleEndianOutput, DelayableLittleEndianOutput
impl LittleEndianByteArrayOutputStream {
    // Java: LittleEndianByteArrayOutputStream(_buf: &[u8], int startOffset, int maxWriteLen)
    pub fn new(buf: Vec<u8>, startOffset: i32, maxWriteLen: i32) -> Self {
        // Java: super();
        // Java: if (startOffset >= 0 && startOffset <= buf.length)
        if startOffset >= 0 && startOffset <= buf.len() as i32
        {
            // Java: this._buf = buf;
            // Java: this._writeIndex = startOffset;
            // Java: this._endIndex = startOffset + maxWriteLen;
            let _endIndex = startOffset + maxWriteLen;
            // Java: if (this._endIndex < startOffset || this._endIndex > buf.length)
            if _endIndex < startOffset || _endIndex > buf.len() as i32
            {
                // Java: throw new IllegalArgumentException("calculated end index (" + this._endIndex + ") is out of allowable range (" + this._writeIndex + ".." + buf.length + ")");
                panic!("calculated end index ({}) is out of allowable range ({}..{})", _endIndex, startOffset, buf.len());
            }
            LittleEndianByteArrayOutputStream {
                _buf: buf,
                _endIndex,
                _writeIndex: startOffset,
            }
        }
        // Java: else
        else
        {
            // Java: throw new IllegalArgumentException("Specified startOffset (" + startOffset + ") is out of allowable range (0.." + buf.length + ")");
            panic!("Specified startOffset ({}) is out of allowable range (0..{})", startOffset, buf.len());
        }
    }

    // Java: LittleEndianByteArrayOutputStream(_buf: &[u8], int startOffset)
    pub fn new_from_offset(buf: Vec<u8>, startOffset: i32) -> Self {
        // Java: this(buf, startOffset, buf.length - startOffset);
        let len = buf.len() as i32;
        Self::new(buf, startOffset, len - startOffset)
    }

    // Java: void checkPosition(int i)
    fn checkPosition(&self, i: i32) {
        // Java: if (i > this._endIndex - this._writeIndex)
        if i > self._endIndex - self._writeIndex
        {
            // Java: throw new RuntimeException("Buffer overrun");
            panic!("Buffer overrun");
        }
    }

    // Java: void writeByte(int v)
    pub fn writeByte_impl(&mut self, v: i32) {
        // Java: this.checkPosition(1);
        self.checkPosition(1);
        // Java: this._buf[this._writeIndex++] = (byte)v;
        self._buf[self._writeIndex as usize] = v as u8;
        self._writeIndex += 1;
    }

    // Java: void writeDouble(double v)
    pub fn writeDouble_impl(&mut self, v: f64) {
        // Java: this.writeLong(Double.doubleToLongBits(v));
        self.writeLong_impl(f64::to_bits(v) as i64);
    }

    // Java: void writeInt(int v)
    pub fn writeInt_impl(&mut self, v: i32) {
        // Java: this.checkPosition(4);
        self.checkPosition(4);
        // Java: int i = this._writeIndex;
        let mut i = self._writeIndex as usize;
        // Java: this._buf[i++] = (byte)(v >>> 0 & 255);
        self._buf[i] = (((v as u32) >> 0) & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 8 & 255);
        self._buf[i] = (((v as u32) >> 8) & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 16 & 255);
        self._buf[i] = (((v as u32) >> 16) & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 24 & 255);
        self._buf[i] = (((v as u32) >> 24) & 255) as u8; i += 1;
        // Java: this._writeIndex = i;
        self._writeIndex = i as i32;
    }

    // Java: void writeLong(long v)
    pub fn writeLong_impl(&mut self, v: i64) {
        // Java: this.writeInt((int)(v >> 0));
        self.writeInt_impl((v >> 0) as i32);
        // Java: this.writeInt((int)(v >> 32));
        self.writeInt_impl((v >> 32) as i32);
    }

    // Java: void writeShort(int v)
    pub fn writeShort_impl(&mut self, v: i32) {
        // Java: this.checkPosition(2);
        self.checkPosition(2);
        // Java: int i = this._writeIndex;
        let mut i = self._writeIndex as usize;
        // Java: this._buf[i++] = (byte)(v >>> 0 & 255);
        self._buf[i] = (((v as u32) >> 0) & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 8 & 255);
        self._buf[i] = (((v as u32) >> 8) & 255) as u8; i += 1;
        // Java: this._writeIndex = i;
        self._writeIndex = i as i32;
    }

    // Java: void write(_b: &[u8])
    pub fn write_impl(&mut self, b: &[u8]) {
        // Java: int len = b.length;
        let len = b.len() as i32;
        // Java: this.checkPosition(len);
        self.checkPosition(len);
        // Java: System.arraycopy(b, 0, this._buf, this._writeIndex, len);
        let w = self._writeIndex as usize;
        self._buf[w..w + len as usize].copy_from_slice(b);
        // Java: this._writeIndex += len;
        self._writeIndex += len;
    }

    // Java: void write(_b: &[u8], int offset, int len)
    pub fn write_with_offset_impl(&mut self, b: &[u8], offset: i32, len: i32) {
        // Java: this.checkPosition(len);
        self.checkPosition(len);
        // Java: System.arraycopy(b, offset, this._buf, this._writeIndex, len);
        let w = self._writeIndex as usize;
        self._buf[w..w + len as usize].copy_from_slice(&b[offset as usize..(offset + len) as usize]);
        // Java: this._writeIndex += len;
        self._writeIndex += len;
    }

    // Java: int getWriteIndex()
    pub fn getWriteIndex(&self) -> i32 {
        // Java: return this._writeIndex;
        self._writeIndex
    }

    // Java: LittleEndianOutput createDelayedOutput(int size)
    pub fn createDelayedOutput_impl(&mut self, size: i32) -> Box<dyn LittleEndianOutput> {
        // Java: this.checkPosition(size);
        self.checkPosition(size);
        // Java: LittleEndianOutput result = new LittleEndianByteArrayOutputStream(this._buf, this._writeIndex, size);
        let result = LittleEndianByteArrayOutputStream::new(
            self._buf.clone(), self._writeIndex, size
        );
        // Java: this._writeIndex += size;
        self._writeIndex += size;
        // Java: return result;
        Box::new(result)
    }
}

impl LittleEndianOutput for LittleEndianByteArrayOutputStream {
    fn writeByte(&mut self, var1: i32) { self.writeByte_impl(var1); }
    fn writeShort(&mut self, var1: i32) { self.writeShort_impl(var1); }
    fn writeInt(&mut self, var1: i32) { self.writeInt_impl(var1); }
    fn writeLong(&mut self, var1: i64) { self.writeLong_impl(var1); }
    fn writeDouble(&mut self, var1: f64) { self.writeDouble_impl(var1); }
    fn write(&mut self, var1: &[u8]) { self.write_impl(var1); }
    fn write_with_offset(&mut self, var1: &[u8], var2: i32, var3: i32) { self.write_with_offset_impl(var1, var2, var3); }
}

impl DelayableLittleEndianOutput for LittleEndianByteArrayOutputStream {
    fn createDelayedOutput(&mut self, var1: i32) -> Box<dyn LittleEndianOutput> {
        self.createDelayedOutput_impl(var1)
    }
}
