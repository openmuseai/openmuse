// Java: import cn.wps.moffice.util.FileDataStorage
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import org.apache.poi.hwpf.model.io.HWPFOutputStream
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
// Java: import org.apache.poi.poifs.stream.POIOutputStream
use crate::org::apache::poi::poifs::stream::POIOutputStream::POIOutputStream;
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use super::EscherBlipRecord::EscherBlipRecord;
use super::EscherRecord::EscherRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use super::EscherSerializationListener::EscherSerializationListener;
use std::io::Write;

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
    data[offset + 2] = bytes[2];
    data[offset + 3] = bytes[3];
}

// Java field: short RECORD_ID
pub const RECORD_ID: i16 = -4089;
// Java field: String RECORD_DESCRIPTION
pub const RECORD_DESCRIPTION: &str = "MsofbtBSE";
// Java field: byte BT_ERROR
pub const BT_ERROR: i8 = 0;
// Java field: byte BT_UNKNOWN
pub const BT_UNKNOWN: i8 = 1;
// Java field: byte BT_EMF
pub const BT_EMF: i8 = 2;
// Java field: byte BT_WMF
pub const BT_WMF: i8 = 3;
// Java field: byte BT_PICT
pub const BT_PICT: i8 = 4;
// Java field: byte BT_JPEG
pub const BT_JPEG: i8 = 5;
// Java field: byte BT_PNG
pub const BT_PNG: i8 = 6;
// Java field: byte BT_DIB
pub const BT_DIB: i8 = 7;
// Java field: byte BT_WBIBITMAP
pub const BT_WBIBITMAP: i8 = 8;
// Java field: byte BT_TIFF
pub const BT_TIFF: i8 = 17;
// Java field: byte BT_CMYKJPEG
pub const BT_CMYKJPEG: i8 = 18;
// Java field: byte hdphoto
pub const hdphoto: i8 = 19;

// Java: public class EscherBSERecord extends EscherRecord
pub struct EscherBSERecord {
    // Java field: byte field_1_blipTypeWin32
    pub field_1_blipTypeWin32: i8,
    // Java field: byte field_2_blipTypeMacOS
    pub field_2_blipTypeMacOS: i8,
    // Java field: byte[] field_3_uid
    pub field_3_uid: Vec<u8>,
    // Java field: short field_4_tag
    pub field_4_tag: i16,
    // Java field: int field_5_size
    pub field_5_size: i32,
    // Java field: int field_6_ref
    pub field_6_ref: i32,
    // Java field: int field_7_offset
    pub field_7_offset: i32,
    // Java field: byte field_8_usage
    pub field_8_usage: i8,
    // Java field: byte field_9_name
    pub field_9_name: i8,
    // Java field: byte field_10_unused2
    pub field_10_unused2: i8,
    // Java field: byte field_11_unused3
    pub field_11_unused3: i8,
    // Java field: EscherBlipRecord field_12_blipRecord
    pub field_12_blipRecord: Option<EscherBlipRecord>,
    // Java field: POIOutputStream delayStream
    delayStream: Option<POIOutputStream>,
    // Java field: String extension
    pub extension: Option<String>,
    // Java field: byte[] _remainingData
    pub _remainingData: Vec<u8>,
    pub base: EscherRecord,
}

