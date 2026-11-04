#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

// Java: import cn.wps.base.IBufferReader
use crate::cn::wps::base::IBufferReader::IBufferReader;
// Java: import cn.wps.graphics.data.MetaFileHeader
use crate::cn::wps::graphics::data::MetaFileHeader::MetaFileHeader;
// Java: import cn.wps.graphics.gdi.Size
use crate::cn::wps::graphics::gdi::Size::Size;
// Java: import java.io.IOException
// Java: import java.io.InputStream

use std::io::{self, Read, Seek, SeekFrom};

// Java: public class WMFHeader implements MetaFileHeader
pub struct WMFHeader {
    // Java field: int fileType
    pub(crate) fileType: i32,
    // Java field: int headerSize
    headerSize: i32,
    // Java field: int version
    version: i32,
    // Java field: long fileSize
    fileSize: i64,
    // Java field: int numOfObjects
    pub(crate) numOfObjects: i32,
    // Java field: long maxRecordSize
    maxRecordSize: i64,
    // Java field: int numOfParams
    numOfParams: i32,
    // Java field: int dpi
    pub(crate) dpi: i32,
}

// Java: public class WMFHeader implements MetaFileHeader
impl WMFHeader {
    // Java: boolean isValid()
    pub fn isValid(&self) -> bool {
        // Java: return this.fileType == 1 && this.headerSize == 9 && this.numOfParams == 0;
        self.fileType == 1 && self.headerSize == 9 && self.numOfParams == 0
    }

    // Java: WMFHeader(IBufferReader wmf)
    pub fn new(wmf: &mut dyn IBufferReader) -> Self {
        // Java: super();
        let mut dpi = 96;
        // Java: int testKey = wmf.readUnsignedInt();
        let testKey = wmf.readUnsignedInt_one();
        // Java: if (testKey == -1698247209)
        if testKey == -1698247209i32 {
            // Java: byte[] head = new byte[18];
            let mut head = vec![0u8; 18];
            // Java: wmf.read(head);
            wmf.read_all(&mut head);
            // Java: if (checkSum(head))
            if WMFHeader::checkSum(&head) {
                // Java: this.dpi = (head[10] & 255) + ((head[11] & 255) << 8);
                dpi = (head[10] as i32 & 255) + ((head[11] as i32 & 255) << 8);
            }
        }
        // Java: else
        else {
            // Java: wmf.reset();
            wmf.reset();
        }
        // Java: this.fileType = wmf.readUnsignedShort();
        let fileType = wmf.readUnsignedShort();
        // Java: this.headerSize = wmf.readUnsignedShort();
        let headerSize = wmf.readUnsignedShort();
        // Java: this.version = wmf.readUnsignedShort();
        let version = wmf.readUnsignedShort();
        // Java: this.fileSize = (long)wmf.readUnsignedInt();
        let fileSize = wmf.readUnsignedInt_one() as i64;
        // Java: this.numOfObjects = wmf.readUnsignedShort();
        let numOfObjects = wmf.readUnsignedShort();
        // Java: this.maxRecordSize = (long)wmf.readUnsignedInt();
        let maxRecordSize = wmf.readUnsignedInt_one() as i64;
        // Java: this.numOfParams = wmf.readUnsignedShort();
        let numOfParams = wmf.readUnsignedShort();
        WMFHeader { fileType, headerSize, version, fileSize, numOfObjects, maxRecordSize, numOfParams, dpi }
    }

    // Java: static void skipHeader(InputStream is)
    pub fn skipHeader(is: &mut (impl Read + Seek)) -> io::Result<()> {
        // Java: byte[] tempBytes = new byte[4];
        let mut tempBytes = [0u8; 4];
        // Java: is.read(tempBytes);
        is.read_exact(&mut tempBytes)?;
        // Java: int testKey = (tempBytes[0] & 255) + ((tempBytes[1] & 255) << 8) + ((tempBytes[2] & 255) << 16) + ((tempBytes[3] & 255) << 24);
        let testKey = (tempBytes[0] as i32 & 255) + ((tempBytes[1] as i32 & 255) << 8) + ((tempBytes[2] as i32 & 255) << 16) + ((tempBytes[3] as i32 & 255) << 24);
        // Java: if (testKey == -1698247209)
        if testKey == -1698247209i32 {
            // Java: is.skip(36L);
            is.seek(SeekFrom::Current(36))?;
        }
        // Java: else
        else {
            // Java: is.skip(14L);
            is.seek(SeekFrom::Current(14))?;
        }
        Ok(())
    }

