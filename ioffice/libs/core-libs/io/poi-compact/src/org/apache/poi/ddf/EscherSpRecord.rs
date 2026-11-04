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
pub const RECORD_ID_CONST: i16 = -4086;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtSp";
// Java field: int FLAG_GROUP
pub const FLAG_GROUP: i32 = 1;
// Java field: int FLAG_CHILD
pub const FLAG_CHILD: i32 = 2;
// Java field: int FLAG_PATRIARCH
pub const FLAG_PATRIARCH: i32 = 4;
// Java field: int FLAG_DELETED
pub const FLAG_DELETED: i32 = 8;
// Java field: int FLAG_OLESHAPE
pub const FLAG_OLESHAPE: i32 = 16;
// Java field: int FLAG_HAVEMASTER
pub const FLAG_HAVEMASTER: i32 = 32;
// Java field: int FLAG_FLIPHORIZ
pub const FLAG_FLIPHORIZ: i32 = 64;
// Java field: int FLAG_FLIPVERT
pub const FLAG_FLIPVERT: i32 = 128;
// Java field: int FLAG_CONNECTOR
pub const FLAG_CONNECTOR: i32 = 256;
// Java field: int FLAG_HAVEANCHOR
pub const FLAG_HAVEANCHOR: i32 = 512;
// Java field: int FLAG_BACKGROUND
pub const FLAG_BACKGROUND: i32 = 1024;
// Java field: int FLAG_HASSHAPETYPE
pub const FLAG_HASSHAPETYPE: i32 = 2048;

// Java: public class EscherSpRecord extends EscherRecord
pub struct EscherSpRecord {
    pub(crate) base: EscherRecord,
    // Java field: int type
    pub(crate) shape_type: i32,
    // Java field: int field_1_shapeId
    pub(crate) field_1_shapeId: i32,
    // Java field: int field_2_flags
    pub(crate) field_2_flags: i32,
}

