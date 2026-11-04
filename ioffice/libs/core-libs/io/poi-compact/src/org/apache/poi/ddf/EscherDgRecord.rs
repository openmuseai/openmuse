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
pub const RECORD_ID_CONST: i16 = -4088;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtDg";

// Java: public class EscherDgRecord extends EscherRecord
pub struct EscherDgRecord {
    pub(crate) base: EscherRecord,
    // Java field: int field_1_numShapes
    pub(crate) field_1_numShapes: i32,
    // Java field: int field_2_lastMSOSPID
    pub(crate) field_2_lastMSOSPID: i32,
}

// Java: public class EscherDgRecord extends EscherRecord
impl EscherDgRecord {
    // Java: EscherDgRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherDgRecord {
            base: EscherRecord::new(),
            field_1_numShapes: 0,
            field_2_lastMSOSPID: 0,
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> io::Result<i32> {
        // Java: this.readHeader(data, offset);
        self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let pos = offset + 8;
        // Java: data.seek((long)pos);
        data.seek(pos as i64);
        // Java: this.field_1_numShapes = data.readInt();
        self.field_1_numShapes = data.readInt();
        // Java: this.field_2_lastMSOSPID = data.readInt();
        self.field_2_lastMSOSPID = data.readInt();
        // Java: return this.getRecordSize();
        Ok(self.getRecordSize())
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int pos = offset + 8;
        let _pos = offset + 8;
        // Java: this.field_1_numShapes = data.readInt();
        self.field_1_numShapes = data.readInt();
        // Java: this.field_2_lastMSOSPID = data.readInt();
        self.field_2_lastMSOSPID = data.readInt();
        // Java: return this.getRecordSize();
        Ok(self.getRecordSize())
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize(&self, offset: i32, data: &mut [u8], _listener: &dyn EscherSerializationListener) -> i32 {
        // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(data, offset as usize, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(data, (offset + 2) as usize, self.getRecordId());
        // Java: LittleEndian.putInt(data, offset + 4, 8);
        le_put_int(data, (offset + 4) as usize, 8);
        // Java: LittleEndian.putInt(data, offset + 8, this.field_1_numShapes);
        le_put_int(data, (offset + 8) as usize, self.field_1_numShapes);
        // Java: LittleEndian.putInt(data, offset + 12, this.field_2_lastMSOSPID);
        le_put_int(data, (offset + 12) as usize, self.field_2_lastMSOSPID);
        // Java: listener.afterRecordSerialize(offset + 16, this.getRecordId(), this.getRecordSize(), this);
        // Java: return this.getRecordSize();
        self.getRecordSize()
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 16;
        16
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
        // Java: return -4088;
        -4088
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "Dg";
        String::from("Dg")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex((short)-4088) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  NumShapes: " + this.field_1_numShapes + '\n' + "  LastMSOSPID: " + this.field_2_lastMSOSPID + '\n';
        format!("EscherDgRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  NumShapes: {}\n  LastMSOSPID: {}\n",
            -4088i16 as u16, self.base.getOptions() as u16, self.field_1_numShapes, self.field_2_lastMSOSPID)
    }

    // Java: int getNumShapes()
    pub fn getNumShapes(&self) -> i32 {
        // Java: return this.field_1_numShapes;
        self.field_1_numShapes
    }

    // Java: void setNumShapes(int field_1_numShapes)
    pub fn setNumShapes(&mut self, field_1_num_shapes: i32) {
        // Java: this.field_1_numShapes = field_1_numShapes;
        self.field_1_numShapes = field_1_num_shapes;
    }

    // Java: int getLastMSOSPID()
    pub fn getLastMSOSPID(&self) -> i32 {
        // Java: return this.field_2_lastMSOSPID;
        self.field_2_lastMSOSPID
    }

    // Java: void setLastMSOSPID(int field_2_lastMSOSPID)
    pub fn setLastMSOSPID(&mut self, field_2_last_msospid: i32) {
        // Java: this.field_2_lastMSOSPID = field_2_lastMSOSPID;
        self.field_2_lastMSOSPID = field_2_last_msospid;
    }

    // Java: short getDrawingGroupId()
    pub fn getDrawingGroupId(&self) -> i16 {
        // Java: return (short)(this.getOptions() >> 4);
        (self.base.getOptions() >> 4) as i16
    }

    // Java: void incrementShapeCount()
    pub fn incrementShapeCount(&mut self) {
        // Java: ++this.field_1_numShapes;
        self.field_1_numShapes += 1;
    }
}
