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
pub const RECORD_ID_CONST: i16 = -3810;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtSplitMenuColors";

// Java: public class EscherSplitMenuColorsRecord extends EscherRecord
pub struct EscherSplitMenuColorsRecord {
    pub(crate) base: EscherRecord,
    // Java field: int field_1_color1
    pub(crate) field_1_color1: i32,
    // Java field: int field_2_color2
    pub(crate) field_2_color2: i32,
    // Java field: int field_3_color3
    pub(crate) field_3_color3: i32,
    // Java field: int field_4_color4
    pub(crate) field_4_color4: i32,
}

// Java: public class EscherSplitMenuColorsRecord extends EscherRecord
impl EscherSplitMenuColorsRecord {
    // Java: EscherSplitMenuColorsRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherSplitMenuColorsRecord {
            base: EscherRecord::new(),
            field_1_color1: 0,
            field_2_color2: 0,
            field_3_color3: 0,
            field_4_color4: 0,
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
        // Java: this.field_1_color1 = data.readInt();
        self.field_1_color1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_2_color2 = data.readInt();
        self.field_2_color2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_3_color3 = data.readInt();
        self.field_3_color3 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_4_color4 = data.readInt();
        self.field_4_color4 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: bytesRemaining -= size;
        bytes_remaining -= size;
        // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0 {
            // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expecting no remaining data but got {} byte(s).", bytes_remaining)));
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
        // Java: this.field_1_color1 = data.readInt();
        self.field_1_color1 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_2_color2 = data.readInt();
        self.field_2_color2 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_3_color3 = data.readInt();
        self.field_3_color3 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: this.field_4_color4 = data.readInt();
        self.field_4_color4 = data.readInt();
        // Java: size += 4;
        size += 4;
        // Java: bytesRemaining -= size;
        bytes_remaining -= size;
        // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0 {
            // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expecting no remaining data but got {} byte(s).", bytes_remaining)));
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
        // Java: int pos = offset + 2;
        let mut pos = offset + 2;
        // Java: LittleEndian.putShort(data, pos, this.getRecordId());
        le_put_short(data, pos as usize, self.getRecordId());
        // Java: pos += 2;
        pos += 2;
        // Java: int remainingBytes = this.getRecordSize() - 8;
        let _remaining_bytes = self.getRecordSize() - 8;
        // Java: LittleEndian.putInt(data, pos, remainingBytes);
        le_put_int(data, pos as usize, _remaining_bytes);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_1_color1);
        le_put_int(data, pos as usize, self.field_1_color1);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_2_color2);
        le_put_int(data, pos as usize, self.field_2_color2);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_3_color3);
        le_put_int(data, pos as usize, self.field_3_color3);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putInt(data, pos, this.field_4_color4);
        le_put_int(data, pos as usize, self.field_4_color4);
        // Java: pos += 4;
        let _pos = pos + 4;
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
        // Java: return this.getRecordSize();
        self.getRecordSize()
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 24;
        24
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return -3810;
        -3810
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "SplitMenuColors";
        String::from("SplitMenuColors")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex((short)-3810) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  Color1: 0x" + HexDump.toHex(this.field_1_color1) + '\n' + "  Color2: 0x" + HexDump.toHex(this.field_2_color2) + '\n' + "  Color3: 0x" + HexDump.toHex(this.field_3_color3) + '\n' + "  Color4: 0x" + HexDump.toHex(this.field_4_color4) + '\n' + "";
        format!("EscherSplitMenuColorsRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  Color1: 0x{:08X}\n  Color2: 0x{:08X}\n  Color3: 0x{:08X}\n  Color4: 0x{:08X}\n",
            -3810i16 as u16, self.base.getOptions() as u16,
            self.field_1_color1 as u32, self.field_2_color2 as u32, self.field_3_color3 as u32, self.field_4_color4 as u32)
    }

    // Java: int getColor1()
    pub fn getColor1(&self) -> i32 {
        // Java: return this.field_1_color1;
        self.field_1_color1
    }

    // Java: void setColor1(int field_1_color1)
    pub fn setColor1(&mut self, field_1_color1: i32) {
        // Java: this.field_1_color1 = field_1_color1;
        self.field_1_color1 = field_1_color1;
    }

    // Java: int getColor2()
    pub fn getColor2(&self) -> i32 {
        // Java: return this.field_2_color2;
        self.field_2_color2
    }

    // Java: void setColor2(int field_2_color2)
    pub fn setColor2(&mut self, field_2_color2: i32) {
        // Java: this.field_2_color2 = field_2_color2;
        self.field_2_color2 = field_2_color2;
    }

    // Java: int getColor3()
    pub fn getColor3(&self) -> i32 {
        // Java: return this.field_3_color3;
        self.field_3_color3
    }

    // Java: void setColor3(int field_3_color3)
    pub fn setColor3(&mut self, field_3_color3: i32) {
        // Java: this.field_3_color3 = field_3_color3;
        self.field_3_color3 = field_3_color3;
    }

    // Java: int getColor4()
    pub fn getColor4(&self) -> i32 {
        // Java: return this.field_4_color4;
        self.field_4_color4
    }

    // Java: void setColor4(int field_4_color4)
    pub fn setColor4(&mut self, field_4_color4: i32) {
        // Java: this.field_4_color4 = field_4_color4;
        self.field_4_color4 = field_4_color4;
    }
}
