// Java: import java.io.IOException
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

// Java field: short RECORD_ID
pub const RECORD_ID_CONST: i16 = -4079;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtClientData";

// Java: public class EscherClientDataRecord extends EscherRecord
pub struct EscherClientDataRecord {
    pub(crate) base: EscherRecord,
    // Java field: byte[] remainingData
    pub(crate) remainingData: Vec<u8>,
}

// Java: public class EscherClientDataRecord extends EscherRecord
impl EscherClientDataRecord {
    // Java: EscherClientDataRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherClientDataRecord {
            base: EscherRecord::new(),
            remainingData: Vec::new(),
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: int bytesRemaining = this.readHeader(data, offset);
        let bytes_remaining = self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let pos = offset + 8;
        // Java: data.seek((long)pos);
        data.seek(pos as i64);
        // Java: this.remainingData = new byte[bytesRemaining];
        self.remainingData = vec![0u8; bytes_remaining as usize];
        // Java: data.read(this.remainingData, 0, bytesRemaining);
        data.read_off(&mut self.remainingData, 0, bytes_remaining);
        // Java: return 8 + bytesRemaining;
        Ok(8 + bytes_remaining)
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let bytes_remaining = self.base.getRemainingBytes();
        // Java: this.remainingData = new byte[bytesRemaining];
        self.remainingData = vec![0u8; bytes_remaining as usize];
        // Java: data.read(this.remainingData);
        data.read(&mut self.remainingData);
        // Java: return 8 + bytesRemaining;
        Ok(8 + bytes_remaining)
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&mut self, offset: i32, data: &mut [u8], listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: if (this.remainingData == null)
        if self.remainingData.is_empty() {
            // Java: this.remainingData = new byte[0];
        }
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(data, (offset + 2) as usize, self.getRecordId());
        // Java: LittleEndian.putInt(data, offset + 4, this.remainingData.length);
        le_put_int(data, (offset + 4) as usize, self.remainingData.len() as i32);
        // Java: System.arraycopy(this.remainingData, 0, data, offset + 8, this.remainingData.length);
        let len = self.remainingData.len();
        let start = (offset + 8) as usize;
        data[start..start + len].copy_from_slice(&self.remainingData);
        // Java: int pos = offset + 8 + this.remainingData.length;
        let pos = offset + 8 + len as i32;
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
        // Java: return pos - offset;
        pos - offset
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 8 + (this.remainingData == null ? 0 : this.remainingData.length);
        8 + self.remainingData.len() as i32
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return -4079;
        -4079
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "ClientData";
        String::from("ClientData")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String nl = System.getProperty("line.separator");
        // Java: String extraData = "";
        // Java: return this.getClass().getName() + ":" + nl + "  RecordId: 0x" + HexDump.toHex((short)-4079) + nl + "  Options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  Extra Data:" + nl + extraData;
        format!("EscherClientDataRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  Extra Data:\n", -4079i16 as u16, self.base.getOptions() as u16)
    }

    // Java: byte[] getRemainingData()
    pub fn getRemainingData(&self) -> &[u8] {
        // Java: return this.remainingData;
        &self.remainingData
    }

    // Java: void setRemainingData(byte[] remainingData)
    pub fn setRemainingData(&mut self, remaining_data: Vec<u8>) {
        // Java: this.remainingData = remainingData;
        self.remainingData = remaining_data;
    }
}