// Java: public class EscherBSERecord extends EscherRecord
impl EscherBSERecord {
    // Java: EscherBSERecord()
    pub fn new() -> Self {
        // Java: super();
        EscherBSERecord {
            field_1_blipTypeWin32: 0,
            field_2_blipTypeMacOS: 0,
            field_3_uid: vec![0u8; 16],
            field_4_tag: 0,
            field_5_size: 0,
            field_6_ref: 0,
            field_7_offset: 0,
            field_8_usage: 0,
            field_9_name: 0,
            field_10_unused2: 0,
            field_11_unused3: 0,
            field_12_blipRecord: None,
            delayStream: None,
            extension: None,
            _remainingData: Vec::new(),
            base: EscherRecord::new(),
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, record_factory: &dyn EscherRecordFactory) -> i32 {
        // Java: int bytesRemaining = this.readHeader(data, offset);
        let mut bytes_remaining = self.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let pos = offset + 8;
        // Java: data.seek((long)pos);
        data.seek(pos as i64);
        // Java: this.field_1_blipTypeWin32 = data.readByte();
        self.field_1_blipTypeWin32 = data.readByte();
        // Java: this.field_2_blipTypeMacOS = data.readByte();
        self.field_2_blipTypeMacOS = data.readByte();
        // Java: data.read(this.field_3_uid = new byte[16], 0, 16);
        self.field_3_uid = vec![0u8; 16];
        data.read_off(&mut self.field_3_uid, 0, 16);
        // Java: this.field_4_tag = data.readShort();
        self.field_4_tag = data.readShort();
        // Java: this.field_5_size = data.readInt();
        self.field_5_size = data.readInt();
        // Java: this.field_6_ref = data.readInt();
        self.field_6_ref = data.readInt();
        // Java: this.field_7_offset = data.readInt();
        self.field_7_offset = data.readInt();
        // Java: this.field_8_usage = data.readByte();
        self.field_8_usage = data.readByte();
        // Java: this.field_9_name = data.readByte();
        self.field_9_name = data.readByte();
        // Java: this.field_10_unused2 = data.readByte();
        self.field_10_unused2 = data.readByte();
        // Java: this.field_11_unused3 = data.readByte();
        self.field_11_unused3 = data.readByte();
        // Java: bytesRemaining -= 36;
        bytes_remaining -= 36;
        // Java: this.extension = getBlipType(this.field_1_blipTypeWin32);
        self.extension = Self::getBlipType(self.field_1_blipTypeWin32);
        // Java: int bytesRead = 0;
        let mut bytes_read = 0;
        // Java: if (bytesRemaining > 0)
        if bytes_remaining > 0 {
            // Java: EscherRecord escherRecord = recordFactory.createRecord(data, pos + 36);
            // Java: if (escherRecord instanceof EscherBlipRecord)
            let mut blip = EscherBlipRecord::new();
            // Java: this.field_12_blipRecord = (EscherBlipRecord)escherRecord;
            // Java: bytesRead = this.field_12_blipRecord.fillFields(data, pos + 36, recordFactory);
            bytes_read = blip.fillFields_stream(data, pos + 36, record_factory);
            self.field_12_blipRecord = Some(blip);
        }
        // Java: int var10000 = pos + 36 + bytesRead;
        let _var10000 = pos + 36 + bytes_read;
        // Java: bytesRemaining -= bytesRead;
        bytes_remaining -= bytes_read;
        // Java: this._remainingData = new byte[bytesRemaining];
        self._remainingData = vec![0u8; bytes_remaining as usize];
        // Java: data.read(this._remainingData, 0, bytesRemaining);
        data.read_off(&mut self._remainingData, 0, bytes_remaining);
        // Java: return bytesRemaining + 8 + 36 + (this.field_12_blipRecord == null ? 0 : bytesRead);
        bytes_remaining + 8 + 36 + if self.field_12_blipRecord.is_none() { 0 } else { bytes_read }
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, record_factory: &dyn EscherRecordFactory, picture_directory: &str, _extension: &str) -> i32 {
        // Java: int bytesRemaining = this.getRemainingBytes();
        let mut bytes_remaining = self.base.getRemainingBytes();
        // Java: int pos = offset + 8;
        let pos = offset + 8;
        // Java: this.field_1_blipTypeWin32 = data.readByte();
        self.field_1_blipTypeWin32 = data.readByte();
        // Java: this.field_2_blipTypeMacOS = data.readByte();
        self.field_2_blipTypeMacOS = data.readByte();
        // Java: this.field_3_uid = new byte[16];
        self.field_3_uid = vec![0u8; 16];
        // Java: data.read(this.field_3_uid);
        data.read(&mut self.field_3_uid);
        // Java: this.field_4_tag = data.readShort();
        self.field_4_tag = data.readShort();
        // Java: this.field_5_size = data.readInt();
        self.field_5_size = data.readInt();
        // Java: this.field_6_ref = data.readInt();
        self.field_6_ref = data.readInt();
        // Java: this.field_7_offset = data.readInt();
        self.field_7_offset = data.readInt();
        // Java: this.field_8_usage = data.readByte();
        self.field_8_usage = data.readByte();
        // Java: this.field_9_name = data.readByte();
        self.field_9_name = data.readByte();
        // Java: this.field_10_unused2 = data.readByte();
        self.field_10_unused2 = data.readByte();
        // Java: this.field_11_unused3 = data.readByte();
        self.field_11_unused3 = data.readByte();
        // Java: bytesRemaining -= 36;
        bytes_remaining -= 36;
        // Java: int bytesRead = 0;
        let mut bytes_read = 0;
        // Java: if (bytesRemaining > 0)
        if bytes_remaining > 0 {
            // Java: this.field_12_blipRecord = (EscherBlipRecord)recordFactory.createRecord(data, pos + 36);
            let mut blip = EscherBlipRecord::new();
            // Java: bytesRead = this.field_12_blipRecord.fillFields(data, pos + 36, recordFactory, pictureDirectory, getBlipType(this.field_1_blipTypeWin32));
            let blip_type = Self::getBlipType(self.field_1_blipTypeWin32).unwrap_or_default();
            bytes_read = blip.fillFields_block(data, pos + 36, record_factory, picture_directory, &blip_type);
            self.field_12_blipRecord = Some(blip);
        }
        // Java: int var10000 = pos + 36 + bytesRead;
        let _var10000 = pos + 36 + bytes_read;
        // Java: bytesRemaining -= bytesRead;
        bytes_remaining -= bytes_read;
        // Java: if (bytesRemaining > 0)
        if bytes_remaining > 0 {
            // Java: this._remainingData = new byte[bytesRemaining];
            self._remainingData = vec![0u8; bytes_remaining as usize];
            // Java: data.read(this._remainingData);
            data.read(&mut self._remainingData);
        }
        // Java: return bytesRemaining + 8 + 36 + (this.field_12_blipRecord == null ? 0 : this.field_12_blipRecord.getRecordSize());
        bytes_remaining + 8 + 36 + if let Some(ref blip) = self.field_12_blipRecord { blip.getRecordSize() } else { 0 }
    }

    // Java: int serialize(OutputStream stream)
    pub fn serialize_to_output(&mut self, stream: &mut dyn Write) -> i32 {
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: byte[] data = new byte[44];
        let mut data = [0u8; 44];
        // Java: if (this._remainingData == null)
        // Java: this._remainingData = new byte[0];
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(&mut data, offset, self.base.getOptions());
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(data, offset, this.getRecordId());
        le_put_short(&mut data, offset, self.base.getRecordId());
        // Java: offset += 2;
        offset += 2;
        // Java: int blipSize = this.field_12_blipRecord != null && this.field_7_offset != -1 && this.delayStream == null ? this.field_12_blipRecord.getRecordSize() : 0;
        let blip_size = if self.field_12_blipRecord.is_some() && self.field_7_offset != -1 && self.delayStream.is_none() {
            self.field_12_blipRecord.as_ref().unwrap().getRecordSize()
        } else { 0 };
        // Java: int remainingBytes = this._remainingData.length + 36 + blipSize;
        let remaining_bytes = self._remainingData.len() as i32 + 36 + blip_size;
        // Java: LittleEndian.putInt(data, offset, remainingBytes);
        le_put_int(&mut data, offset, remaining_bytes);
        // Java: offset += 4;
        offset += 4;
        // Java: data[offset++] = this.field_1_blipTypeWin32;
        data[offset] = self.field_1_blipTypeWin32 as u8;
        offset += 1;
        // Java: data[offset++] = this.field_2_blipTypeMacOS;
        data[offset] = self.field_2_blipTypeMacOS as u8;
        offset += 1;
        // Java: for(int i = 0; i < 16; ++i)
        for i in 0..16 {
            // Java: data[offset] = this.field_3_uid[i];
            data[offset] = self.field_3_uid[i];
            // Java: ++offset;
            offset += 1;
        }
        // Java: LittleEndian.putShort(data, offset, this.field_4_tag);
        le_put_short(&mut data, offset, self.field_4_tag);
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putInt(data, offset, this.field_5_size);
        le_put_int(&mut data, offset, self.field_5_size);
        // Java: offset += 4;
        offset += 4;
        // Java: LittleEndian.putInt(data, offset, this.field_6_ref);
        le_put_int(&mut data, offset, self.field_6_ref);
        // Java: offset += 4;
        offset += 4;
        // Java: if (this.delayStream != null && this.field_12_blipRecord != null)
        if self.delayStream.is_some() && self.field_12_blipRecord.is_some() {
            // Java: this.field_7_offset = this.delayStream.getOffset();
            self.field_7_offset = self.delayStream.as_ref().unwrap().getOffset();
        }
        // Java: LittleEndian.putInt(data, offset, this.field_7_offset);
        le_put_int(&mut data, offset, self.field_7_offset);
        // Java: offset += 4;
        offset += 4;
        // Java: data[offset++] = this.field_8_usage;
        data[offset] = self.field_8_usage as u8;
        offset += 1;
        // Java: data[offset++] = this.field_9_name;
        data[offset] = self.field_9_name as u8;
        offset += 1;
        // Java: data[offset++] = this.field_10_unused2;
        data[offset] = self.field_10_unused2 as u8;
        offset += 1;
        // Java: data[offset] = this.field_11_unused3;
        data[offset] = self.field_11_unused3 as u8;
        // Java: stream.write(data);
        let _ = stream.write(&data);
        // Java: int bytesWritten = 0;
        let mut bytes_written = 0i32;
        // Java: if (this.field_12_blipRecord != null)
        if self.field_12_blipRecord.is_some() {
            // Java: if (this.delayStream != null)
            if self.delayStream.is_some() {
                // Java: this.field_12_blipRecord.serialize((HWPFOutputStream)this.delayStream);
                let delay = self.delayStream.as_mut().unwrap();
                bytes_written = self.field_12_blipRecord.as_mut().unwrap().serialize_to_output(delay);
            }
            // Java: else
            else {
                // Java: bytesWritten = this.field_12_blipRecord.serialize(stream);
                bytes_written = self.field_12_blipRecord.as_mut().unwrap().serialize_to_output(stream);
            }
        }
        // Java: stream.write(this._remainingData);
        let _ = stream.write(&self._remainingData);
        // Java: return 44 + this._remainingData.length + bytesWritten;
        44 + self._remainingData.len() as i32 + bytes_written
    }

    // Java: int serialize(int offset, byte[] data, EscherSerializationListener listener)
    pub fn serialize_with_listener(&mut self, _offset: i32, _data: &mut [u8], _listener: &mut dyn EscherSerializationListener) -> i32 {
        // Java: throw new RuntimeException("This should never be called");
        panic!("This should never be called");
    }

    // Java: void setDelayStream(POIOutputStream out)
    pub fn setDelayStream(&mut self, out: POIOutputStream) {
        // Java: this.delayStream = out;
        self.delayStream = Some(out);
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: int field_12_size = 0;
        let mut field_12_size = 0;
        // Java: if (this.field_12_blipRecord != null && this.field_7_offset != -1 && this.delayStream == null)
        if self.field_12_blipRecord.is_some() && self.field_7_offset != -1 && self.delayStream.is_none() {
            // Java: field_12_size = this.field_12_blipRecord.getRecordSize();
            field_12_size = self.field_12_blipRecord.as_ref().unwrap().getRecordSize();
        }
        // Java: int remaining_size = 0;
        let mut remaining_size = 0;
        // Java: if (this._remainingData != null)
        // Java: remaining_size = this._remainingData.length;
        remaining_size = self._remainingData.len() as i32;
        // Java: return 44 + field_12_size + remaining_size;
        44 + field_12_size + remaining_size
    }

    // Java: FileDataStorage getPictureData()
    pub fn getPictureData(&self) -> Option<&Vec<u8>> {
        // Java: return this.field_12_blipRecord.getPictureData();
        self.field_12_blipRecord.as_ref().and_then(|b| b.getPictureData())
    }

    // Java: String getPictureName()
    pub fn getPictureName(&self) -> Option<String> {
        // Java: return null == this.field_12_blipRecord ? null : this.field_12_blipRecord.getPictureName() + "." + this.extension;
        match &self.field_12_blipRecord {
            None => None,
            Some(blip) => {
                let name = blip.getPictureName().cloned().unwrap_or_default();
                let ext = self.extension.as_deref().unwrap_or("");
                Some(format!("{}.{}", name, ext))
            }
        }
    }

    // Java: String getExtension()
    pub fn getExtension(&self) -> Option<&String> {
        // Java: return this.extension;
        self.extension.as_ref()
    }

    // Java: String getRecordName()
    pub fn getRecordName(&self) -> String {
        // Java: return "BSE";
        String::from("BSE")
    }

    // Java: byte getBlipTypeWin32()
    pub fn getBlipTypeWin32(&self) -> i8 {
        // Java: return this.field_1_blipTypeWin32;
        self.field_1_blipTypeWin32
    }

    // Java: void setBlipTypeWin32(byte blipTypeWin32)
    pub fn setBlipTypeWin32(&mut self, blip_type_win32: i8) {
        // Java: this.field_1_blipTypeWin32 = blipTypeWin32;
        self.field_1_blipTypeWin32 = blip_type_win32;
    }

    // Java: byte getBlipTypeMacOS()
    pub fn getBlipTypeMacOS(&self) -> i8 {
        // Java: return this.field_2_blipTypeMacOS;
        self.field_2_blipTypeMacOS
    }

    // Java: void setBlipTypeMacOS(byte blipTypeMacOS)
    pub fn setBlipTypeMacOS(&mut self, blip_type_mac_os: i8) {
        // Java: this.field_2_blipTypeMacOS = blipTypeMacOS;
        self.field_2_blipTypeMacOS = blip_type_mac_os;
    }

    // Java: byte[] getUid()
    pub fn getUid(&self) -> &Vec<u8> {
        // Java: return this.field_3_uid;
        &self.field_3_uid
    }

    // Java: void setUid(byte[] uid)
    pub fn setUid(&mut self, uid: Vec<u8>) {
        // Java: this.field_3_uid = uid;
        self.field_3_uid = uid;
    }

    // Java: short getTag()
    pub fn getTag(&self) -> i16 {
        // Java: return this.field_4_tag;
        self.field_4_tag
    }

    // Java: void setTag(short tag)
    pub fn setTag(&mut self, tag: i16) {
        // Java: this.field_4_tag = tag;
        self.field_4_tag = tag;
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return this.field_5_size;
        self.field_5_size
    }

    // Java: void setSize(int size)
    pub fn setSize(&mut self, size: i32) {
        // Java: this.field_5_size = size;
        self.field_5_size = size;
    }

    // Java: int getRef()
    pub fn getRef(&self) -> i32 {
        // Java: return this.field_6_ref;
        self.field_6_ref
    }

    // Java: void setRef(int ref)
    pub fn setRef(&mut self, r: i32) {
        // Java: this.field_6_ref = ref;
        self.field_6_ref = r;
    }

    // Java: int getOffset()
    pub fn getOffset(&self) -> i32 {
        // Java: return this.field_7_offset;
        self.field_7_offset
    }

    // Java: void setOffset(int offset)
    pub fn setOffset(&mut self, offset: i32) {
        // Java: this.field_7_offset = offset;
        self.field_7_offset = offset;
    }

    // Java: byte getUsage()
    pub fn getUsage(&self) -> i8 {
        // Java: return this.field_8_usage;
        self.field_8_usage
    }

    // Java: void setUsage(byte usage)
    pub fn setUsage(&mut self, usage: i8) {
        // Java: this.field_8_usage = usage;
        self.field_8_usage = usage;
    }

    // Java: byte getName()
    pub fn getName(&self) -> i8 {
        // Java: return this.field_9_name;
        self.field_9_name
    }

    // Java: void setName(byte name)
    pub fn setName(&mut self, name: i8) {
        // Java: this.field_9_name = name;
        self.field_9_name = name;
    }

    // Java: byte getUnused2()
    pub fn getUnused2(&self) -> i8 {
        // Java: return this.field_10_unused2;
        self.field_10_unused2
    }

    // Java: void setUnused2(byte unused2)
    pub fn setUnused2(&mut self, unused2: i8) {
        // Java: this.field_10_unused2 = unused2;
        self.field_10_unused2 = unused2;
    }

    // Java: byte getUnused3()
    pub fn getUnused3(&self) -> i8 {
        // Java: return this.field_11_unused3;
        self.field_11_unused3
    }

    // Java: void setUnused3(byte unused3)
    pub fn setUnused3(&mut self, unused3: i8) {
        // Java: this.field_11_unused3 = unused3;
        self.field_11_unused3 = unused3;
    }

    // Java: EscherBlipRecord getBlipRecord()
    pub fn getBlipRecord(&self) -> Option<&EscherBlipRecord> {
        // Java: return this.field_12_blipRecord;
        self.field_12_blipRecord.as_ref()
    }

    // Java: void setBlipRecord(EscherBlipRecord blipRecord)
    pub fn setBlipRecord(&mut self, blip_record: EscherBlipRecord) {
        // Java: this.field_12_blipRecord = blipRecord;
        self.field_12_blipRecord = Some(blip_record);
    }

    // Java: byte[] getRemainingData()
    pub fn getRemainingData(&self) -> &Vec<u8> {
        // Java: return this._remainingData;
        &self._remainingData
    }

    // Java: void setRemainingData(byte[] remainingData)
    pub fn setRemainingData(&mut self, remaining_data: Vec<u8>) {
        // Java: this._remainingData = remainingData;
        self._remainingData = remaining_data;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String extraData = this._remainingData == null ? null : HexDump.toHex(this._remainingData, 32);
        let extra_data = format!("{} bytes", self._remainingData.len());
        // Java: return this.getClass().getName() + ":" + '\n' + "  RecordId: 0x" + HexDump.toHex((short)-4089) + '\n' + "  Options: 0x" + HexDump.toHex(this.getOptions()) + '\n' + "  BlipTypeWin32: " + this.field_1_blipTypeWin32 + '\n' + "  BlipTypeMacOS: " + this.field_2_blipTypeMacOS + '\n' + "  SUID: " + (this.field_3_uid == null ? "" : HexDump.toHex(this.field_3_uid)) + '\n' + "  Tag: " + this.field_4_tag + '\n' + "  Size: " + this.field_5_size + '\n' + "  Ref: " + this.field_6_ref + '\n' + "  Offset: " + this.field_7_offset + '\n' + "  Usage: " + this.field_8_usage + '\n' + "  Name: " + this.field_9_name + '\n' + "  Unused2: " + this.field_10_unused2 + '\n' + "  Unused3: " + this.field_11_unused3 + '\n' + "  blipRecord: " + this.field_12_blipRecord + '\n' + "  Extra Data:" + '\n' + extraData;
        format!("EscherBSERecord:\n  RecordId: 0x{:04X}\n  Options: 0x{:04X}\n  BlipTypeWin32: {}\n  BlipTypeMacOS: {}\n  Tag: {}\n  Size: {}\n  Ref: {}\n  Offset: {}\n  Usage: {}\n  Name: {}\n  Unused2: {}\n  Unused3: {}\n  Extra Data:\n{}",
            RECORD_ID, self.base.getOptions(),
            self.field_1_blipTypeWin32, self.field_2_blipTypeMacOS,
            self.field_4_tag, self.field_5_size, self.field_6_ref, self.field_7_offset,
            self.field_8_usage, self.field_9_name, self.field_10_unused2, self.field_11_unused3,
            extra_data)
    }

    // Java: static String getBlipType(byte b)
    pub fn getBlipType(b: i8) -> Option<String> {
        // Java: switch (b)
        match b {
            // Java: case 0:
            // Java: return "jpg";
            0 => Some(String::from("jpg")),
            // Java: case 1:
            // Java: return "jpg";
            1 => Some(String::from("jpg")),
            // Java: case 2:
            // Java: return "emf";
            2 => Some(String::from("emf")),
            // Java: case 3:
            // Java: return "wmf";
            3 => Some(String::from("wmf")),
            // Java: case 4:
            // Java: return "pict";
            4 => Some(String::from("pict")),
            // Java: case 5:
            // Java: return "jpeg";
            5 => Some(String::from("jpeg")),
            // Java: case 6:
            // Java: return "png";
            6 => Some(String::from("png")),
            // Java: case 7:
            // Java: return "dib";
            7 => Some(String::from("dib")),
            // Java: case 8:
            // Java: case 9:
            // Java: case 10:
            // Java: case 11:
            // Java: case 12:
            // Java: case 13:
            // Java: case 14:
            // Java: case 15:
            // Java: case 16:
            // Java: default:
            // Java: return null;
            // Java: case 17:
            // Java: return "tiff";
            17 => Some(String::from("tiff")),
            _ => None,
        }
    }
}
