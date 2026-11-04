// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;

// Java: public class DataInputBlock
pub struct DataInputBlock {
    // Java field: BlockBuf _blockBuf
    pub(crate) _blockBuf: Option<BlockBuf>,
    // Java field: byte[] _buf
    pub(crate) _buf: Vec<u8>,
    // Java field: int _readIndex
    pub(crate) _readIndex: i32,
    // Java field: int _maxIndex
    pub(crate) _maxIndex: i32,
}

// Java: public class DataInputBlock
impl DataInputBlock {
    // Java: DataInputBlock(BlockBuf blockBuf, int startOffset)
    pub fn new(mut blockBuf: BlockBuf, startOffset: i32) -> Self {
        // Java: super();
        // Java: this._blockBuf = blockBuf;
        // Java: this._buf = this._blockBuf.getBuf();
        let buf = blockBuf.getBuf().to_vec();
        // Java: this._readIndex = startOffset;
        let _readIndex = startOffset;
        // Java: this._maxIndex = this._buf.length;
        let _maxIndex = buf.len() as i32;
        DataInputBlock {
            _blockBuf: Some(blockBuf),
            _buf: buf,
            _readIndex,
            _maxIndex,
        }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this._blockBuf.recycle();
        if let Some(bb) = self._blockBuf.take() {
            bb.recycle();
        }
        // Java: this._buf = null;
        self._buf.clear();
    }

    // Java: int deltaSeek(int delta)
    pub fn deltaSeek(&mut self, delta: i32) -> i32 {
        // Java: this._readIndex += delta;
        self._readIndex += delta;
        // Java: return this._readIndex;
        self._readIndex
    }

    // Java: int deltaSeekAvailable(int delta)
    pub fn deltaSeekAvailable(&self, delta: i32) -> i32 {
        // Java: int new_pos = this._readIndex + delta;
        let new_pos = self._readIndex + delta;
        // Java: return new_pos >= 0 && new_pos <= this._maxIndex ? this._maxIndex - new_pos : -1;
        if new_pos >= 0 && new_pos <= self._maxIndex { self._maxIndex - new_pos } else { -1 }
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: return this._maxIndex - this._readIndex;
        self._maxIndex - self._readIndex
    }

    // Java: byte readByte()
    pub fn readByte(&mut self) -> i8 {
        // Java: return this._buf[this._readIndex++];
        let val = self._buf[self._readIndex as usize] as i8;
        self._readIndex += 1;
        val
    }

    // Java: int readUByte()
    pub fn readUByte(&mut self) -> i32 {
        // Java: return this._buf[this._readIndex++] & 255;
        let val = (self._buf[self._readIndex as usize] as i32) & 255;
        self._readIndex += 1;
        val
    }

    // Java: int readUShortLE()
    pub fn readUShortLE(&mut self) -> i32 {
        // Java: int i = this._readIndex;
        let mut i = self._readIndex as usize;
        // Java: int b0 = this._buf[i++] & 255;
        let b0 = (self._buf[i] as i32) & 255;
        i += 1;
        // Java: int b1 = this._buf[i++] & 255;
        let b1 = (self._buf[i] as i32) & 255;
        i += 1;
        // Java: this._readIndex = i;
        self._readIndex = i as i32;
        // Java: return (b1 << 8) + (b0 << 0);
        (b1 << 8) + b0
    }

    // Java: int readIntLE()
    pub fn readIntLE(&mut self) -> i32 {
        // Java: int i = this._readIndex;
        let mut i = self._readIndex as usize;
        // Java: int b0 = this._buf[i++] & 255;
        let b0 = (self._buf[i] as i32) & 255; i += 1;
        // Java: int b1 = this._buf[i++] & 255;
        let b1 = (self._buf[i] as i32) & 255; i += 1;
        // Java: int b2 = this._buf[i++] & 255;
        let b2 = (self._buf[i] as i32) & 255; i += 1;
        // Java: int b3 = this._buf[i++] & 255;
        let b3 = (self._buf[i] as i32) & 255; i += 1;
        // Java: this._readIndex = i;
        self._readIndex = i as i32;
        // Java: return (b3 << 24) + (b2 << 16) + (b1 << 8) + (b0 << 0);
        (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
    }

    // Java: long readLongLE()
    pub fn readLongLE(&mut self) -> i64 {
        // Java: int i = this._readIndex;
        let mut i = self._readIndex as usize;
        // Java: int b0 = this._buf[i++] & 255;
        let b0 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b1 = this._buf[i++] & 255;
        let b1 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b2 = this._buf[i++] & 255;
        let b2 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b3 = this._buf[i++] & 255;
        let b3 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b4 = this._buf[i++] & 255;
        let b4 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b5 = this._buf[i++] & 255;
        let b5 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b6 = this._buf[i++] & 255;
        let b6 = (self._buf[i] as i64) & 255; i += 1;
        // Java: int b7 = this._buf[i++] & 255;
        let b7 = (self._buf[i] as i64) & 255; i += 1;
        // Java: this._readIndex = i;
        self._readIndex = i as i32;
        // Java: return ((long)b7 << 56) + ((long)b6 << 48) + ((long)b5 << 40) + ((long)b4 << 32) + ((long)b3 << 24) + (long)(b2 << 16) + (long)(b1 << 8) + (long)(b0 << 0);
        (b7 << 56) + (b6 << 48) + (b5 << 40) + (b4 << 32) + (b3 << 24) + (b2 << 16) + (b1 << 8) + b0
    }

    // Java: void readFully(byte[] buf, int off, int len)
    pub fn readFully(&mut self, buf: &mut [u8], off: i32, len: i32) {
        // Java: System.arraycopy(this._buf, this._readIndex, buf, off, len);
        let src_start = self._readIndex as usize;
        let dst_start = off as usize;
        let length = len as usize;
        buf[dst_start..dst_start + length].copy_from_slice(&self._buf[src_start..src_start + length]);
        // Java: this._readIndex += len;
        self._readIndex += len;
    }

    // Java: BlockBuf getBlockBuf()
    pub fn getBlockBuf(&self) -> Option<&BlockBuf> {
        // Java: return this._blockBuf;
        self._blockBuf.as_ref()
    }

    // Java: void setPosition(int position)
    pub fn setPosition(&mut self, position: i32) {
        // Java: this._readIndex = position;
        self._readIndex = position;
    }
}
