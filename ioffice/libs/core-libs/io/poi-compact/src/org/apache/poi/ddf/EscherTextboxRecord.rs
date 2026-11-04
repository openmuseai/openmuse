// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian
// Java: import org.apache.poi.util.RecordFormatException

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

// Java field: short RECORD_ID
pub const RECORD_ID_CONST: i16 = -4083;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "msofbtClientTextbox";
// Java field: byte[] NO_BYTES

// Java: public class EscherTextboxRecord extends EscherRecord
pub struct EscherTextboxRecord {
    pub(crate) base: EscherRecord,
    // Java field: byte[] thedata
    pub(crate) thedata: Vec<u8>,
}

// Java: public class EscherTextboxRecord extends EscherRecord
impl EscherTextboxRecord {
    // Java: EscherTextboxRecord()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.thedata = NO_BYTES;
        EscherTextboxRecord {
            base: EscherRecord::new(),
            thedata: Vec::new(),
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: int bytesRemaining = this.readHeader(data, offset);
        let bytes_remaining = self.base.readHeader_stream(data, offset);
        // Java: data.seek((long)(offset + 8));
        data.seek((offset + 8) as i64);
        // Java: this.thedata = new byte[bytesRemaining];
        self.thedata = vec![0u8; bytes_remaining as usize];
        // Java: data.read(this.thedata, 0, bytesRemaining);
        data.read_off(&mut self.thedata, 0, bytes_remaining);
        // Java: return bytesRemaining + 8;
        Ok(bytes_remaining + 8)
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: this.thedata = new byte[this.getRemainingBytes()];
        let remaining = self.base.getRemainingBytes();
        self.thedata = vec![0u8; remaining as usize];
        // Java: data.read(this.thedata);
        data.read(&mut self.thedata);
        // Java: return this.getRemainingBytes() + 8;
        Ok(remaining + 8)
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], _listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(data, (offset + 2) as usize, self.base.getRecordId());
        // Java: int remainingBytes = this.thedata.length;
        let remaining_bytes = self.thedata.len() as i32;
        // Java: LittleEndian.putInt(data, offset + 4, remainingBytes);
        le_put_int(data, (offset + 4) as usize, remaining_bytes);
        // Java: System.arraycopy(this.thedata, 0, data, offset + 8, this.thedata.length);
        let start = (offset + 8) as usize;
        let len = self.thedata.len();
        data[start..start + len].copy_from_slice(&self.thedata);
        // Java: int pos = offset + 8 + this.thedata.length;
        let pos = offset + 8 + len as i32;
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
        // Java: int size = pos - offset;
        let size = pos - offset;
        // Java: if (size != this.getRecordSize())
        if size != self.getRecordSize() {
            // Java: throw new RecordFormatException(size + " bytes written but getRecordSize() reports " + this.getRecordSize());
            panic!("{} bytes written but getRecordSize() reports {}", size, self.getRecordSize());
        }
        // Java: else
        {
            // Java: return size;
            size
        }
    }

    // Java: byte[] getData()
    pub fn getData(&self) -> &[u8] {
        // Java: return this.thedata;
        &self.thedata
    }

    // Java: void setData(byte[] b, int start, int length)
    pub fn setData_range(&mut self, b: &[u8], start: i32, length: i32) {
        // Java: this.thedata = new byte[length];
        self.thedata = vec![0u8; length as usize];
        // Java: System.arraycopy(b, start, this.thedata, 0, length);
        let s = start as usize;
        let l = length as usize;
        self.thedata[..l].copy_from_slice(&b[s..s + l]);
    }

    // Java: void setData(byte[] b)
    pub fn setData(&mut self, b: &[u8]) {
        // Java: this.setData(b, 0, b.length);
        let len = b.len() as i32;
        self.setData_range(b, 0, len);
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 8 + this.thedata.length;
        8 + self.thedata.len() as i32
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "ClientTextbox";
        String::from("ClientTextbox")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String nl = System.getProperty("line.separator");
        // Java: String theDumpHex = "";
        let mut the_dump_hex = String::new();
        // Java: try
        {
            // Java: if (this.thedata.length != 0)
            if !self.thedata.is_empty() {
                // Java: theDumpHex = "  Extra Data:" + nl;
                the_dump_hex = String::from("  Extra Data:\n");
                // Java: theDumpHex = theDumpHex + HexDump.dump(this.thedata, 0L, 0);
            }
        }
        // Java: catch (Exception var4)
        {
            // Java: theDumpHex = "Error!!";
        }
        // Java: return this.getClass().getName() + ":" + nl + "  isContainer: " + this.isContainerRecord() + nl + "  options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  recordId: 0x" + HexDump.toHex(this.getRecordId()) + nl + "  numchildren: " + this.getChildRecords().size() + nl + theDumpHex;
        format!("EscherTextboxRecord:\n  isContainer: false\n  options: 0x{:04X}\n  recordId: 0x{:04X}\n  numchildren: 0\n{}",
            self.base.getOptions() as u16, self.base.getRecordId() as u16, the_dump_hex)
    }
}