    // Java: static boolean checkSum(byte[] head)
    pub fn checkSum(head: &[u8]) -> bool {
        // Java: int checkSum = (head[16] & 255) + ((head[17] & 255) << 8);
        let checkSum = (head[16] as i32 & 255) + ((head[17] as i32 & 255) << 8);
        // Java: int checkResult = 22289;
        let mut checkResult = 22289i32;
        // Java: for(int i = 0; i < 16; i += 2)
        let mut i = 0;
        while i < 16 {
            // Java: checkResult ^= (head[i] & 255) + ((head[i + 1] & 255) << 8);
            checkResult ^= (head[i] as i32 & 255) + ((head[i + 1] as i32 & 255) << 8);
            i += 2;
        }
        // Java: return checkResult == checkSum;
        checkResult == checkSum
    }

    // Java: static Size getSize(InputStream is)
    pub fn getSize(is: &mut (impl Read + Seek)) -> io::Result<Option<Size>> {
        // Java: byte[] tempBytes = new byte[4];
        let mut tempBytes = [0u8; 4];
        // Java: is.read(tempBytes);
        is.read_exact(&mut tempBytes)?;
        // Java: int testKey = (tempBytes[0] & 255) + ((tempBytes[1] & 255) << 8) + ((tempBytes[2] & 255) << 16) + ((tempBytes[3] & 255) << 24);
        let testKey = (tempBytes[0] as i32 & 255) + ((tempBytes[1] as i32 & 255) << 8) + ((tempBytes[2] as i32 & 255) << 16) + ((tempBytes[3] as i32 & 255) << 24);
        // Java: if (testKey == -1698247209)
        if testKey == -1698247209i32 {
            // Java: byte[] head = new byte[18];
            let mut head = [0u8; 18];
            // Java: is.read(head, 0, 18);
            is.read_exact(&mut head)?;
            // Java: is.skip(18L);
            is.seek(SeekFrom::Current(18))?;
            // Java: if (checkSum(head))
            if WMFHeader::checkSum(&head) {
                // Java: int dpi = (head[10] & 255) + ((head[11] & 255) << 8);
                let dpi = (head[10] as i32 & 255) + ((head[11] as i32 & 255) << 8);
                // Java: if (dpi > 0)
                if dpi > 0 {
                    // Java: int width = (short)((head[6] & 255) + ((head[7] & 255) << 8)) - (short)((head[2] & 255) + ((head[3] & 255) << 8));
                    let width = ((head[6] as i32 & 255) + ((head[7] as i32 & 255) << 8)) as i16 as i32
                        - ((head[2] as i32 & 255) + ((head[3] as i32 & 255) << 8)) as i16 as i32;
                    // Java: int height = (short)((head[8] & 255) + ((head[9] & 255) << 8)) - (short)((head[4] & 255) + ((head[5] & 255) << 8));
                    let height = ((head[8] as i32 & 255) + ((head[9] as i32 & 255) << 8)) as i16 as i32
                        - ((head[4] as i32 & 255) + ((head[5] as i32 & 255) << 8)) as i16 as i32;
                    // Java: float scale = 96.0F / (float)dpi;
                    let scale = 96.0f32 / dpi as f32;
                    // Java: if ((float)width > scale && (float)height > scale)
                    if width as f32 > scale && height as f32 > scale {
                        // Java: return new Size((float)width * scale, (float)height * scale);
                        return Ok(Some(Size::new(width as f32 * scale, height as f32 * scale)));
                    }
                }
            }
        }
        // Java: else
        else {
            // Java: is.skip(14L);
            is.seek(SeekFrom::Current(14))?;
        }
        // Java: return null;
        Ok(None)
    }

    // Java: int getHandles()
    pub fn getHandles(&self) -> i32 {
        // Java: return this.numOfObjects;
        self.numOfObjects
    }
}

impl MetaFileHeader for WMFHeader {
    fn isValid(&self) -> bool { self.isValid() }
    fn getHandles(&self) -> i32 { self.getHandles() }
}
