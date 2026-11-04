// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitFieldFactory
use super::EscherContainerRecord::EscherContainerRecord;
use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::DefaultEscherRecordFactory::DefaultEscherRecordFactory;
use std::collections::HashMap;

// Java: public class EscherSpgrContainerRecord extends EscherContainerRecord
pub struct EscherSpgrContainerRecord {
    pub(crate) base: EscherContainerRecord,
    // Java field: DocumentInputStream mData
    mData: Option<DocumentInputStream>,
    // Java field: int mOffset
    mOffset: i32,
    // Java field: EscherRecordFactory mEscherRecordFactory
    mEscherRecordFactory: Option<DefaultEscherRecordFactory>,
    // Java field: int mBytesRemaining
    pub(crate) mBytesRemaining: i32,
    // Java field: int mPos
    mPos: i32,
    // Java field: int mZIndex
    mZIndex: i32,
}

// Java: public class EscherSpgrContainerRecord extends EscherContainerRecord
impl EscherSpgrContainerRecord {
    // Java: EscherSpgrContainerRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherSpgrContainerRecord {
          base: EscherContainerRecord::new(),
          mData: None,
          mOffset: 0,
          mEscherRecordFactory: None,
          mBytesRemaining: 0,
          mPos: 0,
          mZIndex: 0,
      }
    }

    // Java: int fillFields(DocumentInputStream data, int pOffset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: DocumentInputStream, p_offset: i32, record_factory: DefaultEscherRecordFactory) -> i32 {
      // Java: this.mData = data;
      self.mData = Some(data);
      // Java: this.mOffset = pOffset;
      self.mOffset = p_offset;
      // Java: this.mPos = this.mOffset + 8;
      self.mPos = self.mOffset + 8;
      // Java: this.mEscherRecordFactory = recordFactory;
      self.mEscherRecordFactory = Some(record_factory);
      // Java: this.mZIndex = 1;
      self.mZIndex = 1;
      // Java: this.mBytesRemaining = this.readHeader(data, pOffset);
      if let Some(ref mut data) = self.mData {
          self.mBytesRemaining = self.base.base.readHeader_stream(data, p_offset);
      }
      // Java: return this.mBytesRemaining + 8;
      self.mBytesRemaining + 8
    }

    // Java: int fillFields(DocumentInputStream data, int pOffset, EscherRecordFactory recordFactory, boolean useEscherSpgrContainerRecord, boolean fillProps)
    pub fn fillFields_stream_flags(&mut self, data: DocumentInputStream, p_offset: i32, record_factory: DefaultEscherRecordFactory, _use_escher_spgr: bool, _fill_props: bool) -> i32 {
      // Java: return this.fillFields(data, pOffset, recordFactory);
      self.fillFields_stream(data, p_offset, record_factory)
    }

    // Java: EscherRecord findChildByShapeId(int shapeId)
    pub fn findChildByShapeId(&mut self, shape_id: i32) -> Option<EscherRecord> {
      // Java: do
      loop
      {
         // Java: EscherRecord child = super.findChildByShapeId(shapeId);
         let child_idx = self.base._childRecordsMap.get(&shape_id).copied();
         // Java: if (null != child)
            if let Some(idx) = child_idx
         {
            // Java: this._childRecords.remove(child);
            let child = self.base._childRecords.remove(idx);
            // Java: this._childRecordsMap.remove(shapeId);
            self.base._childRecordsMap.remove(&shape_id);
            // Java: return child;
            return Some(child);
         }
         // Java: while(this.next())
         if !self.next() {
            break;
         }
      }
      // Java: return null;
      None
    }

    // Java: void addChildRecord(EscherRecord record)
    pub fn addChildRecord(&mut self, record: EscherRecord) {
      // Java: this._childRecords.add(record);
      self.base._childRecords.push(record);
      let record_id = self.base._childRecords.last().unwrap().getRecordId();
      // Java: switch (record.getRecordId())
        match record_id
      {
         // Java: case -4093:
         -4093 => {
            // Java: EscherContainerRecord spgrContainer = (EscherContainerRecord)record;
            // Java: EscherContainerRecord sp = (EscherContainerRecord)spgrContainer.getChildById((short)-4092);
            // Java: if (sp != null)
            // Java: EscherSpRecord fbtSp = (EscherSpRecord)sp.getChildById((short)-4086);
            // Java: if (fbtSp != null && BitFieldFactory.getInstance(1).getValue(fbtSp.getFlags()) != 0)
            // Java: spgrContainer.zIndex = this.mZIndex++;
            // Java: this._childRecordsMap.put(fbtSp.getShapeId(), record);
            self.mZIndex += 1;
            // Java: break;
         }
         // Java: case -4092:
         -4092 => {
            // Java: EscherContainerRecord spContainer = (EscherContainerRecord)record;
            // Java: EscherSpRecord fbSp = (EscherSpRecord)spContainer.getChildById((short)-4086);
            // Java: if (fbSp != null)
            // Java: spContainer.zIndex = this.mZIndex++;
            // Java: this._childRecordsMap.put(fbSp.getShapeId(), record);
            self.mZIndex += 1;
         }
         _ => {}
      }
    }

    // Java: boolean next()
    fn next(&mut self) -> bool {
      // Java: this.mData.seek((long)this.mPos);
      if let Some(ref mut data) = self.mData {
          data.seek(self.mPos as i64);
      } else {
          return false;
      }
      // Java: if (this.mBytesRemaining <= 0)
        if self.mBytesRemaining <= 0
      {
         // Java: return false;
         return false;
      }
      // Java: else if (this.mData.available() < 8)
        else if self.mData.as_ref().map_or(true, |d| d.available() < 8)
      {
         // Java: return false;
         return false;
      }
      // Java: else
        else
      {
         // Java: EscherRecord child = this.mEscherRecordFactory.createRecord(this.mData, this.mPos, false);
         let mut child = if let (Some(ref mut data), Some(ref factory)) = (&mut self.mData, &self.mEscherRecordFactory) {
             factory.createRecord_stream_flag(data, self.mPos, false)
         } else {
             return false;
         };
         // Java: int childBytesWritten = 0;
         // Java: try
         // Java: childBytesWritten = child.fillFields(this.mData, this.mPos, this.mEscherRecordFactory);
         let child_bytes_written = if let (Some(ref mut data), Some(ref factory)) = (&mut self.mData, &self.mEscherRecordFactory) {
             child.fillFields_stream(data, self.mPos, factory).unwrap_or(0)
         } else {
             0
         };
         // Java: catch (IOException e)
         // Java: e.printStackTrace();
         // Java: return false;
         if child_bytes_written == 0 {
             return false;
         }
         // Java: this.mPos += childBytesWritten;
         self.mPos += child_bytes_written;
         // Java: this.mBytesRemaining -= childBytesWritten;
         self.mBytesRemaining -= child_bytes_written;
         // Java: this.addChildRecord(child);
         self.addChildRecord(child);
         // Java: return true;
         true
      }
    }

}
