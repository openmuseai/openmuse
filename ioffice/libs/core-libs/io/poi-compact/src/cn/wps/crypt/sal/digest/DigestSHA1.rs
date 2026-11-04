// Java: import java.util.Arrays
use super::IDigest::IDigest;
use super::DigestError;

// Java field: int DIGEST_LENGTH_SHA1
pub const DIGEST_LENGTH_SHA1: i32 = 20;
// Java field: int H0
const H0: i32 = 1732584193;
// Java field: int H1
const H1: i32 = -271733879_i32;
// Java field: int H2
const H2: i32 = -1732584194_i32;
// Java field: int H3
const H3: i32 = 271733878;
// Java field: int H4
const H4: i32 = -1009589776_i32;
// Java field: int BYTES_OFFSET
const BYTES_OFFSET: usize = 81;
// Java field: int HASH_OFFSET
const HASH_OFFSET: usize = 82;

// Java: public class DigestSHA1 implements IDigest
pub struct DigestSHA1 {
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: int[] buffer
    buffer: Vec<i32>,
    // Java field: byte[] oneByte
    oneByte: Vec<u8>,
    // Java field: int messageLength
    messageLength: i32,
}

// Java: public class DigestSHA1 implements IDigest
impl DigestSHA1 {
    // Java: DigestSHA1()
    pub fn new() -> Self {
        // Java: super();
        let mut s = DigestSHA1 {
            mAlgorithm: 0,
            buffer: vec![0i32; 87],
            oneByte: vec![0u8; 1],
            messageLength: 0,
        };
        // Java: this.init();
        s.init_state();
        s
    }

