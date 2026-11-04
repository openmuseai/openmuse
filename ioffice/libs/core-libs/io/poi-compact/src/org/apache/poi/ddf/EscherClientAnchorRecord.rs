// Java: import java.io.IOException
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian
use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::EscherSerializationListener::EscherSerializationListener;

fn le_put_short(data: &mut [u8], offset: i32, value: i16) {
    let off = offset as usize;
    data[off] = value as u8;
    data[off + 1] = (value >> 8) as u8;
}

fn le_put_int(data: &mut [u8], offset: i32, value: i32) {
    let off = offset as usize;
    data[off] = value as u8;
    data[off + 1] = (value >> 8) as u8;
    data[off + 2] = (value >> 16) as u8;
    data[off + 3] = (value >> 24) as u8;
}

// Java field: short RECORD_ID
pub const RECORD_ID: i16 = -4080;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtClientAnchor";

// Java: public class EscherClientAnchorRecord extends EscherRecord
pub struct EscherClientAnchorRecord {
    pub(crate) base: EscherRecord,
    // Java field: short field_1_flag
    pub(crate) field_1_flag: i16,
    // Java field: short field_2_col1
    pub(crate) field_2_col1: i16,
    // Java field: short field_3_dx1
    pub(crate) field_3_dx1: i16,
    // Java field: short field_4_row1
    pub(crate) field_4_row1: i16,
    // Java field: short field_5_dy1
    pub(crate) field_5_dy1: i16,
    // Java field: short field_6_col2
    pub(crate) field_6_col2: i16,
    // Java field: short field_7_dx2
    pub(crate) field_7_dx2: i16,
    // Java field: short field_8_row2
    pub(crate) field_8_row2: i16,
    // Java field: short field_9_dy2
    pub(crate) field_9_dy2: i16,
    // Java field: byte[] remainingData
    pub(crate) remainingData: Vec<u8>,
    // Java field: boolean shortRecord
    shortRecord: bool,
    // Java field: boolean wordFormat
    pub(crate) wordFormat: bool,
}

// Java: public class EscherClientAnchorRecord extends EscherRecord
impl EscherClientAnchorRecord {
    // Java: EscherClientAnchorRecord()
    pub fn new() -> Self {
      // Java: super();
      EscherClientAnchorRecord {
          base: EscherRecord::new(),
          field_1_flag: 0,
          field_2_col1: 0,
          field_3_dx1: 0,
          field_4_row1: 0,
          field_5_dy1: 0,
          field_6_col2: 0,
          field_7_dx2: 0,
          field_8_row2: 0,
          field_9_dy2: 0,
          remainingData: Vec::new(),
          shortRecord: false,
          wordFormat: false,
      }
    }