// Java: public class EscherSpRecord extends EscherRecord
impl EscherSpRecord {
    // Java: EscherSpRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherSpRecord {
            base: EscherRecord::new(),
            shape_type: 0,
            field_1_shapeId: 0,
            field_2_flags: 0,
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: this.readHeader(data, offset);
        self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let pos = offset + 8;
        // Java: this.type = getBits(data, offset);
        self.shape_type = Self::getBits(data, offset) as i32;
        // Java: data.seek((long)pos);
        data.seek(pos as i64);
        // Java: this.field_1_shapeId = data.readInt();
        self.field_1_shapeId = data.readInt();
        // Java: this.field_2_flags = data.readInt();
        self.field_2_flags = data.readInt();
        // Java: return this.getRecordSize();
        Ok(self.getRecordSize())
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: this.field_1_shapeId = data.readInt();
        self.field_1_shapeId = data.readInt();
        // Java: byte[] intdata = new byte[4];
        let mut intdata = [0u8; 4];
        // Java: LittleEndian.putInt(intdata, this.field_1_shapeId);
        le_put_int(&mut intdata, 0, self.field_1_shapeId);
        // Java: int b0 = LittleEndian.getByte(intdata, 0) & 240;
        let b0 = (intdata[0] as i32) & 240;
        // Java: int b1 = LittleEndian.getByte(intdata, 1) & 255;
        let b1 = (intdata[1] as i32) & 255;
        // Java: this.type = (short)((b1 << 4) + (b0 >> 4));
        self.shape_type = ((b1 << 4) + (b0 >> 4)) as i16 as i32;
        // Java: this.field_2_flags = data.readInt();
        self.field_2_flags = data.readInt();
        // Java: return this.getRecordSize();
        Ok(self.getRecordSize())
    }

    // Java: int getShapeType()
    pub fn getShapeType(&self) -> i32 {
        // Java: return this.type;
        self.shape_type
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], _listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(data, (offset + 2) as usize, self.getRecordId());
        // Java: int remainingBytes = 8;
        let _remaining_bytes = 8;
        // Java: LittleEndian.putInt(data, offset + 4, remainingBytes);
        le_put_int(data, (offset + 4) as usize, 8);
        // Java: LittleEndian.putInt(data, offset + 8, this.field_1_shapeId);
        le_put_int(data, (offset + 8) as usize, self.field_1_shapeId);
        // Java: LittleEndian.putInt(data, offset + 12, this.field_2_flags);
        le_put_int(data, (offset + 12) as usize, self.field_2_flags);
        // Java: listener.afterRecordSerialize(offset + this.getRecordSize(), this.getRecordId(), this.getRecordSize(), this);
        // Java: return 16;
        16
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 16;
        16
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return -4086;
        -4086
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "Sp";
        String::from("Sp")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String nl = System.getProperty("line.separator");
        // Java: return this.getClass().getName() + ":" + nl + "  RecordId: 0x" + HexDump.toHex((short)-4086) + nl + "  Options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  ShapeId: " + this.field_1_shapeId + nl + "  Flags: " + this.decodeFlags(this.field_2_flags) + " (0x" + HexDump.toHex(this.field_2_flags) + ")" + nl;
        format!("EscherSpRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  ShapeId: {}\n  Flags: {} (0x{:08X})\n",
            -4086i16 as u16, self.base.getOptions() as u16, self.field_1_shapeId,
            self.decodeFlags(self.field_2_flags), self.field_2_flags as u32)
    }

    // Java: String decodeFlags(int flags)
    fn decodeFlags(&self, flags: i32) -> String {
        // Java: StringBuffer result = new StringBuffer();
        let mut result = String::new();
        // Java: result.append((flags & 1) != 0 ? "|GROUP" : "");
        if (flags & 1) != 0 { result.push_str("|GROUP"); }
        // Java: result.append((flags & 2) != 0 ? "|CHILD" : "");
        if (flags & 2) != 0 { result.push_str("|CHILD"); }
        // Java: result.append((flags & 4) != 0 ? "|PATRIARCH" : "");
        if (flags & 4) != 0 { result.push_str("|PATRIARCH"); }
        // Java: result.append((flags & 8) != 0 ? "|DELETED" : "");
        if (flags & 8) != 0 { result.push_str("|DELETED"); }
        // Java: result.append((flags & 16) != 0 ? "|OLESHAPE" : "");
        if (flags & 16) != 0 { result.push_str("|OLESHAPE"); }
        // Java: result.append((flags & 32) != 0 ? "|HAVEMASTER" : "");
        if (flags & 32) != 0 { result.push_str("|HAVEMASTER"); }
        // Java: result.append((flags & 64) != 0 ? "|FLIPHORIZ" : "");
        if (flags & 64) != 0 { result.push_str("|FLIPHORIZ"); }
        // Java: result.append((flags & 128) != 0 ? "|FLIPVERT" : "");
        if (flags & 128) != 0 { result.push_str("|FLIPVERT"); }
        // Java: result.append((flags & 256) != 0 ? "|CONNECTOR" : "");
        if (flags & 256) != 0 { result.push_str("|CONNECTOR"); }
        // Java: result.append((flags & 512) != 0 ? "|HAVEANCHOR" : "");
        if (flags & 512) != 0 { result.push_str("|HAVEANCHOR"); }
        // Java: result.append((flags & 1024) != 0 ? "|BACKGROUND" : "");
        if (flags & 1024) != 0 { result.push_str("|BACKGROUND"); }
        // Java: result.append((flags & 2048) != 0 ? "|HASSHAPETYPE" : "");
        if (flags & 2048) != 0 { result.push_str("|HASSHAPETYPE"); }
        // Java: if (result.length() > 0)
        if !result.is_empty() {
            // Java: result.deleteCharAt(0);
            result.remove(0);
        }
        // Java: return result.toString();
        result
    }

    // Java: int getShapeId()
    pub fn getShapeId(&self) -> i32 {
        // Java: return this.field_1_shapeId;
        self.field_1_shapeId
    }

    // Java: void setShapeId(int field_1_shapeId)
    pub fn setShapeId(&mut self, field_1_shape_id: i32) {
        // Java: this.field_1_shapeId = field_1_shapeId;
        self.field_1_shapeId = field_1_shape_id;
    }

    // Java: int getFlags()
    pub fn getFlags(&self) -> i32 {
        // Java: return this.field_2_flags;
        self.field_2_flags
    }

    // Java: void setFlags(int field_2_flags)
    pub fn setFlags(&mut self, field_2_flags: i32) {
        // Java: this.field_2_flags = field_2_flags;
        self.field_2_flags = field_2_flags;
    }

    // Java: static short getBits(DocumentInputStream data, int offset)
    fn getBits(data: &mut DocumentInputStream, offset: i32) -> i16 {
        // Java: data.seek((long)offset);
        data.seek(offset as i64);
        // Java: int b0 = data.readByte() & 240;
        let b0 = (data.readByte() as i32) & 240;
        // Java: int b1 = data.readByte() & 255;
        let b1 = (data.readByte() as i32) & 255;
        // Java: return (short)((b1 << 4) + (b0 >> 4));
        ((b1 << 4) + (b0 >> 4)) as i16
    }
}
