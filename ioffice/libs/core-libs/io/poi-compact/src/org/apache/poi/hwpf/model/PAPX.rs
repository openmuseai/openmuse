// Java: import java.io.IOException
// Java: import org.apache.poi.hwpf.sprm.SprmBuffer
use crate::org::apache::poi::hwpf::sprm::SprmBuffer::SprmBuffer;
// Java: import org.apache.poi.hwpf.sprm.SprmOperation
use crate::org::apache::poi::hwpf::sprm::SprmOperation::SprmOperation;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.LittleEndian

use super::BytePropertyNode::BytePropertyNode;
use super::CharIndexTranslator::CharIndexTranslator;
use std::any::Any;
use std::ops::{Deref, DerefMut};

fn le_get_short(buf: &[u8], offset: i32) -> i16 {
    let o = offset as usize;
    i16::from_le_bytes([buf[o], buf[o + 1]])
}

fn le_get_unsigned_byte(buf: &[u8], offset: i32) -> i16 {
    buf[offset as usize] as i16
}

// Java: public class PAPX extends BytePropertyNode
pub struct PAPX {
    pub(crate) base: BytePropertyNode,
    // Java field: int _hugeGrpprlOffset
    pub(crate) _hugeGrpprlOffset: i32,
}

impl Deref for PAPX {
    type Target = BytePropertyNode;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl DerefMut for PAPX {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}

// Java: public class PAPX extends BytePropertyNode
impl PAPX {
    // Java: PAPX(int fcStart, int fcEnd, CharIndexTranslator translator, byte[] papx, DocumentInputStream dataStream)
    pub fn new(fc_start: i32, fc_end: i32, translator: Box<dyn CharIndexTranslator>, papx: Vec<u8>, data_stream: Option<&mut DocumentInputStream>) -> Self {
      // Java: super(fcStart, fcEnd, translator, papx);
      let papx_clone = papx.clone();
      let base = BytePropertyNode::new(fc_start, fc_end, translator, Box::new(papx));
      let mut papx_obj = PAPX { base, _hugeGrpprlOffset: -1 };
      // Java: byte[] buf = this.findHuge(papx, dataStream);
      let buf = papx_obj.findHuge(&papx_clone, data_stream);
      // Java: if (buf != null)
      if let Some(b) = buf {
         // Java: this._buf = buf;
         papx_obj.base.base._buf = Box::new(b);
      }
      papx_obj
    }

    // Java: PAPX(int fcStart, int fcEnd, CharIndexTranslator translator, SprmBuffer sprm, DocumentInputStream dataStream)
    pub fn new_from_sprm(fc_start: i32, fc_end: i32, translator: Box<dyn CharIndexTranslator>, sprm: &SprmBuffer, data_stream: Option<&mut DocumentInputStream>) -> Self {
      // Java: super(fcStart, fcEnd, translator, sprm.toByteArray());
      let sprm_bytes = sprm._buf.clone();
      let base = BytePropertyNode::new(fc_start, fc_end, translator, Box::new(sprm_bytes.clone()));
      let mut papx_obj = PAPX { base, _hugeGrpprlOffset: -1 };
      // Java: byte[] buf = this.findHuge(sprm.toByteArray(), dataStream);
      let buf = papx_obj.findHuge(&sprm_bytes, data_stream);
      // Java: if (buf != null)
      if let Some(b) = buf {
         // Java: this._buf = buf;
         papx_obj.base.base._buf = Box::new(b);
      }
      papx_obj
    }

    pub fn new_simple(fc_start: i32, fc_end: i32, papx: Vec<u8>) -> Self {
      let base = BytePropertyNode::new_simple(fc_start, fc_end, Box::new(papx));
      PAPX { base, _hugeGrpprlOffset: -1 }
    }

