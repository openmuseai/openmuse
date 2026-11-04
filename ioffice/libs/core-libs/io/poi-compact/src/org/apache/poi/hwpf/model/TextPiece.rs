// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.UnsupportedEncodingException
// Java: import org.apache.poi.hwpf.util.KStringBuffer
use crate::org::apache::poi::hwpf::util::KStringBuffer::KStringBuffer;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use super::PieceDescriptor::PieceDescriptor;
use std::sync::{Arc, Mutex};
use std::io::Write;

// Java: public class TextPiece extends PropertyNode implements Comparable
pub struct TextPiece {
    // Java field: boolean usesUnicode
    pub(crate) usesUnicode: bool,
    // Java field: int charCount
    pub(crate) charCount: i32,
    // Java field: int bytesLength
    pub(crate) bytesLength: i32,
    // Java field: boolean isTemped
    isTemped: bool,
    // Java field: DocumentInputStream text
    text: Option<DocumentInputStream>,
    // Java field: int offset
    offset: i32,
    // Java field: PieceDescriptor pd
    pub(crate) pd: Arc<Mutex<PieceDescriptor>>,
    // Java field: char[] singleCharArray
    pub(crate) singleCharArray: Vec<char>,
    pub(crate) _buf: KStringBuffer,
    pub(crate) cpStart: i32,
    pub(crate) cpEnd: i32,
}

// Java: public class TextPiece extends PropertyNode implements Comparable
impl TextPiece {
    // Java: TextPiece(int start, int end, DocumentInputStream text, int offset, int len, PieceDescriptor pd)
    pub fn new(start: i32, end: i32, text: DocumentInputStream, offset: i32, len: i32, pd: PieceDescriptor) -> Self {
      // Java: super(start, end, buildInitSB(text, offset, len, pd));
      let uses_unicode = pd.isUnicode();
      // Java: this.text = text;
      // Java: this.offset = offset;
      // Java: this.usesUnicode = pd.isUnicode();
      // Java: this.pd = pd;
      // Java: this.charCount = end - start;
      // Java: this.bytesLength = len;
      // Java: if (end < start)
        if end < start
      {
         // Java: throw new IllegalStateException("Told we're of negative size! start=" + start + " end=" + end);
         panic!("Told we're of negative size! start={} end={}", start, end);
      }
      TextPiece {
          usesUnicode: uses_unicode,
          charCount: end - start,
          bytesLength: len,
          isTemped: true,
          text: Some(text),
          offset,
          pd: Arc::new(Mutex::new(pd)),
          singleCharArray: vec!['\0'; 1],
          _buf: KStringBuffer::new_empty(),
          cpStart: start,
          cpEnd: end,
      }
    }

    // Java: static KStringBuffer buildInitSB(DocumentInputStream text, int offset, int len, PieceDescriptor pd)
    fn buildInitSB(_text: &DocumentInputStream, _offset: i32, _len: i32, _pd: &PieceDescriptor) -> KStringBuffer {
      // Java: return new KStringBuffer("");
      KStringBuffer::new_empty()
    }

    // Java: void discardDiskContent()
    pub fn discardDiskContent(&mut self) {
      // Java: this.isTemped = false;
      self.isTemped = false;
    }

    // Java: void forceUnicode()
    pub fn forceUnicode(&mut self) {
      // Java: if (!this.usesUnicode)
        if !self.usesUnicode
      {
         // Java: this.usesUnicode = true;
         self.usesUnicode = true;
         // Java: this.pd.forceUnicode();
         self.pd.lock().unwrap().forceUnicode();
      }
    }

    // Java: boolean isUnicode()
    pub fn isUnicode(&self) -> bool {
      // Java: return this.usesUnicode;
      self.usesUnicode
    }

    // Java: PieceDescriptor getPieceDescriptor()
    pub fn getPieceDescriptor(&self) -> Arc<Mutex<PieceDescriptor>> {
      // Java: return this.pd;
      self.pd.clone()
    }