    pub fn getOptions(&self) -> i16 { self.base.getOptions() }
    pub fn getRemainingBytes(&self) -> i32 { self.base._header.remainingBytes }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> i32 {
      // Java: int bytesRemaining = this.readHeader(data, offset);
      let mut bytes_remaining = self.base.readHeader_stream(data, offset);
      // Java: int pos = offset + 8;
      let pos = offset + 8;
      // Java: int size = 0;
      let mut size = 0;
      // Java: data.seek((long)pos);
      data.seek(pos as i64);
      // Java: if (bytesRemaining == 4)
        if bytes_remaining == 4
      {
         // Java: this.wordFormat = true;
         self.wordFormat = true;
      }
      // Java: else
        else
      {
         // Java: this.field_1_flag = data.readShort();
         self.field_1_flag = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: this.field_2_col1 = data.readShort();
         self.field_2_col1 = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: this.field_3_dx1 = data.readShort();
         self.field_3_dx1 = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: this.field_4_row1 = data.readShort();
         self.field_4_row1 = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: if (bytesRemaining >= 18)
            if bytes_remaining >= 18
         {
            // Java: this.field_5_dy1 = data.readShort();
            self.field_5_dy1 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_6_col2 = data.readShort();
            self.field_6_col2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_7_dx2 = data.readShort();
            self.field_7_dx2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_8_row2 = data.readShort();
            self.field_8_row2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_9_dy2 = data.readShort();
            self.field_9_dy2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.shortRecord = false;
            self.shortRecord = false;
         }
         // Java: else
            else
         {
            // Java: this.shortRecord = true;
            self.shortRecord = true;
         }
      }
      // Java: bytesRemaining -= size;
      bytes_remaining -= size;
      // Java: if (bytesRemaining > 0)
        if bytes_remaining > 0
      {
         // Java: this.remainingData = new byte[bytesRemaining];
         self.remainingData = vec![0u8; bytes_remaining as usize];
         // Java: data.read(this.remainingData, 0, bytesRemaining);
         data.read_off(&mut self.remainingData, 0, bytes_remaining);
         // Java: return 8 + size + bytesRemaining;
         return 8 + size + bytes_remaining;
      }
      // Java: else
        else
      {
         // Java: return 8 + size;
         return 8 + size;
      }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> i32 {
      // Java: int bytesRemaining = this.getRemainingBytes();
      let mut bytes_remaining = self.getRemainingBytes();
      // Java: int pos = offset + 8;
      // Java: int size = 0;
      let mut size = 0;
      // Java: if (bytesRemaining == 4)
        if bytes_remaining == 4
      {
         // Java: this.wordFormat = true;
         self.wordFormat = true;
      }
      // Java: else
        else
      {
         // Java: this.field_1_flag = data.readShort();
         self.field_1_flag = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: this.field_2_col1 = data.readShort();
         self.field_2_col1 = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: this.field_3_dx1 = data.readShort();
         self.field_3_dx1 = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: this.field_4_row1 = data.readShort();
         self.field_4_row1 = data.readShort();
         // Java: size += 2;
         size += 2;
         // Java: if (bytesRemaining >= 18)
            if bytes_remaining >= 18
         {
            // Java: this.field_5_dy1 = data.readShort();
            self.field_5_dy1 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_6_col2 = data.readShort();
            self.field_6_col2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_7_dx2 = data.readShort();
            self.field_7_dx2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_8_row2 = data.readShort();
            self.field_8_row2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.field_9_dy2 = data.readShort();
            self.field_9_dy2 = data.readShort();
            // Java: size += 2;
            size += 2;
            // Java: this.shortRecord = false;
            self.shortRecord = false;
         }
         // Java: else
            else
         {
            // Java: this.shortRecord = true;
            self.shortRecord = true;
         }
      }
      // Java: bytesRemaining -= size;
      bytes_remaining -= size;
      // Java: this.remainingData = new byte[bytesRemaining];
      self.remainingData = vec![0u8; bytes_remaining as usize];
      // Java: data.read(this.remainingData);
      data.read(&mut self.remainingData);
      // Java: return 8 + size + bytesRemaining;
      8 + size + bytes_remaining
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize_with_listener(&mut self, offset: i32, data: &mut [u8], listener: &mut dyn EscherSerializationListener) -> i32 {
      // Java: listener.beforeRecordSerialize(offset, this.getRecordId(), this);
      listener.beforeRecordSerialize(offset, self.getRecordId(), &self.base);
      // Java: if (this.remainingData == null)
      if self.remainingData.is_empty()
      {
         // Java: this.remainingData = new byte[0];
      }
      // Java: LittleEndian.putShort(data, offset, this.getOptions());
      le_put_short(data, offset, self.getOptions());
      // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
      le_put_short(data, offset + 2, self.getRecordId());
      // Java: int pos = offset + 8;
      let mut pos = offset + 8;
      // Java: if (this.wordFormat && this.remainingData.length == 4)
        if self.wordFormat && self.remainingData.len() == 4
      {
         // Java: LittleEndian.putInt(data, offset + 4, 4);
         le_put_int(data, offset + 4, 4);
         // Java: System.arraycopy(this.remainingData, 0, data, pos, this.remainingData.length);
         let rem_len = self.remainingData.len();
         data[pos as usize..pos as usize + rem_len].copy_from_slice(&self.remainingData);
         // Java: pos += this.remainingData.length;
         pos += rem_len as i32;
      }
      // Java: else
        else
      {
         // Java: int remainingBytes = this.remainingData.length + (this.shortRecord ? 8 : 18);
         let remaining_bytes = self.remainingData.len() as i32 + if self.shortRecord { 8 } else { 18 };
         // Java: LittleEndian.putInt(data, offset + 4, remainingBytes);
         le_put_int(data, offset + 4, remaining_bytes);
         // Java: LittleEndian.putShort(data, offset + 8, this.field_1_flag);
         le_put_short(data, offset + 8, self.field_1_flag);
         // Java: LittleEndian.putShort(data, offset + 10, this.field_2_col1);
         le_put_short(data, offset + 10, self.field_2_col1);
         // Java: LittleEndian.putShort(data, offset + 12, this.field_3_dx1);
         le_put_short(data, offset + 12, self.field_3_dx1);
         // Java: LittleEndian.putShort(data, offset + 14, this.field_4_row1);
         le_put_short(data, offset + 14, self.field_4_row1);
         // Java: if (!this.shortRecord)
            if !self.shortRecord
         {
            // Java: LittleEndian.putShort(data, offset + 16, this.field_5_dy1);
            le_put_short(data, offset + 16, self.field_5_dy1);
            // Java: LittleEndian.putShort(data, offset + 18, this.field_6_col2);
            le_put_short(data, offset + 18, self.field_6_col2);
            // Java: LittleEndian.putShort(data, offset + 20, this.field_7_dx2);
            le_put_short(data, offset + 20, self.field_7_dx2);
            // Java: LittleEndian.putShort(data, offset + 22, this.field_8_row2);
            le_put_short(data, offset + 22, self.field_8_row2);
            // Java: LittleEndian.putShort(data, offset + 24, this.field_9_dy2);
            le_put_short(data, offset + 24, self.field_9_dy2);
         }
         // Java: System.arraycopy(this.remainingData, 0, data, offset + (this.shortRecord ? 16 : 26), this.remainingData.length);
         let dest_offset = offset + if self.shortRecord { 16 } else { 26 };
         let rem_len = self.remainingData.len();
         data[dest_offset as usize..dest_offset as usize + rem_len].copy_from_slice(&self.remainingData);
         // Java: pos += (this.shortRecord ? 8 : 18) + this.remainingData.length;
         pos += (if self.shortRecord { 8 } else { 18 }) + rem_len as i32;
      }
      // Java: listener.afterRecordSerialize(pos, this.getRecordId(), pos - offset, this);
      listener.afterRecordSerialize(pos, self.getRecordId(), pos - offset, &self.base);
      // Java: return pos - offset;
      pos - offset
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
      // Java: return this.wordFormat ? 12 : 8 + (this.shortRecord ? 8 : 18) + (this.remainingData == null ? 0 : this.remainingData.length);
      if self.wordFormat {
          12
      } else {
          8 + (if self.shortRecord { 8 } else { 18 }) + self.remainingData.len() as i32
      }
    }

    // Java: short getRecordId()
    pub fn getRecordId(&self) -> i16 {
      // Java: return -4080;
      -4080
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
      // Java: return "ClientAnchor";
      "ClientAnchor".to_string()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
      // Java: String nl = System.getProperty("line.separator");
      let nl = "\n";
      // Java: String extraData = "";
      let extra_data = "";
      // Java: return ...
      format!("EscherClientAnchorRecord:{nl}\
          \x20 RecordId: 0x{:04x}{nl}\
          \x20 Options: 0x{:04x}{nl}\
          \x20 Flag: {}{nl}\
          \x20 Col1: {}{nl}\
          \x20 DX1: {}{nl}\
          \x20 Row1: {}{nl}\
          \x20 DY1: {}{nl}\
          \x20 Col2: {}{nl}\
          \x20 DX2: {}{nl}\
          \x20 Row2: {}{nl}\
          \x20 DY2: {}{nl}\
          \x20 Extra Data:{nl}\
          {extra_data}",
          RECORD_ID as u16,
          self.getOptions() as u16,
          self.field_1_flag,
          self.field_2_col1,
          self.field_3_dx1,
          self.field_4_row1,
          self.field_5_dy1,
          self.field_6_col2,
          self.field_7_dx2,
          self.field_8_row2,
          self.field_9_dy2,
          nl = nl)
    }

    // Java: short getFlag()
    pub fn getFlag(&self) -> i16 {
      // Java: return this.field_1_flag;
      self.field_1_flag
    }

    // Java: void setFlag(short field_1_flag)
    pub fn setFlag(&mut self, field_1_flag: i16) {
      // Java: this.field_1_flag = field_1_flag;
      self.field_1_flag = field_1_flag;
    }

    // Java: short getCol1()
    pub fn getCol1(&self) -> i16 {
      // Java: return this.field_2_col1;
      self.field_2_col1
    }

    // Java: void setCol1(short field_2_col1)
    pub fn setCol1(&mut self, field_2_col1: i16) {
      // Java: this.field_2_col1 = field_2_col1;
      self.field_2_col1 = field_2_col1;
    }

    // Java: short getDx1()
    pub fn getDx1(&self) -> i16 {
      // Java: return this.field_3_dx1;
      self.field_3_dx1
    }

    // Java: void setDx1(short field_3_dx1)
    pub fn setDx1(&mut self, field_3_dx1: i16) {
      // Java: this.field_3_dx1 = field_3_dx1;
      self.field_3_dx1 = field_3_dx1;
    }

    // Java: short getRow1()
    pub fn getRow1(&self) -> i16 {
      // Java: return this.field_4_row1;
      self.field_4_row1
    }

    // Java: void setRow1(short field_4_row1)
    pub fn setRow1(&mut self, field_4_row1: i16) {
      // Java: this.field_4_row1 = field_4_row1;
      self.field_4_row1 = field_4_row1;
    }

    // Java: short getDy1()
    pub fn getDy1(&self) -> i16 {
      // Java: return this.field_5_dy1;
      self.field_5_dy1
    }

    // Java: void setDy1(short field_5_dy1)
    pub fn setDy1(&mut self, field_5_dy1: i16) {
      // Java: this.shortRecord = false;
      self.shortRecord = false;
      // Java: this.field_5_dy1 = field_5_dy1;
      self.field_5_dy1 = field_5_dy1;
    }

    // Java: short getCol2()
    pub fn getCol2(&self) -> i16 {
      // Java: return this.field_6_col2;
      self.field_6_col2
    }

    // Java: void setCol2(short field_6_col2)
    pub fn setCol2(&mut self, field_6_col2: i16) {
      // Java: this.shortRecord = false;
      self.shortRecord = false;
      // Java: this.field_6_col2 = field_6_col2;
      self.field_6_col2 = field_6_col2;
    }

    // Java: short getDx2()
    pub fn getDx2(&self) -> i16 {
      // Java: return self.field_7_dx2;
      self.field_7_dx2
    }

    // Java: void setDx2(short field_7_dx2)
    pub fn setDx2(&mut self, field_7_dx2: i16) {
      // Java: this.shortRecord = false;
      self.shortRecord = false;
      // Java: this.field_7_dx2 = field_7_dx2;
      self.field_7_dx2 = field_7_dx2;
    }

    // Java: short getRow2()
    pub fn getRow2(&self) -> i16 {
      // Java: return this.field_8_row2;
      self.field_8_row2
    }

    // Java: void setRow2(short field_8_row2)
    pub fn setRow2(&mut self, field_8_row2: i16) {
      // Java: this.shortRecord = false;
      self.shortRecord = false;
      // Java: this.field_8_row2 = field_8_row2;
      self.field_8_row2 = field_8_row2;
    }

    // Java: short getDy2()
    pub fn getDy2(&self) -> i16 {
      // Java: return this.field_9_dy2;
      self.field_9_dy2
    }

    // Java: void setDy2(short field_9_dy2)
    pub fn setDy2(&mut self, field_9_dy2: i16) {
      // Java: this.shortRecord = false;
      self.shortRecord = false;
      // Java: this.field_9_dy2 = field_9_dy2;
      self.field_9_dy2 = field_9_dy2;
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

    // Java: boolean isWordFormat()
    pub fn isWordFormat(&self) -> bool {
      // Java: return this.wordFormat;
      self.wordFormat
    }

    // Java: void setWordFormat(boolean wordFormat)
    pub fn setWordFormat(&mut self, word_format: bool) {
      // Java: this.wordFormat = wordFormat;
      self.wordFormat = word_format;
    }

}
