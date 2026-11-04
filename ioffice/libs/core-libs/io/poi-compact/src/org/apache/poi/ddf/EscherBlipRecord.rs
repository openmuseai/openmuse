// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.util.FileDataStorage
// Java: import cn.wps.moffice.util.StreamUtil
// Java: import java.io.IOException
// Java: import java.io.InputStream
// Java: import java.io.OutputStream
// Java: import java.security.SecureRandom
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::EscherSerializationListener::EscherSerializationListener;
use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};

static RANDOM_COUNTER: AtomicU32 = AtomicU32::new(0);

fn get_random_int() -> i32 {
    RANDOM_COUNTER.fetch_add(1, Ordering::Relaxed) as i32
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

// Java field: short RECORD_ID_START
pub const RECORD_ID_START: i16 = -4072;
// Java field: short RECORD_ID_END
pub const RECORD_ID_END: i16 = -3817;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "msofbtBlip";
// Java field: int HEADER_SIZE
const HEADER_SIZE: i32 = 8;

// Java: public class EscherBlipRecord extends EscherRecord
pub struct EscherBlipRecord {
    // Java field: byte[] field_1_UID
    pub field_1_UID: Vec<u8>,
    // Java field: byte[] field_2_UID
    field_2_UID: Vec<u8>,
    // Java field: FileDataStorage mPictureData
    pub mPictureData: Option<Vec<u8>>,
    // Java field: int bytesAfterHeader
    pub bytesAfterHeader: i32,
    // Java field: int mark
    pub mark: i32,
    // Java field: String fileName
    pub fileName: Option<String>,
    // Java field: SecureRandom _random
    pub base: EscherRecord,
}

// Java: public class EscherBlipRecord extends EscherRecord
impl EscherBlipRecord {
    // Java: EscherBlipRecord()
    pub fn new() -> Self {
        // Java: super();
        EscherBlipRecord {
            field_1_UID: Vec::new(),
            field_2_UID: Vec::new(),
            mPictureData: None,
            bytesAfterHeader: 0,
            mark: 0,
            fileName: None,
            base: EscherRecord::new(),
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory) -> i32 {
        // Java: this.bytesAfterHeader = this.readHeader(data, offset);
        self.bytesAfterHeader = self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let pos = offset + 8;
        // Java: this.mark = pos;
        self.mark = pos;
        // Java: this.fileName = "picture-" + getRandomInt();
        self.fileName = Some(format!("picture-{}", get_random_int()));
        // Java: return this.bytesAfterHeader + 8;
        self.bytesAfterHeader + 8
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, _record_factory: &dyn EscherRecordFactory, _picture_directory: &str, _extension: &str) -> i32 {
        // Java: int bytesAfterHeader = this.getRemainingBytes();
        let bytes_after_header = self.base.getRemainingBytes();
        // Java: int pos = offset + 8;
        let _pos = offset + 8;
        // Java: byte[] blipData = new byte[bytesAfterHeader];
        let mut blip_data = vec![0u8; bytes_after_header as usize];
        // Java: data.read(blipData);
        data.read(&mut blip_data);
        // Java: this.mPictureData = FileDataStorage.createFileDataStorage(blipData);
        self.mPictureData = Some(blip_data);
        // Java: return bytesAfterHeader + 8;
        bytes_after_header + 8
    }

    // Java: int serialize(OutputStream stream)
    pub fn serialize_to_output(&mut self, stream: &mut dyn Write) -> i32 {
        // Java: int offset = 0;
        let offset = 0;
        // Java: byte[] data = new byte[4];
        let mut data = [0u8; 4];
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(&mut data, offset, self.base.getOptions());
        // Java: LittleEndian.putShort(data, offset + 2, this.getRecordId());
        le_put_short(&mut data, offset + 2, self.base.getRecordId());
        // Java: stream.write(data);
        let _ = stream.write(&data);
        // Java: InputStream reader = this.mPictureData.getReader();
        // Java: StreamUtil.copyStream(reader, stream);
        // Java: reader.close();
        if let Some(ref pd) = self.mPictureData {
            let _ = stream.write(pd);
        }
        // Java: return this.mPictureData.getSize() + 4;
        self.mPictureData.as_ref().map(|p| p.len() as i32).unwrap_or(0) + 4
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize_with_listener(&mut self, _offset: i32, _data: &mut [u8], _listener: &mut dyn EscherSerializationListener) -> i32 {
        // Java: throw new RuntimeException("This should never be called");
        panic!("This should never be called");
    }

    // Java: void readPicture(DocumentInputStream data, File picFile, String extension)
    pub fn readPicture(&mut self, data: &mut DocumentInputStream, _extension: &str) {
        // Java: File file = new File(picFile.getParentFile(), "picture-" + getRandomInt() + "." + extension);
        // Java: data.seek((long)this.mark);
        data.seek(self.mark as i64);
        // Java: this.mPictureData = FileDataStorage.createFileDataStorage(data, this.bytesAfterHeader, file);
        let mut buf = vec![0u8; self.bytesAfterHeader as usize];
        data.readFully(&mut buf);
        self.mPictureData = Some(buf);
    }

    // Java: FileDataStorage getPictureData()
    pub fn getPictureData(&self) -> Option<&Vec<u8>> {
        // Java: return this.mPictureData;
        self.mPictureData.as_ref()
    }

    // Java: String getPictureName()
    pub fn getPictureName(&self) -> Option<&String> {
        // Java: return this.fileName;
        self.fileName.as_ref()
    }

    // Java: byte[] getPicturedata()
    pub fn getPicturedata(&self) -> Option<&Vec<u8>> {
        // Java: return this.mPictureData.getData();
        self.mPictureData.as_ref()
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return this.mPictureData.getSize() + 8;
        self.mPictureData.as_ref().map(|p| p.len() as i32).unwrap_or(0) + 8
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "Blip";
        String::from("Blip")
    }

    // Java: byte[] getUID()
    pub fn getUID(&self) -> &Vec<u8> {
        // Java: return this.field_1_UID;
        &self.field_1_UID
    }

    // Java: void setUID(byte[] uid)
    pub fn setUID(&mut self, uid: Vec<u8>) {
        // Java: this.field_1_UID = uid;
        self.field_1_UID = uid;
    }

    // Java: void setPictureData(FileDataStorage pData)
    pub fn setPictureData_storage(&mut self, p_data: Vec<u8>) {
        // Java: this.mPictureData = pData;
        self.mPictureData = Some(p_data);
    }

    // Java: void setPictureData(byte[] data)
    pub fn setPictureData_bytes(&mut self, data: &[u8]) {
        // Java: if (data != null)
        // Java: try
        // Java: this.mPictureData = FileDataStorage.createFileDataStorage(data);
        self.mPictureData = Some(data.to_vec());
        // Java: catch (IOException e)
        // Java: throw new RuntimeException(e.getMessage());
    }

    // Java: static int getRandomInt()
    pub fn getRandomInt() -> i32 {
        // Java: return _random.nextInt();
        get_random_int()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String extraData = this.mPictureData.toString();
        let extra_data = format!("{} bytes", self.mPictureData.as_ref().map(|p| p.len()).unwrap_or(0));
        // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex(this.getRecordId()) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  Extra Data:" + '\n' + extraData;
        format!("EscherBlipRecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  Extra Data:\n{}",
            self.base.getRecordId(), self.base.getOptions(), extra_data)
    }
}
