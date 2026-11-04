// Java: import cn.wps.crypt.sal.CipherUtil
use crate::cn::wps::crypt::sal::CipherUtil::CipherUtil;
// Java: import cn.wps.crypt.sal.DigestUtil
use crate::cn::wps::crypt::sal::DigestUtil::DigestUtil;
// Java: import cn.wps.crypt.sal.cipher.Cipher
use crate::cn::wps::crypt::sal::cipher::Cipher::Cipher;
// Java: import cn.wps.crypt.sal.cipher.CipherDirection
use crate::cn::wps::crypt::sal::cipher::CipherDirection::CIPHER_DIRECTION_DECODE;
// Java: import cn.wps.crypt.sal.cipher.CipherError
use crate::cn::wps::crypt::sal::cipher::CipherError::CIPHER_E_NONE;
// Java: import cn.wps.crypt.sal.cipher.CipherMode
use crate::cn::wps::crypt::sal::cipher::CipherMode::CIPHER_MODE_STREAM;
// Java: import cn.wps.crypt.sal.digest.IDigest
use crate::cn::wps::crypt::sal::digest::IDigest::IDigest;
// Java: import java.security.MessageDigest
// Java: import java.security.NoSuchAlgorithmException
// Java: import java.util.Arrays
// Java: import org.apache.poi.util.LittleEndian

fn le_put_short(data: &mut [u8], offset: usize, value: i16) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
}
fn le_put_int(data: &mut [u8], offset: usize, value: i32) {
    data[offset] = value as u8;
    data[offset + 1] = (value >> 8) as u8;
    data[offset + 2] = (value >> 16) as u8;
    data[offset + 3] = (value >> 24) as u8;
}

// Java: public class MSCodec_Std97
pub struct MSCodec_Std97 {
    // Java field: IDigest mDigest
    pub(crate) mDigest: Box<dyn IDigest>,
    // Java field: Cipher mCipher
    pub(crate) mCipher: Box<dyn Cipher>,
    // Java field: byte[] mDigestValue
    pub(crate) mDigestValue: Vec<u8>,
}

// Java: public class MSCodec_Std97
impl MSCodec_Std97 {
    // Java: MSCodec_Std97()
    pub fn new() -> Self {
        // Java: super();
        // Java: this.mCipher = CipherUtil.create(1, CipherMode.Cipher_ModeStream);
        let mCipher = CipherUtil::create(1, CIPHER_MODE_STREAM).expect("Failed to create cipher");
        // Java: this.mDigestValue = new byte[16];
        let mDigestValue = vec![0u8; 16];
        let mDigest = DigestUtil::create(1).expect("Failed to create digest");
        MSCodec_Std97 {
            mDigest,
            mCipher,
            mDigestValue,
        }
    }