    // Java: KStringBuffer getStringBuffer()
    pub fn getStringBuffer(&mut self) -> &mut KStringBuffer {
      // Java: return (KStringBuffer)this._buf;
      &mut self._buf
    }

    // Java: byte[] getRawBytes()
    pub fn getRawBytes(&self) -> Vec<u8> {
      // Java: try
      {
         // Java: return ((KStringBuffer)this._buf).toString().getBytes(this.usesUnicode ? "UTF-16LE" : "Cp1252");
         let s = self._buf.to_string();
         if self.usesUnicode {
             s.encode_utf16().flat_map(|c| c.to_le_bytes()).collect()
         } else {
             s.bytes().collect()
         }
      }
      // Java: catch (UnsupportedEncodingException var2)
      // Java: throw new RuntimeException("Your Java is broken! It doesn't know about basic, required character encodings!");
    }

    // Java: void writeRawBytes(OutputStream out)
    pub fn writeRawBytes(&mut self, out: &mut dyn Write) {
      // Java: KStringBuffer sb = this.getStringBuffer();
      let sb_str = self._buf.to_string();
      // Java: if (!this.usesUnicode)
        if !self.usesUnicode
      {
         // Java: byte[] b = sb.toString().getBytes("Cp1252");
         let b: Vec<u8> = sb_str.bytes().collect();
         // Java: out.write(b);
         let _ = out.write_all(&b);
      }
      // Java: else
        else
      {
         // Java: int nTotal = sb.length();
         let n_total = sb_str.len();
         // Java: char[] pBuffer = sb.getDirectBuffer();
         let p_buffer: Vec<char> = sb_str.chars().collect();
         // Java: int nBlockChars = 2048;
         let n_block_chars: usize = 2048;
         // Java: byte[] b = new byte[nBlockChars << 1];
         let mut b = vec![0u8; n_block_chars << 1];
         // Java: int nWrite;
         // Java: for(int nTotalWritten = 0; nTotalWritten < nTotal; nTotalWritten += nWrite)
         let mut n_total_written: usize = 0;
         while n_total_written < n_total
         {
            // Java: nWrite = Math.min(nTotal - nTotalWritten, nBlockChars);
            let n_write = std::cmp::min(n_total - n_total_written, n_block_chars);
            // Java: for(int i = 0; i < nWrite; ++i)
                for i in 0..n_write
            {
               // Java: char c = pBuffer[nTotalWritten + i];
               let c = p_buffer[n_total_written + i] as u16;
               // Java: b[i << 1] = (byte)(c & 255);
               b[i << 1] = (c & 255) as u8;
               // Java: b[(i << 1) + 1] = (byte)(c >> 8);
               b[(i << 1) + 1] = (c >> 8) as u8;
            }
            // Java: out.write(b, 0, nWrite << 1);
            let _ = out.write_all(&b[0..n_write << 1]);
            n_total_written += n_write;
         }
      }
    }

