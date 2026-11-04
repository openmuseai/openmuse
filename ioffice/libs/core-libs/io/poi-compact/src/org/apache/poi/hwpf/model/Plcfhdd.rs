// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::io::Write;

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: int SIZE
pub(crate) const SIZE: i32 = 24;

// Java: public class TagHdd
#[derive(Clone)]
pub struct TagHdd {
    // Java field: int evenHeader
    pub(crate) evenHeader: i32,
    // Java field: int oddHeader
    pub(crate) oddHeader: i32,
    // Java field: int evenFooter
    pub(crate) evenFooter: i32,
    // Java field: int oddFooter
    pub(crate) oddFooter: i32,
    // Java field: int firstHeader
    pub(crate) firstHeader: i32,
    // Java field: int firstFooter
    pub(crate) firstFooter: i32,
}

// Java: public class Plcfhdd
pub struct Plcfhdd {
    // Java field: ArrayList<TagHdd> _hddList
    pub(crate) _hddList: Vec<TagHdd>,
    // Java field: int[] _end
    _end: Vec<i32>,
}

// Java: public class Plcfhdd
impl Plcfhdd {
    // Java: Plcfhdd(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = Plcfhdd { _hddList: Vec::new(), _end: vec![0; 2] };
      // Java: int count = (size - 8) / 24;
      let count = (size - 8) / 24;
      // Java: for(int i = 0; i < count; ++i)
      for _i in 0..count
      {
         // Java: this._hddList.add(new TagHdd(tableStream, offset));
         this._hddList.push(TagHdd::new_from_stream(table_stream, offset));
         // Java: offset += 24;
         offset += 24;
      }
      this
    }

    // Java: Plcfhdd()
    pub fn new() -> Self {
      // Java: super();
      Plcfhdd { _hddList: Vec::new(), _end: vec![0; 2] }
    }

    // Java: ArrayList<TagHdd> getHddList()
    pub fn getHddList(&self) -> &Vec<TagHdd> {
      // Java: return this._hddList;
      &self._hddList
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
      // Java: return this._hddList.size();
      self._hddList.len() as i32
    }

    // Java: TagHdd getTagHDD(int index)
    pub fn getTagHDD(&self, index: i32) -> Option<&TagHdd> {
      // Java: return index >= 0 && index < this._hddList.size() ? (TagHdd)this._hddList.get(index) : null;
      if index >= 0 && (index as usize) < self._hddList.len() {
          Some(&self._hddList[index as usize])
      } else {
          None
      }
    }

    // Java: void addTagHdd(int evenH, int oddH, int evenF, int oddF, int firstH, int firstF)
    pub fn addTagHdd(&mut self, even_h: i32, odd_h: i32, even_f: i32, odd_f: i32, first_h: i32, first_f: i32) {
      // Java: TagHdd tagHdd = new TagHdd();
      let mut tag_hdd = TagHdd::new();
      // Java: tagHdd.evenHeader = evenH;
      tag_hdd.evenHeader = even_h;
      // Java: tagHdd.oddHeader = oddH;
      tag_hdd.oddHeader = odd_h;
      // Java: tagHdd.evenFooter = evenF;
      tag_hdd.evenFooter = even_f;
      // Java: tagHdd.oddFooter = oddF;
      tag_hdd.oddFooter = odd_f;
      // Java: tagHdd.firstHeader = firstH;
      tag_hdd.firstHeader = first_h;
      // Java: tagHdd.firstFooter = firstF;
      tag_hdd.firstFooter = first_f;
      // Java: this._hddList.add(tagHdd);
      self._hddList.push(tag_hdd);
    }

    // Java: void addEnd(int secondToLast, int last)
    pub fn addEnd(&mut self, second_to_last: i32, last: i32) {
      // Java: this._end[0] = secondToLast;
      self._end[0] = second_to_last;
      // Java: this._end[1] = last;
      self._end[1] = last;
    }

    // Java: void addFirstTagHdd()
    pub fn addFirstTagHdd(&mut self) {
      // Java: TagHdd first = new TagHdd();
      let first = TagHdd::new();
      // Java: this._hddList.add(0, first);
      self._hddList.insert(0, first);
    }

    // Java: void writeTo(HWPFOutputStream tableStream)
    pub fn writeTo(&self, table_stream: &mut HWPFOutputStream) {
      // Java: if (this.getSize() > 0)
      if self.getSize() > 0
      {
         // Java: int count = this.getSize();
         let count = self.getSize();
         // Java: for(int i = 0; i < count; ++i)
         for i in 0..count
         {
            // Java: tableStream.write(((TagHdd)this._hddList.get(i)).toByteArray());
            let _ = table_stream.write(&self._hddList[i as usize].toByteArray());
         }
         // Java: byte[] end = new byte[8];
         let mut end = vec![0u8; 8];
         // Java: LittleEndian.putInt(end, 0, this._end[0]);
         le_put_int(&mut end, 0, self._end[0]);
         // Java: LittleEndian.putInt(end, 4, this._end[1]);
         le_put_int(&mut end, 4, self._end[1]);
         // Java: tableStream.write(end);
         let _ = table_stream.write(&end);
      }
    }

}

