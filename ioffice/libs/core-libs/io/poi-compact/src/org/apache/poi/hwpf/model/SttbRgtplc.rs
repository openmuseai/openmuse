// Java: import cn.wps.base.assertion.IOAssert
// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use crate::org::apache::poi::hwpf::model::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use std::io::Write;

fn le_put_short(buf: &mut [u8], offset: i32, value: i16) {
    let o = offset as usize;
    let v = value as u16;
    buf[o] = (v & 0xFF) as u8;
    buf[o + 1] = (v >> 8) as u8;
}

fn le_put_int(buf: &mut [u8], offset: i32, value: i32) {
    let o = offset as usize;
    buf[o] = (value & 0xFF) as u8;
    buf[o+1] = ((value >> 8) & 0xFF) as u8;
    buf[o+2] = ((value >> 16) & 0xFF) as u8;
    buf[o+3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: int TPLC_COUNT_PER_ENTRY
pub(crate) const TPLC_COUNT_PER_ENTRY: i32 = 9;

// Java: public class SttbRgtplc
pub struct SttbRgtplc {
    // Java field: short _fExtend
    _fExtend: i16,
    // Java field: short _cData
    pub(crate) _cData: i16,
    // Java field: short _cbExtra
    _cbExtra: i16,
    // Java field: short _cchData
    _cchData: i16,
    // Java field: int[][] _entries
    _entries: Option<Vec<Option<Vec<i32>>>>,
}

// Java: public class SttbRgtplc
impl SttbRgtplc {
    // Java: SttbRgtplc(DocumentInputStream tableStream, int offset, int size)
    pub fn new_from_stream(table_stream: &mut DocumentInputStream, mut offset: i32, size: i32) -> Self {
      // Java: super();
      let mut this = SttbRgtplc { _fExtend: -1, _cData: 0, _cbExtra: 0, _cchData: 18, _entries: None };
      // Java: if (size != 0)
      if size != 0
      {
         // Java: tableStream.seek((long)offset);
         table_stream.seek(offset as i64);
         // Java: this._fExtend = tableStream.readShort();
         this._fExtend = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this._cData = tableStream.readShort();
         this._cData = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this._cbExtra = tableStream.readShort();
         this._cbExtra = table_stream.readShort();
         // Java: offset += 2;
         offset += 2;
         // Java: this.initEntries();
         this.initEntries();
         // Java: for(int i = 0; i < this._cData; ++i)
         for i in 0..this._cData as usize
         {
            // Java: this._cchData = tableStream.readShort();
            this._cchData = table_stream.readShort();
            // Java: offset += 2;
            offset += 2;
            // Java: IOAssert.assertTrue("_cchData must be either 0x0 or 0x12!", this._cchData == 0 || this._cchData == 18);
            assert!(this._cchData == 0 || this._cchData == 18, "_cchData must be either 0x0 or 0x12!");
            // Java: if (this._cchData != 0)
            if this._cchData != 0
            {
               // Java: this._entries[i] = new int[9];
               let mut entry = vec![0i32; 9];
               // Java: for(int j = 0; j < 9; ++j)
               for j in 0..9usize
               {
                  // Java: int tplc = tableStream.readInt();
                  let tplc = table_stream.readInt();
                  // Java: this._entries[i][j] = tplc;
                  entry[j] = tplc;
                  // Java: offset += 4;
                  offset += 4;
               }
               this._entries.as_mut().unwrap()[i] = Some(entry);
            }
            // Java: else
            else
            {
               // Java: this._entries[i] = null;
               this._entries.as_mut().unwrap()[i] = None;
            }
         }
      }
      this
    }

    // Java: SttbRgtplc(int entryCount)
    pub fn new(entry_count: i32) -> Self {
      // Java: super();
      let mut this = SttbRgtplc { _fExtend: -1, _cData: 0, _cbExtra: 0, _cchData: 18, _entries: None };
      // Java: this._cData = (short)entryCount;
      this._cData = entry_count as i16;
      // Java: this.initEntries();
      this.initEntries();
      this
    }

    // Java: void initEntries()
    pub fn initEntries(&mut self) {
      // Java: this._entries = new int[this._cData][];
      self._entries = Some(vec![None; self._cData as usize]);
    }

    // Java: int[] getTplcEntry(int index)
    pub fn getTplcEntry(&self, index: i32) -> Option<&Vec<i32>> {
      // Java: int[] entry = null;
      let mut entry: Option<&Vec<i32>> = None;
      // Java: if (index >= 0 && index < this._entries.length)
      if index >= 0 && self._entries.as_ref().map_or(false, |e| (index as usize) < e.len())
      {
         // Java: entry = this._entries[index];
         entry = self._entries.as_ref().unwrap()[index as usize].as_ref();
      }
      // Java: return entry;
      entry
    }

    // Java: void setTplcEntry(int index, int[] entry)
    pub fn setTplcEntry(&mut self, index: i32, entry: Option<&[i32]>) {
      // Java: IOAssert.assertNotNull("_entries should not be null!", this._entries);
      assert!(self._entries.is_some(), "_entries should not be null!");
      // Java: if (null != entry && entry.length != 0 && (index >= 0 || index < this._entries.length))
      if entry.is_some() && !entry.unwrap().is_empty() && (index >= 0 || (index as usize) < self._entries.as_ref().unwrap().len())
      {
         // Java: IOAssert.assertTrue("the length of entry should be equal to 9 !", entry.length == 9);
         assert!(entry.unwrap().len() == 9, "the length of entry should be equal to 9 !");
         // Java: this._entries[index] = new int[9];
         let mut new_entry = vec![0i32; 9];
         // Java: for(int i = 0; i < entry.length && i < this._entries[index].length; ++i)
         let src = entry.unwrap();
         for i in 0..std::cmp::min(src.len(), 9)
         {
            // Java: this._entries[index][i] = entry[i];
            new_entry[i] = src[i];
         }
         self._entries.as_mut().unwrap()[index as usize] = Some(new_entry);
      }
    }

    // Java: int getEntryCount()
    pub fn getEntryCount(&self) -> i32 {
      // Java: return this._cData;
      self._cData as i32
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&self, out: &mut HWPFOutputStream) {
      // Java: IOAssert.assertNotNull("_entries should not be null!", this._entries);
      assert!(self._entries.is_some(), "_entries should not be null!");
      // Java: if (this._cData > 0 && null != this._entries)
      if self._cData > 0 && self._entries.is_some()
      {
         // Java: int offset = 0;
         let mut offset: i32 = 0;
         // Java: byte[] header = new byte[6];
         let mut header = vec![0u8; 6];
         // Java: LittleEndian.putShort(header, offset, this._fExtend);
         le_put_short(&mut header, offset, self._fExtend);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(header, offset, this._cData);
         le_put_short(&mut header, offset, self._cData);
         // Java: offset += 2;
         offset += 2;
         // Java: LittleEndian.putShort(header, offset, this._cbExtra);
         le_put_short(&mut header, offset, self._cbExtra);
         // Java: offset += 2;
         offset += 2;
         // Java: out.write(header);
         let _ = out.write(&header);
         // Java: byte[] shortBuf = new byte[2];
         let mut short_buf = vec![0u8; 2];
         // Java: byte[] intBuf = new byte[4];
         let mut int_buf = vec![0u8; 4];
         // Java: for(int[] tplcs : this._entries)
         for tplcs in self._entries.as_ref().unwrap()
         {
            // Java: this._cchData = (short)(tplcs == null ? 0 : 18);
            let cch_data: i16 = if tplcs.is_none() { 0 } else { 18 };
            // Java: LittleEndian.putShort(shortBuf, 0, this._cchData);
            le_put_short(&mut short_buf, 0, cch_data);
            // Java: out.write(shortBuf);
            let _ = out.write(&short_buf);
            // Java: if (this._cchData != 0)
            if cch_data != 0
            {
               // Java: for(int tplc : tplcs)
               for &tplc in tplcs.as_ref().unwrap()
               {
                  // Java: LittleEndian.putInt(intBuf, 0, tplc);
                  le_put_int(&mut int_buf, 0, tplc);
                  // Java: out.write(intBuf);
                  let _ = out.write(&int_buf);
               }
            }
         }
      }
    }

}
