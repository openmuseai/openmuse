// Java: import org.apache.poi.util.LittleEndian

use super::MSCodec_Xor95::MSCodec_Xor95;

// Java: public class MSCodec_XorWord95 extends MSCodec_Xor95
pub struct MSCodec_XorWord95 {
    pub(crate) base: MSCodec_Xor95,
}

// Java: public class MSCodec_XorWord95 extends MSCodec_Xor95
impl MSCodec_XorWord95 {
    // Java: MSCodec_XorWord95()
    pub fn new() -> Self {
        // Java: super(7);
        MSCodec_XorWord95 {
            base: MSCodec_Xor95::new(7),
        }
    }

    // Java: byte[] decode(byte[] data, int nOffset, int nBytes)
    pub fn decode(&mut self, data: &mut [u8], nOffset: usize, nBytes: usize) -> Vec<u8> {
        // Java: int nPos = this.mnOffset;
        let mut nPos = self.base.mnOffset as usize;
        // Java: for(int index = nOffset; index < nBytes + nOffset && index < data.length; ++index)
        let mut index = nOffset;
        while index < nBytes + nOffset && index < data.len() {
            // Java: int cChar = data[index] & 255 ^ this.mKey[nPos] & 255;
            let cChar = (data[index] as i32 & 255) ^ (self.base.mKey[nPos] as i32 & 255);
            // Java: if (data[index] != 0 && cChar != 0)
            if data[index] != 0 && cChar != 0 {
                // Java: data[index] = (byte)(cChar & 255);
                data[index] = (cChar & 255) as u8;
            }
            // Java: if (nPos < 15)
            if nPos < 15 {
                // Java: ++nPos;
                nPos += 1;
            }
            // Java: else
            else {
                // Java: nPos = 0;
                nPos = 0;
            }
            index += 1;
        }
        // Java: this.skip(nBytes);
        self.base.skip(nBytes as i32);
        // Java: return LittleEndian.getByteArray(data, nOffset, nBytes);
        let end = std::cmp::min(nOffset + nBytes, data.len());
        data[nOffset..end].to_vec()
    }

    // Java: byte[] encode(byte[] data, int nOffset, int nBytes)
    pub fn encode(&mut self, data: &mut [u8], nOffset: usize, nBytes: usize) -> Vec<u8> {
        // Java: return this.decode(data, nOffset, nBytes);
        self.decode(data, nOffset, nBytes)
    }
}
