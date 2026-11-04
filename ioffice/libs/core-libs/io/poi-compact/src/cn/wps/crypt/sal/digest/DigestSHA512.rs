use super::IDigest::IDigest;
use super::DigestError;

// Java field: int DIGEST_LENGTH_SHA512
pub const DIGEST_LENGTH_SHA512: i32 = 64;
// Java field: long H0
const H0: i64 = 7640891576956012808_i64;
// Java field: long H1
const H1: i64 = -4942790177534073029_i64;
// Java field: long H2
const H2: i64 = 4354685564936845355_i64;
// Java field: long H3
const H3: i64 = -6534734903238641935_i64;
// Java field: long H4
const H4: i64 = 5840696475078001361_i64;
// Java field: long H5
const H5: i64 = -7276294671716946913_i64;
// Java field: long H6
const H6: i64 = 2270897969802886507_i64;
// Java field: long H7
const H7: i64 = 6620516959819538809_i64;
// Java field: int DIGEST_CBLOCK_SHA512
const DIGEST_CBLOCK_SHA512: usize = 128;
// Java field: int BYTES_OFFSET
const BYTES_OFFSET: usize = 81;
// Java field: int HASH_OFFSET
const HASH_OFFSET: usize = 82;
// Java field: long[] constants
const CONSTANTS: &[i64] = &[4794697086780616226_i64,8158064640168781261_i64,-5349999486874862801_i64,-1606136188198331460_i64,4131703408338449720_i64,6480981068601479193_i64,-7908458776815382629_i64,-6116909921290321640_i64,-2880145864133508542_i64,1334009975649890238_i64,2608012711638119052_i64,6128411473006802146_i64,8268148722764581231_i64,-9160688886553864527_i64,-7215885187991268811_i64,-4495734319001033068_i64,-1973867731355612462_i64,-1171420211273849373_i64,1135362057144423861_i64,2597628984639134821_i64,3308224258029322869_i64,5365058923640841347_i64,6679025012923562964_i64,8573033837759648693_i64,-7476448914759557205_i64,-6327057829258317296_i64,-5763719355590565569_i64,-4658551843659510044_i64,-4116276920077217854_i64,-3051310485924567259_i64,489312712824947311_i64,1452737877330783856_i64,2861767655752347644_i64,3322285676063803686_i64,5560940570517711597_i64,5996557281743188959_i64,7280758554555802590_i64,8532644243296465576_i64,-9096487096722542874_i64,-7894198246740708037_i64,-6719396339535248540_i64,-6333637450476146687_i64,-4446306890439682159_i64,-4076793802049405392_i64,-3345356375505022440_i64,-2983346525034927856_i64,-860691631967231958_i64,1182934255886127544_i64,1847814050463011016_i64,2177327727835720531_i64,2830643537854262169_i64,3796741975233480872_i64,4115178125766777443_i64,5681478168544905931_i64,6601373596472566643_i64,7507060721942968483_i64,8399075790359081724_i64,8693463985226723168_i64,-8878714635349349518_i64,-8302665154208450068_i64,-8016688836872298968_i64,-6606660893046293015_i64,-4685533653050689259_i64,-4147400797238176981_i64,-3880063495543823972_i64,-3348786107499101689_i64,-1523767162380948706_i64,-757361751448694408_i64,500013540394364858_i64,748580250866718886_i64,1242879168328830382_i64,1977374033974150939_i64,2944078676154940804_i64,3659926193048069267_i64,4368137639120453308_i64,4836135668995329356_i64,5532061633213252278_i64,6448918945643986474_i64,6902733635092675308_i64,7801388544844847127_i64];

// Java: public class DigestSHA512 implements IDigest
pub struct DigestSHA512 {
    // Java field: int mAlgorithm
    mAlgorithm: i32,
    // Java field: byte[] mBuffer
    mBuffer: Vec<u8>,
    // Java field: long[] mW
    mW: Vec<i64>,
    // Java field: long mMessageLength
    mMessageLength: i64,
}

