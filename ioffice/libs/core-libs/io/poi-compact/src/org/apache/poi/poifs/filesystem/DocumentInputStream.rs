// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.moffice.crash.FileDamagedException
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import org.apache.poi.poifs.common.BlockBuf
use crate::org::apache::poi::poifs::common::BlockBuf::BlockBuf;
// Java: import org.apache.poi.poifs.storage.DataInputBlock
use crate::org::apache::poi::poifs::storage::DataInputBlock::DataInputBlock;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.LittleEndianRandomAccessInput

use super::POIFSDocument::POIFSDocument;

// Java field: int EOF
pub(crate) const EOF: i32 = -1;
// Java field: int SIZE_SHORT
pub(crate) const SIZE_SHORT: i32 = 2;
// Java field: int SIZE_INT
pub(crate) const SIZE_INT: i32 = 4;
// Java field: int SIZE_LONG
pub(crate) const SIZE_LONG: i32 = 8;

fn le_get_ushort(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset + 1] as i32 & 0xFF) << 8)
}
fn le_get_int(buf: &[u8], offset: usize) -> i32 {
    (buf[offset] as i32 & 0xFF) | ((buf[offset+1] as i32 & 0xFF) << 8) |
    ((buf[offset+2] as i32 & 0xFF) << 16) | ((buf[offset+3] as i32 & 0xFF) << 24)
}
fn le_get_long(buf: &[u8], offset: usize) -> i64 {
    le_get_int(buf, offset) as i64 & 0xFFFFFFFF | ((le_get_int(buf, offset+4) as i64) << 32)
}

// Java: public class DocumentInputStream extends InputStream implements LittleEndianRandomAccessInput
pub struct DocumentInputStream {
    // Java field: int _current_offset
    pub(crate) _current_offset: i32,
    // Java field: int _marked_offset
    pub(crate) _marked_offset: i32,
    // Java field: int _document_size
    pub(crate) _document_size: i32,
    // Java field: boolean _closed
    pub(crate) _closed: bool,
    // Java field: POIFSDocument _document
    pub(crate) _document: Option<*mut POIFSDocument>,
    // Java field: DataInputBlock _currentBlock
    pub(crate) _currentBlock: Option<DataInputBlock>,
    // Java field: byte[] _tmpbuf
    pub(crate) _tmpbuf: Vec<u8>,
}

// Java: public class DocumentInputStream extends InputStream implements LittleEndianRandomAccessInput
impl DocumentInputStream {
    // Java: DocumentInputStream()
    pub fn new_empty() -> Self {
        // Java: super();
        // Java: this._document_size = 0;
        // Java: this._currentBlock = null;
        // Java: this._document = null;
        DocumentInputStream {
            _current_offset: 0,
            _marked_offset: 0,
            _document_size: 0,
            _closed: false,
            _document: None,
            _currentBlock: None,
            _tmpbuf: vec![0u8; 8],
        }
    }

    // Java: DocumentInputStream(DocumentEntry document)
    pub fn new_from_entry() -> Self {
        // Java: super();
        // Java: if (!(document instanceof DocumentNode))
        // Java: throw new IOException("Cannot open internal document storage");
        // Java: else
        // Java: this._current_offset = 0;
        // Java: this._marked_offset = 0;
        // Java: this._document_size = document.getSize();
        // Java: this._closed = false;
        // Java: this._document = ((DocumentNode)document).getDocument();
        // Java: BlockBuf buf = BlockBuf.obtain(this._document.getInputBlockSize());
        // Java: this._currentBlock = new DataInputBlock(buf, 0);
        // Java: this.readBlock(this._current_offset);
        Self::new_empty()
    }

    // Java: DocumentInputStream(POIFSDocument document)
    pub fn new_from_document(document: &mut POIFSDocument) -> Self {
        // Java: super();
        // Java: this._current_offset = 0;
        // Java: this._marked_offset = 0;
        // Java: this._document_size = document.getSize();
        let doc_size = document.getSize();
        // Java: this._closed = false;
        // Java: this._document = document;
        // Java: BlockBuf buf = BlockBuf.obtain(this._document.getInputBlockSize());
        let buf = BlockBuf::obtain(document.getInputBlockSize());
        // Java: this._currentBlock = new DataInputBlock(buf, 0);
        let mut current_block = DataInputBlock::new(buf, 0);
        // Java: this.readBlock(this._current_offset);
        let _ = document.readBlock(0, &mut current_block);
        DocumentInputStream {
            _current_offset: 0,
            _marked_offset: 0,
            _document_size: doc_size,
            _closed: false,
            _document: Some(document as *mut POIFSDocument),
            _currentBlock: Some(current_block),
            _tmpbuf: vec![0u8; 8],
        }
    }

