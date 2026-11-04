#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java: import java.io.IOException
// Java: import java.io.OutputStream
use std::io::{self, Write};
// Java: import org.apache.poi.util.LittleEndian

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 4].copy_from_slice(&bytes);
}

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    let bytes = value.to_le_bytes();
    data[offset..offset + 2].copy_from_slice(&bytes);
}

// Java field: int BITMAPINFOHEADER_SIZE
pub(crate) const BITMAPINFOHEADER_SIZE: i32 = 40;
// Java field: int RGBQUAD_SIZE
pub(crate) const RGBQUAD_SIZE: i32 = 4;
// Java field: int BIBITCOUNT_OFFSET
pub(crate) const BIBITCOUNT_OFFSET: i32 = 14;
// Java field: int BICLRUSED_OFFSET
pub(crate) const BICLRUSED_OFFSET: i32 = 32;

// Java: public class BitmapFileHeader
pub struct BitmapFileHeader {
    // Java field: byte[] bfType
    bfType: Option<Vec<u8>>,
    // Java field: int bfSize
    bfSize: i32,
    // Java field: short bfReserved1
    bfReserved1: i16,
    // Java field: short bfReserved2
    bfReserved2: i16,
    // Java field: int bfOffBits
    bfOffBits: i32,
}

// Java: public class BitmapFileHeader
impl BitmapFileHeader {
    // Java: BitmapFileHeader(byte[] content)
    pub fn new(content: &[u8]) -> Self {
        // Java: this(content, 0, content.length);
        Self::new_with_offset(content, 0, content.len() as i32)
    }

    // Java: BitmapFileHeader(byte[] content, int offset, int len)
    pub fn new_with_offset(content: &[u8], offset: i32, len: i32) -> Self {
        // Java: super();
        // Java: this.bfType = new byte[]{66, 77};
        let bfType = Some(vec![66u8, 77u8]);
        // Java: this.bfSize = len + this.getSize();
        let bfSize = len + 14;
        // Java: this.bfReserved1 = 0;
        let bfReserved1: i16 = 0;
        // Java: this.bfReserved2 = 0;
        let bfReserved2: i16 = 0;
        // Java: this.bfOffBits = this.getSize() + 40;
        let mut bfOffBits = 14 + 40;
        // Java: this.bfOffBits += 4 * this.dibNumColors(content, offset, len);
        bfOffBits += 4 * Self::dibNumColors_static(content, offset, len);
        BitmapFileHeader {
            bfType,
            bfSize,
            bfReserved1,
            bfReserved2,
            bfOffBits,
        }
    }

    // Java: int dibNumColors(byte[] content, int offset, int len)
    fn dibNumColors_static(content: &[u8], offset: i32, len: i32) -> i32 {
        // Java: int biClrUsed = content[32 + offset];
        let biClrUsed = content[(32 + offset) as usize] as i32;
        // Java: if (biClrUsed != 0)
        if biClrUsed != 0 {
            // Java: return biClrUsed;
            return biClrUsed;
        }
        // Java: else
        else {
            // Java: int biBitCount = content[14 + offset];
            let biBitCount = content[(14 + offset) as usize] as i32;
            // Java: return biBitCount > 9 ? 0 : 1 << biBitCount & 511;
            return if biBitCount > 9 { 0 } else { (1 << biBitCount) & 511 };
        }
    }

    // Java: int dibNumColors(byte[] content, int offset, int len)
    pub fn dibNumColors(&self, content: &[u8], offset: i32, len: i32) -> i32 {
        Self::dibNumColors_static(content, offset, len)
    }

    // Java: byte[] serialize()
    pub fn serialize(&self) -> Vec<u8> {
        // Java: byte[] header = new byte[14];
        let mut header = vec![0u8; 14];
        // Java: int pos = 2;
        let mut pos: usize = 2;
        // Java: System.arraycopy(this.bfType, 0, header, 0, 2);
        if let Some(ref bf_type) = self.bfType {
            header[0..2].copy_from_slice(&bf_type[0..2]);
        }
        // Java: LittleEndian.putInt(header, pos, this.bfSize);
        le_put_int(&mut header, pos, self.bfSize);
        // Java: pos += 4;
        pos += 4;
        // Java: LittleEndian.putShort(header, pos, this.bfReserved1);
        le_put_short(&mut header, pos, self.bfReserved1);
        // Java: pos += 2;
        pos += 2;
        // Java: LittleEndian.putShort(header, pos, this.bfReserved2);
        le_put_short(&mut header, pos, self.bfReserved2);
        // Java: pos += 2;
        pos += 2;
        // Java: LittleEndian.putInt(header, pos, this.bfOffBits);
        le_put_int(&mut header, pos, self.bfOffBits);
        // Java: return header;
        header
    }

    // Java: void write(OutputStream out)
    pub fn write(&self, out: &mut dyn Write) -> io::Result<()> {
        // Java: byte[] header = this.serialize();
        let header = self.serialize();
        // Java: out.write(header);
        out.write_all(&header)?;
        Ok(())
    }

    // Java: int getSize()
    pub fn getSize(&self) -> i32 {
        // Java: return 14;
        14
    }
}