    // Java: void InitKey(short[] passData, byte[] unique)
    pub fn InitKey(&mut self, passData: &[i16], unique: &[u8]) {
        // Java: byte[] pKeyData = new byte[64];
        let mut pKeyData = vec![0u8; 64];
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: for(int n = 16; i < n && passData[i] != 0; ++i)
        let n: usize = 16;
        while i < n && i < passData.len() && passData[i] != 0 {
            // Java: LittleEndian.putShort(pKeyData, 2 * i, passData[i]);
            le_put_short(&mut pKeyData, 2 * i, passData[i]);
            i += 1;
        }
        // Java: LittleEndian.putByte(pKeyData, 2 * i, 128);
        pKeyData[2 * i] = 128;
        // Java: LittleEndian.putByte(pKeyData, 56, i << 4);
        pKeyData[56] = (i << 4) as u8;
        // Java: this.mDigest.update(pKeyData, 0, pKeyData.length);
        self.mDigest.update(&pKeyData, 0, pKeyData.len() as i32);
        // Java: byte[] digest = null;
        // Java: try
        // Java: MessageDigest md5 = MessageDigest.getInstance("MD5");
        // Java: md5.update(pKeyData);
        // Java: digest = md5.digest();
        // Java: Arrays.equals(pKeyData, digest);
        // Java: catch (NoSuchAlgorithmException var8)
        // Java: this.mDigest.raw(pKeyData);
        self.mDigest.raw(&mut pKeyData);
        // Java: for(int var10 = 0; var10 < 16; ++var10)
        for _var10 in 0..16 {
            // Java: this.mDigest.update(pKeyData, 0, 5);
            self.mDigest.update(&pKeyData, 0, 5);
            // Java: this.mDigest.update(unique, 0, 16);
            self.mDigest.update(unique, 0, 16);
        }
        // Java: LittleEndian.putByte(pKeyData, 16, 128);
        pKeyData[16] = 128;
        // Java: Arrays.fill(pKeyData, 17, pKeyData.length - 1, (byte)0);
        for j in 17..(pKeyData.len() - 1) {
            pKeyData[j] = 0;
        }
        // Java: LittleEndian.putByte(pKeyData, 56, 128);
        pKeyData[56] = 128;
        // Java: LittleEndian.putByte(pKeyData, 57, 10);
        pKeyData[57] = 10;
        // Java: this.mDigest.update(pKeyData, 16, pKeyData.length - 16);
        self.mDigest.update(&pKeyData[16..], 0, (pKeyData.len() - 16) as i32);
        // Java: this.mDigest.raw(this.mDigestValue);
        self.mDigest.raw(&mut self.mDigestValue);
        // Java: byte[] var9 = null;
    }

    // Java: boolean verifyKey(byte[] saltData, byte[] saltDigest)
    pub fn verifyKey(&mut self, saltData: &[u8], saltDigest: &[u8]) -> bool {
        // Java: boolean result = false;
        let mut result = false;
        // Java: if (this.initCipher(0))
        if self.initCipher(0) {
            // Java: byte[] digest = new byte[16];
            let mut digest = vec![0u8; 16];
            // Java: byte[] buffer = new byte[64];
            let mut buffer = vec![0u8; 64];
            // Java: this.mCipher.decode(saltData, 0, 16, buffer);
            self.mCipher.decode(saltData, 0, 16, &mut buffer);
            // Java: LittleEndian.putByte(buffer, 16, 128);
            buffer[16] = 128;
            // Java: Arrays.fill(buffer, 17, buffer.length - 1, (byte)0);
            for j in 17..(buffer.len() - 1) {
                buffer[j] = 0;
            }
            // Java: LittleEndian.putByte(buffer, 56, 128);
            buffer[56] = 128u8;
            // Java: this.mDigest.update(buffer, 0, buffer.length);
            self.mDigest.update(&buffer, 0, buffer.len() as i32);
            // Java: this.mDigest.raw(digest);
            self.mDigest.raw(&mut digest);
            // Java: this.mCipher.decode(saltDigest, 0, 16, buffer);
            self.mCipher.decode(saltDigest, 0, 16, &mut buffer);
            // Java: result = Arrays.equals(LittleEndian.getByteArray(buffer, 0, 16), digest);
            result = buffer[..16] == digest[..];
            // Java: byte[] var6 = null;
            // Java: Object var7 = null;
        }
        // Java: return result;
        result
    }

