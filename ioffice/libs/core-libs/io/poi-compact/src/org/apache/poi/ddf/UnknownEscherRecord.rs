// Java: import cn.wps.base.io.MemoryWriter
// Java: import java.io.IOException
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use std::io;
use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::EscherSerializationListener::EscherSerializationListener;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java field: byte[] NO_BYTES

// Java: public class UnknownEscherRecord extends EscherRecord
pub struct UnknownEscherRecord {
    pub(crate) base: EscherRecord,
    // Java field: byte[] thedata
    pub(crate) thedata: Vec<u8>,
    // Java field: List<EscherRecord> _childRecords
    pub(crate) _childRecords: Vec<EscherRecord>,
}

// Java: public class UnknownEscherRecord extends EscherRecord
impl UnknownEscherRecord {
    // Java: UnknownEscherRecord()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.thedata = NO_BYTES;
        // Java: this._childRecords = new ArrayList();
        UnknownEscherRecord {
            base: EscherRecord::new(),
            thedata: Vec::new(),
            _childRecords: Vec::new(),
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: int bytesRemaining = this.readHeader(data, offset);
        let mut bytes_remaining = self.base.readHeader_stream(data, offset);
        // Java: if (bytesRemaining < 0)
        if bytes_remaining < 0 {
            // Java: return -1;
            return Ok(-1);
        }
        // Java: else
        {
            // Java: int avaliable = data.size() - (offset + 8);
            let avaliable = data.size() - (offset + 8);
            // Java: if (bytesRemaining > avaliable)
            if bytes_remaining > avaliable {
                // Java: bytesRemaining = avaliable;
                bytes_remaining = avaliable;
            }
            // Java: if (!this.isContainerRecord())
            if !self.base.isContainerRecord() {
                // Java: data.seek((long)(offset + 8));
                data.seek((offset + 8) as i64);
                // Java: this.thedata = new byte[bytesRemaining];
                self.thedata = vec![0u8; bytes_remaining as usize];
                // Java: data.read(this.thedata, 0, bytesRemaining);
                data.read_off(&mut self.thedata, 0, bytes_remaining);
                // Java: return bytesRemaining + 8;
                Ok(bytes_remaining + 8)
            }
            // Java: else
            else {
                // Java: int bytesWritten = 0;
                let mut bytes_written = 0;
                // Java: this.thedata = new byte[0];
                self.thedata = Vec::new();
                // Java: offset += 8;
                let mut offset = offset + 8;
                // Java: bytesWritten += 8;
                bytes_written += 8;
                // Java: while(bytesRemaining > 0)
                while bytes_remaining > 0 {
                    // Java: EscherRecord child = recordFactory.createRecord(data, offset);
                    // Java: int childBytesWritten = child.fillFields(data, offset, recordFactory);
                    // Java: bytesWritten += childBytesWritten;
                    let child_bytes_written = 0i32; // stub
                    bytes_written += child_bytes_written;
                    // Java: offset += childBytesWritten;
                    offset += child_bytes_written;
                    // Java: bytesRemaining -= childBytesWritten;
                    bytes_remaining -= child_bytes_written;
                    // Java: this.getChildRecords().add(child);
                    break; // prevent infinite loop in stub
                }
                // Java: return bytesWritten;
                Ok(bytes_written)
            }
        }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let mut bytes_remaining = self.base.getRemainingBytes();
        // Java: if (!this.isContainerRecord())
        if !self.base.isContainerRecord() {
            // Java: this.thedata = new byte[bytesRemaining];
            self.thedata = vec![0u8; bytes_remaining as usize];
            // Java: data.read(this.thedata);
            data.read(&mut self.thedata);
            // Java: return bytesRemaining + 8;
            Ok(bytes_remaining + 8)
        }
        // Java: else
        else {
            // Java: int bytesWritten = 0;
            let mut bytes_written = 0;
            // Java: this.thedata = new byte[0];
            self.thedata = Vec::new();
            // Java: offset += 8;
            let mut offset = offset + 8;
            // Java: bytesWritten += 8;
            bytes_written += 8;
            // Java: while(bytesRemaining > 0)
            while bytes_remaining > 0 {
                // Java: EscherRecord child = recordFactory.createRecord(data, offset);
                // Java: int childBytesWritten = child.fillFields(data, offset, recordFactory, pictureDirectory, extension);
                // Java: bytesWritten += childBytesWritten;
                let child_bytes_written = 0i32; // stub
                bytes_written += child_bytes_written;
                // Java: offset += childBytesWritten;
                offset += child_bytes_written;
                // Java: bytesRemaining -= childBytesWritten;
                bytes_remaining -= child_bytes_written;
                // Java: this.getChildRecords().add(child);
                break; // prevent infinite loop in stub
            }
            // Java: return bytesWritten;
            Ok(bytes_written)
        }
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], _listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(data, (offset + 2) as usize, self.base.getRecordId());
        // Java: int remainingBytes = this.thedata.length;
        let mut remaining_bytes = self.thedata.len() as i32;
        // Java: for(EscherRecord r : this._childRecords)
        for r in &self._childRecords {
            // Java: remainingBytes += r.getRecordSize();
            remaining_bytes += r.getRecordSize();
        }
        // Java: LittleEndian.putInt(data, offset + 4, remainingBytes);
        le_put_int(data, (offset + 4) as usize, remaining_bytes);
        // Java: System.arraycopy(this.thedata, 0, data, offset + 8, this.thedata.length);
        let start = (offset + 8) as usize;
        let len = self.thedata.len();
        data[start..start + len].copy_from_slice(&self.thedata);
        // Java: int pos = offset + 8 + this.thedata.length;
        let pos = offset + 8 + len as i32;
        // Java: for(EscherRecord r : this._childRecords)
        // Java: try
        // Java: pos += r.serialize(new MemoryWriter(data, pos));
        // Java: catch (IOException var9)
        // Java: throw new RuntimeException("What ??");
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
        // Java: return pos - offset;
        pos - offset
    }

    // Java: byte[] getData()
    pub fn getData(&self) -> &[u8] {
        // Java: return this.thedata;
        &self.thedata
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 8 + this.thedata.length;
        8 + self.thedata.len() as i32
    }

    // Java: List<EscherRecord> getChildRecords()
    pub fn getChildRecords(&mut self) -> &mut Vec<EscherRecord> {
        // Java: return this._childRecords;
        &mut self._childRecords
    }

    // Java: void setChildRecords(List<EscherRecord> childRecords)
    pub fn setChildRecords(&mut self, child_records: Vec<EscherRecord>) {
        // Java: this._childRecords = childRecords;
        self._childRecords = child_records;
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "Unknown 0x" + HexDump.toHex(this.getRecordId());
        format!("Unknown 0x{:04X}", self.base.getRecordId() as u16)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: StringBuffer children = new StringBuffer();
        let mut children = String::new();
        // Java: if (this.getChildRecords().size() > 0)
        if !self._childRecords.is_empty() {
            // Java: children.append("  children: \n");
            children.push_str("  children: \n");
            // Java: for(EscherRecord record : this._childRecords)
            // Java: children.append(record.toString());
            // Java: children.append('\n');
        }
        // Java: String theDumpHex = HexDump.toHex(this.thedata, 32);
        // Java: return this.getClass().getName() + ":" + '\n' + "  isContainer: " + this.isContainerRecord() + '\n' + "  options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  recordId: 0x" + HexDump.toHex(this.getRecordId()) + '\n' + "  numchildren: " + this.getChildRecords().size() + '\n' + theDumpHex + children.toString();
        format!("UnknownEscherRecord:\n  isContainer: {}\n  options: 0x{:04X}\n  recordId: 0x{:04X}\n  numchildren: {}\n{}",
            self.base.isContainerRecord(), self.base.getOptions() as u16, self.base.getRecordId() as u16,
            self._childRecords.len(), children)
    }

    // Java: void addChildRecord(EscherRecord childRecord)
    pub fn addChildRecord(&mut self, child_record: EscherRecord) {
        // Java: this.getChildRecords().add(childRecord);
        self._childRecords.push(child_record);
    }
}