    // Java: byte[] findHuge(byte[] grpprl, DocumentInputStream datastream)
    pub fn findHuge(&mut self, grpprl: &[u8], datastream: Option<&mut DocumentInputStream>) -> Option<Vec<u8>> {
      // Java: if (grpprl.length == 8 && datastream != null)
      if grpprl.len() == 8 && datastream.is_some() {
         let ds = datastream.unwrap();
         // Java: SprmOperation sprm = new SprmOperation();
         let mut sprm = SprmOperation::new();
         // Java: sprm.init(grpprl, 2);
         sprm.init(grpprl, 2);
         // Java: if ((sprm.getOperation() == 69 || sprm.getOperation() == 70) && sprm.getSizeCode() == 3)
         if (sprm.getOperation() == 69 || sprm.getOperation() == 70) && sprm.getSizeCode() == 3 {
            // Java: int hugeGrpprlOffset = sprm.getOperand();
            let huge_grpprl_offset = sprm.getOperand();
            // Java: datastream.seek((long)hugeGrpprlOffset);
            ds.seek(huge_grpprl_offset as i64);
            // Java: int grpprlSize = datastream.readUShort();
            let grpprl_size = ds.readUShort();
            // Java: byte[] hugeGrpprl = new byte[grpprlSize + 2];
            let mut huge_grpprl = vec![0u8; (grpprl_size + 2) as usize];
            // Java: hugeGrpprl[0] = grpprl[0];
            huge_grpprl[0] = grpprl[0];
            // Java: hugeGrpprl[1] = grpprl[1];
            huge_grpprl[1] = grpprl[1];
            // Java: datastream.read(hugeGrpprl, 2, grpprlSize);
            ds.read_off(&mut huge_grpprl, 2, grpprl_size);
            // Java: this._hugeGrpprlOffset = hugeGrpprlOffset;
            self._hugeGrpprlOffset = huge_grpprl_offset;
            // Java: return hugeGrpprl;
            return Some(huge_grpprl);
         }
      }
      // Java: return null;
      None
    }

    // Java: byte[] getGrpprl()
    pub fn getGrpprl(&self) -> &[u8] {
      // Java: return (byte[])this._buf;
      if let Some(buf) = self.base.base._buf.downcast_ref::<Vec<u8>>() {
          buf.as_slice()
      } else {
          &[]
      }
    }

    // Java: int getHugeGrpprlOffset()
    pub fn getHugeGrpprlOffset(&self) -> i32 {
      // Java: return this._hugeGrpprlOffset;
      self._hugeGrpprlOffset
    }

    // Java: short getIstd()
    pub fn getIstd(&self) -> i16 {
      // Java: byte[] buf = this.getGrpprl();
      let buf = self.getGrpprl();
      // Java: if (buf.length == 0)
      if buf.is_empty() {
         // Java: return 0;
         return 0;
      }
      // Java: else
      else {
         // Java: return buf.length == 1 ? (short)LittleEndian.getUnsignedByte(buf, 0) : LittleEndian.getShort(buf);
         if buf.len() == 1 {
             le_get_unsigned_byte(buf, 0)
         } else {
             le_get_short(buf, 0)
         }
      }
    }

    // Java: SprmBuffer getSprmBuf()
    pub fn getSprmBuf(&self) -> SprmBuffer {
      // Java: return new SprmBuffer((byte[])this._buf);
      let buf = self.getGrpprl().to_vec();
      SprmBuffer { _buf: buf }
    }

    // Java: int getStart()
    pub fn getStart(&mut self) -> i32 {
      // Java: if (this.translator != null)
      if self.base.base.translator.is_some() {
         // Java: this.cpStart = this.translator.getCharIndexForNormal(this.cpStart, true);
         let cp_start = self.base.base.cpStart;
         let cp_end = self.base.base.cpEnd;
         if let Some(ref translator) = self.base.base.translator {
             self.base.base.cpStart = translator.getCharIndexForNormal(cp_start);
             // Java: this.cpEnd = this.translator.getCharIndexForNormal(this.cpEnd);
             self.base.base.cpEnd = translator.getCharIndexForNormal(cp_end);
         }
         // Java: this.translator = null;
         self.base.base.translator = None;
      }
      // Java: return this.cpStart;
      self.base.base.cpStart
    }

    // Java: int getEnd()
    pub fn getEnd(&mut self) -> i32 {
      // Java: if (this.translator != null)
      if self.base.base.translator.is_some() {
         // Java: this.cpStart = this.translator.getCharIndexForNormal(this.cpStart, true);
         let cp_start = self.base.base.cpStart;
         let cp_end = self.base.base.cpEnd;
         if let Some(ref translator) = self.base.base.translator {
             self.base.base.cpStart = translator.getCharIndexForNormal(cp_start);
             // Java: this.cpEnd = this.translator.getCharIndexForNormal(this.cpEnd);
             self.base.base.cpEnd = translator.getCharIndexForNormal(cp_end);
         }
         // Java: this.translator = null;
         self.base.base.translator = None;
      }
      // Java: return this.cpEnd;
      self.base.base.cpEnd
    }

}
