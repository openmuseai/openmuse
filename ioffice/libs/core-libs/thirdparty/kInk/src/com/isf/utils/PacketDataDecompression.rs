// Java: import cn.wps.base.assertion.KSAssert
use super::BytesReader::BytesReader;
use super::Bitpacking::Bitpacking;
use super::Huffman::Huffman;

// Java field: int Mask_Algorithm
const Mask_Algorithm: i32 = 192;
// Java field: int Mask_Data
const Mask_Data: i32 = 63;
// Java field: int Algorithm_BitPacking
const Algorithm_BitPacking: i32 = 0;
// Java field: int Algorithm_DefaultCompression
const Algorithm_DefaultCompression: i32 = 128;
// Java field: int Algorithm_BestCompression
const Algorithm_BestCompression: i32 = 192;

// Java: public class PacketDataDecompression
pub struct PacketDataDecompression<'a> {
    // Java field: BytesReader mReader
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: long mSize
    mSize: i64,
    // Java field: int[] mDecompressedData
    mDecompressedData: &'a mut [i32],
}

// Java: public class PacketDataDecompression
impl<'a> PacketDataDecompression<'a> {
    // Java: PacketDataDecompression(BytesReader reader, int[] out, long size)
    pub fn PacketDataDecompression(reader: &mut BytesReader, out: &'a mut [i32], size: i64) -> Self {
        // Java: super();
        // Java: KSAssert.assertNotNull("reader should not be null!", reader);
        // Java: this.mReader = reader;
        // Java: this.mAlgorithm = this.mReader.getByte();
        let mAlgorithm = reader.getByte() as i32;
        // Java: this.mDecompressedData = out;
        // Java: this.mSize = size - 1L;
        let mSize = size - 1;
        Self {
            mAlgorithm,
            mSize,
            mDecompressedData: out,
        }
    }

    // Java: void inflate()
    pub fn inflate(&mut self, reader: &mut BytesReader) {
        // Java: int type = this.mAlgorithm & 192;
        let r#type = self.mAlgorithm & 192;
        // Java: int dataBits = this.mAlgorithm & 63;
        let dataBits = self.mAlgorithm & 63;
        // Java: switch (type)
        match r#type {
            // Java: case 0:
            0 => {
                // Java: Bitpacking.inflate(this.mReader, dataBits, this.mDecompressedData);
                Bitpacking::inflate(reader, dataBits, self.mDecompressedData);
                // Java: break;
            }
            // Java: case 128:
            128 => {
                // Java: Huffman.inflate(this.mReader, dataBits, this.mDecompressedData);
                Huffman::inflate(reader, dataBits, self.mDecompressedData);
                // Java: break;
            }
            // Java: case 192:
            192 => {
                // Java: this.inflate_BestCompression();
                self.inflate_BestCompression(reader);
                // Java: break;
            }
            // Java: default:
            _ => {
                // Java: KSAssert.fail("It should not reach here!");
            }
        }
    }

    // Java: void inflate_BestCompression()
    fn inflate_BestCompression(&mut self, _reader: &mut BytesReader) {
        // Java: KSAssert.assertNotNull("mReader should not be null!", this.mReader);
    }
}