// Java: public class DigestSHA512 implements IDigest
impl DigestSHA512 {
    // Java: DigestSHA512()
    pub fn new() -> Self {
        // Java: super();
        let mut s = DigestSHA512 {
            mAlgorithm: 0,
            mBuffer: vec![0u8; 128],
            mW: vec![0i64; 90],
            mMessageLength: 0,
        };
        // Java: this.init();
        s.init_state();
        s
    }

    // Java: int init(byte[] data, int offset, int nLen)
    pub fn init_with_data(&mut self, _data: &[u8], _offset: i32, _nLen: i32) -> i32 {
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
        // Java: else if (this.mAlgorithm != 6)
        else if self.mAlgorithm != 6 {
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
            // Java: this.updateHash(data, offset, nLen);
            self.updateHash(data, offset, nLen);
            // Java: this.mMessageLength += (long)nLen;
            self.mMessageLength += nLen as i64;
            // Java: return DigestError.Digest_E_None;
            return DigestError::DIGEST_E_NONE;
        }
    }

    // Java: int get(byte[] buffer)
    pub fn get_digest(&mut self, buffer: &mut [u8]) -> i32 {
        // Java: if (buffer == null)
        if buffer.is_empty() {
            // Java: return DigestError.Digest_E_Argument;
            return DigestError::DIGEST_E_ARGUMENT;
        }
        // Java: else if (this.mAlgorithm != 6)
        else if self.mAlgorithm != 6 {
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
        // Java: int result = DigestError.Digest_E_Unknown;
        let mut result = DigestError::DIGEST_E_UNKNOWN;
        // Java: this.init();
        self.init_state();
        // Java: result = this.update(data, offset, nLen);
        result = self.update_data(data, offset, nLen);
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
        // Java: return 64;
        64
    }

    // Java: void dispose()
    pub fn dispose_impl(&mut self) {
        // Java: this.init();
        self.init_state();
    }

    // Java: void init()
    fn init_state(&mut self) {
        // Java: this.mAlgorithm = 6;
        self.mAlgorithm = 6;
        // Java: this.mMessageLength = 0L;
        self.mMessageLength = 0;
        // Java: this.mBuffer = new byte[128];
        self.mBuffer = vec![0u8; 128];
        // Java: this.mW[81] = 0L;
        self.mW[81] = 0;
        // Java: this.mW[82] = 7640891576956012808L;
        self.mW[82] = 7640891576956012808_i64;
        // Java: this.mW[83] = -4942790177534073029L;
        self.mW[83] = -4942790177534073029_i64;
        // Java: this.mW[84] = 4354685564936845355L;
        self.mW[84] = 4354685564936845355_i64;
        // Java: this.mW[85] = -6534734903238641935L;
        self.mW[85] = -6534734903238641935_i64;
        // Java: this.mW[86] = 5840696475078001361L;
        self.mW[86] = 5840696475078001361_i64;
        // Java: this.mW[87] = -7276294671716946913L;
        self.mW[87] = -7276294671716946913_i64;
        // Java: this.mW[88] = 2270897969802886507L;
        self.mW[88] = 2270897969802886507_i64;
        // Java: this.mW[89] = 6620516959819538809L;
        self.mW[89] = 6620516959819538809_i64;
    }

    // Java: void updateHash(byte[] data, int offset, int nLen)
    fn updateHash(&mut self, data: &[u8], offset: i32, nLen: i32) {
        // Java: int mod = (int)(this.mMessageLength % 128L);
        let modv = (self.mMessageLength % 128) as usize;
        // Java: int nRest = 128 - mod;
        let nRest = 128 - modv;
        // Java: int i = 0;
        let mut i: usize = 0;
        // Java: if (nLen >= nRest)
        if (nLen as usize) >= nRest {
            // Java: System.arraycopy(data, offset, this.mBuffer, mod, nRest);
            self.mBuffer[modv..modv + nRest]
                .copy_from_slice(&data[offset as usize..offset as usize + nRest]);
            // Java: this.computeHash(this.mBuffer, 0);
            let buf_copy = self.mBuffer.clone();
            self.computeHash(&buf_copy, 0);
            // Java: for(i = nRest; i + 128 - 1 < nLen; i += 128)
            i = nRest;
            while i + 128 - 1 < nLen as usize {
                // Java: this.computeHash(data, offset + i);
                self.computeHash(data, (offset as usize + i) as i32);
                i += 128;
            }
            // Java: mod = 0;
        }
        // Java: if (i < nLen)
        if i < nLen as usize {
            // Java: System.arraycopy(data, offset + i, this.mBuffer, mod, nLen - i);
            let dest_start = if (nLen as usize) >= nRest { 0 } else { modv };
            let remaining = nLen as usize - i;
            self.mBuffer[dest_start..dest_start + remaining]
                .copy_from_slice(&data[offset as usize + i..offset as usize + i + remaining]);
        }
    }

    // Java: void computeHash(byte[] arrW, int offset)
    fn computeHash(&mut self, arrW: &[u8], offset: i32) {
        // Java: long a = this.mW[82];
        let mut a = self.mW[82];
        // Java: long b = this.mW[83];
        let mut b = self.mW[83];
        // Java: long c = this.mW[84];
        let mut c = self.mW[84];
        // Java: long d = this.mW[85];
        let mut d = self.mW[85];
        // Java: long e = this.mW[86];
        let mut e = self.mW[86];
        // Java: long f = this.mW[87];
        let mut f = self.mW[87];
        // Java: long g = this.mW[88];
        let mut g = self.mW[88];
        // Java: long h = this.mW[89];
        let mut h = self.mW[89];

        // Java: for(int i = 0; i < 16; ++i)
        let mut off = offset as usize;
        for _i in 0..16usize {
            // Java: this.mW[i] = (long)arrW[offset++] << 56 | ((long)arrW[offset++] & 255L) << 48 | ((long)arrW[offset++] & 255L) << 40 | ((long)arrW[offset++] & 255L) << 32 | ((long)arrW[offset++] & 255L) << 24 | ((long)arrW[offset++] & 255L) << 16 | ((long)arrW[offset++] & 255L) << 8 | (long)arrW[offset++] & 255L;
            self.mW[_i] =
                ((arrW[off] as i64) << 56) |
                ((arrW[off + 1] as i64 & 255) << 48) |
                ((arrW[off + 2] as i64 & 255) << 40) |
                ((arrW[off + 3] as i64 & 255) << 32) |
                ((arrW[off + 4] as i64 & 255) << 24) |
                ((arrW[off + 5] as i64 & 255) << 16) |
                ((arrW[off + 6] as i64 & 255) << 8) |
                (arrW[off + 7] as i64 & 255);
            off += 8;
        }

        // Java: for(int i = 16; i < 80; ++i)
        for i in 16..80usize {
            // Java: long temp1 = this.mW[i - 2];
            let temp1 = self.mW[i - 2];
            // Java: long temp2 = this.mW[i - 15];
            let temp2 = self.mW[i - 15];
            // Java: this.mW[i] = ((temp1 >>> 19 | temp1 << 45) ^ (temp1 >>> 61 | temp1 << 3) ^ temp1 >>> 6) + this.mW[i - 7] + ((temp2 >>> 1 | temp2 << 63) ^ (temp2 >>> 8 | temp2 << 56) ^ temp2 >>> 7) + this.mW[i - 16];
            let t1u = temp1 as u64;
            let t2u = temp2 as u64;
            let sigma1 = ((t1u >> 19 | t1u << 45) ^ (t1u >> 61 | t1u << 3) ^ (t1u >> 6)) as i64;
            let sigma0 = ((t2u >> 1 | t2u << 63) ^ (t2u >> 8 | t2u << 56) ^ (t2u >> 7)) as i64;
            self.mW[i] = sigma1.wrapping_add(self.mW[i - 7]).wrapping_add(sigma0).wrapping_add(self.mW[i - 16]);
        }

        // Java: for(int i = 0; i < 80; ++i)
        for i in 0..80usize {
            // Java: long temp1 = h + ((e >>> 14 | e << 50) ^ (e >>> 18 | e << 46) ^ (e >>> 41 | e << 23)) + (e & f ^ ~e & g) + constants[i] + this.mW[i];
            let eu = e as u64;
            let au = a as u64;
            let s1 = ((eu >> 14 | eu << 50) ^ (eu >> 18 | eu << 46) ^ (eu >> 41 | eu << 23)) as i64;
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(CONSTANTS[i]).wrapping_add(self.mW[i]);
            // Java: long temp2 = ((a >>> 28 | a << 36) ^ (a >>> 34 | a << 30) ^ (a >>> 39 | a << 25)) + (a & b ^ a & c ^ b & c);
            let s0 = ((au >> 28 | au << 36) ^ (au >> 34 | au << 30) ^ (au >> 39 | au << 25)) as i64;
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            // Java: h = g;
            h = g;
            // Java: g = f;
            g = f;
            // Java: f = e;
            f = e;
            // Java: e = d + temp1;
            e = d.wrapping_add(temp1);
            // Java: d = c;
            d = c;
            // Java: c = b;
            c = b;
            // Java: b = a;
            b = a;
            // Java: a = temp1 + temp2;
            a = temp1.wrapping_add(temp2);
        }

        // Java: long[] var10000 = this.mW;
        // Java: var10000[82] += a;
        self.mW[82] = self.mW[82].wrapping_add(a);
        // Java: var10000 = this.mW;
        // Java: var10000[83] += b;
        self.mW[83] = self.mW[83].wrapping_add(b);
        // Java: var10000 = this.mW;
        // Java: var10000[84] += c;
        self.mW[84] = self.mW[84].wrapping_add(c);
        // Java: var10000 = this.mW;
        // Java: var10000[85] += d;
        self.mW[85] = self.mW[85].wrapping_add(d);
        // Java: var10000 = this.mW;
        // Java: var10000[86] += e;
        self.mW[86] = self.mW[86].wrapping_add(e);
        // Java: var10000 = this.mW;
        // Java: var10000[87] += f;
        self.mW[87] = self.mW[87].wrapping_add(f);
        // Java: var10000 = this.mW;
        // Java: var10000[88] += g;
        self.mW[88] = self.mW[88].wrapping_add(g);
        // Java: var10000 = this.mW;
        // Java: var10000[89] += h;
        self.mW[89] = self.mW[89].wrapping_add(h);
    }

    // Java: void processDigest(byte[] buffer, int offset)
    fn processDigest(&mut self, buffer: &mut [u8], offset: i32) {
        // Java: byte[] paddingBuf = this.padBuffer();
        let paddingBuf = self.padBuffer();
        // Java: this.update(paddingBuf, 0, paddingBuf.length);
        let len = paddingBuf.len() as i32;
        self.update_data(&paddingBuf, 0, len);
        // Java: this.getResult(buffer, offset);
        self.getResult(buffer, offset as usize);
        // Java: this.init();
        self.init_state();
    }

    // Java: byte[] padBuffer()
    fn padBuffer(&self) -> Vec<u8> {
        // Java: int mod = (int)(this.mMessageLength % 128L);
        let modv = (self.mMessageLength % 128) as i32;
        // Java: int nPadding = mod < 112 ? 112 - mod : 240 - mod;
        let mut nPadding = if modv < 112 { 112 - modv } else { 240 - modv };
        // Java: byte[] result = new byte[nPadding + 16];
        let mut result = vec![0u8; (nPadding + 16) as usize];
        // Java: result[0] = -128;
        result[0] = 0x80u8;
        // Java: long bits = this.mMessageLength << 3;
        let bits = self.mMessageLength << 3;
        // Java: nPadding += 8;
        nPadding += 8;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 56));
        result[nPadding as usize] = ((bits as u64) >> 56) as u8; nPadding += 1;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 48));
        result[nPadding as usize] = ((bits as u64) >> 48) as u8; nPadding += 1;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 40));
        result[nPadding as usize] = ((bits as u64) >> 40) as u8; nPadding += 1;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 32));
        result[nPadding as usize] = ((bits as u64) >> 32) as u8; nPadding += 1;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 24));
        result[nPadding as usize] = ((bits as u64) >> 24) as u8; nPadding += 1;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 16));
        result[nPadding as usize] = ((bits as u64) >> 16) as u8; nPadding += 1;
        // Java: result[nPadding++] = (byte)((int)(bits >>> 8));
        result[nPadding as usize] = ((bits as u64) >> 8) as u8; nPadding += 1;
        // Java: result[nPadding] = (byte)((int)bits);
        result[nPadding as usize] = bits as u8;
        // Java: return result;
        result
    }

    // Java: void getResult(byte[] digest, int offset)
    fn getResult(&self, digest: &mut [u8], mut offset: usize) {
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 56));
        digest[offset] = ((self.mW[82] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 48));
        digest[offset] = ((self.mW[82] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 40));
        digest[offset] = ((self.mW[82] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 32));
        digest[offset] = ((self.mW[82] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 24));
        digest[offset] = ((self.mW[82] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 16));
        digest[offset] = ((self.mW[82] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[82] >>> 8));
        digest[offset] = ((self.mW[82] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[82]);
        digest[offset] = self.mW[82] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 56));
        digest[offset] = ((self.mW[83] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 48));
        digest[offset] = ((self.mW[83] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 40));
        digest[offset] = ((self.mW[83] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 32));
        digest[offset] = ((self.mW[83] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 24));
        digest[offset] = ((self.mW[83] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 16));
        digest[offset] = ((self.mW[83] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[83] >>> 8));
        digest[offset] = ((self.mW[83] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[83]);
        digest[offset] = self.mW[83] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 56));
        digest[offset] = ((self.mW[84] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 48));
        digest[offset] = ((self.mW[84] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 40));
        digest[offset] = ((self.mW[84] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 32));
        digest[offset] = ((self.mW[84] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 24));
        digest[offset] = ((self.mW[84] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 16));
        digest[offset] = ((self.mW[84] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[84] >>> 8));
        digest[offset] = ((self.mW[84] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[84]);
        digest[offset] = self.mW[84] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 56));
        digest[offset] = ((self.mW[85] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 48));
        digest[offset] = ((self.mW[85] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 40));
        digest[offset] = ((self.mW[85] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 32));
        digest[offset] = ((self.mW[85] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 24));
        digest[offset] = ((self.mW[85] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 16));
        digest[offset] = ((self.mW[85] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[85] >>> 8));
        digest[offset] = ((self.mW[85] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[85]);
        digest[offset] = self.mW[85] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 56));
        digest[offset] = ((self.mW[86] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 48));
        digest[offset] = ((self.mW[86] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 40));
        digest[offset] = ((self.mW[86] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 32));
        digest[offset] = ((self.mW[86] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 24));
        digest[offset] = ((self.mW[86] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 16));
        digest[offset] = ((self.mW[86] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[86] >>> 8));
        digest[offset] = ((self.mW[86] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[86]);
        digest[offset] = self.mW[86] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 56));
        digest[offset] = ((self.mW[87] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 48));
        digest[offset] = ((self.mW[87] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 40));
        digest[offset] = ((self.mW[87] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 32));
        digest[offset] = ((self.mW[87] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 24));
        digest[offset] = ((self.mW[87] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 16));
        digest[offset] = ((self.mW[87] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[87] >>> 8));
        digest[offset] = ((self.mW[87] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[87]);
        digest[offset] = self.mW[87] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 56));
        digest[offset] = ((self.mW[88] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 48));
        digest[offset] = ((self.mW[88] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 40));
        digest[offset] = ((self.mW[88] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 32));
        digest[offset] = ((self.mW[88] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 24));
        digest[offset] = ((self.mW[88] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 16));
        digest[offset] = ((self.mW[88] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[88] >>> 8));
        digest[offset] = ((self.mW[88] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[88]);
        digest[offset] = self.mW[88] as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 56));
        digest[offset] = ((self.mW[89] as u64) >> 56) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 48));
        digest[offset] = ((self.mW[89] as u64) >> 48) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 40));
        digest[offset] = ((self.mW[89] as u64) >> 40) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 32));
        digest[offset] = ((self.mW[89] as u64) >> 32) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 24));
        digest[offset] = ((self.mW[89] as u64) >> 24) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 16));
        digest[offset] = ((self.mW[89] as u64) >> 16) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)(this.mW[89] >>> 8));
        digest[offset] = ((self.mW[89] as u64) >> 8) as u8; offset += 1;
        // Java: digest[offset++] = (byte)((int)this.mW[89]);
        digest[offset] = self.mW[89] as u8;
    }
}

impl IDigest for DigestSHA512 {
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