    // Java: void init()
    fn init_state(&mut self) {
        // Java: this.mAlgorithm = 3;
        self.mAlgorithm = 3;
        // Java: this.messageLength = 0;
        self.messageLength = 0;
        // Java: this.buffer[81] = 0;
        self.buffer[81] = 0;
        // Java: this.buffer[82] = 1732584193;
        self.buffer[82] = 1732584193;
        // Java: this.buffer[83] = -271733879;
        self.buffer[83] = -271733879_i32;
        // Java: this.buffer[84] = -1732584194;
        self.buffer[84] = -1732584194_i32;
        // Java: this.buffer[85] = 271733878;
        self.buffer[85] = 271733878;
        // Java: this.buffer[86] = -1009589776;
        self.buffer[86] = -1009589776_i32;
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
        // Java: else if (this.mAlgorithm != 3)
        else if self.mAlgorithm != 3 {
            // Java: return DigestError.Digest_E_Algorithm;
            return DigestError::DIGEST_E_ALGORITHM;
        }
        // Java: else
        else {
            // Java: this.processDigest(buffer, 0);
            self.processDigest(buffer, 0);
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

    // Java: int init(byte[] data, int offset, int nLen)
    pub fn init_with_data(&mut self, _data: &[u8], _offset: i32, _nLen: i32) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int raw(byte[] buffer)
    pub fn raw_impl(&mut self, _buffer: &mut [u8]) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int update(byte[] data, int offset, int nLen)
    pub fn update_data(&mut self, data: &[u8], offset: i32, nLen: i32) -> i32 {
        // Java: if (data == null)
        if data.is_empty() && nLen > 0 {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 3)
        else if self.mAlgorithm != 3 {
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
            // Java: updateHash(this.buffer, data, offset, offset + nLen - 1);
            Self::updateHash(&mut self.buffer, data, offset, offset + nLen - 1);
            // Java: this.messageLength += nLen;
            self.messageLength += nLen;
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return 20;
        20
    }

    // Java: void engineUpdate(byte input)
    fn engineUpdate(&mut self, input: u8) {
        // Java: this.oneByte[0] = input;
        self.oneByte[0] = input;
        // Java: updateHash(this.buffer, this.oneByte, 0, 0);
        let oneByte = self.oneByte.clone();
        Self::updateHash(&mut self.buffer, &oneByte, 0, 0);
        // Java: ++this.messageLength;
        self.messageLength += 1;
    }

    // Java: void processDigest(byte[] digest, int offset)
    fn processDigest(&mut self, digest: &mut [u8], offset: i32) {
        // Java: long nBits = (long)(this.messageLength << 3);
        let nBits: i64 = (self.messageLength << 3) as i64;
        // Java: this.engineUpdate((byte)-128);
        self.engineUpdate(0x80u8);
        // Java: int i = 0;
        let mut i: i32 = 0;
        // Java: int lastWord = this.buffer[81] + 3 >> 2;
        let lastWord = (self.buffer[81] + 3) >> 2;
        // Java: if (this.buffer[81] != 0)
        if self.buffer[81] != 0 {
            // Java: if (lastWord < 15)
            if lastWord < 15 {
                // Java: i = lastWord;
                i = lastWord;
            }
            // Java: else
            else {
                // Java: if (lastWord == 15)
                if lastWord == 15 {
                    // Java: this.buffer[15] = 0;
                    self.buffer[15] = 0;
                }
                // Java: computeHash(this.buffer);
                Self::computeHash(&mut self.buffer);
                // Java: i = 0;
                i = 0;
            }
        }
        // Java: Arrays.fill(this.buffer, i, 14, 0);
        for idx in (i as usize)..14 {
            self.buffer[idx] = 0;
        }
        // Java: this.buffer[14] = (int)(nBits >>> 32);
        self.buffer[14] = ((nBits as u64) >> 32) as i32;
        // Java: this.buffer[15] = (int)(nBits & -1L);
        self.buffer[15] = (nBits & -1i64) as i32;
        // Java: computeHash(this.buffer);
        Self::computeHash(&mut self.buffer);
        // Java: int j = offset;
        let mut j = offset as usize;
        // Java: for(int var9 = 82; var9 < 87; ++var9)
        for var9 in 82..87 {
            // Java: int k = this.buffer[var9];
            let k = self.buffer[var9] as u32;
            // Java: digest[j] = (byte)(k >>> 24);
            digest[j] = (k >> 24) as u8;
            // Java: digest[j + 1] = (byte)(k >>> 16);
            digest[j + 1] = (k >> 16) as u8;
            // Java: digest[j + 2] = (byte)(k >>> 8);
            digest[j + 2] = (k >> 8) as u8;
            // Java: digest[j + 3] = (byte)k;
            digest[j + 3] = k as u8;
            // Java: j += 4;
            j += 4;
        }
        // Java: this.init();
        self.init_state();
    }

    // Java: static void updateHash(int[] intArray, byte[] byteInput, int fromByte, int toByte)
    fn updateHash(intArray: &mut Vec<i32>, byteInput: &[u8], fromByte: i32, toByte: i32) {
        // Java: int index = intArray[81];
        let index = intArray[81];
        // Java: int i = fromByte;
        let mut i = fromByte;
        // Java: int wordIndex = index >> 2;
        let mut wordIndex = index >> 2;
        // Java: int byteIndex = index & 3;
        let mut byteIndex = index & 3;
        // Java: intArray[81] = index + toByte - fromByte + 1 & 63;
        intArray[81] = (index + toByte - fromByte + 1) & 63;
        // Java: if (byteIndex != 0)
        if byteIndex != 0 {
            // Java: while(i <= toByte && byteIndex < 4)
            while i <= toByte && byteIndex < 4 {
                // Java: intArray[wordIndex] |= (byteInput[i] & 255) << (3 - byteIndex << 3);
                intArray[wordIndex as usize] |= ((byteInput[i as usize] as i32) & 255) << ((3 - byteIndex) << 3);
                // Java: ++byteIndex;
                byteIndex += 1;
                // Java: ++i;
                i += 1;
            }
            // Java: if (byteIndex == 4)
            if byteIndex == 4 {
                // Java: ++wordIndex;
                wordIndex += 1;
                // Java: if (wordIndex == 16)
                if wordIndex == 16 {
                    // Java: computeHash(intArray);
                    Self::computeHash(intArray);
                    // Java: wordIndex = 0;
                    wordIndex = 0;
                }
            }
            // Java: if (i > toByte)
            if i > toByte {
                // Java: return;
                return;
            }
        }
        // Java: int maxWord = toByte - i + 1 >> 2;
        let maxWord = (toByte - i + 1) >> 2;
        // Java: for(int k = 0; k < maxWord; ++k)
        for _k in 0..maxWord {
            // Java: intArray[wordIndex] = (byteInput[i] & 255) << 24 | (byteInput[i + 1] & 255) << 16 | (byteInput[i + 2] & 255) << 8 | byteInput[i + 3] & 255;
            intArray[wordIndex as usize] =
                ((byteInput[i as usize] as i32 & 255) << 24) |
                ((byteInput[(i + 1) as usize] as i32 & 255) << 16) |
                ((byteInput[(i + 2) as usize] as i32 & 255) << 8) |
                (byteInput[(i + 3) as usize] as i32 & 255);
            // Java: i += 4;
            i += 4;
            // Java: ++wordIndex;
            wordIndex += 1;
            // Java: if (wordIndex >= 16)
            if wordIndex >= 16 {
                // Java: computeHash(intArray);
                Self::computeHash(intArray);
                // Java: wordIndex = 0;
                wordIndex = 0;
            }
        }
        // Java: int nBytes = toByte - i + 1;
        let nBytes = toByte - i + 1;
        // Java: if (nBytes != 0)
        if nBytes != 0 {
            // Java: int w = (byteInput[i] & 255) << 24;
            let mut w = ((byteInput[i as usize] as i32) & 255) << 24;
            // Java: if (nBytes != 1)
            if nBytes != 1 {
                // Java: w |= (byteInput[i + 1] & 255) << 16;
                w |= ((byteInput[(i + 1) as usize] as i32) & 255) << 16;
                // Java: if (nBytes != 2)
                if nBytes != 2 {
                    // Java: w |= (byteInput[i + 2] & 255) << 8;
                    w |= ((byteInput[(i + 2) as usize] as i32) & 255) << 8;
                }
            }
            // Java: intArray[wordIndex] = w;
            intArray[wordIndex as usize] = w;
        }
    }

    // Java: static void computeHash(int[] arrW)
    fn computeHash(arrW: &mut Vec<i32>) {
        // Java: int a = arrW[82];
        let mut a = arrW[82];
        // Java: int b = arrW[83];
        let mut b = arrW[83];
        // Java: int c = arrW[84];
        let mut c = arrW[84];
        // Java: int d = arrW[85];
        let mut d = arrW[85];
        // Java: int e = arrW[86];
        let mut e = arrW[86];
        // Java: for(int t = 16; t < 80; ++t)
        for t in 16..80 {
            // Java: int temp = arrW[t - 3] ^ arrW[t - 8] ^ arrW[t - 14] ^ arrW[t - 16];
            let temp = arrW[t - 3] ^ arrW[t - 8] ^ arrW[t - 14] ^ arrW[t - 16];
            // Java: arrW[t] = temp << 1 | temp >>> 31;
            arrW[t] = (temp << 1) | ((temp as u32) >> 31) as i32;
        }
        // Java: for(int t = 0; t < 20; ++t)
        for t in 0..20 {
            // Java: int temp = (a << 5 | a >>> 27) + (b & c | ~b & d) + e + arrW[t] + 1518500249;
            let temp = ((a << 5) | ((a as u32) >> 27) as i32)
                .wrapping_add((b & c) | ((!b) & d))
                .wrapping_add(e)
                .wrapping_add(arrW[t])
                .wrapping_add(1518500249);
            // Java: e = d;
            e = d;
            // Java: d = c;
            d = c;
            // Java: c = b << 30 | b >>> 2;
            c = (b << 30) | ((b as u32) >> 2) as i32;
            // Java: b = a;
            b = a;
            // Java: a = temp;
            a = temp;
        }
        // Java: for(int t = 20; t < 40; ++t)
        for t in 20..40 {
            // Java: int temp = (a << 5 | a >>> 27) + (b ^ c ^ d) + e + arrW[t] + 1859775393;
            let temp = ((a << 5) | ((a as u32) >> 27) as i32)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(e)
                .wrapping_add(arrW[t])
                .wrapping_add(1859775393);
            // Java: e = d;
            e = d;
            // Java: d = c;
            d = c;
            // Java: c = b << 30 | b >>> 2;
            c = (b << 30) | ((b as u32) >> 2) as i32;
            // Java: b = a;
            b = a;
            // Java: a = temp;
            a = temp;
        }
        // Java: for(int t = 40; t < 60; ++t)
        for t in 40..60 {
            // Java: int temp = (a << 5 | a >>> 27) + (b & c | b & d | c & d) + e + arrW[t] + -1894007588;
            let temp = ((a << 5) | ((a as u32) >> 27) as i32)
                .wrapping_add((b & c) | (b & d) | (c & d))
                .wrapping_add(e)
                .wrapping_add(arrW[t])
                .wrapping_add(-1894007588_i32);
            // Java: e = d;
            e = d;
            // Java: d = c;
            d = c;
            // Java: c = b << 30 | b >>> 2;
            c = (b << 30) | ((b as u32) >> 2) as i32;
            // Java: b = a;
            b = a;
            // Java: a = temp;
            a = temp;
        }
        // Java: for(int t = 60; t < 80; ++t)
        for t in 60..80 {
            // Java: int temp = (a << 5 | a >>> 27) + (b ^ c ^ d) + e + arrW[t] + -899497514;
            let temp = ((a << 5) | ((a as u32) >> 27) as i32)
                .wrapping_add(b ^ c ^ d)
                .wrapping_add(e)
                .wrapping_add(arrW[t])
                .wrapping_add(-899497514_i32);
            // Java: e = d;
            e = d;
            // Java: d = c;
            d = c;
            // Java: c = b << 30 | b >>> 2;
            c = (b << 30) | ((b as u32) >> 2) as i32;
            // Java: b = a;
            b = a;
            // Java: a = temp;
            a = temp;
        }
        // Java: arrW[82] += a;
        arrW[82] = arrW[82].wrapping_add(a);
        // Java: arrW[83] += b;
        arrW[83] = arrW[83].wrapping_add(b);
        // Java: arrW[84] += c;
        arrW[84] = arrW[84].wrapping_add(c);
        // Java: arrW[85] += d;
        arrW[85] = arrW[85].wrapping_add(d);
        // Java: arrW[86] += e;
        arrW[86] = arrW[86].wrapping_add(e);
    }
}

impl IDigest for DigestSHA1 {
    fn init(&mut self, _var1: &[u8], _var2: i32, _var3: i32) -> i32 {
        self.init_with_data(_var1, _var2, _var3)
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
