#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import cn.wps.moffice.kfs.File
// Java: import cn.wps.moffice.kfs.FileOutputStream
// Java: import cn.wps.moffice.util.FileDataStorage
// Java: import cn.wps.moffice.util.FileUtil
// Java: import cn.wps.poi.drawing.KPictureRegistry
// Java: import cn.wps.poi.drawing.PictureUtil
// Java: import java.io.IOException
// Java: import java.io.OutputStream
// Java: import java.util.UUID
// Java: import org.apache.poi.hwpf.usermodel.BitmapFileHeader
// Java: import org.apache.poi.poifs.common.IBlockDocumentInputStream
// Java: import org.apache.poi.poifs.filesystem.DocumentInputStream
// Java: import org.apache.poi.util.HexDump
// Java: import org.apache.poi.util.LittleEndian

use std::io::Write;
use std::sync::atomic::{AtomicU32, Ordering};

use super::EscherBlipRecord::EscherBlipRecord;
use super::EscherRecordFactory::EscherRecordFactory;
use crate::org::apache::poi::poifs::common::IBlockDocumentInputStream::IBlockDocumentInputStream;
use crate::org::apache::poi::poifs::filesystem::DocumentInputStream::DocumentInputStream;
use crate::org::apache::poi::hwpf::usermodel::BitmapFileHeader::BitmapFileHeader;

static UUID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn generate_uuid() -> String {
    let v = UUID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:08x}-{:04x}-{:04x}-{:04x}-{:012x}", v, v & 0xFFFF, (v >> 4) & 0xFFFF, (v >> 8) & 0xFFFF, v as u64)
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 2].copy_from_slice(&bytes);
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 4].copy_from_slice(&bytes);
}

fn hex_dump_short(v: i16) -> String {
    format!("{:04X}", v as u16)
}

fn hex_dump_byte(v: i8) -> String {
    format!("{:02X}", v as u8)
}

fn hex_dump_bytes(v: &[u8]) -> String {
    v.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join("")
}

// Java field: String TAG
pub(crate) const TAG: &str = "";
// Java field: short RECORD_ID_JPEG
pub(crate) const RECORD_ID_JPEG: i16 = -4067;
// Java field: short RECORD_ID_PNG
pub(crate) const RECORD_ID_PNG: i16 = -4066;
// Java field: short RECORD_ID_DIB
pub(crate) const RECORD_ID_DIB: i16 = -4065;
// Java field: short RECORD_ID_TIFF
pub(crate) const RECORD_ID_TIFF: i16 = -4055;
// Java field: int HEADER_SIZE
pub(crate) const HEADER_SIZE: i32 = 8;

// Java: public class EscherBitmapBlip extends EscherBlipRecord
pub struct EscherBitmapBlip {
    // Java field: byte field_2_marker
    pub(crate) field_2_marker: i8,
    // Java field: int pictureSize
    pictureSize: i32,
    pub base: EscherBlipRecord,
}

// Java: public class EscherBitmapBlip extends EscherBlipRecord
impl EscherBitmapBlip {
    // Java: EscherBitmapBlip()
    pub fn new() -> Self {
        // Java: super();
        EscherBitmapBlip {
            field_2_marker: -1,
            pictureSize: 0,
            base: EscherBlipRecord::new(),
        }
    }

    // Java: int fillFields(DocumentInputStream data, int offset, EscherRecordFactory recordFactory)
    pub fn fillFields_stream(&mut self, data: &mut DocumentInputStream, offset: i32, record_factory: &dyn EscherRecordFactory) -> i32 {
        // Java: int bytesAfterHeader = this.readHeader(data, offset);
        let bytesAfterHeader = self.base.base.readHeader_stream(data, offset);
        // Java: int pos = offset + 8;
        let mut pos = offset + 8;
        // Java: data.seek((long)pos);
        data.seek(pos as i64);
        // Java: this.field_1_UID = new byte[16];
        self.base.field_1_UID = vec![0u8; 16];
        // Java: data.read(this.field_1_UID, 0, 16);
        data.read_off(&mut self.base.field_1_UID, 0, 16);
        // Java: pos += 16;
        pos += 16;
        // Java: if (data.available() > 0)
        if data.available() > 0 {
            // Java: this.field_2_marker = data.readByte();
            self.field_2_marker = data.readByte();
        }
        // Java: ++pos;
        pos += 1;
        // Java: this.pictureSize = bytesAfterHeader - 17;
        self.pictureSize = bytesAfterHeader - 17;
        // Java: this.mark = pos;
        self.base.mark = pos;
        // Java: this.fileName = UUID.randomUUID().toString();
        self.base.fileName = Some(generate_uuid());
        // Java: return bytesAfterHeader + 8;
        bytesAfterHeader + 8
    }

