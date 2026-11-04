use std::sync::OnceLock;
use super::LittleEndianOutput::LittleEndianOutput;

// Java field: int[] sizeTable
static SIZE_TABLE: OnceLock<Vec<i32>> = OnceLock::new();

fn get_size_table() -> &'static Vec<i32> {
    SIZE_TABLE.get_or_init(|| vec![9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999, i32::MAX])
}

// Java: public class LittleEndianByteArrayOutput implements LittleEndianOutput
pub struct LittleEndianByteArrayOutput {
    // Java field: byte[] _buf
    buf: Vec<u8>,
    // Java field: int _endIndex
    end_index: i32,
    // Java field: int _writeIndex
    pub(crate) write_index: i32,
    // Java field: int _capacityIndex
    capacity_index: i32,
}

// Java: public class LittleEndianByteArrayOutput implements LittleEndianOutput
impl LittleEndianByteArrayOutput {
    // Java: LittleEndianByteArrayOutput()
    pub fn new() -> Self {
        // Java: super();
        let st = get_size_table();
        // Java: this._endIndex = sizeTable[this._capacityIndex];
        let end_index = st[0];
        // Java: this._buf = new byte[sizeTable[this._capacityIndex]];
        let buf = vec![0u8; st[0] as usize];
        LittleEndianByteArrayOutput {
            buf,
            end_index,
            write_index: 0,
            capacity_index: 0,
        }
    }

    // Java: void checkPosition(int i)
    fn check_position(&mut self, i: i32) {
        let st = get_size_table();
        // Java: if (i > this._endIndex - this._writeIndex)
        if i > self.end_index - self.write_index {
            // Java: while(i > this._endIndex - this._writeIndex)
            while i > self.end_index - self.write_index {
                // Java: ++this._capacityIndex;
                self.capacity_index += 1;
                // Java: if (this._capacityIndex > sizeTable.length - 1)
                if self.capacity_index > st.len() as i32 - 1 {
                    // Java: throw new IllegalArgumentException("Specified data size can't be stored");
                    panic!("Specified data size can't be stored");
                }
                // Java: this._endIndex = sizeTable[this._capacityIndex];
                self.end_index = st[self.capacity_index as usize];
            }
            // Java: byte[] _bufTmp = new byte[sizeTable[this._capacityIndex]];
            let mut buf_tmp = vec![0u8; st[self.capacity_index as usize] as usize];
            // Java: for(int j = 0; j < this._writeIndex; ++j)
            for j in 0..self.write_index as usize {
                // Java: _bufTmp[j] = this._buf[j];
                buf_tmp[j] = self.buf[j];
            }
            // Java: this._buf = _bufTmp;
            self.buf = buf_tmp;
        }
    }

    // Java: void writeByte(int v)
    pub fn write_byte(&mut self, v: i32) {
        // Java: this.checkPosition(1);
        self.check_position(1);
        // Java: this._buf[this._writeIndex++] = (byte)v;
        self.buf[self.write_index as usize] = v as u8;
        self.write_index += 1;
    }

    // Java: void writeDouble(double v)
    pub fn write_double(&mut self, v: f64) {
        // Java: this.writeLong(Double.doubleToLongBits(v));
        self.write_long(if v.is_nan(){0x7ff8000000000000u64 as i64}else{v.to_bits()as i64});
    }

    // Java: void writeInt(int v)
    pub fn write_int(&mut self, v: i32) {
        // Java: this.checkPosition(4);
        self.check_position(4);
        // Java: int i = this._writeIndex;
        let mut i = self.write_index as usize;
        // Java: this._buf[i++] = (byte)(v >>> 0 & 255);
        self.buf[i] = (v & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 8 & 255);
        self.buf[i] = ((v as u32 >> 8) & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 16 & 255);
        self.buf[i] = ((v as u32 >> 16) & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 24 & 255);
        self.buf[i] = ((v as u32 >> 24) & 255) as u8; i += 1;
        // Java: this._writeIndex = i;
        self.write_index = i as i32;
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
        // Java: this.checkPosition(2);
        self.check_position(2);
        // Java: int i = this._writeIndex;
        let mut i = self.write_index as usize;
        // Java: this._buf[i++] = (byte)(v >>> 0 & 255);
        self.buf[i] = (v & 255) as u8; i += 1;
        // Java: this._buf[i++] = (byte)(v >>> 8 & 255);
        self.buf[i] = ((v as u32 >> 8) & 255) as u8; i += 1;
        // Java: this._writeIndex = i;
        self.write_index = i as i32;
    }

    // Java: void write(byte[] b)
    pub fn write_bytes(&mut self, b: &[u8]) {
        // Java: int len = b.length;
        let len = b.len() as i32;
        // Java: this.checkPosition(len);
        self.check_position(len);
        // Java: System.arraycopy(b, 0, this._buf, this._writeIndex, len);
        let wi = self.write_index as usize;
        self.buf[wi..wi + len as usize].copy_from_slice(b);
        // Java: this._writeIndex += len;
        self.write_index += len;
    }

    // Java: void write(byte[] b, int offset, int len)
    pub fn write_bytes_offset(&mut self, b: &[u8], offset: i32, len: i32) {
        // Java: this.checkPosition(len);
        self.check_position(len);
        // Java: System.arraycopy(b, offset, this._buf, this._writeIndex, len);
        let wi = self.write_index as usize;
        let off = offset as usize;
        let l = len as usize;
        self.buf[wi..wi + l].copy_from_slice(&b[off..off + l]);
        // Java: this._writeIndex += len;
        self.write_index += len;
    }

    // Java: byte[] getBytes()
    pub fn get_bytes(&self) -> Vec<u8> {
        // Java: byte[] tmp = new byte[this._writeIndex];
        // Java: System.arraycopy(this._buf, 0, tmp, 0, this._writeIndex);
        // Java: return tmp;
        self.buf[..self.write_index as usize].to_vec()
    }

    // Java: int getWriteIndex()
    pub fn get_write_index(&self) -> i32 {
        // Java: return this._writeIndex;
        self.write_index
    }
}

impl LittleEndianOutput for LittleEndianByteArrayOutput {
    fn writeByte(&mut self, var1: i32) { self.write_byte(var1) }
    fn writeShort(&mut self, var1: i32) { self.write_short(var1) }
    fn writeInt(&mut self, var1: i32) { self.write_int(var1) }
    fn writeLong(&mut self, var1: i64) { self.write_long(var1) }
    fn writeDouble(&mut self, var1: f64) { self.write_double(var1) }
    fn write(&mut self, var1: &[u8]) { self.write_bytes(var1) }
    fn write_with_offset(&mut self, var1: &[u8], var2: i32, var3: i32) { self.write_bytes_offset(var1, var2, var3) }
}
