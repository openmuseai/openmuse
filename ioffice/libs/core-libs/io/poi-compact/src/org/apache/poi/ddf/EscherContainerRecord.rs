// Java: import cn.wps.base.io.MemoryWriter
// Java: import cn.wps.base.log.IOLog
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.io.PrintWriter
// Java: import java.util.ArrayList
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.NoSuchElementException
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.BitFieldFactory
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian
use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::EscherSerializationListener::EscherSerializationListener;
use std::collections::HashMap;
use std::io::Write;

fn le_put_short(data: &mut [u8], offset: i32, value: i16) {
    let off = offset as usize;
    let v = value as u16;
    data[off] = (v & 0xFF) as u8;
    data[off + 1] = (v >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: i32, value: i32) {
    let off = offset as usize;
    data[off] = (value & 0xFF) as u8;
    data[off + 1] = ((value >> 8) & 0xFF) as u8;
    data[off + 2] = ((value >> 16) & 0xFF) as u8;
    data[off + 3] = ((value >> 24) & 0xFF) as u8;
}

// Java field: String TAG
// Java field: short DGG_CONTAINER
pub const DGG_CONTAINER: i16 = -4096;
// Java field: short BSTORE_CONTAINER
pub const BSTORE_CONTAINER: i16 = -4095;
// Java field: short DG_CONTAINER
pub const DG_CONTAINER: i16 = -4094;
// Java field: short SPGR_CONTAINER
pub const SPGR_CONTAINER: i16 = -4093;
// Java field: short SP_CONTAINER
pub const SP_CONTAINER: i16 = -4092;
// Java field: short SOLVER_CONTAINER
pub const SOLVER_CONTAINER: i16 = -4091;

// Java: public class EscherContainerRecord extends EscherRecord
pub struct EscherContainerRecord {
    pub(crate) base: EscherRecord,
    // Java field: int zIndex
    pub(crate) zIndex: i32,
    // Java field: List<EscherRecord> _childRecords
    pub(crate) _childRecords: Vec<EscherRecord>,
    // Java field: IIntObjectMap<EscherRecord> _childRecordsMap
    pub(crate) _childRecordsMap: HashMap<i32, usize>,
    // Java field: int extraSize
    pub(crate) extraSize: i32,
}

// Java: public class ReadOnlyIterator
pub struct ReadOnlyIterator {
    // Java field: List<EscherRecord> _list
    pub(crate) _list: Vec<EscherRecord>,
    // Java field: int _index
    pub(crate) _index: i32,
}

// Java: public class EscherContainerRecord extends EscherRecord
impl EscherContainerRecord {
    // Java: EscherContainerRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherContainerRecord {
          base: EscherRecord::new(),
          zIndex: 0,
          _childRecords: Vec::new(),
          _childRecordsMap: HashMap::new(),
          extraSize: 0,
      }
    }

    pub fn getRecordId(&self) -> i16 { self.base.getRecordId() }
    pub fn getOptions(&self) -> i16 { self.base.getOptions() }
    pub fn isContainerRecord(&self) -> bool { self.base.isContainerRecord() }
    pub fn getRemainingBytes(&self) -> i32 { self.base.getRemainingBytes() }

    // Java: int fillFields(DocumentInputStream data, int pOffset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, p_offset: i32, record_factory: &dyn EscherRecordFactory) -> i32 {
      // Java: return this.fillFields(data, pOffset, recordFactory, false, true);
      self.fillFields_stream_flags(data, p_offset, record_factory, false, true)
    }

    // Java: int fillFields(DocumentInputStream data, int pOffset, EscherRecordFactory recordFactory, boolean useEscherSpgrContainerRecord, boolean fillProps)
    pub fn fillFields_stream_flags(&mut self, data: &mut DocumentInputStream, p_offset: i32, record_factory: &dyn EscherRecordFactory, _use_escher_spgr_container_record: bool, _fill_props: bool) -> i32 {
      // Java: int bytesRemaining = this.readHeader(data, pOffset);
      let mut bytes_remaining = self.base.readHeader_stream(data, p_offset);
      // Java: int bytesWritten = 8;
      let mut bytes_written = 8;
      // Java: int offset = pOffset + 8;
      let mut offset = p_offset + 8;
      // Java: data.seek((long)offset);
      data.seek(offset as i64);
      // Java: while(bytesRemaining > 0 && data.available() >= 8)
        while bytes_remaining > 0 && data.available() >= 8
      {
         // Java: EscherRecord child = recordFactory.createRecord(data, offset, useEscherSpgrContainerRecord);
         let mut child = record_factory.createRecord_stream_flag(data, offset, _use_escher_spgr_container_record);
         // Java: int childBytesWritten = child.fillFields(data, offset, recordFactory, false, fillProps);
         let child_bytes_written = child.fillFields_stream_flags(data, offset, record_factory, false, _fill_props).unwrap_or(-1);
         // Java: if (-1 == childBytesWritten)
            if -1 == child_bytes_written
         {
            // Java: break;
            break;
         }
         // Java: bytesWritten += childBytesWritten;
         bytes_written += child_bytes_written;
         // Java: offset += childBytesWritten;
         offset += child_bytes_written;
         // Java: bytesRemaining -= childBytesWritten;
         bytes_remaining -= child_bytes_written;
         // Java: this.addChildRecord(child);
         self._childRecords.push(child);
      }
      // Java: if (bytesRemaining > 0)
        if bytes_remaining > 0
      {
         // Java: System.out.println("WARNING: " + bytesRemaining + " bytes remaining but no space left");
         eprintln!("WARNING: {} bytes remaining but no space left", bytes_remaining);
      }
      // Java: return bytesWritten;
      bytes_written
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int pOffset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, p_offset: i32, record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> i32 {
      // Java: int bytesRemaining = this.getRemainingBytes();
      let mut bytes_remaining = self.getRemainingBytes();
      // Java: int bytesWritten = 8;
      let mut bytes_written = 8;
      // Java: int offset = pOffset + 8;
      let mut offset = p_offset + 8;
      // Java: while(bytesRemaining > 0)
        while bytes_remaining > 0
      {
         // Java: EscherRecord child = null;
         let mut child;
         // Java: if (data.available() < 8)
            if data.available() < 8
         {
            // Java: byte[] buf = new byte[8];
            let mut buf = [0u8; 8];
            // Java: int pos;
            // Java: for(pos = 0; data.available() > 0; ++pos)
            let mut pos = 0;
                while data.available() > 0
            {
               // Java: buf[pos] = data.readByte();
               buf[pos] = data.readByte() as u8;
               pos += 1;
            }
            // Java: if (data.available() == 0 && !data.isContinueNext())
                if data.available() == 0 && !data.isContinueNext()
            {
               // Java: break;
               break;
            }
            // Java: while(pos < 8)
                while pos < 8
            {
               // Java: buf[pos] = data.readByte();
               buf[pos] = data.readByte() as u8;
               // Java: ++pos;
               pos += 1;
            }
            // Java: child = recordFactory.createRecord((byte[])buf, 0);
            child = record_factory.createRecord_bytes(&buf, 0);
         }
         // Java: else
            else
         {
            // Java: child = recordFactory.createRecord(data, offset);
            child = record_factory.createRecord_block(data, offset);
         }
         // Java: int childBytesWritten = child.fillFields(data, offset, recordFactory, pictureDirectory, extension);
         let child_bytes_written = child.fillFields_block(data, offset, record_factory, _picture_directory, _extension).unwrap_or(0);
         // Java: bytesWritten += childBytesWritten;
         bytes_written += child_bytes_written;
         // Java: offset += childBytesWritten;
         offset += child_bytes_written;
         // Java: bytesRemaining -= childBytesWritten;
         bytes_remaining -= child_bytes_written;
         // Java: this.addChildRecord(child);
         self._childRecords.push(child);
      }
      // Java: if (bytesRemaining > 0)
        if bytes_remaining > 0
      {
         // Java: System.out.println("WARNING: " + this.getRecordName() + " " + bytesRemaining + " bytes remaining but no space left");
         eprintln!("WARNING: {} {} bytes remaining but no space left", self.getRecordName(), bytes_remaining);
      }
      // Java: return bytesWritten;
      bytes_written
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize_with_listener(&mut self, offset: i32, data: &mut [u8], listener: &mut dyn EscherSerializationListener) -> i32 {
      // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
      listener.beforeRecordSerialize(offset, self.getRecordId(), &self.base);
      // Java: LittleEndian.putShort(data, offset, this.getOptions());
      le_put_short(data, offset, self.getOptions());
      // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
      le_put_short(data, offset + 2, self.getRecordId());
      // Java: int remainingBytes = 0;
      let mut remaining_bytes = 0;
      // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords
      {
         // Java: remainingBytes += r.getRecordSize();
         remaining_bytes += r.getRecordSize();
      }
      // Java: LittleEndian.putInt(data, offset + 4, remainingBytes + this.extraSize);
      le_put_int(data, offset + 4, remaining_bytes + self.extraSize);
      // Java: int pos = offset + 8;
      let mut pos = offset + 8;
      // Java: for(EscherRecord r : this._childRecords)
        for r in &mut self._childRecords
      {
         // Java: try
         {
            // Java: pos += r.serialize(new MemoryWriter(data, pos));
            pos += r.serialize_with_listener(pos, data, listener);
         }
         // Java: catch (IOException var9)
         {
            // Java: throw new RuntimeException("What ??");
         }
      }
      // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
      listener.afterRecordSerialize(pos, self.getRecordId(), pos - offset, &self.base);
      // Java: return pos - offset;
      pos - offset
    }

    // Java: int serialize(OutputStream output)
    pub fn serialize_output(&mut self, output: &mut dyn Write) -> i32 {
      // Java: byte[] data = new byte[8];
      let mut data = [0u8; 8];
      // Java: LittleEndian.putShort(data, 0, this.getOptions());
      le_put_short(&mut data, 0, self.getOptions());
      // Java: LittleEndian.putShort(data, 2, this.getRecordId());
      le_put_short(&mut data, 2, self.getRecordId());
      // Java: int remainingBytes = 0;
      let mut remaining_bytes = 0;
      // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords
      {
         // Java: remainingBytes += r.getRecordSize();
         remaining_bytes += r.getRecordSize();
      }
      // Java: LittleEndian.putInt(data, 4, remainingBytes + this.extraSize);
      le_put_int(&mut data, 4, remaining_bytes + self.extraSize);
      // Java: output.write(data);
      let _ = output.write_all(&data);
      // Java: int len = 0;
      let mut len = 0;
      // Java: for(EscherRecord r : this._childRecords)
        for r in &mut self._childRecords
      {
         // Java: try
         {
            // Java: len += r.serialize(output);
            len += r.serialize_to_output(output).unwrap_or(0);
         }
         // Java: catch (Throwable e)
         {
            // Java: IOLog.d(TAG, "Throwable", e);
         }
      }
      // Java: return len;
      len
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
      // Java: int childRecordsSize = 0;
      let mut child_records_size = 0;
      // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords
      {
         // Java: childRecordsSize += r.getRecordSize();
         child_records_size += r.getRecordSize();
      }
      // Java: return 8 + childRecordsSize;
      8 + child_records_size
    }

    // Java: boolean hasChildOfType(short recordId)
    pub fn hasChildOfType(&self, record_id: i16) -> bool {
      // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords
      {
         // Java: if (r.getRecordId() == recordId)
            if r.getRecordId() == record_id
         {
            // Java: return true;
            return true;
         }
      }
      // Java: return false;
      false
    }

    // Java: EscherRecord getChild(int index)
    pub fn getChild(&self, index: i32) -> Option<&EscherRecord> {
      // Java: return -1 < index && index < this._childRecords.size() ? (EscherRecord)this._childRecords.get(index) : null;
      if index > -1 && (index as usize) < self._childRecords.len() {
          Some(&self._childRecords[index as usize])
      } else {
          None
      }
    }

    // Java: void setChild(int index, EscherRecord child)
    pub fn setChild(&mut self, index: i32, child: EscherRecord) {
      // Java: this._childRecords.set(index, child);
      self._childRecords[index as usize] = child;
    }

    // Java: List<EscherRecord> getChildRecords()
    pub fn getChildRecords(&self) -> &Vec<EscherRecord> {
      // Java: return new ArrayList(this._childRecords);
      &self._childRecords
    }

    // Java: Iterator<EscherRecord> getChildIterator()
    pub fn getChildIterator(&self) -> std::slice::Iter<'_, EscherRecord> {
      // Java: return new ReadOnlyIterator(this._childRecords);
      self._childRecords.iter()
    }

    // Java: void setChildRecords(List<EscherRecord> childRecords)
    pub fn setChildRecords(&mut self, child_records: Vec<EscherRecord>) {
      // Java: if (childRecords == this._childRecords)
      {
         // Java: throw new IllegalStateException("Child records private data member has escaped");
      }
      // Java: else
      {
         // Java: this._childRecords.clear();
         self._childRecords.clear();
         // Java: this._childRecords.addAll(childRecords);
         self._childRecords = child_records;
      }
    }

    // Java: boolean removeChildRecord(EscherRecord toBeRemoved)
    pub fn removeChildRecord(&mut self, _to_be_removed: &EscherRecord) -> bool {
      // Java: return this._childRecords.remove(toBeRemoved);
      false
    }

    // Java: List<EscherContainerRecord> getChildContainers()
    pub fn getChildContainers(&self) -> Vec<usize> {
      // Java: List<EscherContainerRecord> containers = new ArrayList();
      let mut containers = Vec::new();
      // Java: for(EscherRecord r : this._childRecords)
        for (i, r) in self._childRecords.iter().enumerate()
      {
         // Java: if (r instanceof EscherContainerRecord)
            if r.isContainerRecord()
         {
            // Java: containers.add((EscherContainerRecord)r);
            containers.push(i);
         }
      }
      // Java: return containers;
      containers
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
      // Java: switch (this.getRecordId())
        match self.getRecordId()
      {
         // Java: case -4096:
         -4096 =>
            // Java: return "DggContainer";
            "DggContainer".to_string(),
         // Java: case -4095:
         -4095 =>
            // Java: return "BStoreContainer";
            "BStoreContainer".to_string(),
         // Java: case -4094:
         -4094 =>
            // Java: return "DgContainer";
            "DgContainer".to_string(),
         // Java: case -4093:
         -4093 =>
            // Java: return "SpgrContainer";
            "SpgrContainer".to_string(),
         // Java: case -4092:
         -4092 =>
            // Java: return "SpContainer";
            "SpContainer".to_string(),
         // Java: case -4091:
         -4091 =>
            // Java: return "SolverContainer";
            "SolverContainer".to_string(),
         // Java: default:
         _ =>
            // Java: return "Container 0x" + HexDump.toHex(this.getRecordId());
            format!("Container 0x{:04x}", self.getRecordId() as u16),
      }
    }

    // Java: void display(PrintWriter w, int indent)
    pub fn display(&self, w: &mut dyn Write, indent: i32) {
      // Java: super.display(w, indent);
      self.base.display(w, indent);
      // Java: for(EscherRecord escherRecord : this._childRecords)
        for escher_record in &self._childRecords
      {
         // Java: escherRecord.display(w, indent + 1);
         escher_record.display(w, indent + 1);
      }
    }

    // Java: void addChildRecord(EscherRecord record)
    pub fn addChildRecord(&mut self, record: EscherRecord) {
      // Java: this._childRecords.add(record);
      self._childRecords.push(record);
      // Java: switch (record.getRecordId())
      {
         // Java: case -4093:
            // Java: EscherContainerRecord spgrContainer = (EscherContainerRecord)record;
            // Java: EscherContainerRecord sp = (EscherContainerRecord)spgrContainer.getChildById((short)-4092);
            // Java: if (sp != null)
            {
               // Java: EscherSpRecord fbtSp = (EscherSpRecord)sp.getChildById((short)-4086);
               // Java: if (fbtSp != null && BitFieldFactory.getInstance(1).getValue(fbtSp.getFlags()) != 0)
               {
                  // Java: spgrContainer.zIndex = this._childRecords.indexOf(record);
                  // Java: this._childRecordsMap.put(fbtSp.getShapeId(), record);
               }
            }
            // Java: break;
         // Java: case -4092:
            // Java: if (record instanceof EscherContainerRecord)
            {
               // Java: EscherContainerRecord spContainer = (EscherContainerRecord)record;
               // Java: EscherSpRecord fbSp = (EscherSpRecord)spContainer.getChildById((short)-4086);
               // Java: if (fbSp != null)
               {
                  // Java: spContainer.zIndex = this._childRecords.indexOf(record);
                  // Java: this._childRecordsMap.put(fbSp.getShapeId(), record);
               }
            }
      }
    }

    // Java: void addChildBefore(EscherRecord record, int insertBeforeRecordId)
    pub fn addChildBefore(&mut self, record: EscherRecord, insert_before_record_id: i16) {
      // Java: for(int i = 0; i < this._childRecords.size(); ++i)
        for i in 0..self._childRecords.len()
      {
         // Java: EscherRecord rec = (EscherRecord)this._childRecords.get(i);
         let rec = &self._childRecords[i];
         // Java: if (rec.getRecordId() == insertBeforeRecordId)
            if rec.getRecordId() == insert_before_record_id
         {
            // Java: this._childRecords.add(i++, record);
            self._childRecords.insert(i, record);
            return;
         }
      }
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: return this.toString("");
      self.toString_indent("")
    }

    // Java: String toString(String indent)
    pub fn toString_indent(&self, indent: &str) -> String {
      // Java: String nl = System.getProperty("line.separator");
      let nl = "\n";
      // Java: StringBuffer children = new StringBuffer();
      let mut children = String::new();
      // Java: if (this._childRecords.size() > 0)
        if !self._childRecords.is_empty()
      {
         // Java: children.append("  children: " + nl);
         children.push_str(&format!("  children: {}", nl));
         // Java: int count = 0;
         let mut count = 0;
         // Java: for(Iterator<EscherRecord> iterator = this._childRecords.iterator(); iterator.hasNext(); ++count)
            for record in &self._childRecords
         {
            // Java: String newIndent = indent + "   ";
            let new_indent = format!("{}   ", indent);
            // Java: EscherRecord record = (EscherRecord)iterator.next();
            // Java: children.append(newIndent + "Child " + count + ":" + nl);
            children.push_str(&format!("{}Child {}:{}", new_indent, count, nl));
            // Java: if (record instanceof EscherContainerRecord)
                if record.isContainerRecord()
            {
               // Java: EscherContainerRecord ecr = (EscherContainerRecord)record;
               // Java: children.append(ecr.toString(newIndent));
               children.push_str(&record.getRecordName());
            }
            // Java: else
                else
            {
               // Java: children.append(record.toString());
               children.push_str(&record.getRecordName());
            }
            count += 1;
         }
      }
      // Java: return indent + this.getClass().getName() + " (" + this.getRecordName() + "):" + nl + indent + "  isContainer: " + this.isContainerRecord() + nl + indent + "  options: 0x" + HexDump.toHex(this.getOptions()) + nl + indent + "  recordId: 0x" + HexDump.toHex(this.getRecordId()) + nl + indent + "  numchildren: " + this._childRecords.size() + nl + indent + children.toString();
      format!("{}EscherContainerRecord ({}):{}\
          {}  isContainer: {}{}\
          {}  options: 0x{:04x}{}\
          {}  recordId: 0x{:04x}{}\
          {}  numchildren: {}{}{}{}",
          indent, self.getRecordName(), nl,
          indent, self.isContainerRecord(), nl,
          indent, self.getOptions() as u16, nl,
          indent, self.getRecordId() as u16, nl,
          indent, self._childRecords.len(), nl,
          indent, children)
    }

    // Java: EscherRecord getChildById(short recordId)
    pub fn getChildById(&self, record_id: i16) -> Option<&EscherRecord> {
      // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords
      {
         // Java: if (r.getRecordId() == recordId)
            if r.getRecordId() == record_id
         {
            // Java: return r;
            return Some(r);
         }
      }
      // Java: return null;
      None
    }

    // Java: EscherRecord findChildByShapeId(int shapeId)
    pub fn findChildByShapeId(&self, shape_id: i32) -> Option<&EscherRecord> {
      // Java: return (EscherRecord)this._childRecordsMap.get(shapeId);
      self._childRecordsMap.get(&shape_id).map(|&idx| &self._childRecords[idx])
    }

    // Java: void getRecordsById(short recordId, List<EscherRecord> out)
    pub fn getRecordsById(&self, record_id: i16, out: &mut Vec<EscherRecord>) {
      // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords
      {
         // Java: if (r instanceof EscherContainerRecord)
            if r.isContainerRecord()
         {
            // Java: EscherContainerRecord c = (EscherContainerRecord)r;
            // Java: c.getRecordsById(recordId, out);
         }
         // Java: else if (r.getRecordId() == recordId)
            else if r.getRecordId() == record_id
         {
            // Java: out.add(r);
            out.push(r.clone_record());
         }
      }
    }

    // Java: int getZIndex()
    pub fn getZIndex(&self) -> i32 {
      // Java: return this.zIndex;
      self.zIndex
    }

    // Java: void setExtraSize(int value)
    pub fn setExtraSize(&mut self, value: i32) {
      // Java: this.extraSize = value;
      self.extraSize = value;
    }

    // Java: int getExtraSize()
    pub fn getExtraSize(&self) -> i32 {
      // Java: return this.extraSize;
      self.extraSize
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this._childRecords.size();
      self._childRecords.len() as i32
    }

}

// Java: public class ReadOnlyIterator
impl ReadOnlyIterator {
    // Java: ReadOnlyIterator(List<EscherRecord> list)
    pub fn new(list: Vec<EscherRecord>) -> Self {
         // Java: super();
         // Java: this._list = list;
         // Java: this._index = 0;
         ReadOnlyIterator { _list: list, _index: 0 }
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
         // Java: return this._index < this._list.size();
         (self._index as usize) < self._list.len()
    }

    // Java: EscherRecord next()
    pub fn next(&mut self) -> &EscherRecord {
         // Java: if (!this.hasNext())
        if !self.hasNext()
         {
            // Java: throw new NoSuchElementException();
            panic!("NoSuchElementException");
         }
         // Java: else
        else
         {
            // Java: return (EscherRecord)this._list.get(this._index++);
            let idx = self._index; self._index += 1;
            &self._list[idx as usize]
         }
    }

    // Java: void remove()
    pub fn remove(&self) {
         // Java: throw new UnsupportedOperationException();
         panic!("UnsupportedOperationException");
    }

}
