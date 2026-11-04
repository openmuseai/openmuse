// Java: import cn.wps.kfc.api.list.IIntList
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
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
pub const RECORD_ID: i16 = -3814;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtColorMRU";

// Java: public class EscherColorMRURecord extends EscherRecord
pub struct EscherColorMRURecord {
    pub(crate) base: EscherRecord,
    // Java field: IIntList rgmsocr
    pub(crate) rgmsocr: Vec<i32>,
}

// Java: public class EscherColorMRURecord extends EscherRecord
impl EscherColorMRURecord {
    // Java: EscherColorMRURecord()
    pub fn new() -> Self {
        // Java: super();
        EscherColorMRURecord {
            base: EscherRecord::new(),
            rgmsocr: Vec::new(),
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
        // Java: while(bytesRemaining > 0)
        while bytes_remaining > 0 {
            // Java: this.rgmsocr.add(data.readInt());
            self.rgmsocr.push(data.readInt());
            // Java: size += 4;
            size += 4;
            // Java: bytesRemaining -= size;
            bytes_remaining -= size;
        }
        // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0 {
            // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expecting no remaining data but got {} byte(s).", bytes_remaining)));
        }
        // Java: else
        {
            // Java: return 8 + size;
            Ok(8 + size)
        }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, _offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> io::Result<i32> {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let mut bytes_remaining = self.base.getRemainingBytes();
        // Java: int size;
        // Java: for(size = 0; bytesRemaining > 0; bytesRemaining -= size)
        let mut size = 0;
        while bytes_remaining > 0 {
            // Java: this.rgmsocr.add(data.readInt());
            self.rgmsocr.push(data.readInt());
            // Java: size += 4;
            size += 4;
            bytes_remaining -= size;
        }
        // Java: if (bytesRemaining != 0)
        if bytes_remaining != 0 {
            // Java: throw new RecordFormatException("Expecting no remaining data but got " + bytesRemaining + " byte(s).");
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Expecting no remaining data but got {} byte(s).", bytes_remaining)));
        }
        // Java: else
        {
            // Java: return 8 + size;
            Ok(8 + size)
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
        le_put_short(data, pos as usize, self.base.getRecordId());
        // Java: pos += 2;
        pos += 2;
        // Java: int remainingBytes = this.getRecordSize() - 8;
        let remaining_bytes = self.getRecordSize() - 8;
        // Java: LittleEndian.putInt(data, pos, remainingBytes);
        le_put_int(data, pos as usize, remaining_bytes);
        // Java: for(int i = 0; i < this.rgmsocr.size(); ++i)
        for i in 0..self.rgmsocr.len() {
            // Java: LittleEndian.putInt(data, pos, this.rgmsocr.get(i));
            le_put_int(data, pos as usize, self.rgmsocr[i]);
            // Java: pos += 4;
            pos += 4;
        }
        // Java: listener.afterRecordSerialize(pos, this.getRecordId(), remainingBytes, this);
        // Java: return this.getRecordSize();
        self.getRecordSize()
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return 8 + this.rgmsocr.size() * 4;
        8 + self.rgmsocr.len() as i32 * 4
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "ColorMRU";
        String::from("ColorMRU")
    }
}
