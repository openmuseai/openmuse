// Java: import java.util.Arrays
use super::IDigest::IDigest;
use super::DigestError;

// Java field: int DIGEST_LENGTH_MD2
pub const DIGEST_LENGTH_MD2: i32 = 16;
// Java field: int DIGEST_CBLOCK_MD2
const DIGEST_CBLOCK_MD2: usize = 16;
// Java field: int DIGEST_LBLOCK_MD2
const DIGEST_LBLOCK_MD2: usize = 16;
// Java field: int[] S
pub static S: &[i32] = &[41,46,67,201,162,216,124,1,61,54,84,161,236,240,6,19,98,167,5,243,192,199,115,140,152,147,43,217,188,76,130,202,30,155,87,60,253,212,224,22,103,66,111,24,138,23,229,18,190,78,196,214,218,158,222,73,160,251,245,142,187,47,238,122,169,104,121,145,21,178,7,63,148,194,16,137,11,34,95,33,128,127,93,154,90,144,50,39,53,62,204,231,191,247,151,3,255,25,48,179,72,165,181,209,215,94,146,42,172,86,170,198,79,184,56,210,150,164,125,182,118,252,107,226,156,116,4,241,69,157,112,89,100,113,135,32,134,91,207,101,230,45,168,2,27,96,37,173,174,176,185,246,28,70,97,105,52,64,126,15,85,71,163,35,221,81,175,58,195,92,249,206,186,197,234,38,44,83,13,110,133,40,132,9,211,223,205,244,65,129,77,82,106,220,55,200,108,193,171,250,36,225,123,8,12,189,177,74,120,136,149,139,227,99,232,109,233,203,213,254,59,0,29,57,242,239,183,14,102,88,208,228,166,119,114,248,235,117,75,10,49,68,80,180,143,237,31,26,219,153,141,51,159,17,131,20];

// Java: public class DigestMD2 implements IDigest
pub struct DigestMD2 {
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: int mLength
    mLength: i32,
    // Java field: int mDataLen
    mDataLen: i32,
    // Java field: byte[] mData
    mData: Vec<u8>,
    // Java field: byte[] mState
    mState: Vec<u8>,
    // Java field: byte[] mChksum
    mChksum: Vec<u8>,
}

// Java: public class DigestMD2 implements IDigest
impl DigestMD2 {
    // Java field: int[] S
    pub const S_TABLE: &[i32] = S;

    // Java: DigestMD2()
    pub fn new() -> Self {
        // Java: super();
        let mut s = DigestMD2 {
            mAlgorithm: 0,
            mLength: 0,
            mDataLen: 0,
            // Java: this.mData = new byte[DIGEST_CBLOCK_MD2];
            mData: vec![0u8; DIGEST_CBLOCK_MD2],
            // Java: this.mState = new byte[DIGEST_LBLOCK_MD2];
            mState: vec![0u8; DIGEST_LBLOCK_MD2],
            // Java: this.mChksum = new byte[DIGEST_LBLOCK_MD2];
            mChksum: vec![0u8; DIGEST_LBLOCK_MD2],
        };
        // Java: this.init();
        s.init_state();
        s
    }

