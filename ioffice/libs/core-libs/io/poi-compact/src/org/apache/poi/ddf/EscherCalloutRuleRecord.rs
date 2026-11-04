// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
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
pub const RECORD_ID: i16 = -4073;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtCalloutRule";

// Java: public class EscherCalloutRuleRecord extends EscherRecord
pub struct EscherCalloutRuleRecord {
    pub(crate) base: EscherRecord,
    // Java field: int field_1_ruid
    pub(crate) field_1_ruid: i32,
    // Java field: int field_2_spid
    pub(crate) field_2_spid: i32,
}

// Java: public class EscherCalloutRuleRecord extends EscherRecord
impl EscherCalloutRuleRecord {
    // Java: EscherCalloutRuleRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherCalloutRuleRecord {
            base: EscherRecord::new(),
            field_1_ruid: 0,
            field_2_spid: 0,
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let bytes_remaining = self.base.getRemainingBytes();
        // Java: if (bytesRemaining != 8)
        if bytes_remaining != 8 {
            // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expecting no remaining data but got {} byte(s).", bytes_remaining)));
        }
        // Java: else
        {
            // Java: this.field_1_ruid = data.readInt();
            self.field_1_ruid = data.readInt();
            // Java: this.field_2_spid = data.readInt();
            self.field_2_spid = data.readInt();
            // Java: return bytesRemaining + 8;
            Ok(bytes_remaining + 8)
        }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let bytes_remaining = self.base.getRemainingBytes();
        // Java: if (bytesRemaining != 8)
        if bytes_remaining != 8 {
            // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expecting no remaining data but got {} byte(s).", bytes_remaining)));
        }
        // Java: else
        {
            // Java: this.field_1_ruid = data.readInt();
            self.field_1_ruid = data.readInt();
            // Java: this.field_2_spid = data.readInt();
            self.field_2_spid = data.readInt();
            // Java: return bytesRemaining + 8;
            Ok(bytes_remaining + 8)
        }
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: int pos = offset + 2;
        let mut pos = offset + 2;
        // Java: LittleEndian.putShort(data, pos, this.getRecordId());
        le_put_short(data, pos as usize, self.base.getRecordId());
        // Java: pos += 2;
        pos += 2;
        // Java: int remainingBytes = this.getRecordSize() - 8;
        let remaining_bytes = self.getRecordSize() - 8;
        // Java: LittleEndian.putInt(data, pos, remainingBytes);
        le_put_int(data, pos as usize, remaining_bytes);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_1_ruid);
        le_put_int(data, pos as usize, self.field_1_ruid);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_2_spid);
        le_put_int(data, pos as usize, self.field_2_spid);
        // Java: pos += 4;
        pos += 4;
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
        // Java: return this.getRecordSize();
        self.getRecordSize()
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 16;
        16
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "CalloutRule";
        String::from("CalloutRule")
    }
}