// Java: public class TagHdd
impl TagHdd {
    // Java: TagHdd()
    pub fn new() -> Self {
         // Java: super();
         TagHdd { evenHeader: 0, oddHeader: 0, evenFooter: 0, oddFooter: 0, firstHeader: 0, firstFooter: 0 }
    }

    // Java: TagHdd(DocumentInputStream buf, int offset)
    pub fn new_from_stream(buf: &mut DocumentInputStream, offset: i32) -> Self {
         // Java: super();
         let mut this = TagHdd { evenHeader: 0, oddHeader: 0, evenFooter: 0, oddFooter: 0, firstHeader: 0, firstFooter: 0 };
         // Java: buf.seek((long)offset);
         buf.seek(offset as i64);
         // Java: this.evenHeader = buf.readInt();
         this.evenHeader = buf.readInt();
         // Java: offset += 4;
         // Java: this.oddHeader = buf.readInt();
         this.oddHeader = buf.readInt();
         // Java: offset += 4;
         // Java: this.evenFooter = buf.readInt();
         this.evenFooter = buf.readInt();
         // Java: offset += 4;
         // Java: this.oddFooter = buf.readInt();
         this.oddFooter = buf.readInt();
         // Java: offset += 4;
         // Java: this.firstHeader = buf.readInt();
         this.firstHeader = buf.readInt();
         // Java: offset += 4;
         // Java: this.firstFooter = buf.readInt();
         this.firstFooter = buf.readInt();
         this
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: return super.hashCode();
         0
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&TagHdd>) -> bool {
         // Java: if (o == null)
         if o.is_none()
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: TagHdd tagHdd = (TagHdd)o;
            let tag_hdd = o.unwrap();
            // Java: return this.evenHeader == tagHdd.evenHeader && this.oddHeader == tagHdd.oddHeader && this.evenFooter == tagHdd.evenFooter && this.oddFooter == tagHdd.oddFooter && this.firstHeader == tagHdd.firstHeader && this.firstFooter == tagHdd.firstFooter;
            return self.evenHeader == tag_hdd.evenHeader && self.oddHeader == tag_hdd.oddHeader && self.evenFooter == tag_hdd.evenFooter && self.oddFooter == tag_hdd.oddFooter && self.firstHeader == tag_hdd.firstHeader && self.firstFooter == tag_hdd.firstFooter;
         }
    }

    // Java: byte[] toByteArray()
    pub fn toByteArray(&self) -> Vec<u8> {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] buf = new byte[24];
         let mut buf = vec![0u8; 24];
         // Java: LittleEndian.putInt(buf, offset, this.evenHeader);
         le_put_int(&mut buf, offset, self.evenHeader);
         // Java: offset += 4;
         offset += 4;
         // Java: LittleEndian.putInt(buf, offset, this.oddHeader);
         le_put_int(&mut buf, offset, self.oddHeader);
         // Java: offset += 4;
         offset += 4;
         // Java: LittleEndian.putInt(buf, offset, this.evenFooter);
         le_put_int(&mut buf, offset, self.evenFooter);
         // Java: offset += 4;
         offset += 4;
         // Java: LittleEndian.putInt(buf, offset, this.oddFooter);
         le_put_int(&mut buf, offset, self.oddFooter);
         // Java: offset += 4;
         offset += 4;
         // Java: LittleEndian.putInt(buf, offset, this.firstHeader);
         le_put_int(&mut buf, offset, self.firstHeader);
         // Java: offset += 4;
         offset += 4;
         // Java: LittleEndian.putInt(buf, offset, this.firstFooter);
         le_put_int(&mut buf, offset, self.firstFooter);
         // Java: return buf;
         buf
    }

}