    // Java: int fillFields(IBlockDocumentInputStream data, int offset, EscherRecordFactory recordFactory, String pictureDirectory, String extension)
    pub fn fillFields_block(&mut self, data: &mut dyn IBlockDocumentInputStream, offset: i32, record_factory: &dyn EscherRecordFactory, pictureDirectory: &str, extension: &str) -> i32 {
        // Java: int bytesAfterHeader = this.getRemainingBytes();
        let bytesAfterHeader = self.base.base.getRemainingBytes();
        // Java: int pos = offset + 8;
        let mut pos = offset + 8;
        // Java: this.field_1_UID = new byte[16];
        self.base.field_1_UID = vec![0u8; 16];
        // Java: data.read(this.field_1_UID);
        data.read(&mut self.base.field_1_UID);
        // Java: pos += 16;
        pos += 16;
        // Java: this.field_2_marker = data.readByte();
        self.field_2_marker = data.readByte();
        // Java: ++pos;
        pos += 1;
        // Java: this.pictureSize = bytesAfterHeader - 17;
        self.pictureSize = bytesAfterHeader - 17;
        // Java: String szName = this.fileName = UUID.randomUUID().toString() + "." + extension;
        let szName = format!("{}.{}", generate_uuid(), extension);
        self.base.fileName = Some(szName.clone());
        // Java: File directory = new File(pictureDirectory);
        let directory = std::path::Path::new(pictureDirectory);
        // Java: if (!directory.exists())
        if !directory.exists() {
            // Java: directory.mkdirs();
            let _ = std::fs::create_dir_all(directory);
        }
        // Java: File file = new File(pictureDirectory + szName);
        let file_path = format!("{}{}", pictureDirectory, szName);
        // Java: if (!file.exists())
        if !std::path::Path::new(&file_path).exists() {
            // Java: file.createNewFile();
            let _ = std::fs::File::create(&file_path);
        }
        // Java: if (extension.equals("dib"))
        if extension == "dib" {
            // Java: byte[] pData = new byte[this.pictureSize];
            let mut pData = vec![0u8; self.pictureSize as usize];
            // Java: data.read(pData);
            data.read(&mut pData);
            // Java: BitmapFileHeader bfHeader = new BitmapFileHeader(pData);
            let bfHeader = BitmapFileHeader::new(&pData);
            // Java: FileOutputStream writer = new FileOutputStream(file);
            if let Ok(mut writer) = std::fs::File::create(&file_path) {
                // Java: writer.write(bfHeader.serialize());
                let _ = writer.write_all(&bfHeader.serialize());
                // Java: writer.write(pData);
                let _ = writer.write_all(&pData);
                // Java: FileUtil.closeQuietly(writer);
            }
            // Java: this.mPictureData = new FileDataStorage(file);
            self.base.mPictureData = Some(pData);
        }
        // Java: else
        else {
            // Java: data.dump(file, this.pictureSize);
            let mut buf = vec![0u8; self.pictureSize as usize];
            data.read(&mut buf);
            // Java: this.mPictureData = new FileDataStorage(file);
            self.base.mPictureData = Some(buf);
        }
        // Java: this.mPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4", this.field_1_UID);
        // Java: KPictureRegistry.instance().register(this.mPictureData);
        // Java: return bytesAfterHeader + 8;
        bytesAfterHeader + 8
    }

    // Java: void readPicture(DocumentInputStream data, File picFile, String extension)
    pub fn readPicture(&mut self, data: &mut DocumentInputStream, pic_file: &str, extension: &str) {
        // Java: data.seek((long)this.mark);
        data.seek(self.base.mark as i64);
        // Java: if (!picFile.exists())
        if !std::path::Path::new(pic_file).exists() {
            // Java: picFile.createNewFile();
            let _ = std::fs::File::create(pic_file);
        }
        // Java: if (extension.equals("dib"))
        if extension == "dib" {
            // Java: byte[] pData = new byte[this.pictureSize];
            let mut pData = vec![0u8; self.pictureSize as usize];
            // Java: data.read(pData);
            data.readFully(&mut pData);
            // Java: BitmapFileHeader bfHeader = new BitmapFileHeader(pData);
            let bfHeader = BitmapFileHeader::new(&pData);
            // Java: FileOutputStream writer = new FileOutputStream(picFile);
            if let Ok(mut writer) = std::fs::File::create(pic_file) {
                // Java: writer.write(bfHeader.serialize());
                let _ = writer.write_all(&bfHeader.serialize());
                // Java: writer.write(pData);
                let _ = writer.write_all(&pData);
                // Java: FileUtil.closeQuietly(writer);
            }
            // Java: this.mPictureData = new FileDataStorage(picFile);
            self.base.mPictureData = Some(pData);
        }
        // Java: else
        else {
            // Java: this.mPictureData = FileDataStorage.createFileDataStorage(data, this.pictureSize, picFile);
            let mut buf = vec![0u8; self.pictureSize as usize];
            data.readFully(&mut buf);
            self.base.mPictureData = Some(buf);
        }
        // Java: this.mPictureData.setUserData("cn.wps.poi.drawing.KPictureRegistry.KEY_RAW_DATA_MD4", this.field_1_UID);
        // Java: KPictureRegistry.instance().register(this.mPictureData);
    }

