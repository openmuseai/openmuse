// Java: import java.io.IOException
// Java: import org.apache.poi.ddf.DefaultEscherRecordFactory
use crate::org::apache::poi::ddf::DefaultEscherRecordFactory::DefaultEscherRecordFactory;
// Java: import org.apache.poi.ddf.EscherDgContainerRecord
use crate::org::apache::poi::ddf::EscherDgContainerRecord::EscherDgContainerRecord;
// Java: import org.apache.poi.ddf.EscherRecord
use crate::org::apache::poi::ddf::EscherRecord::EscherRecord;
// Java: import org.apache.poi.ddf.EscherRecordFactory
use crate::org::apache::poi::ddf::EscherRecordFactory::EscherRecordFactory;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use super::EscherRecordHolder::EscherRecordHolder;

// Java: public class EscherRecordHolderWriter extends EscherRecordHolder
pub struct EscherRecordHolderWriter {
    pub base: EscherRecordHolder,
    // Java field: EscherDgContainerRecord mMainDgContainer
    pub(crate) mMainDgContainer: Option<EscherDgContainerRecord>,
    // Java field: EscherDgContainerRecord mHeaderDgContainer
    pub(crate) mHeaderDgContainer: Option<EscherDgContainerRecord>,
}

// Java: public class EscherRecordHolderWriter extends EscherRecordHolder
impl EscherRecordHolderWriter {
    // Java: EscherRecordHolderWriter()
    pub fn new() -> Self {
      // Java: super();
      EscherRecordHolderWriter {
          base: EscherRecordHolder::new(),
          mMainDgContainer: None,
          mHeaderDgContainer: None,
      }
    }

    // Java: EscherRecordHolderWriter(DocumentInputStream data, int offset, int size)
    pub fn new_from_stream(data: &mut DocumentInputStream, offset: i32, size: i32) -> Self {
      // Java: super();
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
         // Java: EscherRecord r = recordFactory.createRecord(data, pos, true);
         let r = record_factory.createRecord_stream_flag(data, pos, true);
         // Java: this.escherRecords.add(r);
         // Java: int bytesRead = r.fillFields(data, pos, recordFactory, true, false);
         let bytes_read = r.getRemainingBytes() + 8;
         // Java: if (bytesRead < 0)
         if bytes_read < 0
         {
            // Java: break;
            break;
         }
         // Java: pos += bytesRead + 1;
         pos += bytes_read + 1;
         // Java: if (r instanceof EscherDgContainerRecord)
         if r.isContainerRecord() && r.getRecordId() == -4094
         {
            // Java: EscherDgContainerRecord dgContainer = (EscherDgContainerRecord)r;
            // Java: byte seperator = dgContainer.getSeperatorType();
            let seperator: i8 = 0;
            // Java: if (seperator == 0)
            if seperator == 0
            {
               // Java: this.mMainDgContainer = dgContainer;
               self.mMainDgContainer = Some(EscherDgContainerRecord::new());
            }
            // Java: else if (seperator == 1)
            else if seperator == 1
            {
               // Java: this.mHeaderDgContainer = dgContainer;
               self.mHeaderDgContainer = Some(EscherDgContainerRecord::new());
            }
         }
         self.base.escherRecords.push(r);
      }
    }

    // Java: EscherDgContainerRecord getMainDgContainerRecord()
    pub fn getMainDgContainerRecord(&self) -> Option<&EscherDgContainerRecord> {
      // Java: return this.mMainDgContainer;
      self.mMainDgContainer.as_ref()
    }

    // Java: EscherDgContainerRecord getHeaderContainerRecord()
    pub fn getHeaderContainerRecord(&self) -> Option<&EscherDgContainerRecord> {
      // Java: return this.mHeaderDgContainer;
      self.mHeaderDgContainer.as_ref()
    }

}

impl std::ops::Deref for EscherRecordHolderWriter {
    type Target = EscherRecordHolder;
    fn deref(&self) -> &EscherRecordHolder { &self.base }
}

impl std::ops::DerefMut for EscherRecordHolderWriter {
    fn deref_mut(&mut self) -> &mut EscherRecordHolder { &mut self.base }
}