    // Java: String substring(int start, int end)
    pub fn substring(&mut self, start: i32, end: i32) -> String {
      // Java: if (!this.isTemped)
        if !self.isTemped
      {
         // Java: KStringBuffer buf = this.getStringBuffer();
         let buf_str = self._buf.to_string();
         let buf_len = buf_str.len() as i32;
         // Java: if (start < 0)
            if start < 0
         {
            // Java: throw new StringIndexOutOfBoundsException("Can't request a substring before 0 - asked for " + start);
            panic!("Can't request a substring before 0 - asked for {}", start);
         }
         // Java: else if (end > buf.length())
            else if end > buf_len
         {
            // Java: throw new StringIndexOutOfBoundsException("Index " + end + " out of range 0 -> " + buf.length());
            panic!("Index {} out of range 0 -> {}", end, buf_len);
         }
         // Java: else if (end < start)
            else if end < start
         {
            // Java: throw new StringIndexOutOfBoundsException("Asked for text from " + start + " to " + end + ", which has an end before the start!");
            panic!("Asked for text from {} to {}, which has an end before the start!", start, end);
         }
         // Java: else
            else
         {
            // Java: return buf.substring(start, end);
            return buf_str[start as usize..end as usize].to_string();
         }
      }
      // Java: else
        else
      {
         // Java: try
         {
            // Java: int len = this.usesUnicode ? (end - start) * 2 : end - start;
            let len = if self.usesUnicode { (end - start) * 2 } else { end - start };
            // Java: byte[] data = new byte[len];
            let mut data = vec![0u8; len as usize];
            // Java: this.text.seek((long)(this.offset + (this.usesUnicode ? start * 2 : start)));
            let seek_pos = self.offset + if self.usesUnicode { start * 2 } else { start };
            if let Some(ref mut text) = self.text {
                text.seek(seek_pos as i64);
            // Java: this.text.read(data, 0, len);
                text.read_off(&mut data, 0, len);
            }
            // Java: String str;
            // Java: if (this.pd.isUnicode())
            let str_result;
            let pd = self.pd.lock().unwrap();
                if pd.isUnicode()
            {
               // Java: str = new String(data, 0, len, "UTF-16LE");
               let u16_data: Vec<u16> = data.chunks(2).map(|c| u16::from_le_bytes([c[0], c.get(1).copied().unwrap_or(0)])).collect();
               str_result = String::from_utf16_lossy(&u16_data);
            }
            // Java: else
                else
            {
               // Java: str = new String(data, 0, len, "Cp1252");
               str_result = data.iter().map(|&b| b as char).collect();
            }
            // Java: return str;
            return str_result;
         }
         // Java: catch (IOException var6)
         // Java: throw new RuntimeException("Your Java is broken! It doesn't know about basic, required character encodings!");
      }
    }

