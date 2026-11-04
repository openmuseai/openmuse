// Java: import java.util.Arrays
// Java: import org.apache.poi.util.LittleEndian
use super::IDigest::IDigest;
use super::DigestError;

// Java field: int DIGEST_LENGTH_MD5
pub const DIGEST_LENGTH_MD5: i32 = 16;
// Java field: int DIGEST_CBLOCK_MD5
const DIGEST_CBLOCK_MD5: usize = 64;
// Java field: int DIGEST_LBLOCK_MD5
const DIGEST_LBLOCK_MD5: usize = 16;

fn le_get_int(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_le_bytes();
    data[offset] = bytes[0];
    data[offset + 1] = bytes[1];
    data[offset + 2] = bytes[2];
    data[offset + 3] = bytes[3];
}

// Java: public class DigestMD5 implements IDigest
pub struct DigestMD5 {
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: int mLength
    mLength: i32,
    // Java field: int mDataLen
    mDataLen: i32,
    // Java field: byte[] mData
    mData: Vec<u8>,
    // Java field: int mA
    mA: i32,
    // Java field: int mB
    mB: i32,
    // Java field: int mC
    mC: i32,
    // Java field: int mD
    mD: i32,
    // Java field: int mL
    mL: i32,
    // Java field: int mH
    mH: i32,
}

// Java: public class DigestMD5 implements IDigest
impl DigestMD5 {
    // Java: DigestMD5()
    pub fn new() -> Self {
        // Java: super();
        let mut s = DigestMD5 {
            mAlgorithm: 0,
            mLength: 0,
            mDataLen: 0,
            mData: vec![0u8; 64],
            mA: 0,
            mB: 0,
            mC: 0,
            mD: 0,
            mL: 0,
            mH: 0,
        };
        // Java: this.init();
        s.init_state();
        s
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: this.init();
        self.init_state();
    }

