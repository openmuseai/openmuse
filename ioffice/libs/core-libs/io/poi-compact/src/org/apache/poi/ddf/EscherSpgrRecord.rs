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
pub const RECORD_ID_CONST: i16 = -4087;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtSpgr";

// Java: public class EscherSpgrRecord extends EscherRecord
pub struct EscherSpgrRecord {
    pub(crate) base: EscherRecord,
    // Java field: int field_1_rectX1
    pub(crate) field_1_rectX1: i32,
    // Java field: int field_2_rectY1
    pub(crate) field_2_rectY1: i32,
    // Java field: int field_3_rectX2
    pub(crate) field_3_rectX2: i32,
    // Java field: int field_4_rectY2
    pub(crate) field_4_rectY2: i32,
}

// Java: public class EscherSpgrRecord extends EscherRecord
impl EscherSpgrRecord {
    // Java: EscherSpgrRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherSpgrRecord {
            base: EscherRecord::new(),
            field_1_rectX1: 0,
            field_2_rectY1: 0,
            field_3_rectX2: 0,
            field_4_rectY2: 0,
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: int bytesRemaining = this.readHeader(data, offset);
        let mut bytes_remaining = self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let _pos = offset + 8;
        // Java: int size = 0;
        let mut size = 0;
        // Java: data.seek((long)pos);
        data.seek((offset + 8) as i64);
        // Java: this.field_1_rectX1 = data.readInt();
        self.field_1_rectX1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_2_rectY1 = data.readInt();
        self.field_2_rectY1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_3_rectX2 = data.readInt();
        self.field_3_rectX2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_4_rectY2 = data.readInt();
        self.field_4_rectY2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: bytesRemaining -= size;
        bytes_remaining -= size;
        // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0 {
            // Java: throw new RecordFormatException("Expected no remaining bytes but got " + bytesRemaining);
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected no remaining bytes but got {}", bytes_remaining)));
        }
        // Java: else
        {
            // Java: return 8 + size + bytesRemaining;
            Ok(8 + size + bytes_remaining)
        }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let mut bytes_remaining = self.base.getRemainingBytes();
        // Java: int pos = offset + 8;
        let _pos = offset + 8;
        // Java: int size = 0;
        let mut size = 0;
        // Java: this.field_1_rectX1 = data.readInt();
        self.field_1_rectX1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_2_rectY1 = data.readInt();
        self.field_2_rectY1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_3_rectX2 = data.readInt();
        self.field_3_rectX2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_4_rectY2 = data.readInt();
        self.field_4_rectY2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: bytesRemaining -= size;
        bytes_remaining -= size;
        // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0 {
            // Java: throw new RecordFormatException("Expected no remaining bytes but got " + bytesRemaining);
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expected no remaining bytes but got {}", bytes_remaining)));
        }
        // Java: else
        {
            // Java: return 8 + size + bytesRemaining;
            Ok(8 + size + bytes_remaining)
        }
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], _listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(data, (offset + 2) as usize, self.getRecordId());
        // Java: int remainingBytes = 16;
        let _remaining_bytes = 16;
        // Java: LittleEndian.putInt(data, offset + 4, remainingBytes);
        le_put_int(data, (offset + 4) as usize, 16);
        // Java: LittleEndian.putInt(data, offset + 8, this.field_1_rectX1);
        le_put_int(data, (offset + 8) as usize, self.field_1_rectX1);
        // Java: LittleEndian.putInt(data, offset + 12, this.field_2_rectY1);
        le_put_int(data, (offset + 12) as usize, self.field_2_rectY1);
        // Java: LittleEndian.putInt(data, offset + 16, this.field_3_rectX2);
        le_put_int(data, (offset + 16) as usize, self.field_3_rectX2);
        // Java: LittleEndian.putInt(data, offset + 20, this.field_4_rectY2);
        le_put_int(data, (offset + 20) as usize, self.field_4_rectY2);
        // Java: listener.afterRecordSerialize(offset + this.getRecordSize(), this.getRecordId(), offset + this.getRecordSize(), this);
        // Java: return 24;
        24
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 24;
        24
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return -4087;
        -4087
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "Spgr";
        String::from("Spgr")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex((short)-4087) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  RectX: " + this.field_1_rectX1 + '\n' + "  RectY: " + this.field_2_rectY1 + '\n' + "  RectWidth: " + this.field_3_rectX2 + '\n' + "  RectHeight: " + this.field_4_rectY2 + '\n';
        format!("EscherSpgrRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  RectX: {}\n  RectY: {}\n  RectWidth: {}\n  RectHeight: {}\n",
            -4087i16 as u16, self.base.getOptions() as u16, self.field_1_rectX1, self.field_2_rectY1, self.field_3_rectX2, self.field_4_rectY2)
    }

    // Java: int getRectX1()
    pub fn getRectX1(&self) -> i32 {
        // Java: return this.field_1_rectX1;
        self.field_1_rectX1
    }

    // Java: void setRectX1(int x1)
    pub fn setRectX1(&mut self, x1: i32) {
        // Java: this.field_1_rectX1 = x1;
        self.field_1_rectX1 = x1;
    }

    // Java: int getRectY1()
    pub fn getRectY1(&self) -> i32 {
        // Java: return this.field_2_rectY1;
        self.field_2_rectY1
    }

    // Java: void setRectY1(int y1)
    pub fn setRectY1(&mut self, y1: i32) {
        // Java: this.field_2_rectY1 = y1;
        self.field_2_rectY1 = y1;
    }

    // Java: int getRectX2()
    pub fn getRectX2(&self) -> i32 {
        // Java: return this.field_3_rectX2;
        self.field_3_rectX2
    }

    // Java: void setRectX2(int x2)
    pub fn setRectX2(&mut self, x2: i32) {
        // Java: this.field_3_rectX2 = x2;
        self.field_3_rectX2 = x2;
    }

    // Java: int getRectY2()
    pub fn getRectY2(&self) -> i32 {
        // Java: return this.field_4_rectY2;
        self.field_4_rectY2
    }

    // Java: void setRectY2(int rectY2)
    pub fn setRectY2(&mut self, rect_y2: i32) {
        // Java: this.field_4_rectY2 = rectY2;
        self.field_4_rectY2 = rect_y2;
    }
}