    // Java: char[] getSubString(int start, int end)
    pub fn getSubString(&mut self, start: i32, end: i32) -> Vec<char> {
      // Java: if (!this.isTemped)
        if !self.isTemped
      {
         // Java: KStringBuffer buf = this.getStringBuffer();
         let buf_str = self._buf.to_string();
         let buf_len = buf_str.len() as i32;
         // Java: if (start < 0)
            if start < 0
         {
            // Java: throw new StringIndexOutOfBoundsException("Can't request a substring before 0 - asked for " + start);
            panic!("Can't request a substring before 0 - asked for {}", start);
         }
         // Java: else if (end > buf.length())
            else if end > buf_len
         {
            // Java: throw new StringIndexOutOfBoundsException("Index " + end + " out of range 0 -> " + buf.length());
            panic!("Index {} out of range 0 -> {}", end, buf_len);
         }
         // Java: else if (end < start)
            else if end < start
         {
            // Java: throw new StringIndexOutOfBoundsException("Asked for text from " + start + " to " + end + ", which has an end before the start!");
            panic!("Asked for text from {} to {}, which has an end before the start!", start, end);
         }
         // Java: else
            else
         {
            // Java: return buf.substring(start, end).toCharArray();
            return buf_str[start as usize..end as usize].chars().collect();
         }
      }
      // Java: else if (end - start == 1)
        else if end - start == 1
      {
         // Java: if (this.usesUnicode)
            if self.usesUnicode
         {
            // Java: this.text.seek((long)(this.offset + (start << 1)));
            if let Some(ref mut text) = self.text {
                text.seek((self.offset + (start << 1)) as i64);
            // Java: this.singleCharArray[0] = (char)this.text.readUShort();
                self.singleCharArray[0] = char::from_u32(text.readUShort() as u32).unwrap_or('\0');
            }
         }
         // Java: else
            else
         {
            // Java: this.text.seek((long)(this.offset + start));
            if let Some(ref mut text) = self.text {
                text.seek((self.offset + start) as i64);
            // Java: this.singleCharArray[0] = CompressedExceptiveCharacter.convertTo(this.text.readUByte());
                self.singleCharArray[0] = char::from_u32(text.readUByte() as u32).unwrap_or('\0');
            }
         }
         // Java: return this.singleCharArray;
         return self.singleCharArray.clone();
      }
      // Java: else
        else
      {
         // Java: int len = this.usesUnicode ? end - start << 1 : end - start;
         let len = if self.usesUnicode { (end - start) << 1 } else { end - start };
         // Java: int startOff = this.offset + (this.usesUnicode ? start << 1 : start);
         let start_off = self.offset + if self.usesUnicode { start << 1 } else { start };
         // Java: int endOff = startOff + len;
         let end_off = start_off + len;
         // Java: int bulk = 512;
         let _bulk = 512;
         // Java: PageBuffer data = PageBuffer.Pool.InstanceBig.obtain();
         // Java: byte[] buf = data.getByteArray();
         let mut buf = vec![0u8; 512];
         // Java: char[] content = null;
         // Java: if (this.usesUnicode)
         let content_len = if self.usesUnicode
         {
            // Java: content = CharArrayPool.sInstance.obtain(len >> 1);
            (len >> 1) as usize
         }
         // Java: else
            else
         {
            // Java: content = CharArrayPool.sInstance.obtain(len);
            len as usize
         };
         let mut content = vec!['\0'; content_len];
         // Java: int destOff = 0;
         let mut dest_off: usize = 0;
         // Java: char[] var22;
         // Java: try
         {
            // Java: this.text.seek((long)startOff);
            if let Some(ref mut text) = self.text {
                text.seek(start_off as i64);
            // Java: int off = startOff;
            let mut off = start_off;
            // Java: while(off < endOff)
                while off < end_off
            {
               // Java: int leftLen = endOff - off;
               let left_len = end_off - off;
               // Java: int readLen = leftLen < 512 ? leftLen : 512;
               let read_len = if left_len < 512 { left_len } else { 512 };
               // Java: this.text.read(buf, 0, readLen);
               text.read_off(&mut buf, 0, read_len);
               // Java: off += readLen;
               off += read_len;
               // Java: if (this.usesUnicode)
                    if self.usesUnicode
               {
                  // Java: for(int i = 0; i < readLen; ++i)
                  let mut i = 0i32;
                  while i < read_len
                  {
                     // Java: int var25 = destOff++;
                     let var25 = dest_off; dest_off += 1;
                     // Java: int var10002 = buf[i] & 255;
                     let var10002 = (buf[i as usize] & 255) as u32;
                     // Java: ++i;
                     i += 1;
                     // Java: content[var25] = (char)(var10002 | buf[i] << 8);
                     let hi = (buf[i as usize] as u32) << 8;
                     content[var25] = char::from_u32(var10002 | hi).unwrap_or('\0');
                     i += 1;
                  }
               }
               // Java: else
                    else
               {
                  // Java: for(int i = 0; i < readLen; ++i)
                  for i in 0..read_len as usize
                  {
                     // Java: content[destOff++] = CompressedExceptiveCharacter.convertTo(buf[i]);
                     content[dest_off] = char::from_u32(buf[i] as u32).unwrap_or('\0');
                     dest_off += 1;
                  }
               }
            }
            }
            // Java: var22 = content;
         }
         // Java: catch (IOException var18)
         // Java: throw new RuntimeException("Your Java is broken! It doesn't know about basic, required character encodings!");
         // Java: finally
         {
            // Java: PageBuffer.Pool.InstanceBig.recycle(data);
         }
         // Java: return var22;
         return content;
      }
    }

    // Java: void adjustForDelete(int start, int length)
    pub fn adjustForDelete(&mut self, start: i32, length: i32) {
      // Java: int myStart = this.getStart();
      let my_start = self.getStart();
      // Java: int myEnd = this.getEnd();
      let my_end = self.getEnd();
      // Java: int end = start + length;
      let end = start + length;
      // Java: if (start <= myEnd && end >= myStart)
        if start <= my_end && end >= my_start
      {
         // Java: int overlapStart = Math.max(myStart, start);
         let _overlap_start = std::cmp::max(my_start, start);
         // Java: int overlapEnd = Math.min(myEnd, end);
         let _overlap_end = std::cmp::min(my_end, end);
         // Java: this.getStringBuffer().delete(overlapStart, overlapEnd);
      }
      // Java: super.adjustForDelete(start, length);
    }

