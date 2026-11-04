// Java: import cn.wps.crypt.CryptHelper
use crate::cn::wps::crypt::CryptHelper::CryptHelper;
// Java: import org.apache.poi.util.LittleEndian

use super::MSCodec_Xor95::MSCodec_Xor95;

// Java: public class MSCodec_XorXLS95 extends MSCodec_Xor95
pub struct MSCodec_XorXLS95 {
    pub(crate) base: MSCodec_Xor95,
}

// Java: public class MSCodec_XorXLS95 extends MSCodec_Xor95
impl MSCodec_XorXLS95 {
    // Java: MSCodec_XorXLS95()
    pub fn new() -> Self {
        // Java: super(2);
        MSCodec_XorXLS95 {
            base: MSCodec_Xor95::new(2),
        }
    }

    // Java: byte[] decode(byte[] data, int nOffset, int nBytes)
    pub fn decode(&mut self, data: &mut [u8], nOffset: usize, nBytes: usize) -> Vec<u8> {
        // Java: int nPos = this.mnOffset;
        let mut nPos = self.base.mnOffset as usize;
        // Java: for(int index = nOffset; index < nBytes + nOffset && index < data.length; ++index)
        let mut index = nOffset;
        while index < nBytes + nOffset && index < data.len() {
            // Java: byte b = CryptHelper.lclRotateLeft((byte)data[index], (byte)3);
            let b = CryptHelper::lclRotateLeft(data[index], 3);
            // Java: int cChar = b ^ this.mKey[nPos] & 255;
            let cChar = (b as i32) ^ (self.base.mKey[nPos] as i32 & 255);
            // Java: data[index] = (byte)(cChar & 255);
            data[index] = (cChar & 255) as u8;
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
        // Java: return nOffset == 0 ? data : LittleEndian.getByteArray(data, nOffset, nBytes);
        if nOffset == 0 {
            data.to_vec()
        } else {
            let end = std::cmp::min(nOffset + nBytes, data.len());
            data[nOffset..end].to_vec()
        }
    }

    // Java: byte[] encode(byte[] data, int nOffset, int nBytes)
    pub fn encode(&mut self, data: &mut [u8], nOffset: usize, nBytes: usize) -> Vec<u8> {
        // Java: int nPos = this.mnOffset;
        let mut nPos = self.base.mnOffset as usize;
        // Java: for(int index = nOffset; index < nBytes + nOffset && index < data.length; ++index)
        let mut index = nOffset;
        while index < nBytes + nOffset && index < data.len() {
            // Java: byte cChar = (byte)(data[index] ^ this.mKey[nPos] & 255);
            let cChar = (data[index] as i32 ^ (self.base.mKey[nPos] as i32 & 255)) as u8;
            // Java: byte b = CryptHelper.lclRotateRight(cChar, (byte)3);
            let b = CryptHelper::lclRotateRight(cChar, 3);
            // Java: data[index] = (byte)(b & 255);
            data[index] = b & 255;
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
        // Java: return nOffset == 0 ? data : LittleEndian.getByteArray(data, nOffset, nBytes);
        if nOffset == 0 {
            data.to_vec()
        } else {
            let end = std::cmp::min(nOffset + nBytes, data.len());
            data[nOffset..end].to_vec()
        }
    }
}
