// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.StringUtil

use std::io::Write;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

fn put_unicode_le(s: &str, buf: &mut [u8], offset: i32) {
    let o = offset as usize;
    for (i, ch) in s.encode_utf16().enumerate() {
        let bytes = ch.to_le_bytes();
        let pos = o + i * 2;
        if pos + 1 < buf.len() {
            buf[pos] = bytes[0];
            buf[pos + 1] = bytes[1];
        }
    }
}

// Java: public class BookmarkName
pub struct BookmarkName {
    // Java field: short size
    pub(crate) size: i16,
    // Java field: String name
    pub(crate) name: String,
}

// Java: public class Sttbfbkmk
pub struct Sttbfbkmk {
    // Java field: DocumentInputStream _stream
    _stream: Option<DocumentInputStream>,
    // Java field: short _fExtend
    _fExtend: i16,
    // Java field: short _nCount
    pub(crate) _nCount: i16,
    // Java field: short _cbExtra
    _cbExtra: i16,
    // Java field: ArrayList<BookmarkName> _bookmarkList
    _bookmarkList: Vec<BookmarkName>,
    // Java field: int numberOfBytes
    numberOfBytes: i32,
    // Java field: int _preBkNameIndex
    _preBkNameIndex: i32,
    // Java field: int _bkNameOffset
    _bkNameOffset: i32,
}

// Java: public class Sttbfbkmk
impl Sttbfbkmk {
    // Java: Sttbfbkmk()
    pub fn new() -> Self {
      // Java: super();
      Sttbfbkmk { _stream: None, _fExtend: -1, _nCount: 0, _cbExtra: 0, _bookmarkList: Vec::new(), numberOfBytes: 0, _preBkNameIndex: -1, _bkNameOffset: 0 }
    }

    // Java: Sttbfbkmk(DocumentInputStream stream, int offset, int size)
    pub fn new_from_stream(mut stream: DocumentInputStream, mut offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = Sttbfbkmk { _stream: None, _fExtend: -1, _nCount: 0, _cbExtra: 0, _bookmarkList: Vec::new(), numberOfBytes: 0, _preBkNameIndex: -1, _bkNameOffset: 0 };
      // Java: this._stream = stream;
      // Java: this._stream.seek((long)offset);
      stream.seek(offset as i64);
      // Java: this._fExtend = this._stream.readShort();
      this._fExtend = stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._nCount = this._stream.readShort();
      this._nCount = stream.readShort();
      // Java: offset += 2;
      offset += 2;
      // Java: this._cbExtra = this._stream.readShort();
      this._cbExtra = stream.readShort();
      // Java: this.numberOfBytes = size - 6;
      this.numberOfBytes = size - 6;
      // Java: this._bkNameOffset = offset + 2;
      this._bkNameOffset = offset + 2;
      this._stream = Some(stream);
      this
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._nCount;
      self._nCount as i32
    }

    // Java: void setSize(int count)
    pub fn setSize(&mut self, count: i32) {
      // Java: this._nCount = (short)count;
      self._nCount = count as i16;
    }

    // Java: String getBookmarkName(int index)
    pub fn getBookmarkName(&mut self, index: i32) -> String {
      // Java: IOAssert.assertTrue("index - _preBkNameIndex == 1 should be true!", index - this._preBkNameIndex == 1);
      assert!(index - self._preBkNameIndex == 1, "index - _preBkNameIndex == 1 should be true!");
      // Java: this._stream.seek((long)this._bkNameOffset);
      let stream = self._stream.as_mut().unwrap();
      stream.seek(self._bkNameOffset as i64);
      // Java: int nameSize = this._stream.readShort() * 2;
      let name_size = stream.readShort() as i32 * 2;
      // Java: byte[] bkNameBuf = new byte[nameSize];
      let mut bk_name_buf = vec![0u8; name_size as usize];
      // Java: this._stream.read(bkNameBuf, 0, nameSize);
      stream.read_off(&mut bk_name_buf, 0, name_size);
      // Java: this._bkNameOffset += 2 + nameSize;
      self._bkNameOffset += 2 + name_size;
      // Java: this._preBkNameIndex = index;
      self._preBkNameIndex = index;
      // Java: return new String(bkNameBuf, 0, nameSize, "UTF-16LE");
      let chars: Vec<u16> = bk_name_buf.chunks(2)
          .map(|c| u16::from_le_bytes([c[0], if c.len() > 1 { c[1] } else { 0 }]))
          .collect();
      String::from_utf16_lossy(&chars)
    }

    // Java: void addBookmarkName(BookmarkName bookmarkName)
    pub fn addBookmarkName(&mut self, bookmark_name: BookmarkName) {
      // Java: this._bookmarkList.add(bookmarkName);
      let num_bytes = bookmark_name.getNumberOfBytes();
      self._bookmarkList.push(bookmark_name);
      // Java: this.numberOfBytes += bookmarkName.getNumberOfBytes();
      self.numberOfBytes += num_bytes;
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
      // Java: if (this._nCount > 0)
      if self._nCount > 0
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[6];
         let mut buf = vec![0u8; 6];
         // Java: LittleEndian.putShort(buf, offset, this._fExtend);
         le_put_short(&mut buf, offset, self._fExtend);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._nCount);
         le_put_short(&mut buf, offset, self._nCount);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(buf, offset, this._cbExtra);
         le_put_short(&mut buf, offset, self._cbExtra);
         // Java: offset += 2;
         offset += 2;
         // Java: out.write(buf);
         let _ = out.write(&buf);
         // Java: offset = 0;
         offset = 0;
         // Java: buf = new byte[this.numberOfBytes];
         let mut buf = vec![0u8; self.numberOfBytes as usize];
         // Java: for(int i = 0; i < this._nCount; ++i)
         for i in 0..self._nCount as usize
         {
            // Java: BookmarkName bookmark = (BookmarkName)this._bookmarkList.get(i);
            let bookmark = &self._bookmarkList[i];
            // Java: bookmark.serialize(buf, offset);
            bookmark.serialize(&mut buf, offset);
            // Java: offset += bookmark.getNumberOfBytes();
            offset += bookmark.getNumberOfBytes();
         }
         // Java: out.write(buf);
         let _ = out.write(&buf);
      }
    }

}

// Java: public class BookmarkName
impl BookmarkName {
    // Java: BookmarkName()
    pub fn new() -> Self {
         // Java: super();
         BookmarkName { size: 0, name: String::new() }
    }

    // Java: int getNumberOfBytes()
    pub fn getNumberOfBytes(&self) -> i32 {
         // Java: return 2 + this.size * 2;
         2 + (self.size as i32) * 2
    }

    // Java: void serialize(byte[] buf, int offset)
    pub fn serialize(&self, buf: &mut [u8], offset: i32) {
         // Java: LittleEndian.putShort(buf, offset, this.size);
         le_put_short(buf, offset, self.size);
         // Java: StringUtil.putUnicodeLE(this.name, buf, offset + 2);
         put_unicode_le(&self.name, buf, offset + 2);
    }

}