    // Java: int characterLength()
    pub fn characterLength(&self) -> i32 {
      // Java: return this.charCount;
      self.charCount
    }

    // Java: int bytesLength()
    pub fn bytesLength(&self) -> i32 {
      // Java: return this.bytesLength;
      self.bytesLength
    }

    // Java: KStringBuffer insert(int offset, String str)
    pub fn insert_str(&mut self, offset: i32, str_val: &str) -> &mut KStringBuffer {
      // Java: if (this.isTemped)
        if self.isTemped
      {
         // Java: this.isTemped = false;
         self.isTemped = false;
         // Java: String strValue;
         // Java: try
         {
            // Java: byte[] data = new byte[this.bytesLength];
            let mut data = vec![0u8; self.bytesLength as usize];
            // Java: this.text.seek((long)offset);
            if let Some(ref mut text) = self.text {
                text.seek(offset as i64);
            // Java: this.text.read(data, 0, this.bytesLength);
                text.read_off(&mut data, 0, self.bytesLength);
            }
            // Java: if (this.pd.isUnicode())
            let pd = self.pd.lock().unwrap();
                if pd.isUnicode()
            {
               // Java: strValue = new String(data, 0, this.bytesLength, "UTF-16LE");
            }
            // Java: else
                else
            {
               // Java: strValue = new String(data, 0, this.bytesLength, "Cp1252");
            }
         }
         // Java: catch (IOException var5)
         // Java: throw new RuntimeException("Your Java is broken! It doesn't know about basic, required character encodings!");
         // Java: this._buf = new StringBuffer(strValue);
      }
      // Java: this.charCount += str.length();
      self.charCount += str_val.len() as i32;
      // Java: this.bytesLength += str.length() * (this.usesUnicode ? 2 : 1);
      self.bytesLength += str_val.len() as i32 * if self.usesUnicode { 2 } else { 1 };
      // Java: return this.getStringBuffer().insert(offset, str);
      &mut self._buf
    }

    // Java: void insert(int offset, char[] text, int start, int length)
    pub fn insert_chars(&mut self, _offset: i32, _text: &[char], _start: i32, length: i32) {
      // Java: this.charCount += length;
      self.charCount += length;
      // Java: this.bytesLength += length;
      self.bytesLength += length;
      // Java: if (this.usesUnicode)
        if self.usesUnicode
      {
         // Java: this.bytesLength += length;
         self.bytesLength += length;
      }
      // Java: this.getStringBuffer().insert(offset, text, start, length);
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return super.hashCode();
      0
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, _o: &TextPiece) -> bool {
      // Java: if (o == null)
      // Java: return false;
      // Java: else if (!this.limitsAreEqual(o))
      // Java: return false;
      // Java: else
      {
         // Java: TextPiece tp = (TextPiece)o;
         // Java: return this.getStringBuffer().toString().equals(tp.getStringBuffer().toString()) && tp.usesUnicode == this.usesUnicode && this.pd.equals(tp.pd);
         false
      }
    }

    // Java: int getCP()
    pub fn getCP(&self) -> i32 {
      // Java: return this.getStart();
      self.getStart()
    }

    pub fn getStart(&self) -> i32 {
        self.cpStart
    }

    pub fn getEnd(&self) -> i32 {
        self.cpEnd
    }

    pub fn setStart(&mut self, start: i32) {
        self.cpStart = start;
    }

    pub fn setEnd(&mut self, end: i32) {
        self.cpEnd = end;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return String.format("\n[FC:%d-%d, CP:%d-%d] or\t[FC:%xh-%xh, CP:%xh-%xh]", this.offset, this.offset + this.bytesLength, this.getStart(), this.getEnd(), this.offset, this.offset + this.bytesLength, this.getStart(), this.getEnd());
      format!("\n[FC:{}-{}, CP:{}-{}] or\t[FC:{:x}h-{:x}h, CP:{:x}h-{:x}h]",
          self.offset, self.offset + self.bytesLength, self.getStart(), self.getEnd(),
          self.offset, self.offset + self.bytesLength, self.getStart(), self.getEnd())
    }

}