    // Java: boolean initCipher(int nCounter)
    pub fn initCipher(&mut self, nCounter: i32) -> bool {
        // Java: byte[] pKeyData = new byte[64];
        let mut pKeyData = vec![0u8; 64];
        // Java: System.arraycopy(this.mDigestValue, 0, pKeyData, 0, 5);
        pKeyData[..5].copy_from_slice(&self.mDigestValue[..5]);
        // Java: LittleEndian.putInt(pKeyData, 5, nCounter);
        le_put_int(&mut pKeyData, 5, nCounter);
        // Java: LittleEndian.putByte(pKeyData, 9, 128);
        pKeyData[9] = 128;
        // Java: LittleEndian.putByte(pKeyData, 56, 72);
        pKeyData[56] = 72;
        // Java: this.mDigest.update(pKeyData, 0, pKeyData.length);
        self.mDigest.update(&pKeyData, 0, pKeyData.len() as i32);
        // Java: this.mDigest.raw(pKeyData);
        self.mDigest.raw(&mut pKeyData);
        // Java: int result = this.mCipher.init(CipherDirection.Cipher_DirectionDecode, LittleEndian.getByteArray(pKeyData, 0, this.mDigest.getLength()));
        let digest_len = self.mDigest.get_length() as usize;
        let key_data = &pKeyData[..digest_len];
        let result = self.mCipher.init(CIPHER_DIRECTION_DECODE, key_data);
        // Java: byte[] var4 = null;
        // Java: return result == CipherError.Cipher_E_None;
        result == CIPHER_E_NONE
    }

    // Java: boolean decode(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn decode(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> bool {
        // Java: return this.mCipher.decode(data, offset, nLen, buffer) == CipherError.Cipher_E_None;
        self.mCipher.decode(data, offset, nLen, buffer) == CIPHER_E_NONE
    }

    // Java: boolean skip(int nDatLen)
    pub fn skip(&mut self, nDatLen: i32) -> bool {
        // Java: byte[] pnDummy = new byte[1024];
        let mut pnDummy = vec![0u8; 1024];
        // Java: int nDatLeft = nDatLen;
        let mut nDatLeft = nDatLen;
        // Java: boolean bResult;
        let mut bResult;
        // Java: int nBlockLen;
        let mut nBlockLen;
        // Java: for(bResult = true; bResult && nDatLeft > 0; nDatLeft -= nBlockLen)
        bResult = true;
        nBlockLen = 0;
        while bResult && nDatLeft > 0 {
            nDatLeft -= nBlockLen;
            // Java: nBlockLen = Math.min(nDatLeft, 1024);
            nBlockLen = std::cmp::min(nDatLeft, 1024);
            // Java: bResult = this.decode(pnDummy, 0, nBlockLen, pnDummy);
            let mut out = vec![0u8; 1024];
            bResult = self.decode(&pnDummy, 0, nBlockLen, &mut out);
            pnDummy = out;
        }
        // Java: return bResult;
        bResult
    }

    // Java: boolean encode(byte[] data, int offset, int nLen, byte[] buffer)
    pub fn encode(&mut self, data: &[u8], offset: i32, nLen: i32, buffer: &mut [u8]) -> bool {
        // Java: return this.mCipher.encode(data, offset, nLen, buffer) == CipherError.Cipher_E_None;
        self.mCipher.encode(data, offset, nLen, buffer) == CIPHER_E_NONE
    }

    // Java: int getHash(byte[] saltData)
    pub fn getHash(&mut self, saltData: &mut [u8]) -> i32 {
        // Java: byte[] digest = new byte[16];
        let mut digest = vec![0u8; 16];
        // Java: byte[] buffer = new byte[64];
        let mut buffer = vec![0u8; 64];
        // Java: System.arraycopy(saltData, 0, buffer, 0, 16);
        buffer[..16].copy_from_slice(&saltData[..16]);
        // Java: buffer[16] = -128;
        buffer[16] = 0x80u8;
        // Java: buffer[56] = -128;
        buffer[56] = 0x80u8;
        // Java: this.mDigest.update(buffer, 0, buffer.length);
        self.mDigest.update(&buffer, 0, buffer.len() as i32);
        // Java: this.mDigest.raw(digest);
        self.mDigest.raw(&mut digest);
        // Java: this.mCipher.encode(digest, 0, digest.length, saltData);
        let digest_len = digest.len() as i32;
        self.mCipher.encode(&digest, 0, digest_len, saltData);
        // Java: byte[] var5 = null;
        // Java: byte[] var4 = null;
        // Java: return this.mDigest.getLength();
        self.mDigest.get_length()
    }
}