    // Java: void readBlock(int offset)
    fn readBlock_internal(&mut self, offset: i32) {
        // Java: try
        // Java: this._document.readBlock(offset, this._currentBlock);
        if let (Some(doc_ptr), Some(ref mut block)) = (self._document, &mut self._currentBlock) {
            unsafe {
                let doc = &mut *doc_ptr;
                let _ = doc.readBlock(offset, block);
            }
        }
        // Java: catch (IOException e)
        // Java: IOLog.e("DocumentInputStream", "", e);
        // Java: throw new RuntimeException(String.format("Cannot read stream, offset: %d, document %s", offset, this._document.getName()));
    }

    // Java: int available()
    pub fn available(&self) -> i32 {
        // Java: if (this._closed)
        if self._closed {
            // Java: throw new IllegalStateException("cannot perform requested operation on a closed stream");
            panic!("cannot perform requested operation on a closed stream");
        }
        // Java: else
        // Java: return this._document_size - this._current_offset;
        self._document_size - self._current_offset
    }

    // Java: void close()
    pub fn close(&mut self) {
        // Java: this._closed = true;
        self._closed = true;
        // Java: if (null != this._currentBlock)
        if let Some(mut cb) = self._currentBlock.take() {
            // Java: this._currentBlock.dispose();
            cb.dispose();
            // Java: this._currentBlock = null;
        }
    }

    // Java: void mark(int ignoredReadlimit)
    pub fn mark(&mut self, _ignoredReadlimit: i32) {
        // Java: this._marked_offset = this._current_offset;
        self._marked_offset = self._current_offset;
    }

    // Java: boolean markSupported()
    pub fn markSupported(&self) -> bool {
        // Java: return true;
        true
    }

    // Java: int read()
    pub fn read_byte(&mut self) -> i32 {
        // Java: this.dieIfClosed();
        self.dieIfClosed();
        // Java: if (this.atEOD())
        if self.atEOD() {
            // Java: return -1;
            return -1;
        }
        // Java: else
        // Java: int result = this._currentBlock.readUByte();
        let result = self._currentBlock.as_mut().unwrap().readUByte();
        // Java: ++this._current_offset;
        self._current_offset += 1;
        // Java: if (this._currentBlock.available() < 1)
        if self._currentBlock.as_ref().unwrap().available() < 1 {
            // Java: this.readBlock(this._current_offset);
            self.readBlock_internal(self._current_offset);
        }
        // Java: return result;
        result
    }

    // Java: int read(byte[] b)
    pub fn read(&mut self, b: &mut [u8]) -> i32 {
        // Java: return this.read(b, 0, b.length);
        let len = b.len();
        self.read_off(b, 0, len as i32)
    }

    // Java: int read(byte[] b, int off, int len)
    pub fn read_off(&mut self, b: &mut [u8], off: i32, len: i32) -> i32 {
        // Java: this.dieIfClosed();
        self.dieIfClosed();
        // Java: if (len == 0)
        if len == 0 {
            // Java: return 0;
            return 0;
        }
        // Java: else if (this.atEOD())
        else if self.atEOD() {
            // Java: return -1;
            return -1;
        }
        // Java: else
        // Java: int limit = this.available();
        let mut limit = self.available();
        // Java: if (len < limit)
        if len < limit {
            // Java: limit = len;
            limit = len;
        }
        // Java: this.readFully(b, off, limit);
        self.readFully_off(b, off, limit);
        // Java: return limit;
        limit
    }

    // Java: void reset()
    pub fn reset(&mut self) {
        // Java: this._current_offset = this._marked_offset;
        self._current_offset = self._marked_offset;
        // Java: this.readBlock(this._current_offset);
        self.readBlock_internal(self._current_offset);
    }

