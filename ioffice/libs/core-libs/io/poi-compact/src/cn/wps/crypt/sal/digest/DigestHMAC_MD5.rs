// Java: import java.util.Arrays

use super::IDigest::IDigest;
use super::DigestMD5::DigestMD5;
use super::DigestError;

// Java: public class DigestHMAC_MD5 implements IDigest
pub struct DigestHMAC_MD5 {
    // Java field: int DIGEST_LENGTH_HMAC_MD5
    // Java field: int DIGEST_CBLOCK_HMAC_MD5
    // Java field: DigestMD5 md5
    md5: DigestMD5,
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: int mLength
    mLength: i32,
    // Java field: byte[] mOpad
    mOpad: Vec<u8>,
}

const DIGEST_LENGTH_HMAC_MD5: i32 = 16;
const DIGEST_CBLOCK_HMAC_MD5: i32 = 64;

// Java: public class DigestHMAC_MD5 implements IDigest
impl DigestHMAC_MD5 {
    // Java: DigestHMAC_MD5()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mOpad = new byte[DIGEST_CBLOCK_HMAC_MD5];
        let mut s = DigestHMAC_MD5 {
            md5: DigestMD5::new(),
            mAlgorithm: 0,
            mLength: 0,
            mOpad: vec![0u8; DIGEST_CBLOCK_HMAC_MD5 as usize],
        };
        // Java: this.init();
        s.init_reset();
        s
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
        // Java: this.init();
        self.init_reset();
    }

    // Java: int get(byte[] buffer)
    pub fn get_digest(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: if (buffer == null)
        // Java: return DigestError.Digest_E_Argument;
        // Java: else if (this.mAlgorithm != 0)
        if self.mAlgorithm != 0 {
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
            // Java: this.md5.get(buffer);
            self.md5.get(buffer);
            // Java: this.md5.update(this.mOpad, 0, DIGEST_CBLOCK_HMAC_MD5);
            let opad_copy = self.mOpad.clone();
            self.md5.update(&opad_copy, 0, DIGEST_CBLOCK_HMAC_MD5);
            // Java: this.md5.update(buffer, 0, this.mLength);
            let buf_copy = buffer.to_vec();
            let ml = self.mLength;
            self.md5.update(&buf_copy, 0, ml);
            // Java: this.md5.get(buffer);
            self.md5.get(buffer);
            // Java: this.opad();
            self.opad();
            // Java: this.ipad();
            self.ipad();
            // Java: this.opad();
            self.opad();
            // Java: return DigestError.Digest_E_None;
            DigestError::DIGEST_E_NONE
        }
    }

    // Java: int get(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn get_with_params(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 {
        // Java: this.init();
        self.init_reset();
        // Java: int result = this.init(data, offset, nLen);
        let mut result = self.init_data(data, offset, nLen);
        // Java: if (result == DigestError.Digest_E_None)
        if result == DigestError::DIGEST_E_NONE {
            // Java: result = this.md5.update(data, offset, nLen);
            result = self.md5.update(data, offset, nLen);
            // Java: if (result == DigestError.Digest_E_None)
            if result == DigestError::DIGEST_E_NONE {
                // Java: result = this.md5.get(buffer);
                result = self.md5.get(buffer);
            }
        }
        // Java: this.init();
        self.init_reset();
        // Java: return result;
        result
    }

    // Java: int init(byte[] data, int offset, int nLen)
    pub fn init_data(&mut self, data: &[u8], offset: i32, nLen: i32) -> i32 {
        // Java: if (data == null)
        // Java: return DigestError.Digest_E_Argument;
        // Java: else if (this.mAlgorithm != 1)
        if self.mAlgorithm != 1 {
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
            // Java: this.init();
            self.init_reset();
            // Java: if (nLen > DIGEST_CBLOCK_HMAC_MD5)
            if nLen > DIGEST_CBLOCK_HMAC_MD5 {
                // Java: this.md5.update(data, offset, nLen);
                self.md5.update(data, offset, nLen);
                // Java: this.md5.get(this.mOpad);
                let mut opad_buf = vec![0u8; self.mOpad.len()];
                self.md5.get(&mut opad_buf);
                self.mOpad = opad_buf;
            }
            // Java: else
            else {
                // Java: System.arraycopy(data, offset, this.mOpad, 0, nLen);
                let off = offset as usize;
                let len = nLen as usize;
                self.mOpad[..len].copy_from_slice(&data[off..off + len]);
            }
            // Java: this.ipad();
            self.ipad();
            // Java: this.opad();
            self.opad();
            // Java: return DigestError.Digest_E_None;
            DigestError::DIGEST_E_NONE
        }
    }

    // Java: int raw(byte[] buffer)
    pub fn raw(&mut self, _buffer: &mut [u8]) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int update(byte[] data, int offset, int nLen)
    pub fn update_data(&mut self, data: &[u8], offset: i32, nLen: i32) -> i32 {
        // Java: if (data == null)
        // Java: return DigestError.Digest_E_Argument;
        // Java: else if (this.mAlgorithm != 1)
        if self.mAlgorithm != 1 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else
        else {
            // Java: return nLen == 0 ? DigestError.Digest_E_None : this.md5.update(data, offset, nLen);
            if nLen == 0 { DigestError::DIGEST_E_NONE } else { self.md5.update(data, offset, nLen) }
        }
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return DIGEST_LENGTH_HMAC_MD5;
        DIGEST_LENGTH_HMAC_MD5
    }

    // Java: void init()
    fn init_reset(&mut self) {
        // Java: this.mAlgorithm = 4;
        self.mAlgorithm = 4;
        // Java: this.mLength = DIGEST_LENGTH_HMAC_MD5;
        self.mLength = DIGEST_LENGTH_HMAC_MD5;
        // Java: Arrays.fill(this.mOpad, (byte)0);
        self.mOpad.fill(0);
        // Java: this.md5.init();
        self.md5.dispose_impl();
    }

    // Java: void ipad()
    fn ipad(&mut self) {
        // Java: for(int i = 0; i < DIGEST_CBLOCK_HMAC_MD5; ++i)
        for i in 0..DIGEST_CBLOCK_HMAC_MD5 as usize {
            // Java: byte[] var10000 = this.mOpad;
            // Java: var10000[i] = (byte)(var10000[i] ^ 54);
            self.mOpad[i] ^= 54;
        }
        // Java: this.update(this.mOpad, 0, DIGEST_CBLOCK_HMAC_MD5);
        let opad_copy = self.mOpad.clone();
        self.md5.update(&opad_copy, 0, DIGEST_CBLOCK_HMAC_MD5);
        // Java: for(int var2 = 0; var2 < DIGEST_CBLOCK_HMAC_MD5; ++var2)
        for var2 in 0..DIGEST_CBLOCK_HMAC_MD5 as usize {
            // Java: byte[] var3 = this.mOpad;
            // Java: var3[var2] = (byte)(var3[var2] ^ 54);
            self.mOpad[var2] ^= 54;
        }
    }

    // Java: void opad()
    fn opad(&mut self) {
        // Java: for(int i = 0; i < DIGEST_CBLOCK_HMAC_MD5; ++i)
        for i in 0..DIGEST_CBLOCK_HMAC_MD5 as usize {
            // Java: byte[] var10000 = this.mOpad;
            // Java: var10000[i] = (byte)(var10000[i] ^ 92);
            self.mOpad[i] ^= 92;
        }
    }
}

impl IDigest for DigestHMAC_MD5 {
    fn dispose(&mut self) { self.dispose(); }
    fn get(&mut self, buffer: &mut [u8]) -> i32 { self.get_digest(buffer) }
    fn get_with_params(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> i32 { self.get_with_params(data, offset, nLen, buffer) }
    fn init(&mut self, data: &[u8], offset: i32, nLen: i32) -> i32 { self.init_data(data, offset, nLen) }
    fn raw(&mut self, buffer: &mut [u8]) -> i32 { self.raw(buffer) }
    fn update(&mut self, data: &[u8], offset: i32, nLen: i32) -> i32 { self.update_data(data, offset, nLen) }
    fn get_length(&self) -> i32 { self.getLength() }
}