    // Java: int get(byte[] buffer)
    pub fn get_digest(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: if (buffer == null)
        if buffer.is_empty() {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 0)
        else if self.mAlgorithm != 0 {
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
            // Java: for(int i = 0; i < DIGEST_CBLOCK_MD2; ++i)
            for i in 0..DIGEST_CBLOCK_MD2 {
                // Java: buffer[i] = (byte)(this.mState[i] & 255);
                buffer[i] = self.mState[i] & 255;
            }
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

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: this.init();
        self.init_state();
    }

    // Java: int update(byte[] data, int offset, int nLen)
    pub fn update_data(&mut self, data: &[u8], mut offset: i32, mut nLen: i32) -> i32 {
        // Java: if (data == null)
        if data.is_empty() && nLen > 0 {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 0)
        else if self.mAlgorithm != 0 {
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
            // Java: if (this.mDataLen > 0)
            if self.mDataLen > 0 {
                // Java: int n = DIGEST_CBLOCK_MD2 - this.mDataLen;
                let n = DIGEST_CBLOCK_MD2 as i32 - self.mDataLen;
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
            // Java: while(nLen >= DIGEST_CBLOCK_MD2)
            while nLen >= DIGEST_CBLOCK_MD2 as i32 {
                // Java: System.arraycopy(data, offset, this.mData, 0, DIGEST_CBLOCK_MD2);
                self.mData[..DIGEST_CBLOCK_MD2]
                    .copy_from_slice(&data[offset as usize..(offset as usize + DIGEST_CBLOCK_MD2)]);
                // Java: offset += DIGEST_CBLOCK_MD2;
                offset += DIGEST_CBLOCK_MD2 as i32;
                // Java: nLen -= DIGEST_CBLOCK_MD2;
                nLen -= DIGEST_CBLOCK_MD2 as i32;
                // Java: this.update();
                self.update_block();
            }
            // Java: System.arraycopy(data, offset, this.mData, 0, nLen);
            if nLen > 0 {
                self.mData[..nLen as usize]
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

    // Java: int raw(byte[] buffer)
    pub fn raw_impl(&mut self, _buffer: &mut [u8]) -> i32 {
        // Java: return DigestError.Digest_E_None;
        DigestError::DIGEST_E_NONE
    }

    // Java: int getLength()
    pub fn getLength(&self) -> i32 {
        // Java: return DIGEST_LENGTH_MD2;
        DIGEST_LENGTH_MD2
    }

    // Java: void init()
    fn init_state(&mut self) {
        // Java: this.mAlgorithm = 0;
        self.mAlgorithm = 0;
        // Java: this.mLength = DIGEST_LENGTH_MD2;
        self.mLength = DIGEST_LENGTH_MD2;
        // Java: this.mDataLen = 0;
        self.mDataLen = 0;
        // Java: Arrays.fill(this.mData, (byte)0);
        self.mData.fill(0);
        // Java: Arrays.fill(this.mState, (byte)0);
        self.mState.fill(0);
        // Java: Arrays.fill(this.mChksum, (byte)0);
        self.mChksum.fill(0);
    }

    // Java: void update()
    fn update_block(&mut self) {
        // Java: int[] state = new int[48];
        let mut state = vec![0i32; 48];
        // Java: int k = this.mChksum[DIGEST_LBLOCK_MD2 - 1];
        let mut k = self.mChksum[DIGEST_LBLOCK_MD2 - 1] as i32;
        // Java: for(int i = 0; i < 16; ++i)
        for i in 0..16usize {
            // Java: state[i + 0] = this.mState[i];
            state[i] = self.mState[i] as i32;
            // Java: int t;
            // Java: state[i + 16] = t = this.mData[i];
            let t = self.mData[i] as i32;
            state[i + 16] = t;
            // Java: state[i + 32] = t ^ this.mState[i];
            state[i + 32] = t ^ (self.mState[i] as i32);
            // Java: byte[] var10000 = this.mChksum;
            // Java: k = var10000[i] = (byte)(var10000[i] ^ S[t ^ k]);
            let val = (self.mChksum[i] as i32 ^ S[(t ^ k) as usize & 255]) as u8;
            self.mChksum[i] = val;
            k = val as i32;
        }
        // Java: int t = 0;
        let mut t: i32 = 0;
        // Java: for(int var5 = 0; var5 < 18; ++var5)
        for var5 in 0..18 {
            // Java: for(int var6 = 0; var6 < 48; var6 += 8)
            for var6 in (0..48).step_by(8) {
                // Java: t = state[var6 + 0] ^= S[t];
                state[var6] ^= S[t as usize & 255] as i32; t = state[var6];
                // Java: t = state[var6 + 1] ^= S[t];
                state[var6 + 1] ^= S[t as usize & 255] as i32; t = state[var6 + 1];
                // Java: t = state[var6 + 2] ^= S[t];
                state[var6 + 2] ^= S[t as usize & 255] as i32; t = state[var6 + 2];
                // Java: t = state[var6 + 3] ^= S[t];
                state[var6 + 3] ^= S[t as usize & 255] as i32; t = state[var6 + 3];
                // Java: t = state[var6 + 4] ^= S[t];
                state[var6 + 4] ^= S[t as usize & 255] as i32; t = state[var6 + 4];
                // Java: t = state[var6 + 5] ^= S[t];
                state[var6 + 5] ^= S[t as usize & 255] as i32; t = state[var6 + 5];
                // Java: t = state[var6 + 6] ^= S[t];
                state[var6 + 6] ^= S[t as usize & 255] as i32; t = state[var6 + 6];
                // Java: t = state[var6 + 7] ^= S[t];
                state[var6 + 7] ^= S[t as usize & 255] as i32; t = state[var6 + 7];
            }
            // Java: t = t + var5 & 255;
            t = (t + var5) & 255;
        }
        // Java: System.arraycopy(state, 0, this.mState, 0, DIGEST_LBLOCK_MD2);
        for i in 0..DIGEST_LBLOCK_MD2 {
            self.mState[i] = state[i] as u8;
        }
        // Java: int[] var15 = null;
    }

    // Java: void end()
    fn end(&mut self) {
        // Java: int n = DIGEST_CBLOCK_MD2 - this.mDataLen;
        let n = DIGEST_CBLOCK_MD2 as i32 - self.mDataLen;
        // Java: for(int i = this.mDataLen; i < DIGEST_CBLOCK_MD2; ++i)
        for i in (self.mDataLen as usize)..DIGEST_CBLOCK_MD2 {
            // Java: this.mData[i] = (byte)(n & 255);
            self.mData[i] = (n & 255) as u8;
        }
        // Java: this.update();
        self.update_block();
        // Java: for(int var3 = 0; var3 < DIGEST_CBLOCK_MD2; ++var3)
        for var3 in 0..DIGEST_CBLOCK_MD2 {
            // Java: this.mData[var3] = (byte)(this.mChksum[var3] & 255);
            self.mData[var3] = self.mChksum[var3] & 255;
        }
        // Java: this.update();
        self.update_block();
    }
}

impl IDigest for DigestMD2 {
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