    // Java: int get(byte[] buffer)
    pub fn get_digest(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: if (buffer == null)
        if buffer.is_empty() {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 1)
        else if self.mAlgorithm != 1 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else if (this.mLength > buffer.length)
        else if self.mLength > buffer.len() as i32 {
            // Java: return DigestError.Digest_E_BufferSize;
            return DigestError::DIGEST_E_BUFFER_SIZE;
        }
        // Java: else
        else {
            // Java: this.end();
            self.end();
            // Java: LittleEndian.putInt(buffer, 0, this.mA);
            le_put_int(buffer, 0, self.mA);
            // Java: LittleEndian.putInt(buffer, 4, this.mB);
            le_put_int(buffer, 4, self.mB);
            // Java: LittleEndian.putInt(buffer, 8, this.mC);
            le_put_int(buffer, 8, self.mC);
            // Java: LittleEndian.putInt(buffer, 12, this.mD);
            le_put_int(buffer, 12, self.mD);
            // Java: this.init();
            self.init_state();
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int init(byte[] data, int offset, int nLen)
    pub fn init_with_data(&mut self, _data: &[u8], _offset: i32, _nLen: i32) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int raw(byte[] buffer)
    pub fn raw_impl(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: if (buffer == null)
        if buffer.is_empty() {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 1)
        else if self.mAlgorithm != 1 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else if (this.mLength > buffer.length)
        else if self.mLength > buffer.len() as i32 {
            // Java: return DigestError.Digest_E_BufferSize;
            return DigestError::DIGEST_E_BUFFER_SIZE;
        }
        // Java: else
        else {
            // Java: LittleEndian.putInt(buffer, 0, this.mA);
            le_put_int(buffer, 0, self.mA);
            // Java: LittleEndian.putInt(buffer, 4, this.mB);
            le_put_int(buffer, 4, self.mB);
            // Java: LittleEndian.putInt(buffer, 8, this.mC);
            le_put_int(buffer, 8, self.mC);
            // Java: LittleEndian.putInt(buffer, 12, this.mD);
            le_put_int(buffer, 12, self.mD);
            // Java: this.init();
            self.init_state();
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int update(byte[] data, int offset, int nLen)
    pub fn update_data(&mut self, data: &[u8], mut offset: i32, mut nLen: i32) -> i32 {
        // Java: if (data == null)
        if data.is_empty() && nLen > 0 {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 1)
        else if self.mAlgorithm != 1 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else if (nLen == 0)
        else if nLen == 0 {
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
        // Java: else
        else {
            // Java: int len = this.mL + (nLen << 3);
            let len = self.mL.wrapping_add(nLen << 3);
            // Java: if (len < this.mL)
            if len < self.mL {
                // Java: ++this.mH;
                self.mH += 1;
            }
            // Java: this.mH += nLen >> 29;
            self.mH += nLen >> 29;
            // Java: this.mL = len;
            self.mL = len;
            // Java: if (this.mDataLen > 0)
            if self.mDataLen > 0 {
                // Java: int n = 64 - this.mDataLen;
                let n = 64 - self.mDataLen;
                // Java: if (nLen < n)
                if nLen < n {
                    // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, nLen);
                    self.mData[self.mDataLen as usize..(self.mDataLen + nLen) as usize]
                        .copy_from_slice(&data[offset as usize..(offset + nLen) as usize]);
                    // Java: this.mDataLen += nLen;
                    self.mDataLen += nLen;
                    // Java: return DigestError.Digest_E_None;
                    return DigestError::DIGEST_E_NONE;
                }
                // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, n);
                self.mData[self.mDataLen as usize..(self.mDataLen + n) as usize]
                    .copy_from_slice(&data[offset as usize..(offset + n) as usize]);
                // Java: offset += n;
                offset += n;
                // Java: nLen -= n;
                nLen -= n;
                // Java: this.update();
                self.update_block();
                // Java: this.mDataLen = 0;
                self.mDataLen = 0;
            }
            // Java: while(nLen >= 64)
            while nLen >= 64 {
                // Java: System.arraycopy(data, offset, this.mData, 0, 64);
                self.mData[..64].copy_from_slice(&data[offset as usize..(offset + 64) as usize]);
                // Java: offset += 64;
                offset += 64;
                // Java: nLen -= 64;
                nLen -= 64;
                // Java: this.update();
                self.update_block();
            }
            // Java: System.arraycopy(data, offset, this.mData, this.mDataLen, nLen);
            if nLen > 0 {
                self.mData[self.mDataLen as usize..(self.mDataLen + nLen) as usize]
                    .copy_from_slice(&data[offset as usize..(offset + nLen) as usize]);
            }
            // Java: this.mDataLen = nLen;
            self.mDataLen = nLen;
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int get(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn get_with_params(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: this.init();
        self.init_state();
        // Java: int result = this.update(data, offset, nLen);
        let mut result = self.update_data(data, offset, nLen);
        // Java: if (result == DigestError.Digest_E_None)
        if result == DigestError::DIGEST_E_NONE {
            // Java: result = this.get(buffer);
            result = self.get_digest(buffer);
        }
        // Java: return result;
        result
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return 16;
        16
    }

    // Java: void init()
    fn init_state(&mut self) {
        // Java: this.mAlgorithm = 1;
        self.mAlgorithm = 1;
        // Java: this.mLength = 16;
        self.mLength = 16;
        // Java: this.mDataLen = 0;
        self.mDataLen = 0;
        // Java: this.mA = 1732584193;
        self.mA = 1732584193;
        // Java: this.mB = -271733879;
        self.mB = -271733879_i32;
        // Java: this.mC = -1732584194;
        self.mC = -1732584194_i32;
        // Java: this.mD = 271733878;
        self.mD = 271733878;
        // Java: this.mL = 0;
        self.mL = 0;
        // Java: this.mH = 0;
        self.mH = 0;
        // Java: Arrays.fill(this.mData, (byte)0);
        self.mData.fill(0);
    }

    // Java: void update()
    fn update_block(&mut self) {
        // Java: int A = this.mA;
        let mut a = self.mA;
        // Java: int B = this.mB;
        let mut b = self.mB;
        // Java: int C = this.mC;
        let mut c = self.mC;
        // Java: int D = this.mD;
        let mut d = self.mD;
        // Java: A = this.R0(A, B, C, D, LittleEndian.getInt(this.mData, 0), 7, -680876936);
        a = self.r0(a, b, c, d, le_get_int(&self.mData, 0), 7, -680876936_i32);
        // Java: D = this.R0(D, A, B, C, LittleEndian.getInt(this.mData, 4), 12, -389564586);
        d = self.r0(d, a, b, c, le_get_int(&self.mData, 4), 12, -389564586_i32);
        // Java: C = this.R0(C, D, A, B, LittleEndian.getInt(this.mData, 8), 17, 606105819);
        c = self.r0(c, d, a, b, le_get_int(&self.mData, 8), 17, 606105819);
        // Java: B = this.R0(B, C, D, A, LittleEndian.getInt(this.mData, 12), 22, -1044525330);
        b = self.r0(b, c, d, a, le_get_int(&self.mData, 12), 22, -1044525330_i32);
        // Java: A = this.R0(A, B, C, D, LittleEndian.getInt(this.mData, 16), 7, -176418897);
        a = self.r0(a, b, c, d, le_get_int(&self.mData, 16), 7, -176418897_i32);
        // Java: D = this.R0(D, A, B, C, LittleEndian.getInt(this.mData, 20), 12, 1200080426);
        d = self.r0(d, a, b, c, le_get_int(&self.mData, 20), 12, 1200080426);
        // Java: C = this.R0(C, D, A, B, LittleEndian.getInt(this.mData, 24), 17, -1473231341);
        c = self.r0(c, d, a, b, le_get_int(&self.mData, 24), 17, -1473231341_i32);
        // Java: B = this.R0(B, C, D, A, LittleEndian.getInt(this.mData, 28), 22, -45705983);
        b = self.r0(b, c, d, a, le_get_int(&self.mData, 28), 22, -45705983_i32);
        // Java: A = this.R0(A, B, C, D, LittleEndian.getInt(this.mData, 32), 7, 1770035416);
        a = self.r0(a, b, c, d, le_get_int(&self.mData, 32), 7, 1770035416);
        // Java: D = this.R0(D, A, B, C, LittleEndian.getInt(this.mData, 36), 12, -1958414417);
        d = self.r0(d, a, b, c, le_get_int(&self.mData, 36), 12, -1958414417_i32);
        // Java: C = this.R0(C, D, A, B, LittleEndian.getInt(this.mData, 40), 17, -42063);
        c = self.r0(c, d, a, b, le_get_int(&self.mData, 40), 17, -42063_i32);
        // Java: B = this.R0(B, C, D, A, LittleEndian.getInt(this.mData, 44), 22, -1990404162);
        b = self.r0(b, c, d, a, le_get_int(&self.mData, 44), 22, -1990404162_i32);
        // Java: A = this.R0(A, B, C, D, LittleEndian.getInt(this.mData, 48), 7, 1804603682);
        a = self.r0(a, b, c, d, le_get_int(&self.mData, 48), 7, 1804603682);
        // Java: D = this.R0(D, A, B, C, LittleEndian.getInt(this.mData, 52), 12, -40341101);
        d = self.r0(d, a, b, c, le_get_int(&self.mData, 52), 12, -40341101_i32);
        // Java: C = this.R0(C, D, A, B, LittleEndian.getInt(this.mData, 56), 17, -1502002290);
        c = self.r0(c, d, a, b, le_get_int(&self.mData, 56), 17, -1502002290_i32);
        // Java: B = this.R0(B, C, D, A, LittleEndian.getInt(this.mData, 60), 22, 1236535329);
        b = self.r0(b, c, d, a, le_get_int(&self.mData, 60), 22, 1236535329);
        // Java: A = this.R1(A, B, C, D, LittleEndian.getInt(this.mData, 4), 5, -165796510);
        a = self.r1(a, b, c, d, le_get_int(&self.mData, 4), 5, -165796510_i32);
        // Java: D = this.R1(D, A, B, C, LittleEndian.getInt(this.mData, 24), 9, -1069501632);
        d = self.r1(d, a, b, c, le_get_int(&self.mData, 24), 9, -1069501632_i32);
        // Java: C = this.R1(C, D, A, B, LittleEndian.getInt(this.mData, 44), 14, 643717713);
        c = self.r1(c, d, a, b, le_get_int(&self.mData, 44), 14, 643717713);
        // Java: B = this.R1(B, C, D, A, LittleEndian.getInt(this.mData, 0), 20, -373897302);
        b = self.r1(b, c, d, a, le_get_int(&self.mData, 0), 20, -373897302_i32);
        // Java: A = this.R1(A, B, C, D, LittleEndian.getInt(this.mData, 20), 5, -701558691);
        a = self.r1(a, b, c, d, le_get_int(&self.mData, 20), 5, -701558691_i32);
        // Java: D = this.R1(D, A, B, C, LittleEndian.getInt(this.mData, 40), 9, 38016083);
        d = self.r1(d, a, b, c, le_get_int(&self.mData, 40), 9, 38016083);
        // Java: C = this.R1(C, D, A, B, LittleEndian.getInt(this.mData, 60), 14, -660478335);
        c = self.r1(c, d, a, b, le_get_int(&self.mData, 60), 14, -660478335_i32);
        // Java: B = this.R1(B, C, D, A, LittleEndian.getInt(this.mData, 16), 20, -405537848);
        b = self.r1(b, c, d, a, le_get_int(&self.mData, 16), 20, -405537848_i32);
        // Java: A = this.R1(A, B, C, D, LittleEndian.getInt(this.mData, 36), 5, 568446438);
        a = self.r1(a, b, c, d, le_get_int(&self.mData, 36), 5, 568446438);
        // Java: D = this.R1(D, A, B, C, LittleEndian.getInt(this.mData, 56), 9, -1019803690);
        d = self.r1(d, a, b, c, le_get_int(&self.mData, 56), 9, -1019803690_i32);
        // Java: C = this.R1(C, D, A, B, LittleEndian.getInt(this.mData, 12), 14, -187363961);
        c = self.r1(c, d, a, b, le_get_int(&self.mData, 12), 14, -187363961_i32);
        // Java: B = this.R1(B, C, D, A, LittleEndian.getInt(this.mData, 32), 20, 1163531501);
        b = self.r1(b, c, d, a, le_get_int(&self.mData, 32), 20, 1163531501);
        // Java: A = this.R1(A, B, C, D, LittleEndian.getInt(this.mData, 52), 5, -1444681467);
        a = self.r1(a, b, c, d, le_get_int(&self.mData, 52), 5, -1444681467_i32);
        // Java: D = this.R1(D, A, B, C, LittleEndian.getInt(this.mData, 8), 9, -51403784);
        d = self.r1(d, a, b, c, le_get_int(&self.mData, 8), 9, -51403784_i32);
        // Java: C = this.R1(C, D, A, B, LittleEndian.getInt(this.mData, 28), 14, 1735328473);
        c = self.r1(c, d, a, b, le_get_int(&self.mData, 28), 14, 1735328473);
        // Java: B = this.R1(B, C, D, A, LittleEndian.getInt(this.mData, 48), 20, -1926607734);
        b = self.r1(b, c, d, a, le_get_int(&self.mData, 48), 20, -1926607734_i32);
        // Java: A = this.R2(A, B, C, D, LittleEndian.getInt(this.mData, 20), 4, -378558);
        a = self.r2(a, b, c, d, le_get_int(&self.mData, 20), 4, -378558_i32);
        // Java: D = this.R2(D, A, B, C, LittleEndian.getInt(this.mData, 32), 11, -2022574463);
        d = self.r2(d, a, b, c, le_get_int(&self.mData, 32), 11, -2022574463_i32);
        // Java: C = this.R2(C, D, A, B, LittleEndian.getInt(this.mData, 44), 16, 1839030562);
        c = self.r2(c, d, a, b, le_get_int(&self.mData, 44), 16, 1839030562);
        // Java: B = this.R2(B, C, D, A, LittleEndian.getInt(this.mData, 56), 23, -35309556);
        b = self.r2(b, c, d, a, le_get_int(&self.mData, 56), 23, -35309556_i32);
        // Java: A = this.R2(A, B, C, D, LittleEndian.getInt(this.mData, 4), 4, -1530992060);
        a = self.r2(a, b, c, d, le_get_int(&self.mData, 4), 4, -1530992060_i32);
        // Java: D = this.R2(D, A, B, C, LittleEndian.getInt(this.mData, 16), 11, 1272893353);
        d = self.r2(d, a, b, c, le_get_int(&self.mData, 16), 11, 1272893353);
        // Java: C = this.R2(C, D, A, B, LittleEndian.getInt(this.mData, 28), 16, -155497632);
        c = self.r2(c, d, a, b, le_get_int(&self.mData, 28), 16, -155497632_i32);
        // Java: B = this.R2(B, C, D, A, LittleEndian.getInt(this.mData, 40), 23, -1094730640);
        b = self.r2(b, c, d, a, le_get_int(&self.mData, 40), 23, -1094730640_i32);
        // Java: A = this.R2(A, B, C, D, LittleEndian.getInt(this.mData, 52), 4, 681279174);
        a = self.r2(a, b, c, d, le_get_int(&self.mData, 52), 4, 681279174);
        // Java: D = this.R2(D, A, B, C, LittleEndian.getInt(this.mData, 0), 11, -358537222);
        d = self.r2(d, a, b, c, le_get_int(&self.mData, 0), 11, -358537222_i32);
        // Java: C = this.R2(C, D, A, B, LittleEndian.getInt(this.mData, 12), 16, -722521979);
        c = self.r2(c, d, a, b, le_get_int(&self.mData, 12), 16, -722521979_i32);
        // Java: B = this.R2(B, C, D, A, LittleEndian.getInt(this.mData, 24), 23, 76029189);
        b = self.r2(b, c, d, a, le_get_int(&self.mData, 24), 23, 76029189);
        // Java: A = this.R2(A, B, C, D, LittleEndian.getInt(this.mData, 36), 4, -640364487);
        a = self.r2(a, b, c, d, le_get_int(&self.mData, 36), 4, -640364487_i32);
        // Java: D = this.R2(D, A, B, C, LittleEndian.getInt(this.mData, 48), 11, -421815835);
        d = self.r2(d, a, b, c, le_get_int(&self.mData, 48), 11, -421815835_i32);
        // Java: C = this.R2(C, D, A, B, LittleEndian.getInt(this.mData, 60), 16, 530742520);
        c = self.r2(c, d, a, b, le_get_int(&self.mData, 60), 16, 530742520);
        // Java: B = this.R2(B, C, D, A, LittleEndian.getInt(this.mData, 8), 23, -995338651);
        b = self.r2(b, c, d, a, le_get_int(&self.mData, 8), 23, -995338651_i32);
        // Java: A = this.R3(A, B, C, D, LittleEndian.getInt(this.mData, 0), 6, -198630844);
        a = self.r3(a, b, c, d, le_get_int(&self.mData, 0), 6, -198630844_i32);
        // Java: D = this.R3(D, A, B, C, LittleEndian.getInt(this.mData, 28), 10, 1126891415);
        d = self.r3(d, a, b, c, le_get_int(&self.mData, 28), 10, 1126891415);
        // Java: C = this.R3(C, D, A, B, LittleEndian.getInt(this.mData, 56), 15, -1416354905);
        c = self.r3(c, d, a, b, le_get_int(&self.mData, 56), 15, -1416354905_i32);
        // Java: B = this.R3(B, C, D, A, LittleEndian.getInt(this.mData, 20), 21, -57434055);
        b = self.r3(b, c, d, a, le_get_int(&self.mData, 20), 21, -57434055_i32);
        // Java: A = this.R3(A, B, C, D, LittleEndian.getInt(this.mData, 48), 6, 1700485571);
        a = self.r3(a, b, c, d, le_get_int(&self.mData, 48), 6, 1700485571);
        // Java: D = this.R3(D, A, B, C, LittleEndian.getInt(this.mData, 12), 10, -1894986606);
        d = self.r3(d, a, b, c, le_get_int(&self.mData, 12), 10, -1894986606_i32);
        // Java: C = this.R3(C, D, A, B, LittleEndian.getInt(this.mData, 40), 15, -1051523);
        c = self.r3(c, d, a, b, le_get_int(&self.mData, 40), 15, -1051523_i32);
        // Java: B = this.R3(B, C, D, A, LittleEndian.getInt(this.mData, 4), 21, -2054922799);
        b = self.r3(b, c, d, a, le_get_int(&self.mData, 4), 21, -2054922799_i32);
        // Java: A = this.R3(A, B, C, D, LittleEndian.getInt(this.mData, 32), 6, 1873313359);
        a = self.r3(a, b, c, d, le_get_int(&self.mData, 32), 6, 1873313359);
        // Java: D = this.R3(D, A, B, C, LittleEndian.getInt(this.mData, 60), 10, -30611744);
        d = self.r3(d, a, b, c, le_get_int(&self.mData, 60), 10, -30611744_i32);
        // Java: C = this.R3(C, D, A, B, LittleEndian.getInt(this.mData, 24), 15, -1560198380);
        c = self.r3(c, d, a, b, le_get_int(&self.mData, 24), 15, -1560198380_i32);
        // Java: B = this.R3(B, C, D, A, LittleEndian.getInt(this.mData, 52), 21, 1309151649);
        b = self.r3(b, c, d, a, le_get_int(&self.mData, 52), 21, 1309151649);
        // Java: A = this.R3(A, B, C, D, LittleEndian.getInt(this.mData, 16), 6, -145523070);
        a = self.r3(a, b, c, d, le_get_int(&self.mData, 16), 6, -145523070_i32);
        // Java: D = this.R3(D, A, B, C, LittleEndian.getInt(this.mData, 44), 10, -1120210379);
        d = self.r3(d, a, b, c, le_get_int(&self.mData, 44), 10, -1120210379_i32);
        // Java: C = this.R3(C, D, A, B, LittleEndian.getInt(this.mData, 8), 15, 718787259);
        c = self.r3(c, d, a, b, le_get_int(&self.mData, 8), 15, 718787259);
        // Java: B = this.R3(B, C, D, A, LittleEndian.getInt(this.mData, 36), 21, -343485551);
        b = self.r3(b, c, d, a, le_get_int(&self.mData, 36), 21, -343485551_i32);
        // Java: this.mA = (int)(((long)this.mA & 4294967295L) + ((long)A & 4294967295L) & -1L);
        self.mA = ((self.mA as u32).wrapping_add(a as u32)) as i32;
        // Java: this.mB = (int)(((long)this.mB & 4294967295L) + ((long)B & 4294967295L) & -1L);
        self.mB = ((self.mB as u32).wrapping_add(b as u32)) as i32;
        // Java: this.mC = (int)(((long)this.mC & 4294967295L) + ((long)C & 4294967295L) & -1L);
        self.mC = ((self.mC as u32).wrapping_add(c as u32)) as i32;
        // Java: this.mD = (int)(((long)this.mD & 4294967295L) + ((long)D & 4294967295L) & -1L);
        self.mD = ((self.mD as u32).wrapping_add(d as u32)) as i32;
    }

    // Java: int F(int x, int y, int z)
    fn f_func(&self, x: i32, y: i32, z: i32) -> i32 {
        // Java: return (y ^ z) & x ^ z;
        ((y ^ z) & x) ^ z
    }

    // Java: int G(int x, int y, int z)
    fn g_func(&self, x: i32, y: i32, z: i32) -> i32 {
        // Java: return (x ^ y) & z ^ y;
        ((x ^ y) & z) ^ y
    }

    // Java: int H(int x, int y, int z)
    fn h_func(&self, x: i32, y: i32, z: i32) -> i32 {
        // Java: return x ^ y ^ z;
        x ^ y ^ z
    }

    // Java: int I(int x, int y, int z)
    fn i_func(&self, x: i32, y: i32, z: i32) -> i32 {
        // Java: return (x | ~z) ^ y;
        (x | !z) ^ y
    }

    // Java: int digestROTL(long a, int n)
    fn digestROTL(&self, a: i64, n: i32) -> i32 {
        // Java: return (int)((a << n | a >> 32 - n) & -1L);
        let au = a as u64 & 0xFFFFFFFF;
        ((au << n) | (au >> (32 - n))) as i32
    }

    // Java: int R0(int a, int b, int c, int d, int k, int s, int t)
    fn r0(&self, mut a: i32, b: i32, c: i32, d: i32, k: i32, s: i32, t: i32) -> i32 {
        // Java: a = (int)(((long)a & 4294967295L) + ((long)k & 4294967295L) + ((long)t & 4294967295L) + (long)this.F(b, c, d) & -1L);
        a = ((a as u32).wrapping_add(k as u32).wrapping_add(t as u32).wrapping_add(self.f_func(b, c, d) as u32)) as i32;
        // Java: a = this.digestROTL((long)a & 4294967295L, s);
        a = self.digestROTL((a as i64) & 4294967295_i64, s);
        // Java: a = (int)(((long)a & 4294967295L) + ((long)b & 4294967295L) & -1L);
        a = ((a as u32).wrapping_add(b as u32)) as i32;
        // Java: return a;
        a
    }

    // Java: int R1(int a, int b, int c, int d, int k, int s, int t)
    fn r1(&self, mut a: i32, b: i32, c: i32, d: i32, k: i32, s: i32, t: i32) -> i32 {
        // Java: a = (int)(((long)a & 4294967295L) + ((long)k & 4294967295L) + ((long)t & 4294967295L) + (long)this.G(b, c, d) & -1L);
        a = ((a as u32).wrapping_add(k as u32).wrapping_add(t as u32).wrapping_add(self.g_func(b, c, d) as u32)) as i32;
        // Java: a = this.digestROTL((long)a & 4294967295L, s);
        a = self.digestROTL((a as i64) & 4294967295_i64, s);
        // Java: a = (int)(((long)a & 4294967295L) + ((long)b & 4294967295L) & -1L);
        a = ((a as u32).wrapping_add(b as u32)) as i32;
        // Java: return a;
        a
    }

    // Java: int R2(int a, int b, int c, int d, int k, int s, int t)
    fn r2(&self, mut a: i32, b: i32, c: i32, d: i32, k: i32, s: i32, t: i32) -> i32 {
        // Java: a = (int)(((long)a & 4294967295L) + ((long)k & 4294967295L) + ((long)t & 4294967295L) + (long)this.H(b, c, d) & -1L);
        a = ((a as u32).wrapping_add(k as u32).wrapping_add(t as u32).wrapping_add(self.h_func(b, c, d) as u32)) as i32;
        // Java: a = this.digestROTL((long)a & 4294967295L, s);
        a = self.digestROTL((a as i64) & 4294967295_i64, s);
        // Java: a = (int)(((long)a & 4294967295L) + ((long)b & 4294967295L) & -1L);
        a = ((a as u32).wrapping_add(b as u32)) as i32;
        // Java: return a;
        a
    }

    // Java: int R3(int a, int b, int c, int d, int k, int s, int t)
    fn r3(&self, mut a: i32, b: i32, c: i32, d: i32, k: i32, s: i32, t: i32) -> i32 {
        // Java: a = (int)(((long)a & 4294967295L) + ((long)k & 4294967295L) + ((long)t & 4294967295L) + (long)this.I(b, c, d) & -1L);
        a = ((a as u32).wrapping_add(k as u32).wrapping_add(t as u32).wrapping_add(self.i_func(b, c, d) as u32)) as i32;
        // Java: a = this.digestROTL((long)a & 4294967295L, s);
        a = self.digestROTL((a as i64) & 4294967295_i64, s);
        // Java: a = (int)(((long)a & 4294967295L) + ((long)b & 4294967295L) & -1L);
        a = ((a as u32).wrapping_add(b as u32)) as i32;
        // Java: return a;
        a
    }

    // Java: void end()
    fn end(&mut self) {
        // Java: short[] end = new short[]{128, 0, 0, 0};
        let end_vals: [i32; 4] = [128, 0, 0, 0];
        // Java: int i = this.mDataLen >> 2;
        let i = (self.mDataLen >> 2) as usize;
        // Java: switch (this.mDataLen & 3)
        match self.mDataLen & 3 {
            // Java: case 1:
            1 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) & 255);
                let v = le_get_int(&self.mData, i * 4) & 255;
                le_put_int(&mut self.mData, i * 4, v);
                // Java: break;
            },
            // Java: case 2:
            2 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) & '\uffff');
                let v = le_get_int(&self.mData, i * 4) & 0xFFFF;
                le_put_int(&mut self.mData, i * 4, v);
                // Java: break;
            },
            // Java: case 3:
            3 => {
                // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) & 16777215);
                let v = le_get_int(&self.mData, i * 4) & 16777215;
                le_put_int(&mut self.mData, i * 4, v);
            },
            _ => {}
        }
        // Java: switch (this.mDataLen & 3)
        let case_val = self.mDataLen & 3;
        // Java: case 0:
        if case_val <= 0 {
            // Java: LittleEndian.putInt(this.mData, i * 4, end[0] << 0);
            le_put_int(&mut self.mData, i * 4, end_vals[0]);
        }
        // Java: case 1:
        if case_val <= 1 {
            // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[1] << 8);
            let v = le_get_int(&self.mData, i * 4) | (end_vals[1] << 8);
            le_put_int(&mut self.mData, i * 4, v);
        }
        // Java: case 2:
        if case_val <= 2 {
            // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[2] << 16);
            let v = le_get_int(&self.mData, i * 4) | (end_vals[2] << 16);
            le_put_int(&mut self.mData, i * 4, v);
        }
        // Java: case 3:
        if case_val <= 3 {
            // Java: LittleEndian.putInt(this.mData, i * 4, LittleEndian.getInt(this.mData, i * 4) | end[3] << 24);
            let v = le_get_int(&self.mData, i * 4) | (end_vals[3] << 24);
            le_put_int(&mut self.mData, i * 4, v);
        }
        // Java: i += 4;
        let mut idx = i + 4;
        // Java: if (i >= 56)
        if idx >= 56 {
            // Java: while(i < 64)
            while idx < 64 {
                // Java: this.mData[i] = 0;
                self.mData[idx] = 0;
                // Java: ++i;
                idx += 1;
            }
            // Java: this.update();
            self.update_block();
            // Java: i = 0;
            idx = 0;
        }
        // Java: while(i < 56)
        while idx < 56 {
            // Java: this.mData[i] = 0;
            self.mData[idx] = 0;
            // Java: ++i;
            idx += 1;
        }
        // Java: LittleEndian.putInt(this.mData, 8, this.mL);
        le_put_int(&mut self.mData, 56, self.mL);
        // Java: LittleEndian.putInt(this.mData, 12, this.mH);
        le_put_int(&mut self.mData, 60, self.mH);
        // Java: this.update();
        self.update_block();
    }
}

impl IDigest for DigestMD5 {
    fn init(&mut self, var1: &[u8], var2: i32, var3: i32) -> i32 {
        self.init_with_data(var1, var2, var3)
    }
    fn update(&mut self, var1: &[u8], var2: i32, var3: i32) -> i32 {
        self.update_data(var1, var2, var3)
    }
    fn get(&mut self, var1: &mut [u8]) -> i32 {
        self.get_digest(var1)
    }
    fn get_with_params(&mut self, var1: &[u8], var2: i32, var3: i32, var4: &mut [u8]) -> i32 {
        self.get_with_params(var1, var2, var3, var4)
    }
    fn raw(&mut self, var1: &mut [u8]) -> i32 {
        self.raw_impl(var1)
    }
    fn get_length(&self) -> i32 {
        self.getLength()
    }
    fn dispose(&mut self) {
        self.dispose_impl();
    }
}
