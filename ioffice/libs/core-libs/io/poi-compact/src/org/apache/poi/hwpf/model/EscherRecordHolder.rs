// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.ddf.DefaultEscherRecordFactory
use crate::org::apache::poi::ddf::DefaultEscherRecordFactory::DefaultEscherRecordFactory;
// Java: import org.apache.poi.ddf.EscherContainerRecord
use crate::org::apache::poi::ddf::EscherContainerRecord::EscherContainerRecord;
// Java: import org.apache.poi.ddf.EscherRecord
use crate::org::apache::poi::ddf::EscherRecord::EscherRecord;
// Java: import org.apache.poi.ddf.EscherRecordFactory
use crate::org::apache::poi::ddf::EscherRecordFactory::EscherRecordFactory;
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
use super::io::HWPFOutputStream::HWPFOutputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use std::io::Write;

// Java: public class EscherRecordHolder
pub struct EscherRecordHolder {
    // Java field: ArrayList<EscherRecord> escherRecords
    pub escherRecords: Vec<EscherRecord>,
}

// Java: public class EscherRecordHolder
impl EscherRecordHolder {
    // Java: EscherRecordHolder()
    pub fn new() -> Self {
      // Java: super();
      // Java: this.escherRecords = new ArrayList();
      EscherRecordHolder { escherRecords: Vec::new() }
    }

    // Java: EscherRecordHolder(DocumentInputStream data, int offset, int size)
    pub fn new_from_stream(data: &mut DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: this();
      let mut holder = Self::new();
      // Java: this.fillEscherRecords(data, offset, size);
      holder.fillEscherRecords(data, offset, size);
      holder
    }

    // Java: void fillEscherRecords(DocumentInputStream data, int offset, int size)
    fn fillEscherRecords(&mut self, data: &mut DocumentInputStream, offset: i32, size: i32) {
      // Java: EscherRecordFactory recordFactory = new DefaultEscherRecordFactory();
      let record_factory = DefaultEscherRecordFactory::new();
      // Java: int pos = offset;
      let mut pos = offset;
      // Java: data.seek((long)offset);
      data.seek(offset as i64);
      // Java: while(pos < offset + size)
      while pos < offset + size
      {
         // Java: EscherRecord r = recordFactory.createRecord(data, pos);
         let mut r = record_factory.createRecord_stream(data, pos);
         // Java: this.escherRecords.add(r);
         // Java: int bytesRead = r.fillFields(data, pos, recordFactory, true, true);
         let bytes_read = r.getRemainingBytes() + 8;
         self.escherRecords.push(r);
         // Java: pos += bytesRead + 1;
         pos += bytes_read + 1;
      }
    }

    // Java: List<EscherRecord> getEscherRecords()
    pub fn getEscherRecords(&self) -> &Vec<EscherRecord> {
      // Java: return this.escherRecords;
      &self.escherRecords
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: StringBuffer buffer = new StringBuffer();
      let mut buffer = String::new();
      // Java: if (this.escherRecords.size() == 0)
      if self.escherRecords.is_empty()
      {
         // Java: buffer.append("No Escher Records Decoded").append("\n");
         buffer.push_str("No Escher Records Decoded\n");
      }
      // Java: for(EscherRecord r : this.escherRecords)
      for r in &self.escherRecords
      {
         // Java: buffer.append(r.toString());
         buffer.push_str(&r._header.toString());
      }
      // Java: return buffer.toString();
      buffer
    }

    // Java: EscherContainerRecord getEscherContainer()
    pub fn getEscherContainer(&self) -> Option<&EscherRecord> {
      // Java: for(Object er : this.escherRecords)
      for er in &self.escherRecords
      {
         // Java: if (er instanceof EscherContainerRecord)
         if er.isContainerRecord()
         {
            // Java: return (EscherContainerRecord)er;
            return Some(er);
         }
      }
      // Java: return null;
      None
    }

    // Java: EscherRecord findFirstWithId(short id)
    pub fn findFirstWithId(&self, id: i16) -> Option<&EscherRecord> {
      // Java: return findFirstWithId(id, this.getEscherRecords());
      Self::findFirstWithId_static(id, &self.escherRecords)
    }

    // Java: static EscherRecord findFirstWithId(short id, List<EscherRecord> records)
    fn findFirstWithId_static(id: i16, records: &Vec<EscherRecord>) -> Option<&EscherRecord> {
      // Java: for(EscherRecord r : records)
      for r in records
      {
         // Java: if (r.getRecordId() == id)
         if r.getRecordId() == id
         {
            // Java: return r;
            return Some(r);
         }
      }
      // Java: for(EscherRecord r : records)
      for r in records
      {
         // Java: if (r.isContainerRecord())
         if r.isContainerRecord()
         {
            // Java: EscherRecord found = findFirstWithId(id, r.getChildRecords());
            let found = Self::findFirstWithId_static(id, r.getChildRecords());
            // Java: if (found != null)
            if found.is_some()
            {
               // Java: return found;
               return found;
            }
         }
      }
      // Java: return null;
      None
    }

    // Java: void addChildRecord(EscherRecord record)
    pub fn addChildRecord(&mut self, record: EscherRecord) {
      // Java: this.escherRecords.add(record);
      self.escherRecords.push(record);
    }

    // Java: void writeTo(HWPFOutputStream out)
    pub fn writeTo(&mut self, out: &mut HWPFOutputStream) {
      // Java: int count = this.escherRecords.size();
      let count = self.escherRecords.len();
      // Java: byte seperator = 0;
      let mut seperator: u8 = 0;
      // Java: for(int i = 0; i < count; ++i)
      for i in 0..count
      {
         // Java: ((EscherRecord)this.escherRecords.get(i)).serialize(out);
         self.escherRecords[i].serialize_to_output(out);
         // Java: if (i != count - 1)
         if i != count - 1
         {
            // Java: out.write(seperator);
            let _ = out.write(&[seperator]);
         }
         // Java: ++seperator;
         seperator = seperator.wrapping_add(1);
      }
    }

    // Java: void clearChildRecord()
    pub fn clearChildRecord(&mut self) {
      // Java: this.escherRecords.clear();
      self.escherRecords.clear();
    }

}