    // Java: int serialize(OutputStream stream)
    pub fn serialize_to_stream(&mut self, stream: &mut dyn Write) -> i32 {
        // Java: int nRawPictureSize = this.getRawPictureSize();
        let nRawPictureSize = self.getRawPictureSize();
        // Java: this.calculateMD4();
        self.calculateMD4();
        // Java: int offset = 0;
        let mut offset: usize = 0;
        // Java: int recordSize = 25;
        let mut recordSize: i32 = 25;
        // Java: byte[] data = new byte[recordSize];
        let mut data = vec![0u8; 25];
        // Java: recordSize += nRawPictureSize;
        recordSize += nRawPictureSize;
        // Java: LittleEndian.putShort(data, offset, this.getOptions());
        le_put_short(&mut data, offset, self.base.base.getOptions());
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putShort(data, offset, this.getRecordId());
        le_put_short(&mut data, offset, self.base.base.getRecordId());
        // Java: offset += 2;
        offset += 2;
        // Java: LittleEndian.putInt(data, offset, recordSize - 8);
        le_put_int(&mut data, offset, recordSize - 8);
        // Java: offset += 4;
        offset += 4;
        // Java: System.arraycopy(this.field_1_UID, 0, data, offset, 16);
        let uid_len = std::cmp::min(self.base.field_1_UID.len(), 16);
        data[offset..offset + uid_len].copy_from_slice(&self.base.field_1_UID[..uid_len]);
        // Java: offset += 16;
        offset += 16;
        // Java: data[offset] = this.field_2_marker;
        data[offset] = self.field_2_marker as u8;
        // Java: stream.write(data);
        let _ = stream.write_all(&data);
        // Java: PictureUtil.serialize(this.mPictureData, stream);
        if let Some(ref pd) = self.base.mPictureData {
            let _ = stream.write_all(pd);
        }
        // Java: return recordSize;
        recordSize
    }

    // Java: int getRawPictureSize()
    pub fn getRawPictureSize(&self) -> i32 {
        // Java: int size = this.mPictureData.getSize();
        let mut size = self.base.mPictureData.as_ref().map(|p| p.len() as i32).unwrap_or(0);
        // Java: if (this.getRecordId() == -4065)
        if self.base.base.getRecordId() == -4065 {
            // Java: size -= 14;
            size -= 14;
        }
        // Java: return size;
        size
    }

    // Java: void calculateMD4()
    pub fn calculateMD4(&mut self) {
        // Java: if (this.field_1_UID == null)
        if self.base.field_1_UID.is_empty() {
            // Java: byte[] md4 = PictureUtil.getMD4(this.mPictureData);
            // Java: this.field_1_UID = md4;
            self.base.field_1_UID = vec![0u8; 16];
        }
    }

    // Java: byte[] getUID()
    pub fn getUID(&mut self) -> &Vec<u8> {
        // Java: if (this.field_1_UID == null)
        if self.base.field_1_UID.is_empty() {
            // Java: try
            {
                // Java: this.calculateMD4();
                self.base.field_1_UID = vec![0u8; 16];
            }
            // Java: catch (IOException var2)
            // Java: throw new RuntimeException("Can not calculate MD4");
        }
        // Java: return this.field_1_UID;
        &self.base.field_1_UID
    }

    // Java: int getRecordSize()
    pub fn getRecordSize(&self) -> i32 {
        // Java: return this.mPictureData != null ? 25 + this.getRawPictureSize() : 25 + this.pictureSize;
        if self.base.mPictureData.is_some() {
            25 + self.getRawPictureSize()
        } else {
            25 + self.pictureSize
        }
    }

    // Java: byte getMarker()
    pub fn getMarker(&self) -> i8 {
        // Java: return this.field_2_marker;
        self.field_2_marker
    }

    // Java: void setMarker(byte field_2_marker)
    pub fn setMarker(&mut self, field_2_marker: i8) {
        // Java: this.field_2_marker = field_2_marker;
        self.field_2_marker = field_2_marker;
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: String nl = System.getProperty("line.separator");
        let nl = "\n";
        // Java: String extraData = "";
        let extraData = "";
        // Java: return this.getClass().getName() + ":" + nl + "  RecordId: 0x" + HexDump.toHex(this.getRecordId()) + nl + "  Options: 0x" + HexDump.toHex(this.getOptions()) + nl + "  UID: 0x" + HexDump.toHex(this.field_1_UID) + nl + "  Marker: 0x" + HexDump.toHex(this.field_2_marker) + nl + "  Extra Data:" + nl + extraData;
        format!("EscherBitmapBlip:{}  RecordId: 0x{}{}  Options: 0x{}{}  UID: 0x{}{}  Marker: 0x{}{}  Extra Data:{}{}",
            nl, hex_dump_short(self.base.base.getRecordId()),
            nl, hex_dump_short(self.base.base.getOptions()),
            nl, hex_dump_bytes(&self.base.field_1_UID),
            nl, hex_dump_byte(self.field_2_marker),
            nl, nl, extraData)
    }
}