    // Java: long seek(long pos)
    pub fn seek(&mut self, pos: i64) -> i64 {
        // Java: int _pos = (int)pos;
        let _pos = pos as i32;
        // Java: if (_pos == this._current_offset)
        if _pos == self._current_offset {
            // Java: return pos;
            return pos;
        }
        // Java: else if (pos >= 0L && pos <= (long)this._document_size)
        else if pos >= 0 && pos <= self._document_size as i64 {
            // Java: if (this._closed)
            if self._closed {
                // Java: throw new IllegalStateException("Operand stream alread closed");
                panic!("Operand stream alread closed");
            }
            // Java: else
            // Java: int delta = (int)(pos - (long)this._current_offset);
            let delta = (pos - self._current_offset as i64) as i32;
            // Java: if (this._currentBlock.deltaSeekAvailable(delta) > 0)
            if self._currentBlock.as_ref().unwrap().deltaSeekAvailable(delta) > 0 {
                // Java: this._currentBlock.deltaSeek(delta);
                self._currentBlock.as_mut().unwrap().deltaSeek(delta);
            }
            // Java: else
            else {
                // Java: this.readBlock(_pos);
                self.readBlock_internal(_pos);
            }
            // Java: this._current_offset = _pos;
            self._current_offset = _pos;
            // Java: return (long)this._current_offset;
            self._current_offset as i64
        }
        // Java: else
        else {
            // Java: throw new IllegalArgumentException("Position for seeking out of range");
            panic!("Position for seeking out of range");
        }
    }

    // Java: long skip(long n)
    pub fn skip(&mut self, n: i64) -> i64 {
        // Java: if (this._closed)
        if self._closed {
            // Java: throw new IllegalStateException("cannot perform requested operation on a closed stream");
            panic!("cannot perform requested operation on a closed stream");
        }
        // Java: else if (n < 0L)
        else if n < 0 {
            // Java: return 0L;
            return 0;
        }
        // Java: else
        // Java: int new_offset = this._current_offset + (int)n;
        let mut new_offset = self._current_offset + n as i32;
        // Java: if (new_offset > this._document_size)
        if new_offset > self._document_size {
            // Java: new_offset = this._document_size;
            new_offset = self._document_size;
        }
        // Java: int rval = new_offset - this._current_offset;
        let rval = new_offset - self._current_offset;
        // Java: this._current_offset = new_offset;
        self._current_offset = new_offset;
        // Java: if (rval < this._currentBlock.available())
        if rval < self._currentBlock.as_ref().unwrap().available() {
            // Java: this._currentBlock.deltaSeek(rval);
            self._currentBlock.as_mut().unwrap().deltaSeek(rval);
        }
        // Java: else
        else {
            // Java: this.readBlock(this._current_offset);
            self.readBlock_internal(self._current_offset);
        }
        // Java: return (long)rval;
        rval as i64
    }

    // Java: long tell()
    pub fn tell(&self) -> i64 {
        // Java: return (long)this._current_offset;
        self._current_offset as i64
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
        // Java: return this._document_size;
        self._document_size
    }

    // Java: void dieIfClosed()
    fn dieIfClosed(&self) {
        // Java: if (this._closed)
        if self._closed {
            // Java: throw new IOException("cannot perform requested operation on a closed stream");
            panic!("cannot perform requested operation on a closed stream");
        }
    }

    // Java: boolean atEOD()
    fn atEOD(&self) -> bool {
        // Java: return this._current_offset == this._document_size;
        self._current_offset == self._document_size
    }

