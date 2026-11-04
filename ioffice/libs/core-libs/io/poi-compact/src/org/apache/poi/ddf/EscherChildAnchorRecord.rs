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
pub const RECORD_ID_CONST: i16 = -4081;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtChildAnchor";

// Java: public class EscherChildAnchorRecord extends EscherRecord
pub struct EscherChildAnchorRecord {
    pub(crate) base: EscherRecord,
    // Java field: int field_1_dx1
    pub(crate) field_1_dx1: i32,
    // Java field: int field_2_dy1
    pub(crate) field_2_dy1: i32,
    // Java field: int field_3_dx2
    pub(crate) field_3_dx2: i32,
    // Java field: int field_4_dy2
    pub(crate) field_4_dy2: i32,
}

// Java: public class EscherChildAnchorRecord extends EscherRecord
impl EscherChildAnchorRecord {
    // Java: EscherChildAnchorRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherChildAnchorRecord {
            base: EscherRecord::new(),
            field_1_dx1: 0,
            field_2_dy1: 0,
            field_3_dx2: 0,
            field_4_dy2: 0,
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: this.readHeader(data, offset);
        self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let _pos = offset + 8;
        // Java: int size = 0;
        let mut size = 0;
        // Java: data.seek((long)pos);
        data.seek((offset + 8) as i64);
        // Java: this.field_1_dx1 = data.readInt();
        self.field_1_dx1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_2_dy1 = data.readInt();
        self.field_2_dy1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_3_dx2 = data.readInt();
        self.field_3_dx2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_4_dy2 = data.readInt();
        self.field_4_dy2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: return 8 + size;
        Ok(8 + size)
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int pos = offset + 8;
        let _pos = offset + 8;
        // Java: int size = 0;
        let mut size = 0;
        // Java: this.field_1_dx1 = data.readInt();
        self.field_1_dx1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_2_dy1 = data.readInt();
        self.field_2_dy1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_3_dx2 = data.readInt();
        self.field_3_dx2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_4_dy2 = data.readInt();
        self.field_4_dy2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: return 8 + size;
        Ok(8 + size)
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], _listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: int pos = offset + 2;
        let mut pos = offset + 2;
        // Java: LittleEndian.putShort(data, pos, this.getRecordId());
        le_put_short(data, pos as usize, self.getRecordId());
        // Java: pos += 2;
        pos += 2;
        // Java: LittleEndian.putInt(data, pos, this.getRecordSize() - 8);
        le_put_int(data, pos as usize, self.getRecordSize() - 8);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_1_dx1);
        le_put_int(data, pos as usize, self.field_1_dx1);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_2_dy1);
        le_put_int(data, pos as usize, self.field_2_dy1);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_3_dx2);
        le_put_int(data, pos as usize, self.field_3_dx2);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_4_dy2);
        le_put_int(data, pos as usize, self.field_4_dy2);
        // Java: pos += 4;
        pos += 4;
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
        // Java: return pos - offset;
        pos - offset
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 24;
        24
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return -4081;
        -4081
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "ChildAnchor";
        String::from("ChildAnchor")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String nl = System.getProperty("line.separator");
        // Java: return this.getClass().getName() + ":" + nl + "  RecordId: 0x" + HexDump.toHex((short)-4081) + nl + "  Options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  X1: " + this.field_1_dx1 + nl + "  Y1: " + this.field_2_dy1 + nl + "  X2: " + this.field_3_dx2 + nl + "  Y2: " + this.field_4_dy2 + nl;
        format!("EscherChildAnchorRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  X1: {}\n  Y1: {}\n  X2: {}\n  Y2: {}\n",
            -4081i16 as u16, self.base.getOptions() as u16,
            self.field_1_dx1, self.field_2_dy1, self.field_3_dx2, self.field_4_dy2)
    }

    // Java: int getDx1()
    pub fn getDx1(&self) -> i32 {
        // Java: return this.field_1_dx1;
        self.field_1_dx1
    }

    // Java: void setDx1(int field_1_dx1)
    pub fn setDx1(&mut self, field_1_dx1: i32) {
        // Java: this.field_1_dx1 = field_1_dx1;
        self.field_1_dx1 = field_1_dx1;
    }

    // Java: int getDy1()
    pub fn getDy1(&self) -> i32 {
        // Java: return this.field_2_dy1;
        self.field_2_dy1
    }

    // Java: void setDy1(int field_2_dy1)
    pub fn setDy1(&mut self, field_2_dy1: i32) {
        // Java: this.field_2_dy1 = field_2_dy1;
        self.field_2_dy1 = field_2_dy1;
    }

    // Java: int getDx2()
    pub fn getDx2(&self) -> i32 {
        // Java: return this.field_3_dx2;
        self.field_3_dx2
    }

    // Java: void setDx2(int field_3_dx2)
    pub fn setDx2(&mut self, field_3_dx2: i32) {
        // Java: this.field_3_dx2 = field_3_dx2;
        self.field_3_dx2 = field_3_dx2;
    }

    // Java: int getDy2()
    pub fn getDy2(&self) -> i32 {
        // Java: return this.field_4_dy2;
        self.field_4_dy2
    }

    // Java: void setDy2(int field_4_dy2)
    pub fn setDy2(&mut self, field_4_dy2: i32) {
        // Java: this.field_4_dy2 = field_4_dy2;
        self.field_4_dy2 = field_4_dy2;
    }
}