    // Java: void checkAvaliable(int requestedSize)
    fn checkAvaliable(&self, requestedSize: i32) {
        // Java: if (this._closed)
        if self._closed {
            // Java: throw new IllegalStateException("cannot perform requested operation on a closed stream");
            panic!("cannot perform requested operation on a closed stream");
        }
        // Java: else if (requestedSize > this._document_size - this._current_offset)
        else if requestedSize > self._document_size - self._current_offset {
            // Java: throw new FileDamagedException("Buffer underrun - requested " + requestedSize + " bytes but " + (this._document_size - this._current_offset) + " was available");
            panic!("Buffer underrun - requested {} bytes but {} was available", requestedSize, self._document_size - self._current_offset);
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
        self.readFully_off(buf, 0, len);
    }

    // Java: short readShort()
    pub fn readShort(&mut self) -> i16 {
        // Java: return (short)this.readUShort();
        self.readUShort() as i16
    }

    // Java: void readFully(byte[] buf, int off, int len)
    pub fn readFully_off(&mut self, buf: &mut [u8], off: i32, len: i32) {
        // Java: this.checkAvaliable(len);
        self.checkAvaliable(len);
        // Java: int blockAvailable = this._currentBlock.available();
        let mut blockAvailable = self._currentBlock.as_ref().unwrap().available();
        // Java: if (blockAvailable > len)
        if blockAvailable > len {
            // Java: this._currentBlock.readFully(buf, off, len);
            self._currentBlock.as_mut().unwrap().readFully(buf, off, len);
            // Java: this._current_offset += len;
            self._current_offset += len;
        }
        // Java: else
        else {
            // Java: int remaining = len;
            let mut remaining = len;
            // Java: boolean blockIsExpiring = true;
            let mut blockIsExpiring = true;
            // Java: int reqSize = blockAvailable;
            let mut reqSize = blockAvailable;
            // Java: int posBuf = off;
            let mut posBuf = off;
            // Java: while(remaining > 0)
            while remaining > 0 {
                // Java: this._currentBlock.readFully(buf, posBuf, reqSize);
                self._currentBlock.as_mut().unwrap().readFully(buf, posBuf, reqSize);
                // Java: remaining -= reqSize;
                remaining -= reqSize;
                // Java: posBuf += reqSize;
                posBuf += reqSize;
                // Java: this._current_offset += reqSize;
                self._current_offset += reqSize;
                // Java: if (blockIsExpiring)
                if blockIsExpiring {
                    // Java: this.readBlock(this._current_offset);
                    self.readBlock_internal(self._current_offset);
                    // Java: blockAvailable = this._currentBlock.available();
                    blockAvailable = self._currentBlock.as_ref().unwrap().available();
                    // Java: blockIsExpiring = remaining >= blockAvailable;
                    blockIsExpiring = remaining >= blockAvailable;
                    // Java: if (blockIsExpiring)
                    if blockIsExpiring {
                        // Java: reqSize = blockAvailable;
                        reqSize = blockAvailable;
                    }
                    // Java: else
                    else {
                        // Java: reqSize = remaining;
                        reqSize = remaining;
                    }
                }
            }
        }
    }

    // Java: long readLong()
    pub fn readLong(&mut self) -> i64 {
        // Java: this.checkAvaliable(8);
        self.checkAvaliable(8);
        // Java: int blockAvailable = this._currentBlock.available();
        let blockAvailable = self._currentBlock.as_ref().unwrap().available();
        // Java: long result;
        let result: i64;
        // Java: if (blockAvailable > 8)
        if blockAvailable > 8 {
            // Java: result = this._currentBlock.readLongLE();
            result = self._currentBlock.as_mut().unwrap().readLongLE();
        }
        // Java: else if (blockAvailable == 8)
        else if blockAvailable == 8 {
            // Java: result = this._currentBlock.readLongLE();
            result = self._currentBlock.as_mut().unwrap().readLongLE();
            // Java: this.readBlock(this._current_offset + 8);
            self.readBlock_internal(self._current_offset + 8);
        }
        // Java: else
        else {
            // Java: if (blockAvailable > 0)
            if blockAvailable > 0 {
                // Java: this._currentBlock.readFully(this._tmpbuf, 0, blockAvailable);
                self._currentBlock.as_mut().unwrap().readFully(&mut self._tmpbuf, 0, blockAvailable);
            }
            // Java: this.readBlock(this._current_offset + blockAvailable);
            self.readBlock_internal(self._current_offset + blockAvailable);
            // Java: this._currentBlock.readFully(this._tmpbuf, blockAvailable, 8 - blockAvailable);
            self._currentBlock.as_mut().unwrap().readFully(&mut self._tmpbuf, blockAvailable, 8 - blockAvailable);
            // Java: result = LittleEndian.getLong(this._tmpbuf, 0);
            result = le_get_long(&self._tmpbuf, 0);
        }
        // Java: this._current_offset += 8;
        self._current_offset += 8;
        // Java: return result;
        result
    }

    // Java: int readInt()
    pub fn readInt(&mut self) -> i32 {
        // Java: this.checkAvaliable(4);
        self.checkAvaliable(4);
        // Java: int blockAvailable = this._currentBlock.available();
        let blockAvailable = self._currentBlock.as_ref().unwrap().available();
        // Java: int result;
        let result: i32;
        // Java: if (blockAvailable > 4)
        if blockAvailable > 4 {
            // Java: result = this._currentBlock.readIntLE();
            result = self._currentBlock.as_mut().unwrap().readIntLE();
        }
        // Java: else if (blockAvailable == 4)
        else if blockAvailable == 4 {
            // Java: result = this._currentBlock.readIntLE();
            result = self._currentBlock.as_mut().unwrap().readIntLE();
            // Java: this.readBlock(this._current_offset + 4);
            self.readBlock_internal(self._current_offset + 4);
        }
        // Java: else
        else {
            // Java: if (blockAvailable > 0)
            if blockAvailable > 0 {
                // Java: this._currentBlock.readFully(this._tmpbuf, 0, blockAvailable);
                self._currentBlock.as_mut().unwrap().readFully(&mut self._tmpbuf, 0, blockAvailable);
            }
            // Java: this.readBlock(this._current_offset + blockAvailable);
            self.readBlock_internal(self._current_offset + blockAvailable);
            // Java: this._currentBlock.readFully(this._tmpbuf, blockAvailable, 4 - blockAvailable);
            self._currentBlock.as_mut().unwrap().readFully(&mut self._tmpbuf, blockAvailable, 4 - blockAvailable);
            // Java: result = LittleEndian.getInt(this._tmpbuf, 0);
            result = le_get_int(&self._tmpbuf, 0);
        }
        // Java: this._current_offset += 4;
        self._current_offset += 4;
        // Java: return result;
        result
    }

    // Java: int readUShort()
    pub fn readUShort(&mut self) -> i32 {
        // Java: this.checkAvaliable(2);
        self.checkAvaliable(2);
        // Java: int blockAvailable = this._currentBlock.available();
        let blockAvailable = self._currentBlock.as_ref().unwrap().available();
        // Java: int result;
        let result: i32;
        // Java: if (blockAvailable > 2)
        if blockAvailable > 2 {
            // Java: result = this._currentBlock.readUShortLE();
            result = self._currentBlock.as_mut().unwrap().readUShortLE();
        }
        // Java: else if (blockAvailable == 2)
        else if blockAvailable == 2 {
            // Java: result = this._currentBlock.readUShortLE();
            result = self._currentBlock.as_mut().unwrap().readUShortLE();
            // Java: this.readBlock(this._current_offset + 2);
            self.readBlock_internal(self._current_offset + 2);
        }
        // Java: else
        else {
            // Java: if (blockAvailable == 1)
            if blockAvailable == 1 {
                // Java: this._tmpbuf[0] = this._currentBlock.readByte();
                self._tmpbuf[0] = self._currentBlock.as_mut().unwrap().readByte() as u8;
                // Java: this.readBlock(this._current_offset + blockAvailable);
                self.readBlock_internal(self._current_offset + blockAvailable);
                // Java: this._tmpbuf[1] = this._currentBlock.readByte();
                self._tmpbuf[1] = self._currentBlock.as_mut().unwrap().readByte() as u8;
            }
            // Java: else
            else {
                // Java: this.readBlock(this._current_offset + blockAvailable);
                self.readBlock_internal(self._current_offset + blockAvailable);
                // Java: this._currentBlock.readFully(this._tmpbuf, 0, 2);
                self._currentBlock.as_mut().unwrap().readFully(&mut self._tmpbuf, 0, 2);
            }
            // Java: result = LittleEndian.getUShort(this._tmpbuf, 0);
            result = le_get_ushort(&self._tmpbuf, 0);
        }
        // Java: this._current_offset += 2;
        self._current_offset += 2;
        // Java: return result;
        result
    }

    // Java: int readUByte()
    pub fn readUByte(&mut self) -> i32 {
        // Java: this.checkAvaliable(1);
        self.checkAvaliable(1);
        // Java: int result = this._currentBlock.readUByte();
        let result = self._currentBlock.as_mut().unwrap().readUByte();
        // Java: ++this._current_offset;
        self._current_offset += 1;
        // Java: if (this._currentBlock.available() < 1)
        if self._currentBlock.as_ref().unwrap().available() < 1 {
            // Java: this.readBlock(this._current_offset);
            self.readBlock_internal(self._current_offset);
        }
        // Java: return result;
        result
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this._document.getName() + "@" + (int)this.tell();
        format!("DocumentInputStream@{}", self._current_offset)
    }
}

impl Clone for DocumentInputStream {
    fn clone(&self) -> Self {
        DocumentInputStream {
            _current_offset: self._current_offset,
            _marked_offset: self._marked_offset,
            _document_size: self._document_size,
            _closed: self._closed,
            _document: self._document,
            _currentBlock: None,
            _tmpbuf: self._tmpbuf.clone(),
        }
    }
}
